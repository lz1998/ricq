use bytes::{Buf, Bytes};
use jcers::Jce;

use crate::command::friendlist::*;
use crate::structs::{FriendInfo, GroupInfo, GroupMemberInfo, GroupMemberPermission};
use crate::{jce, RQError, RQResult};

impl super::super::super::Engine {
    // friendlist.getFriendGroupList
    pub fn decode_friend_group_list_response(
        &self,
        mut payload: Bytes,
    ) -> RQResult<FriendListResponse> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion3 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut fl_resp = data.map.remove("FLRESP").ok_or_else(|| {
            RQError::Decode("decode_friend_group_list_response FLRESP not found".into())
        })?;
        fl_resp.advance(1);
        let mut r = Jce::new(&mut fl_resp);
        let total_friend_count: i16 = r.get_by_tag(5).map_err(RQError::from)?;
        let friends: Vec<jce::FriendInfo> = r.get_by_tag(7).map_err(RQError::from)?; // FIXME jce bug
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

    // friendlist.GetTroopListReqV2
    pub fn decode_group_list_response(&self, mut payload: Bytes) -> RQResult<GroupListResponse> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion3 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut fl_resp = data.map.remove("GetTroopListRespV2").ok_or_else(|| {
            RQError::Decode("decode_group_list_response GetTroopListRespV2 not found".into())
        })?;
        fl_resp.advance(1);
        let mut r = Jce::new(&mut fl_resp);
        let vec_cookie: Bytes = r.get_by_tag(4).map_err(RQError::from)?;
        let groups: Vec<jce::TroopNumber> = r.get_by_tag(5).map_err(RQError::from)?;
        let l = groups
            .into_iter()
            .map(|g| GroupInfo {
                uin: g.group_uin,
                code: g.group_code,
                name: g.group_name,
                memo: g.group_memo,
                owner_uin: g.group_owner_uin,
                member_count: g.member_num as u16,
                max_member_count: g.max_group_member_num as u16,
                shut_up_timestamp: g.shut_up_timestamp,
                my_shut_up_timestamp: g.my_shut_up_timestamp,
                ..Default::default()
            })
            .collect();
        Ok(GroupListResponse {
            groups: l,
            vec_cookie,
        })
    }

    // friendlist.GetTroopMemberListReq
    pub fn decode_group_member_list_response(
        &self,
        mut payload: Bytes,
    ) -> RQResult<GroupMemberListResponse> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion3 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut fl_resp = data.map.remove("GTMLRESP").ok_or_else(|| {
            RQError::Decode("decode_group_member_list_response GTMLRESP not found".into())
        })?;
        fl_resp.advance(1);
        let mut r = Jce::new(&mut fl_resp);
        let members: Vec<jce::TroopMemberInfo> = r.get_by_tag(3).map_err(RQError::from)?;
        let next_uin = r.get_by_tag(4).map_err(RQError::from)?;
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

    //friendlist.delFriend
    pub fn decode_remove_friend(&self, mut payload: Bytes) -> RQResult<jce::DelFriendResp> {
        let mut req: jce::RequestPacket = jcers::from_buf(&mut payload)?;

        let mut data: jce::RequestDataVersion3 = jcers::from_buf(&mut req.s_buffer)?;

        let mut r = data
            .map
            .remove("DFRESP")
            .ok_or_else(|| RQError::Decode("decode_remove_friend `DFRESP` not found".into()))?;
        jcers::from_buf(&mut r).map_err(RQError::Jce)
    }
}
