use bytes::{Buf, Bytes};
use jcers::Jce;

use crate::command::common::PbToBytes;
use crate::command::online_push::*;
use crate::{jce, pb, RQError, RQResult};

impl super::super::super::Engine {
    // 解析群消息分片 长消息需要合并
    // OnlinePush.PbPushGroupMsg
    pub fn decode_group_message_packet(&self, payload: Bytes) -> RQResult<GroupMessagePart> {
        let message = pb::msg::PushMessagePacket::from_bytes(&payload)
            .map_err(|_| RQError::Decode("PushMessagePacket".to_string()))?
            .message
            .ok_or(RQError::Decode("message is none".to_string()))?;

        let head = message
            .head
            .as_ref()
            .ok_or(RQError::Decode("head is none".to_string()))?;
        let body = message
            .body
            .as_ref()
            .ok_or(RQError::Decode("body is none".to_string()))?;
        let content = message
            .content
            .as_ref()
            .ok_or(RQError::Decode("content is none".to_string()))?;
        let rich_text = body
            .rich_text
            .as_ref()
            .ok_or(RQError::Decode("rich_text is none".to_string()))?;
        return Ok(GroupMessagePart {
            seq: head
                .msg_seq
                .ok_or(RQError::Decode("msg_seq is none".to_string()))?,
            rand: rich_text
                .attr
                .as_ref()
                .ok_or(RQError::Decode("attr is none".into()))?
                .random
                .ok_or(RQError::Decode("attr.random is none".into()))?,
            group_code: head
                .group_info
                .as_ref()
                .ok_or(RQError::Decode("group_info is none".into()))?
                .group_code
                .ok_or(RQError::Decode("group_info.group_code is none".into()))?,
            from_uin: head
                .from_uin
                .ok_or(RQError::Decode("from_uin is none".into()))?,
            elems: rich_text.elems.clone(),
            time: head
                .msg_time
                .ok_or(RQError::Decode("msg_time is none".into()))?,
            pkg_num: content
                .pkg_num
                .ok_or(RQError::Decode("pkg_num is none".into()))?,
            pkg_index: content
                .pkg_index
                .ok_or(RQError::Decode("pkg_index is none".into()))?,
            div_seq: content
                .div_seq
                .ok_or(RQError::Decode("div_seq is none".into()))?,
            ptt: rich_text.ptt.clone(),
        });
    }

    // todo decode_online_push_req_packet
    pub fn decode_online_push_req_packet(&self, mut payload: Bytes) -> RQResult<ReqPush> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut req = data
            .map
            .remove("req")
            .ok_or(RQError::Decode("req is none".into()))?;
        let mut msg = req
            .remove("OnlinePushPack.SvcReqPushMsg")
            .ok_or(RQError::Decode(
                "OnlinePushPack.SvcReqPushMsg is none".into(),
            ))?;
        let mut jr = Jce::new(&mut msg);
        let uin: i64 = jr.get_by_tag(0).map_err(RQError::from)?;
        let msg_infos: Vec<jce::PushMessageInfo> = jr.get_by_tag(2).map_err(RQError::from)?;

        let infos: Vec<PushInfo> = msg_infos
            .iter()
            .map(|m| {
                let info = PushInfo {
                    msg_seq: m.msg_seq,
                    msg_time: m.msg_time,
                    msg_uid: m.msg_uid,
                    ..Default::default()
                };
                match m.msg_type {
                    732 => {
                        let mut r = m.v_msg.clone();
                        let _group_code = r.get_i32() as i64;
                        let i_type = r.get_u8();
                        r.get_u8();
                        match i_type {
                            0x0c => {}
                            0x10 | 0x11 | 0x14 | 0x15 => {}
                            _ => {}
                        }
                    }
                    528 => {}
                    _ => {}
                }
                info
            })
            .collect();
        Ok(ReqPush {
            resp: ReqPushResp { uin, msg_infos },
            push_infos: infos,
            ..Default::default()
        })
    }

    // TODO 还没测试
    pub fn decode_online_push_trans_packet(&self, payload: Bytes) -> RQResult<OnlinePushTrans> {
        let info = pb::msg::TransMsgInfo::from_bytes(&payload)
            .map_err(|_| RQError::Decode("failed to decode TransMsgInfo".to_string()))?;
        let msg_uid = info.msg_uid.unwrap_or_default();
        let group_uin = info.from_uin.ok_or(RQError::Decode(
            "decode_online_push_trans_packet from_uin is 0".to_string(),
        ))?;
        let mut data = Bytes::from(
            info.msg_data
                .ok_or(RQError::Decode("msg_data is none".into()))?,
        );
        // 去重暂时不做
        match info.msg_type {
            Some(34) => {
                data.get_i32();
                data.get_u8();
                let target = data.get_i32() as i64;
                let typ = data.get_u8() as i32;
                let operator = data.get_i32() as i64;
                match typ {
                    0x02 | 0x82 => {
                        return Ok(OnlinePushTrans::MemberLeave {
                            msg_uid,
                            group_uin,
                            member_uin: target,
                        });
                    }
                    0x03 | 0x83 => {
                        return Ok(OnlinePushTrans::MemberKicked {
                            msg_uid,
                            group_uin,
                            member_uin: target,
                            operator_uin: operator,
                        });
                    }
                    _ => {}
                }
            }
            Some(44) => {
                data.advance(5);
                let var4 = data.get_u8() as i32;
                let mut var5: i64 = 0;
                let target = data.get_i32() as i64;
                if var4 != 0 && var4 != 1 {
                    var5 = data.get_i32() as i64;
                }
                if var5 == 0 && data.len() == 1 {
                    let new_permission = if data.get_u8() == 1 {
                        GroupMemberPermission::Administrator
                    } else {
                        GroupMemberPermission::Member
                    };
                    return Ok(OnlinePushTrans::MemberPermissionChanged {
                        msg_uid,
                        group_uin,
                        member_uin: target,
                        new_permission,
                    });
                }
            }
            _ => {}
        }
        Err(RQError::Decode(
            "decode_online_push_trans_packet unknown error".to_string(),
        ))
    }

    pub fn msg_type_0x210_sub8a_decoder(
        &self,
        uin: i64,
        protobuf: Bytes,
    ) -> RQResult<Vec<FriendMessageRecalledEvent>> {
        let s8a =
            pb::Sub8A::from_bytes(&protobuf).map_err(|_| RQError::Decode("Sub8A".to_string()))?;
        let mut events = Vec::new();
        for m in s8a.msg_info {
            if m.to_uin == uin {
                events.push(FriendMessageRecalledEvent {
                    friend_uin: m.from_uin,
                    message_id: m.msg_seq,
                    time: m.msg_time,
                })
            }
        }
        if !events.is_empty() {
            Ok(events)
        } else {
            Err(RQError::Decode("events length is 0".to_string()))
        }
    }

    pub fn msg_type_0x210_subb3_decoder(&self, protobuf: Bytes) -> RQResult<NewFriendEvent> {
        let msg_add_frd_notify = pb::SubB3::from_bytes(&protobuf)
            .map_err(|_| RQError::Decode("SubB3".to_string()))?
            .msg_add_frd_notify
            .ok_or(RQError::Decode("msg_add_frd_notify is none".to_string()))?;
        let friend = FriendInfo {
            uin: msg_add_frd_notify.uin,
            nick: msg_add_frd_notify.nick,
            ..Default::default()
        };
        Ok(NewFriendEvent { friend })
    }

    // return group number group leave
    pub fn msg_type_0x210_subd4_decoder(&self, protobuf: Bytes) -> RQResult<GroupLeaveEvent> {
        let d4 =
            pb::SubD4::from_bytes(&protobuf).map_err(|_| RQError::Decode("SubD4".to_string()))?;
        Ok(GroupLeaveEvent {
            group_code: d4.uin,
            ..Default::default()
        })
    }

    pub fn msg_type_0x210_sub27_decoder(&self, protobuf: Bytes) -> RQResult<Sub0x27Event> {
        let s27 = pb::msgtype0x210::SubMsg0x27Body::from_bytes(&protobuf)
            .map_err(|_| RQError::Decode("SubMsg0x27Body".to_string()))?;
        let mut sub_0x27_event = Sub0x27Event::default();
        for m in s27.mod_infos {
            if let Some(profile) = m.mod_group_profile {
                for info in profile.group_profile_infos.into_iter() {
                    if let Some(field) = info.field {
                        if field == 1 {
                            sub_0x27_event
                                .group_name_updated_events
                                .push(GroupNameUpdatedEvent {
                                    group_code: profile.group_code.unwrap_or_default() as i64,
                                    new_name: String::from_utf8_lossy(
                                        &info.value.unwrap_or_default(),
                                    )
                                    .to_string(),
                                    operator_uin: profile.cmd_uin.unwrap_or_default() as i64,
                                });
                        }
                    }
                }
            }
            if let Some(ref del_friend) = m.del_friend {
                sub_0x27_event
                    .del_friend_events
                    .append(&mut del_friend.uins.iter().map(|uin| *uin as i64).collect())
            }
        }
        Ok(sub_0x27_event)
    }

    pub fn msg_type_0x210_sub122_decoder(
        &self,
        protobuf: Bytes,
    ) -> RQResult<FriendPokeNotifyEvent> {
        let t = pb::notify::GeneralGrayTipInfo::from_bytes(&protobuf)
            .map_err(|_| RQError::Decode("GeneralGrayTipInfo".to_string()))?;
        let mut sender: i64 = 0;
        let mut receiver: i64 = 0;
        for templ in t.msg_templ_param {
            if templ.name == "uin_str1" {
                sender = templ.value.parse::<i64>().unwrap_or_default()
            } else if templ.name == "uin_str2" {
                receiver = templ.value.parse::<i64>().unwrap_or_default()
            }
        }
        if sender == 0 {
            Err(RQError::Decode(
                "msg_type_0x210_sub122_decoder sender is 0".to_string(),
            ))
        } else {
            Ok(FriendPokeNotifyEvent { sender, receiver })
        }
    }

    pub fn msg_type_0x210_sub44_decoder(&self, protobuf: Bytes) -> RQResult<GroupMemberNeedSync> {
        let b44 =
            pb::Sub44::from_bytes(&protobuf).map_err(|_| RQError::Decode("Sub44".to_string()))?;
        let group_code = b44
            .group_sync_msg
            .ok_or(RQError::Decode(
                "msg_type_0x210_sub44_decoder group_sync_msg is None".to_string(),
            ))?
            .grp_code;
        if group_code != 0 {
            return Ok(GroupMemberNeedSync { group_code });
        }
        Err(RQError::Decode(
            "msg_type_0x210_sub44_decoder unknown error".to_string(),
        ))
    }
}
