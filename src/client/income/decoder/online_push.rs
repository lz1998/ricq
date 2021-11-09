use bytes::{Buf, Bytes};
use jce_struct::Jce;
use crate::client::income::decoder::online_push::OnlinePushTrans::{MemberKicked, MemberLeave, MemberPermissionChanged};
use crate::client::outcome::PbToBytes;
use crate::client::structs::{FriendInfo, GroupMemberPermission};
use crate::jce;
use crate::pb;
use crate::pb::msg;
use crate::pb::msg::{PushMessagePacket, TransMsgInfo};
use crate::pb::notify::GeneralGrayTipInfo;

#[derive(Debug, Default)]
pub struct ReqPush {
    resp: ReqPushResp,
    push_infos: Vec<PushInfo>,
}

#[derive(Debug, Default)]
pub struct ReqPushResp {
    uin: i64,
    msg_infos: Vec<jce::PushMessageInfo>,
}

#[derive(Debug, Default)]
pub struct PushInfo {
    msg_seq: i16,
    msg_time: i64,
    msg_uid: i64,
    push_msg: PushMsg,
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
pub fn decode_online_push_req_packet(payload: &[u8]) -> Option<ReqPush> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: jce::RequestDataVersion2 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut req = data.map.remove("req").unwrap();
    let mut msg = req.remove("OnlinePushPack.SvcReqPushMsg").unwrap();
    let mut jr = Jce::new(&mut msg);
    let uin: i64 = jr.get_by_tag(0);
    let msg_infos: Vec<jce::PushMessageInfo> = jr.get_by_tag(2);

    let infos: Vec<PushInfo> = msg_infos.iter().map(|m| {
        let mut info = PushInfo {
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
    }).collect();
    Some(ReqPush {
        resp: ReqPushResp {
            uin,
            msg_infos,
        },
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
pub fn decode_online_push_trans_packet(payload: &[u8]) -> Option<OnlinePushTrans> {
    let trans_msg_info = TransMsgInfo::from_bytes(payload);
    if trans_msg_info.is_err() {
        return None;
    }
    let info = trans_msg_info.unwrap();
    let msg_uid = info.msg_uid.unwrap_or(0);
    let group_uin = info.from_uin?;
    let mut data = Bytes::from(info.msg_data.unwrap());
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
                    return Some(MemberLeave {
                        msg_uid,
                        group_uin,
                        member_uin: target,
                    });
                }
                0x03 | 0x83 => {
                    return Some(MemberKicked {
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
                let new_permission = if data.get_u8() == 1 { GroupMemberPermission::Administrator } else { GroupMemberPermission::Member };
                return Some(MemberPermissionChanged {
                    msg_uid,
                    group_uin,
                    member_uin: target,
                    new_permission,
                });
            }
        }
        _ => {}
    }
    None
}

#[derive(Debug)]
pub struct GroupMessagePart {
    seq: i32,
    rand: i32,
    group_code: i64,
    from_uin: i64,
    elems: Vec<msg::Elem>,
    time: i32,
    // 语音消息
    ptt: Option<msg::Ptt>,

    // 整个message有多少个part，大于elem.len()时，应等待下一个片段到达后合并
    pkg_num: i32,
    // 分片的第几段
    pkg_index: i32,
    // 分片id，相同id的应该合并，且根据pkg_index排序
    div_seq: i32,
}

// 解析群消息分片 TODO 长消息需要合并
pub fn decode_group_message_packet(payload: &[u8]) -> Option<GroupMessagePart> {
    let pkt = PushMessagePacket::from_bytes(payload);
    if pkt.is_err() {
        return None;
    }
    let message = pkt.unwrap().message.unwrap();

    return Some(GroupMessagePart {
        seq: message.head.as_ref().unwrap().msg_seq.unwrap(),
        rand: message.body.as_ref().unwrap().rich_text.as_ref().unwrap().attr.as_ref().unwrap().random.unwrap(),
        group_code: message.head.as_ref().unwrap().group_info.as_ref().unwrap().group_code.unwrap(),
        from_uin: message.head.as_ref().unwrap().from_uin.unwrap(),
        elems: message.body.as_ref().unwrap().rich_text.as_ref().unwrap().elems.clone(),
        time: message.head.as_ref().unwrap().msg_time.unwrap(),
        pkg_num: message.content.as_ref().unwrap().pkg_num.unwrap(),
        pkg_index: message.content.as_ref().unwrap().pkg_index.unwrap(),
        div_seq: message.content.as_ref().unwrap().div_seq.unwrap(),
        ptt: message.body.as_ref().unwrap().rich_text.as_ref().unwrap().ptt.clone(),
    });
}

pub struct FriendMessageRecalledEvent {
    friend_uin: i64,
    message_id: i32,
    time: i64,
}

pub fn msg_type_0x210_sub8a_decoder(uin: i64, protobuf: &[u8]) -> Option<Vec<FriendMessageRecalledEvent>> {
    let s8a = pb::Sub8A::from_bytes(protobuf);
    if s8a.is_err() {
        return None;
    }
    let s8a = s8a.unwrap();
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
    return if events.len() > 0 { Some(events) } else { None };
}

pub struct NewFriendEvent {
    friend: FriendInfo,
}

pub fn msg_type_0x210_subb3_decoder(protobuf: &[u8]) -> Option<NewFriendEvent> {
    let b3 = pb::SubB3::from_bytes(protobuf);
    if b3.is_err() {
        return None;
    }
    let b3 = b3.unwrap();
    let friend = FriendInfo {
        uin: b3.msg_add_frd_notify.as_ref().unwrap().uin,
        nick: b3.msg_add_frd_notify.unwrap().nick,
        ..Default::default()
    };
    Some(NewFriendEvent {
        friend
    })
}

#[derive(Debug, Default)]
pub struct GroupLeaveEvent {
    group_code: i64,
    operator: i64,
}

// return group number group leave
pub fn msg_type_0x210_subd4_decoder(protobuf: &[u8]) -> Option<GroupLeaveEvent> {
    let d4 = pb::SubD4::from_bytes(protobuf);
    if d4.is_err() {
        return None;
    }
    let d4 = d4.unwrap();
    Some(GroupLeaveEvent {
        group_code: d4.uin,
        ..Default::default()
    })
}

#[derive(Debug, Default)]
pub struct Sub0x27Event {
    group_name_updated_events: Vec<GroupNameUpdatedEvent>,
    del_friend_events: Vec<i64>,
}

#[derive(Debug, Default)]
pub struct GroupNameUpdatedEvent {
    group_code: i64,
    new_name: String,
    operator_uin: i64,
}

pub fn msg_type_0x210_sub27_decoder(protobuf: &[u8]) -> Option<Sub0x27Event> {
    let s27 = pb::msgtype0x210::SubMsg0x27Body::from_bytes(protobuf);
    if s27.is_err() {
        return None;
    }
    let s27 = s27.unwrap();
    let mut sub_0x27_event = Sub0x27Event::default();
    for m in s27.mod_infos {
        if let Some(ref profile) = m.mod_group_profile {
            for info in profile.group_profile_infos.iter() {
                if let Some(field) = info.field {
                    if field == 1 {
                        sub_0x27_event.group_name_updated_events.push(GroupNameUpdatedEvent {
                            group_code: profile.group_code.unwrap_or(0) as i64,
                            new_name: String::from_utf8_lossy(&info.value.as_ref().unwrap_or(&vec![])).to_string(),
                            operator_uin: profile.cmd_uin.unwrap_or(0) as i64,
                        });
                    }
                }
            }
        }
        if let Some(ref del_friend) = m.del_friend {
            sub_0x27_event.del_friend_events.append(&mut del_friend.uins.iter().map(|uin| *uin as i64).collect())
        }
    }
    Some(sub_0x27_event)
}

pub struct FriendPokeNotifyEvent {
    sender: i64,
    receiver: i64,
}

pub fn msg_type_0x210_sub122_decoder(protobuf: &[u8]) -> Option<FriendPokeNotifyEvent> {
    let t = GeneralGrayTipInfo::from_bytes(protobuf);
    if t.is_err() {
        return None;
    }
    let t = t.unwrap();
    let mut sender: i64 = 0;
    let mut receiver: i64 = 0;
    for templ in t.msg_templ_param {
        if templ.name == "uin_str1" {
            sender = templ.value.parse::<i64>().unwrap_or(0)
        } else if templ.name == "uin_str2" {
            receiver = templ.value.parse::<i64>().unwrap_or(0)
        }
    }
    return if sender == 0 {
        None
    } else {
        Some(FriendPokeNotifyEvent {
            sender,
            receiver,
        })
    };
}

// 需要同步群成员
pub struct GroupMemberNeedSync {
    group_code: i64,
}

pub fn msg_type_0x210_sub44_decoder(protobuf: &[u8]) -> Option<GroupMemberNeedSync> {
    let b44 = pb::Sub44::from_bytes(protobuf);
    if b44.is_err() {
        return None;
    }
    let b44 = b44.unwrap();
    if b44.group_sync_msg.is_none() {
        return None;
    }
    let group_code = b44.group_sync_msg?.grp_code;
    if group_code != 0 {
        return Some(GroupMemberNeedSync { group_code });
    }
    None
}