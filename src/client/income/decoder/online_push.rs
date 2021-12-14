use crate::client::errors::RQError;
use crate::client::income::decoder::online_push::OnlinePushTrans::{
    MemberKicked, MemberLeave, MemberPermissionChanged,
};
use crate::client::outcome::PbToBytes;
use crate::client::structs::{FriendInfo, GroupMemberPermission};
use crate::jce;
use crate::pb;
use crate::pb::msg;
use crate::pb::msg::{PushMessagePacket, TransMsgInfo};
use crate::pb::notify::GeneralGrayTipInfo;
use bytes::{Buf, Bytes};
use jcers::Jce;

#[derive(Debug, Default)]
pub struct ReqPush {
    pub resp: ReqPushResp,
    pub push_infos: Vec<PushInfo>,
}

#[derive(Debug, Default)]
pub struct ReqPushResp {
    pub uin: i64,
    pub msg_infos: Vec<jce::PushMessageInfo>,
}

#[derive(Debug, Default)]
pub struct PushInfo {
    pub msg_seq: i16,
    pub msg_time: i64,
    pub msg_uid: i64,
    pub push_msg: PushMsg,
}

#[derive(Debug)]
pub enum PushMsg {
    Unknown,
    T0x2dc {
        group_mute_events: Vec<GroupMuteEvent>,
        group_recalled_events: Vec<GroupMessageRecalledEvent>,
        group_red_bag_lucky_king_events: Vec<GroupRedBagLuckyKingNotifyEvent>,
        group_digest_events: Vec<GroupDigestEvent>,
    },
    T0x210 {},
}

impl Default for PushMsg {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Default)]
pub struct GroupMuteEvent {}

#[derive(Debug, Default)]
pub struct GroupMessageRecalledEvent {}

#[derive(Debug, Default)]
pub struct GroupRedBagLuckyKingNotifyEvent {}

#[derive(Debug, Default)]
pub struct GroupDigestEvent {}

// todo decode_online_push_req_packet
pub fn decode_online_push_req_packet(payload: &[u8]) -> Result<ReqPush, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket =
        jcers::from_buf(&mut payload).map_err(|e| RQError::from(e))?;
    let mut data: jce::RequestDataVersion2 =
        jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::from(e))?;
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
    let uin: i64 = jr.get_by_tag(0).map_err(|e| RQError::from(e))?;
    let msg_infos: Vec<jce::PushMessageInfo> = jr.get_by_tag(2).map_err(|e| RQError::from(e))?;

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
                    let group_code = r.get_i32() as i64;
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

pub enum OnlinePushTrans {
    MemberLeave {
        msg_uid: i64,
        // 和group_code不一样
        group_uin: i64,
        member_uin: i64,
    },
    MemberKicked {
        msg_uid: i64,
        // 和group_code不一样
        group_uin: i64,
        member_uin: i64,
        operator_uin: i64,
    },
    MemberPermissionChanged {
        msg_uid: i64,
        // 和group_code不一样
        group_uin: i64,
        member_uin: i64,
        new_permission: GroupMemberPermission,
    },
}

// TODO 还没测试
pub fn decode_online_push_trans_packet(payload: &[u8]) -> Result<OnlinePushTrans, RQError> {
    let info = TransMsgInfo::from_bytes(payload)
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
                    return Ok(MemberLeave {
                        msg_uid,
                        group_uin,
                        member_uin: target,
                    });
                }
                0x03 | 0x83 => {
                    return Ok(MemberKicked {
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
            let mut var5 = 0 as i64;
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
                return Ok(MemberPermissionChanged {
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GroupMessagePart {
    pub seq: i32,
    pub rand: i32,
    pub group_code: i64,
    pub from_uin: i64,
    pub elems: Vec<msg::Elem>,
    pub time: i32,
    // 语音消息
    pub ptt: Option<msg::Ptt>,

    // 整个message有多少个part，大于elem.len()时，应等待下一个片段到达后合并
    pub pkg_num: i32,
    // 分片的第几段
    pub pkg_index: i32,
    // 分片id，相同id的应该合并，且根据pkg_index排序
    pub div_seq: i32,
}

// 解析群消息分片 TODO 长消息需要合并
pub fn decode_group_message_packet(payload: &[u8]) -> Result<GroupMessagePart, RQError> {
    let message = PushMessagePacket::from_bytes(payload)
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

pub struct FriendMessageRecalledEvent {
    pub friend_uin: i64,
    pub message_id: i32,
    pub time: i64,
}

pub fn msg_type_0x210_sub8a_decoder(
    uin: i64,
    protobuf: &[u8],
) -> Result<Vec<FriendMessageRecalledEvent>, RQError> {
    let s8a = pb::Sub8A::from_bytes(protobuf).map_err(|_| RQError::Decode("Sub8A".to_string()))?;
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
    return if events.len() > 0 {
        Ok(events)
    } else {
        Err(RQError::Decode("events length is 0".to_string()))
    };
}

pub struct NewFriendEvent {
    pub friend: FriendInfo,
}

pub fn msg_type_0x210_subb3_decoder(protobuf: &[u8]) -> Result<NewFriendEvent, RQError> {
    let msg_add_frd_notify = pb::SubB3::from_bytes(protobuf)
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

#[derive(Debug, Default)]
pub struct GroupLeaveEvent {
    pub group_code: i64,
    pub operator: i64,
}

// return group number group leave
pub fn msg_type_0x210_subd4_decoder(protobuf: &[u8]) -> Result<GroupLeaveEvent, RQError> {
    let d4 = pb::SubD4::from_bytes(protobuf).map_err(|_| RQError::Decode("SubD4".to_string()))?;
    Ok(GroupLeaveEvent {
        group_code: d4.uin,
        ..Default::default()
    })
}

#[derive(Debug, Default)]
pub struct Sub0x27Event {
    pub group_name_updated_events: Vec<GroupNameUpdatedEvent>,
    pub del_friend_events: Vec<i64>,
}

#[derive(Debug, Default)]
pub struct GroupNameUpdatedEvent {
    pub group_code: i64,
    pub new_name: String,
    pub operator_uin: i64,
}

pub fn msg_type_0x210_sub27_decoder(protobuf: &[u8]) -> Result<Sub0x27Event, RQError> {
    let s27 = pb::msgtype0x210::SubMsg0x27Body::from_bytes(protobuf)
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
                                new_name: String::from_utf8_lossy(&info.value.unwrap_or_default())
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

pub struct FriendPokeNotifyEvent {
    pub sender: i64,
    pub receiver: i64,
}

pub fn msg_type_0x210_sub122_decoder(protobuf: &[u8]) -> Result<FriendPokeNotifyEvent, RQError> {
    let t = GeneralGrayTipInfo::from_bytes(protobuf)
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
    return if sender == 0 {
        Err(RQError::Decode(
            "msg_type_0x210_sub122_decoder sender is 0".to_string(),
        ))
    } else {
        Ok(FriendPokeNotifyEvent { sender, receiver })
    };
}

// 需要同步群成员
pub struct GroupMemberNeedSync {
    pub group_code: i64,
}

pub fn msg_type_0x210_sub44_decoder(protobuf: &[u8]) -> Result<GroupMemberNeedSync, RQError> {
    let b44 = pb::Sub44::from_bytes(protobuf).map_err(|_| RQError::Decode("Sub44".to_string()))?;
    let group_code = b44
        .group_sync_msg
        .ok_or(RQError::Decode(
            "msg_type_0x210_sub44_decoder group_sync_msg is None".to_string(),
        ))?
        .grp_code;
    if group_code != 0 {
        return Ok(GroupMemberNeedSync { group_code });
    }
    return Err(RQError::Decode(
        "msg_type_0x210_sub44_decoder unknown error".to_string(),
    ));
}
