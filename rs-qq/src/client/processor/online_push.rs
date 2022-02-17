use std::sync::Arc;

use bytes::{Buf, Bytes};
use cached::Cached;
use futures::{stream, StreamExt};

use rq_engine::command::common::PbToBytes;
use rq_engine::command::online_push::{OnlinePushTrans, PushTransInfo};
use rq_engine::msg::MessageChain;
use rq_engine::pb::msg;
use rq_engine::structs::{
    DeleteFriend, FriendInfo, FriendMessageRecall, FriendPoke, GroupLeave, GroupMessage,
    GroupMessageRecall, GroupMute, GroupNameUpdate, NewMember,
};
use rq_engine::{jce, pb};

use crate::client::event::{
    DeleteFriendEvent, FriendMessageRecallEvent, FriendPokeEvent, GroupLeaveEvent,
    GroupMessageEvent, GroupMessageRecallEvent, GroupMuteEvent, GroupNameUpdateEvent,
    MemberPermissionChangeEvent, NewFriendEvent, NewMemberEvent,
};
use crate::client::handler::QEvent;
use crate::client::Client;
use crate::engine::command::online_push::GroupMessagePart;
use crate::{RQError, RQResult};

impl Client {
    pub(crate) async fn process_group_message_part(
        self: &Arc<Self>,
        group_message_part: GroupMessagePart,
    ) -> Result<(), RQError> {
        // self.mark_group_message_readed(group_message_part.group_code, group_message_part.seq).await;

        // receipt message
        if group_message_part.from_uin == self.uin().await {
            if let Some(tx) = self
                .receipt_waiters
                .lock()
                .await
                .remove(&group_message_part.rand)
            {
                let _ = tx.send(group_message_part.seq);
            }
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
                    message: self.parse_group_message(group_msg).await?,
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
        let group_message = GroupMessage {
            seqs: parts.iter().map(|p| p.seq).collect(),
            rands: parts.iter().map(|p| p.rand).collect(),
            group_code: parts.first().map(|p| p.group_code).unwrap_or_default(),
            from_uin: parts.first().map(|p| p.from_uin).unwrap_or_default(),
            time: parts.first().map(|p| p.time).unwrap_or_default(),
            elements: MessageChain::from(
                parts
                    .into_iter().flat_map(|p| p.elems)
                    .collect::<Vec<msg::Elem>>(),
            ),
        };
        //todo extInfo
        //todo group_card_update
        //todo ptt
        Ok(group_message)
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
                            let time = r.get_u32();
                            self.handler
                                .handle(QEvent::GroupMute(GroupMuteEvent {
                                    client: self.clone(),
                                    group_mute: GroupMute {
                                        group_code,
                                        operator_uin: operator,
                                        target_uin: target,
                                        time,
                                    },
                                }))
                                .await;
                        }
                        0x10 | 0x11 | 0x14 | 0x15 => {
                            // group notify msg
                            r.advance(1);
                            let b = pb::notify::NotifyMsgBody::from_bytes(&r).unwrap();
                            if let Some(opt_msg_recall) = b.opt_msg_recall {
                                let operator_uin = opt_msg_recall.uin;
                                let recalls: Vec<pb::notify::RecalledMessageMeta> = opt_msg_recall
                                    .recalled_msg_list
                                    .into_iter()
                                    .filter(|rm| rm.msg_type != 2)
                                    .collect();
                                stream::iter(recalls)
                                    .map(|rm| GroupMessageRecall {
                                        msg_seq: rm.seq,
                                        group_code,
                                        operator_uin,
                                        author_uin: rm.author_uin,
                                        time: rm.time,
                                    })
                                    .for_each(async move |recall| {
                                        self.handler
                                            .handle(QEvent::GroupMessageRecall(
                                                GroupMessageRecallEvent {
                                                    client: self.clone(),
                                                    recall,
                                                },
                                            ))
                                            .await;
                                    })
                                    .await;
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
                            let s8a = pb::Sub8A::from_bytes(&msg.v_protobuf).unwrap();
                            stream::iter(s8a.msg_info)
                                .map(|m| FriendMessageRecall {
                                    msg_seq: m.msg_seq,
                                    friend_uin: m.from_uin,
                                    time: m.msg_time,
                                })
                                .for_each(async move |m| {
                                    self.handler
                                        .handle(QEvent::FriendMessageRecall(
                                            FriendMessageRecallEvent {
                                                client: self.clone(),
                                                recall: m,
                                            },
                                        ))
                                        .await;
                                })
                                .await;
                        }
                        0xB3 => {
                            let msg_add_frd_notify =
                                pb::SubB3::from_bytes(&msg.v_protobuf).unwrap();
                            if let Some(f) = msg_add_frd_notify.msg_add_frd_notify {
                                self.handler
                                    .handle(QEvent::NewFriend(NewFriendEvent {
                                        client: self.clone(),
                                        friend: FriendInfo {
                                            uin: f.uin,
                                            nick: f.nick,
                                            ..Default::default()
                                        },
                                    }))
                                    .await;
                            }
                        }
                        0xD4 => {
                            let d4 = pb::SubD4::from_bytes(&msg.v_protobuf).unwrap();
                            self.handler
                                .handle(QEvent::GroupLeave(GroupLeaveEvent {
                                    client: self.clone(),
                                    leave: GroupLeave {
                                        group_code: d4.uin,
                                        member_uin: self.uin().await,
                                        operator_uin: None,
                                    },
                                }))
                                .await;
                        }
                        0x122 | 0x123 => {
                            let t = pb::notify::GeneralGrayTipInfo::from_bytes(&msg.v_protobuf)
                                .unwrap();
                            let mut sender: i64 = 0;
                            let mut receiver: i64 = 0;
                            for templ in t.msg_templ_param {
                                if templ.name == "uin_str1" {
                                    sender = templ.value.parse::<i64>().unwrap_or_default()
                                } else if templ.name == "uin_str2" {
                                    receiver = templ.value.parse::<i64>().unwrap_or_default()
                                }
                            }
                            if sender != 0 {
                                self.handler
                                    .handle(QEvent::FriendPoke(FriendPokeEvent {
                                        client: self.clone(),
                                        poke: FriendPoke { sender, receiver },
                                    }))
                                    .await;
                            }
                        }
                        0x27 => {
                            let s27 = pb::msgtype0x210::SubMsg0x27Body::from_bytes(&msg.v_protobuf)
                                .unwrap();
                            for mod_info in s27.mod_infos {
                                if let Some(mod_group_profile) = mod_info.mod_group_profile {
                                    for profile_info in mod_group_profile.group_profile_infos {
                                        if profile_info.field.unwrap_or_default() == 1 {
                                            let new_group_name =
                                                String::from_utf8_lossy(profile_info.value())
                                                    .to_string();
                                            let update = GroupNameUpdate {
                                                group_code: mod_group_profile
                                                    .group_code
                                                    .unwrap_or_default()
                                                    as i64,
                                                operator_uin: mod_group_profile
                                                    .cmd_uin
                                                    .unwrap_or_default()
                                                    as i64,
                                                group_name: new_group_name,
                                            };
                                            self.handler
                                                .handle(QEvent::GroupNameUpdate(
                                                    GroupNameUpdateEvent {
                                                        client: self.clone(),
                                                        update,
                                                    },
                                                ))
                                                .await;
                                        }
                                    }
                                }
                                if let Some(del_friend) = mod_info.del_friend {
                                    let delete_friends: Vec<DeleteFriend> = del_friend
                                        .uins
                                        .into_iter()
                                        .map(|uin| DeleteFriend { uin: uin as i64 })
                                        .collect();
                                    stream::iter(delete_friends)
                                        .for_each(async move |delete| {
                                            self.handler
                                                .handle(QEvent::DeleteFriend(DeleteFriendEvent {
                                                    client: self.clone(),
                                                    delete,
                                                }))
                                                .await;
                                        })
                                        .await;
                                }
                            }
                        }
                        0x44 => {
                            let b44 = pb::Sub44::from_bytes(&msg.v_protobuf).unwrap();
                            if let Some(group_sync_msg) = b44.group_sync_msg {
                                if let Some(group) =
                                    self.find_group(group_sync_msg.grp_code, true).await
                                {
                                    let last_join_time = group
                                        .members
                                        .read()
                                        .await
                                        .iter()
                                        .map(|m| m.join_time)
                                        .max()
                                        .unwrap_or_default();
                                    if let Ok(refreshed_members) =
                                        self.get_group_member_list(group.info.code).await
                                    {
                                        let mut members = group.members.write().await;
                                        members.clear();
                                        members.extend(refreshed_members);
                                    }
                                    let new_members: Vec<NewMember> = group
                                        .members
                                        .read()
                                        .await
                                        .iter()
                                        .filter(|m| m.join_time > last_join_time)
                                        .map(|m| NewMember {
                                            group_code: group.info.code,
                                            member_uin: m.uin,
                                        })
                                        .collect();
                                    stream::iter(new_members)
                                        .for_each(async move |new_member| {
                                            self.handler
                                                .handle(QEvent::NewMember(NewMemberEvent {
                                                    client: self.clone(),
                                                    new_member,
                                                }))
                                                .await;
                                        })
                                        .await;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    async fn push_req_exists(&self, info: &jce::PushMessageInfo) -> bool {
        let msg_time = info.msg_time as i32;
        if self.start_time > msg_time {
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
                        leave,
                    }))
                    .await;
            }
            PushTransInfo::MemberPermissionChange(change) => {
                self.handler
                    .handle(QEvent::MemberPermissionChange(
                        MemberPermissionChangeEvent {
                            client: self.clone(),
                            change,
                        },
                    ))
                    .await;
            }
        }
    }

    async fn push_trans_exists(&self, info: &OnlinePushTrans) -> bool {
        let msg_time = info.msg_time as i32;
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
}
