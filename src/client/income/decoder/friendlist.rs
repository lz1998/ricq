use crate::client::errors::RQError;
use crate::client::structs::*;
use crate::jce;
use crate::jce::TroopMemberInfo;
use bytes::{Buf, Bytes};
use jcers::Jce;

#[derive(Debug, Default)]
pub struct FriendListResponse {
    pub list: Vec<FriendInfo>,
    pub total_count: i16,
}

pub fn decode_friend_group_list_response(payload: &[u8]) -> Result<FriendListResponse, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket =
        jcers::from_buf(&mut payload).map_err(|e| RQError::from(e))?;
    let mut data: jce::RequestDataVersion3 =
        jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::from(e))?;
    let mut fl_resp = data.map.remove("FLRESP").ok_or(RQError::Decode(
        "decode_friend_group_list_response FLRESP not found".to_string(),
    ))?;
    fl_resp.advance(1);
    let mut r = Jce::new(&mut fl_resp);
    let total_friend_count: i16 = r.get_by_tag(5).map_err(|e| RQError::from(e))?;
    let friends: Vec<jce::FriendInfo> = r.get_by_tag(7).map_err(|e| RQError::from(e))?; // FIXME jce bug
    Ok(FriendListResponse {
        total_count: total_friend_count,
        list: friends
            .into_iter()
            .map(|f| FriendInfo {
                uin: f.friend_uin,
                nick: f.nick,
                remark: f.remark,
                face_id: f.face_id,
            })
            .collect(),
    })
}

#[derive(Debug)]
pub struct GroupListResponse {
    pub groups: Vec<GroupInfo>,
    pub vec_cookie: Bytes,
}

pub fn decode_group_list_response(payload: &[u8]) -> Result<GroupListResponse, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket =
        jcers::from_buf(&mut payload).map_err(|e| RQError::from(e))?;
    let mut data: jce::RequestDataVersion3 =
        jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::from(e))?;
    let mut fl_resp = data
        .map
        .remove("GetTroopListRespV2")
        .ok_or(RQError::Decode(
            "decode_group_list_response GetTroopListRespV2 not found".to_string(),
        ))?;
    fl_resp.advance(1);
    let mut r = Jce::new(&mut fl_resp);
    let vec_cookie: Bytes = r.get_by_tag(4).map_err(|e| RQError::from(e))?;
    let groups: Vec<jce::TroopNumber> = r.get_by_tag(5).map_err(|e| RQError::from(e))?;
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
            ..Default::default()
        })
    }
    Ok(GroupListResponse {
        groups: l,
        vec_cookie,
    })
}

#[derive(Debug)]
pub struct GroupMemberListResponse {
    pub next_uin: i64,
    pub list: Vec<GroupMemberInfo>,
}

pub fn decode_group_member_list_response(
    payload: &[u8],
) -> Result<GroupMemberListResponse, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket =
        jcers::from_buf(&mut payload).map_err(|e| RQError::from(e))?;
    let mut data: jce::RequestDataVersion3 =
        jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::from(e))?;
    let mut fl_resp = data.map.remove("GTMLRESP").ok_or(RQError::Decode(
        "decode_group_member_list_response GTMLRESP not found".to_string(),
    ))?;
    fl_resp.advance(1);
    let mut r = Jce::new(&mut fl_resp);
    let members: Vec<TroopMemberInfo> = r.get_by_tag(3).map_err(|e| RQError::from(e))?;
    let next_uin = r.get_by_tag(4).map_err(|e| RQError::from(e))?;
    let mut l: Vec<GroupMemberInfo> = Vec::new();
    for m in members {
        l.push(GroupMemberInfo {
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
            ..Default::default()
        })
    }
    Ok(GroupMemberListResponse { next_uin, list: l })
}
