use std::sync::Arc;
use std::time::Duration;

use bytes::{Buf, Bytes};
use cached::Cached;

use prost::Message;
use ricq_core::command::online_push::GroupMessagePart;
use ricq_core::command::online_push::{OnlinePushTrans, PushTransInfo};
use ricq_core::msg::MessageChain;
use ricq_core::structs::{
    DeleteFriend, FriendInfo, FriendMessageRecall, FriendPoke, GroupAudio, GroupAudioMessage,
    GroupLeave, GroupMessage, GroupMessageRecall, GroupMute, GroupNameUpdate, GroupPoke,
};
use ricq_core::{jce, pb};

use crate::client::event::{
    DeleteFriendEvent, FriendMessageRecallEvent, FriendPokeEvent, GroupAudioMessageEvent,
    GroupDisbandEvent, GroupLeaveEvent, GroupMessageEvent, GroupMessageRecallEvent, GroupMuteEvent,
    GroupNameUpdateEvent, GroupPokeEvent, MemberPermissionChangeEvent, NewFriendEvent,
};
use crate::client::handler::QEvent;
use crate::client::Client;
use crate::RQResult;

impl Client {
    pub(crate) async fn process_group_message_part(
        self: &Arc<Self>,
        group_message_part: GroupMessagePart,
    ) -> RQResult<()> {
        // receipt message
        if group_message_part.from_uin == self.uin().await {
            if let Some(tx) = self
                .receipt_waiters
                .lock()
                .await
                .cache_remove(&group_message_part.rand)
            {
                let _ = tx.send(group_message_part.seq);
                return Ok(());
            }
        }

        if let Some(ptt) = group_message_part.ptt {
            self.handler
                .handle(QEvent::GroupAudioMessage(GroupAudioMessageEvent {
                    client: self.clone(),
                    inner: GroupAudioMessage {
                        seqs: vec![group_message_part.seq],
                        rands: vec![group_message_part.rand],
                        group_code: group_message_part.group_code,
                        group_name: group_message_part.group_name,
                        group_card: group_message_part.group_card,
                        from_uin: group_message_part.from_uin,
                        time: group_message_part.time,
                        audio: GroupAudio(ptt),
                    },
                }))
                .await;
            return Ok(());
        }

        // merge parts
        let pkg_num = group_message_part.pkg_num;
        let group_msg = if pkg_num > 1 {
            let mut builder = self.group_message_builder.write().await;
            if builder.cache_misses().unwrap_or_default() > 100 {
                builder.flush();
                builder.cache_reset_metrics();
            }
            // muti-part
            let div_seq = group_message_part.div_seq;
            let parts = builder.cache_get_or_set_with(div_seq, Vec::new);
            parts.push(group_message_part);
            if parts.len() < pkg_num as usize {
                // wait for more parts
                None
            } else {
                Some(builder.cache_remove(&div_seq).unwrap_or_default())
            }
        } else {
            // single-part
            Some(vec![group_message_part])
        };

        // handle message
        if let Some(group_msg) = group_msg {
            // message is finish
            self.handler
                .handle(QEvent::GroupMessage(GroupMessageEvent {
                    client: self.clone(),
                    inner: self.parse_group_message(group_msg).await?,
                }))
                .await; //todo
        }
        Ok(())
    }

    pub(crate) async fn parse_group_message(
        &self,
        mut parts: Vec<GroupMessagePart>,
    ) -> RQResult<GroupMessage> {
        parts.sort_by(|a, b| a.pkg_index.cmp(&b.pkg_index));

        let group_code = parts.first().map(|p| p.group_code).unwrap_or_default();
        let group_name = parts
            .first_mut()
            .map(|p| std::mem::take(&mut p.group_name))
            .unwrap_or_default();
        let group_card = parts
            .first_mut()
            .map(|p| std::mem::take(&mut p.group_card))
            .unwrap_or_default();
        let from_uin = parts.first().map(|p| p.from_uin).unwrap_or_default();
        let time = parts.first().map(|p| p.time).unwrap_or_default();

        let mut seqs = Vec::with_capacity(parts.len());
        let mut rands = Vec::with_capacity(parts.len());
        let mut elements = Vec::with_capacity(6); // number by experience
        for p in parts {
            seqs.push(p.seq);
            rands.push(p.rand);
            elements.extend(p.elems.into_iter().filter_map(|e| e.elem));
        }
        // dbg!(elements.len()); // most of message will be 4, complex message like share card is 5

        Ok(GroupMessage {
            seqs,
            rands,
            group_code,
            group_name,
            group_card,
            from_uin,
            time,
            elements: MessageChain(elements),
        })

        // TODO: extInfo
        // TODO: group_card_update
        // TODO: ptt_store
    }

    pub(crate) async fn process_push_req(self: &Arc<Self>, msg_infos: Vec<jce::PushMessageInfo>) {
        for info in msg_infos {
            if self.push_req_exists(&info).await {
                continue;
            }
            match info.msg_type {
                732 => {
                    let mut r = info.v_msg;
                    let group_code = r.get_u32() as i64;
                    let i_type = r.get_u8();
                    r.get_u8();
                    match i_type {
                        0x0c => {
                            let operator = r.get_u32() as i64;
                            if operator == self.uin().await {
                                continue;
                            }
                            r.advance(6);
                            let target = r.get_u32() as i64;
                            let duration = Duration::from_secs(r.get_u32() as u64);
                            self.handler
                                .handle(QEvent::GroupMute(GroupMuteEvent {
                                    client: self.clone(),
                                    inner: GroupMute {
                                        group_code,
                                        operator_uin: operator,
                                        target_uin: target,
                                        duration,
                                    },
                                }))
                                .await;
                        }
                        0x10 | 0x11 | 0x14 | 0x15 => {
                            // group notify msg
                            r.advance(1);
                            let b = pb::notify::NotifyMsgBody::decode(&*r).unwrap();
                            if let Some(opt_msg_recall) = b.opt_msg_recall {
                                let operator_uin = opt_msg_recall.uin;
                                // use map iterator here will produce massive asm code
                                for rm in opt_msg_recall.recalled_msg_list {
                                    if rm.msg_type == 2 {
                                        continue;
                                    }
                                    self.handler
                                        .handle(QEvent::GroupMessageRecall(
                                            GroupMessageRecallEvent {
                                                client: self.clone(),
                                                inner: GroupMessageRecall {
                                                    msg_seq: rm.seq,
                                                    group_code,
                                                    operator_uin,
                                                    author_uin: rm.author_uin,
                                                    time: rm.time,
                                                },
                                            },
                                        ))
                                        .await;
                                }
                            }

                            if let Some(t) = b.opt_general_gray_tip {
                                let mut sender: i64 = 0;
                                let mut receiver: i64 = 0;
                                for templ in t.msg_templ_param {
                                    match &*templ.name {
                                        "uin_str1" => {
                                            sender = templ.value.parse().unwrap_or_default()
                                        }
                                        "uin_str2" => {
                                            receiver = templ.value.parse().unwrap_or_default()
                                        }
                                        _ => {}
                                    }
                                }
                                if sender != 0 {
                                    self.handler
                                        .handle(QEvent::GroupPoke(GroupPokeEvent {
                                            client: self.clone(),
                                            inner: GroupPoke {
                                                group_code,
                                                sender,
                                                receiver,
                                            },
                                        }))
                                        .await;
                                }
                            }
                            // TODO 一些没什么用的 event 暂时没写
                        }
                        _ => {}
                    }
                }
                528 => {
                    let mut v_msg = info.v_msg;
                    let msg: jce::MsgType0x210 = jcers::from_buf(&mut v_msg).unwrap();
                    match msg.sub_msg_type {
                        0x8A | 0x8B => {
                            let s8a = pb::Sub8A::decode(&*msg.v_protobuf).unwrap();
                            for m in s8a.msg_info {
                                self.handler
                                    .handle(QEvent::FriendMessageRecall(FriendMessageRecallEvent {
                                        client: self.clone(),
                                        inner: FriendMessageRecall {
                                            msg_seq: m.msg_seq,
                                            friend_uin: m.from_uin,
                                            time: m.msg_time,
                                        },
                                    }))
                                    .await;
                            }
                        }
                        0xB3 => {
                            let msg_add_frd_notify = pb::SubB3::decode(&*msg.v_protobuf).unwrap();
                            if let Some(f) = msg_add_frd_notify.msg_add_frd_notify {
                                self.handler
                                    .handle(QEvent::NewFriend(NewFriendEvent {
                                        client: self.clone(),
                                        inner: FriendInfo {
                                            uin: f.uin,
                                            nick: f.nick,
                                            ..Default::default()
                                        },
                                    }))
                                    .await;
                            }
                        }
                        0xD4 => {
                            let d4 = pb::SubD4::decode(&*msg.v_protobuf).unwrap();
                            self.handler
                                .handle(QEvent::GroupLeave(GroupLeaveEvent {
                                    client: self.clone(),
                                    inner: GroupLeave {
                                        group_code: d4.uin,
                                        member_uin: self.uin().await,
                                        operator_uin: None,
                                    },
                                }))
                                .await;
                        }
                        0x122 | 0x123 => {
                            let t =
                                pb::notify::GeneralGrayTipInfo::decode(&*msg.v_protobuf).unwrap();
                            let mut sender: i64 = 0;
                            let mut receiver: i64 = 0;
                            for templ in t.msg_templ_param {
                                if templ.name == "uin_str1" {
                                    sender = templ.value.parse().unwrap_or_default()
                                } else if templ.name == "uin_str2" {
                                    receiver = templ.value.parse().unwrap_or_default()
                                }
                            }
                            if sender != 0 {
                                self.handler
                                    .handle(QEvent::FriendPoke(FriendPokeEvent {
                                        client: self.clone(),
                                        inner: FriendPoke { sender, receiver },
                                    }))
                                    .await;
                            }
                        }
                        0x27 => {
                            let s27 =
                                pb::msgtype0x210::SubMsg0x27Body::decode(&*msg.v_protobuf).unwrap();
                            for mod_info in s27.mod_infos {
                                if let Some(mod_group_profile) = mod_info.mod_group_profile {
                                    for profile_info in mod_group_profile.group_profile_infos {
                                        if profile_info.field.unwrap_or_default() != 1 {
                                            continue;
                                        }
                                        self.handler
                                            .handle(QEvent::GroupNameUpdate(GroupNameUpdateEvent {
                                                client: self.clone(),
                                                inner: GroupNameUpdate {
                                                    group_code: mod_group_profile
                                                        .group_code
                                                        .unwrap_or_default()
                                                        as i64,
                                                    operator_uin: mod_group_profile
                                                        .cmd_uin
                                                        .unwrap_or_default()
                                                        as i64,
                                                    group_name: String::from_utf8_lossy(
                                                        profile_info.value(),
                                                    )
                                                    .into_owned(),
                                                },
                                            }))
                                            .await;
                                    }
                                }
                                if let Some(del_friend) = mod_info.del_friend {
                                    for uin in del_friend.uins {
                                        self.handler
                                            .handle(QEvent::DeleteFriend(DeleteFriendEvent {
                                                client: self.clone(),
                                                inner: DeleteFriend { uin: uin as i64 },
                                            }))
                                            .await;
                                    }
                                }
                            }
                        }
                        0x44 => {
                            // group sync
                            // friend sync
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    async fn push_req_exists(&self, info: &jce::PushMessageInfo) -> bool {
        let msg_time = info.msg_time as i32; // 可能是0，不过滤
        if msg_time != 0 && self.start_time > msg_time {
            return true;
        }
        let mut push_req_cache = self.push_req_cache.write().await;
        let key = (info.msg_seq, info.msg_uid);
        if push_req_cache.cache_get(&key).is_some() {
            return true;
        }
        push_req_cache.cache_set(key, ());
        if push_req_cache.cache_misses().unwrap_or_default() > 10 {
            push_req_cache.flush();
            push_req_cache.cache_reset_metrics();
        }
        false
    }

    pub(crate) async fn process_push_trans(self: &Arc<Self>, push_trans: OnlinePushTrans) {
        if self.push_trans_exists(&push_trans).await {
            return;
        }
        match push_trans.info {
            PushTransInfo::MemberLeave(leave) => {
                self.handler
                    .handle(QEvent::GroupLeave(GroupLeaveEvent {
                        client: self.clone(),
                        inner: leave,
                    }))
                    .await;
            }
            PushTransInfo::MemberPermissionChange(change) => {
                self.handler
                    .handle(QEvent::MemberPermissionChange(
                        MemberPermissionChangeEvent {
                            client: self.clone(),
                            inner: change,
                        },
                    ))
                    .await;
            }
            PushTransInfo::GroupDisband(disband) => {
                self.handler
                    .handle(QEvent::GroupDisband(GroupDisbandEvent {
                        client: self.clone(),
                        inner: disband,
                    }))
                    .await;
            }
        }
    }

    async fn push_trans_exists(&self, info: &OnlinePushTrans) -> bool {
        let msg_time = info.msg_time;
        if self.start_time > msg_time {
            return true;
        }
        let mut push_trans_cache = self.push_trans_cache.write().await;
        let key = (info.msg_seq, info.msg_uid);
        if push_trans_cache.cache_get(&key).is_some() {
            return true;
        }
        push_trans_cache.cache_set(key, ());
        if push_trans_cache.cache_misses().unwrap_or_default() > 10 {
            push_trans_cache.flush();
            push_trans_cache.cache_reset_metrics();
        }
        false
    }

    pub(crate) async fn process_c2c_sync(
        self: &Arc<Self>,
        pkt_seq: i32,
        push: pb::msg::PbPushMsg,
    ) -> RQResult<()> {
        let req = self.engine.read().await.build_delete_online_push_packet(
            self.uin().await,
            push.svrip(),
            Bytes::from(push.push_token.unwrap_or_default()),
            pkt_seq as u16,
            vec![],
        );
        let _ = self.send(req).await?;
        if let Some(msg) = push.msg {
            self.process_message_sync(vec![msg]).await;
        }
        Ok(())
    }

    pub(crate) async fn process_sid_ticket_expired(self: &Arc<Self>, seq: i32) -> RQResult<()> {
        self.request_change_sig(Some(3554528)).await?;
        self.register_client().await?;
        self.send_sid_ticket_expired_response(seq).await?;
        Ok(())
    }
}
