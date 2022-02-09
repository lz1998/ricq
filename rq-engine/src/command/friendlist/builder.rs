use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};
use jcers::JcePut;

use crate::command::common::pack_uni_request_data;
use crate::common::group_code2uin;
use crate::protocol::packet::*;
use crate::{jce, pb};

impl super::super::super::Engine {
    // friendlist.getFriendGroupList
    pub fn build_friend_group_list_request_packet(
        &self,
        friend_start_index: i16,
        friend_list_count: i16,
        group_start_index: i16,
        group_list_count: i16,
    ) -> Packet {
        let mut d50 = BytesMut::new();
        prost::Message::encode(
            &pb::D50ReqBody {
                appid: 1002,
                req_music_switch: 1,
                req_mutualmark_alienation: 1,
                req_ksing_switch: 1,
                req_mutualmark_lbsshare: 1,
                ..Default::default()
            },
            &mut d50,
        )
        .unwrap();

        let req = jce::FriendListRequest {
            reqtype: 3,
            if_reflush: if friend_start_index <= 0 { 0 } else { 1 },
            uin: self.uin(),
            start_index: friend_start_index,
            friend_count: friend_list_count,
            group_id: 0,
            if_get_group_info: if group_list_count <= 0 { 0 } else { 1 },
            group_start_index: group_start_index as u8,
            group_count: group_list_count as u8,
            if_get_msf_group: 0,
            if_show_term_type: 1,
            version: 27,
            uin_list: vec![],
            app_type: 0,
            if_get_dov_id: 0,
            if_get_both_flag: 0,
            d50: Bytes::from(d50),
            d6b: Bytes::new(),
            sns_type_list: vec![13580, 13581, 13582],
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([("FL".to_string(), pack_uni_request_data(&req.freeze()))]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            c_packet_type: 0x003,
            i_request_id: 1921334514,
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetFriendListReq".to_string(),
            s_buffer: buf.freeze(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        };
        self.uni_packet("friendlist.getFriendGroupList", pkt.freeze())
    }

    // friendlist.GetTroopListReqV2
    pub fn build_group_list_request_packet(&self, vec_cookie: &[u8]) -> Packet {
        let req = jce::TroopListRequest {
            uin: self.uin(),
            get_msf_msg_flag: 1,
            cookies: Bytes::from(vec_cookie.to_vec()),
            group_info: vec![],
            group_flag_ext: 1,
            version: 7,
            company_id: 0,
            version_num: 1,
            get_long_group_name: 1,
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([(
                "GetTroopListReqV2Simplify".to_string(),
                pack_uni_request_data(&req.freeze()),
            )]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            c_packet_type: 0x00,
            i_message_type: 0,
            i_request_id: self.next_packet_seq(),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetTroopListReqV2Simplify".to_string(),
            s_buffer: buf.freeze(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        };
        self.uni_packet("friendlist.GetTroopListReqV2", pkt.freeze())
    }

    // friendlist.GetTroopMemberListReq
    pub fn build_group_member_list_request_packet(&self, group_code: i64, next_uin: i64) -> Packet {
        let payload = jce::TroopMemberListRequest {
            uin: self.uin(),
            group_code,
            next_uin,
            group_uin: group_code2uin(group_code),
            version: 2,
            ..Default::default()
        };
        let mut b = BytesMut::new();
        b.put_slice(&[0x0A]);
        b.put_slice(&payload.freeze());
        b.put_slice(&[0x0B]);
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([("GTML".to_string(), b.into())]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            i_request_id: self.next_packet_seq(),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetTroopMemberListReq".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("friendlist.GetTroopMemberListReq", pkt.freeze())
    }

    // friendlist.ModifyGroupCardReq
    pub fn build_edit_group_tag_packet(
        &self,
        group_code: i64,
        member_uin: i64,
        new_tag: String,
    ) -> Packet {
        let payload = jce::ModifyGroupCardRequest {
            group_code,
            uin_info: vec![jce::UinInfo {
                uin: member_uin,
                flag: 31,
                name: new_tag,
                ..Default::default()
            }],
            ..Default::default()
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([(
                "MGCREQ".to_string(),
                pack_uni_request_data(&payload.freeze()),
            )]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            i_request_id: self.next_packet_seq(),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "ModifyGroupCardReq".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("friendlist.ModifyGroupCardReq", pkt.freeze())
    }

    // friendlist.DelFriend
    pub fn build_delete_friend_packet(&self, del_uin: i64) -> Packet {
        let payload = jce::DelFriendReq {
            uin: self.uin(),
            del_uin,
            del_type: 2,
            version: 1,
        };

        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([("DF".to_string(), pack_uni_request_data(&payload.freeze()))]),
        };

        let pkt = jce::RequestPacket {
            i_version: 3,
            i_request_id: self.next_packet_seq(),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "DelFriendReq".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };

        self.uni_packet("friendlist.delFriend", pkt.freeze())
    }
}
