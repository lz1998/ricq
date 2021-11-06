use crate::pb;
use crate::client::outcome::PbToBytes;
use crate::client::structs::{GroupMemberInfo, GroupMemberPermission};

pub fn decode_group_member_info_response(payload: &[u8]) -> Option<GroupMemberInfo> {
    let resp = pb::GroupMemberRspBody::from_bytes(payload).unwrap();
    if resp.mem_info.is_none() {
        return None;
    }
    let mem_info = resp.mem_info.unwrap();
    Some(GroupMemberInfo {
        group_code: resp.group_code,
        uin: mem_info.uin,
        gender: mem_info.sex as u8,
        nickname: String::from_utf8(mem_info.nick.to_owned()).unwrap(),
        card_name: String::from_utf8(mem_info.card.to_owned()).unwrap(),
        level: mem_info.level as u16,
        join_time: mem_info.join,
        last_speak_time: mem_info.last_speak,
        special_title: String::from_utf8(mem_info.special_title.to_owned()).unwrap(),
        special_title_expire_time: mem_info.special_title_expire_time as i64,
        permission: match mem_info.role {
            3 => GroupMemberPermission::Owner,
            2 => GroupMemberPermission::Administrator,
            _ => GroupMemberPermission::Member,
        }, // TODO group owner
        ..Default::default()
    })
}