use bytes::{Bytes};
use jce_struct::Jce;
use crate::client::structs::*;
use crate::jce;
use crate::jce::TroopMemberInfo;

#[derive(Debug, Default)]
pub struct FriendListResponse {
    pub list: Vec<FriendInfo>,
    pub total_count: i16,
}

pub fn decode_friend_group_list_response(payload: &[u8]) -> Option<FriendListResponse> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: jce::RequestDataVersion3 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut fl_resp = data.map.remove("FLRESP")?;
    let mut r = Jce::new(&mut fl_resp);
    let total_friend_count: i16 = r.get_by_tag(5);
    let friends: Vec<jce::FriendInfo> = r.get_by_tag(7); // FIXME jce bug
    Some(FriendListResponse {
        total_count: total_friend_count,
        list: friends.iter().map(|f| FriendInfo {
            uin: f.friend_uin,
            nick: f.nick.to_owned(),
            remark: f.remark.to_owned(),
            face_id: f.face_id,
        }).collect(),
    })
}



#[derive(Debug)]
pub struct GroupListResponse {
    pub groups: Vec<GroupInfo>,
    pub vec_cookie: Bytes,
}

pub fn decode_group_list_response(payload: &[u8]) -> Option<GroupListResponse> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: jce::RequestDataVersion3 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut fl_resp = data.map.remove("GetTroopListRespV2")?;
    let mut r = Jce::new(&mut fl_resp);
    let vec_cookie: Bytes = r.get_by_tag(4);
    let groups: Vec<jce::TroopNumber> = r.get_by_tag(5);
    let mut l: Vec<GroupInfo> = Vec::new();
    for g in groups {
        l.push(GroupInfo {
            uin: g.group_uin,
            code: g.group_code,
            name: g.group_name,
            memo: g.group_memo,
            owner_uin: g.group_owner_uin,
            member_count: g.member_num as u16,
            max_member_count: g.max_group_member_num as u16,
            group_create_time: 0,
            group_level: 0,
            members: vec![],
            last_msg_seq: 0,
        })
    }
    Some(GroupListResponse {
        groups: l,
        vec_cookie,
    })
}


#[derive(Debug)]
pub struct GroupMemberListResponse {
    pub next_uin: i64,
    pub list: Vec<GroupMemberInfo>,
}

pub fn decode_group_member_list_response(payload: &[u8]) -> Option<GroupMemberListResponse> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: jce::RequestDataVersion3 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut fl_resp = data.map.remove("GTMLRESP")?;
    let mut r = Jce::new(&mut fl_resp);
    let members: Vec<TroopMemberInfo> = r.get_by_tag(3);
    let next_uin = r.get_by_tag(4);
    let mut l: Vec<GroupMemberInfo> = Vec::new();
    for m in members {
        l.push(GroupMemberInfo {
            group_code: 0,
            uin: m.member_uin,
            gender: m.gender,
            nickname: m.nick,
            card_name: m.name,
            level: m.member_level as u16,
            join_time: m.join_time,
            last_speak_time: m.last_speak_time,
            special_title: m.special_title,
            special_title_expire_time: m.special_title_expire_time,
            shut_up_timestamp: m.shut_up_timestap,
            permission: match m.flag {
                1 => GroupMemberPermission::Administrator,
                _ => GroupMemberPermission::Member,
            },
        })
    }
    Some(GroupMemberListResponse {
        next_uin,
        list: l,
    })
}