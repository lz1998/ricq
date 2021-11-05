use bytes::{Buf, Bytes};
use prost::DecodeError;
use crate::client::income::decoder::online_push::OnlinePushTrans::{MemberKicked, MemberLeave, MemberPermissionChanged};
use crate::client::outcome::PbToBytes;
use crate::client::structs::{GroupMemberPermission};
use crate::pb::msg::TransMsgInfo;

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