use bytes::{Bytes};
use jce_struct::Jce;
use crate::client::Client;
use crate::client::outcome::PbToBytes;
use crate::client::structs::*;
use crate::jce;
use crate::pb::GroupMemberRspBody;

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

pub async fn decode_group_member_info_response(cli: Client, payload: &[u8]) -> Option<GroupMemberInfo> {
    let resp = GroupMemberRspBody::from_bytes(payload).unwrap();
    if resp.mem_info.is_none() || (resp.mem_info.unwrap().age == 0) { // todo go => rsp.MemInfo.Nick == nil && rsp.MemInfo.Age == 0
        return None
    }
    fn get_permission(payload: &[u8], group: GroupInfo) -> GroupMemberPermission {
        let uin = GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.unwrap().uin;
        let role = GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.unwrap().role;
        return if uin == group.owner_uin { GroupMemberPermission::Owner }
        else if role == 2 { GroupMemberPermission::Administrator }
        else { GroupMemberPermission::Member }
    }
    Some(GroupMemberInfo {
        group: cli.find_group(GroupMemberRspBody::from_bytes(payload).unwrap().group_code).await.unwrap(),
        uin: GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.unwrap().uin,
        gender: GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.as_ref()?.sex as u8,
        nickname: String::from_utf8(GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.as_ref()?.nick.to_vec()).unwrap(),
        card_name: String::from_utf8(GroupMemberRspBody::from_bytes(payload).unwrap().mem_info?.card).unwrap(),
        level: GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.as_ref()?.level as u16,
        join_time: GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.as_ref()?.join,
        last_speak_time: GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.as_ref()?.last_speak,
        special_title: String::from_utf8(GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.as_ref()?.special_title.clone()).unwrap(),
        special_title_expire_time: GroupMemberRspBody::from_bytes(payload).unwrap().mem_info.as_ref()?.special_title_expire_time as i64,
        shut_up_timestamp: 0,
        permission: get_permission(payload, cli.find_group(resp.group_code).await.unwrap())
    })
}