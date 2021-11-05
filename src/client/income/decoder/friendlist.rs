use bytes::{Bytes};
use jce_struct::Jce;
use crate::client::structs::*;
use crate::jce;

#[derive(Debug, Default)]
pub struct FriendListResponse {
    list: Vec<FriendInfo>,
    total_count: i16,
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