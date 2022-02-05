use prost::Message;

use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

// TODO 待测试
impl super::super::super::Engine {
    // MessageSvc.PbSendMsg
    pub fn build_group_sending_packet(
        &self,
        group_code: i64,
        r: i32,
        pkg_num: i32,
        pkg_index: i32,
        pkg_div: i32,
        forward: bool,
        elems: Vec<pb::msg::Elem>,
    ) -> Packet {
        let sync_cookie = self.sync_cookie();
        let req = pb::msg::SendMessageRequest {
            routing_head: Some(pb::msg::RoutingHead {
                grp: Some(pb::msg::Grp {
                    group_code: Some(group_code),
                }),
                ..Default::default()
            }),
            content_head: Some(pb::msg::ContentHead {
                pkg_num: Some(pkg_num),
                pkg_index: Some(pkg_index),
                div_seq: Some(pkg_div),
                ..Default::default()
            }),
            msg_body: Some(pb::msg::MessageBody {
                rich_text: Some(pb::msg::RichText {
                    elems,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            msg_seq: Some(self.next_group_seq()),
            msg_rand: Some(r),
            sync_cookie: Some(sync_cookie),
            msg_via: Some(1),
            msg_ctrl: if forward {
                Some(pb::msg::MsgCtrl { msg_flag: Some(4) })
            } else {
                None
            },
            ..Default::default()
        };
        self.uni_packet("MessageSvc.PbSendMsg", req.to_bytes())
    }

    // build sync_cookie
    fn sync_cookie(&self) -> Vec<u8> {
        if !self.transport.sig.sync_cookie.is_empty() {
            return self.transport.sig.sync_cookie.to_vec();
        }
        let time = chrono::Utc::now().timestamp() as i64;
        pb::msg::SyncCookie {
            time1: Some(time),
            time: Some(time),
            last_sync_time: Some(time),
            ran1: Some(rand::random::<u32>() as i64),
            ran2: Some(rand::random::<u32>() as i64),
            const1: Some(self.transport.sig.sync_const1 as i64),
            const2: Some(self.transport.sig.sync_const2 as i64),
            const3: Some(self.transport.sig.sync_const3 as i64),
            ..Default::default()
        }
        .encode_to_vec()
    }

    // MessageSvc.PbGetMsg
    pub fn build_get_message_request_packet(&self, flag: i32) -> Packet {
        // start = 0, continue = 1, stop = 2
        let sync_cookie = self.sync_cookie();
        let req = pb::msg::GetMessageRequest {
            sync_flag: Some(flag),
            sync_cookie: Some(sync_cookie),
            latest_ramble_number: Some(20),
            other_ramble_number: Some(3),
            online_sync_flag: Some(1),
            context_flag: Some(1),
            msg_req_type: Some(1),
            pubaccount_cookie: Some(vec![]),
            msg_ctrl_buf: Some(vec![]),
            server_buf: Some(vec![]),
            ..Default::default()
        };
        self.uni_packet("MessageSvc.PbGetMsg", req.to_bytes())
    }

    // MessageSvc.PbDeleteMsg
    pub fn build_delete_message_request_packet(&self, items: Vec<pb::MessageItem>) -> Packet {
        let body = pb::DeleteMessageRequest { items }.to_bytes();
        self.uni_packet("MessageSvc.PbDeleteMsg", body)
    }

    // MessageSvc.PbSendMsg
    pub fn build_friend_sending_packet(
        &self,
        target: i64,
        r: i32,
        pkg_num: i32,
        pkg_index: i32,
        pkg_div: i32,
        elems: Vec<pb::msg::Elem>,
    ) -> Packet {
        let sync_cookie = self.sync_cookie();

        let req = pb::msg::SendMessageRequest {
            routing_head: Some(pb::msg::RoutingHead {
                c2c: Some(pb::msg::C2c {
                    to_uin: Some(target),
                }),
                ..Default::default()
            }),
            content_head: Some(pb::msg::ContentHead {
                pkg_num: Some(pkg_num),
                pkg_index: Some(pkg_index),
                div_seq: Some(pkg_div),
                ..Default::default()
            }),
            msg_body: Some(pb::msg::MessageBody {
                rich_text: Some(pb::msg::RichText {
                    elems,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            msg_seq: Some(self.next_friend_seq()),
            msg_rand: Some(r),
            sync_cookie: Some(sync_cookie),
            ..Default::default()
        };
        self.uni_packet("MessageSvc.PbSendMsg", req.to_bytes())
    }

    // MessageSvc.PbSendMsg
    pub fn build_temp_sending_packet(
        &self,
        group_uin: i64,
        user_uin: i64,
        r: i32,
        elems: Vec<pb::msg::Elem>,
    ) -> Packet {
        let sync_cookie = self.sync_cookie();
        let req = pb::msg::SendMessageRequest {
            routing_head: Some(pb::msg::RoutingHead {
                grp_tmp: Some(pb::msg::GrpTmp {
                    group_uin: Some(group_uin),
                    to_uin: Some(user_uin),
                }),
                ..Default::default()
            }),
            content_head: Some(pb::msg::ContentHead {
                pkg_num: Some(1),
                ..Default::default()
            }),
            msg_body: Some(pb::msg::MessageBody {
                rich_text: Some(pb::msg::RichText {
                    elems,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            msg_seq: Some(self.next_friend_seq()),
            msg_rand: Some(r),
            sync_cookie: Some(sync_cookie),
            ..Default::default()
        };
        self.uni_packet("MessageSvc.PbSendMsg", req.to_bytes())
    }

    // MessageSvc.PbGetGroupMsg
    pub fn build_get_group_msg_request(
        &self,
        group_code: i64,
        begin_seq: i64,
        end_seq: i64,
    ) -> Packet {
        let req = pb::msg::GetGroupMsgReq {
            group_code: Some(group_code as u64),
            begin_seq: Some(begin_seq as u64),
            end_seq: Some(end_seq as u64),
            public_group: Some(false),
            ..Default::default()
        };
        self.uni_packet("MessageSvc.PbGetGroupMsg", req.to_bytes())
    }

    pub fn build_private_recall_packet(&self, uin: i64, msg_seq: i32, random: i32) -> Packet {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let req = pb::msg::MsgWithDrawReq {
            c2c_with_draw: vec![pb::msg::C2cMsgWithDrawReq {
                msg_info: vec![pb::msg::C2cMsgInfo {
                    from_uin: Some(self.uin()),
                    to_uin: Some(uin),
                    msg_time: Some(time as i64),
                    msg_uid: Some(0x0100_0000_0000_0000),
                    msg_seq: Some(msg_seq),
                    msg_random: Some(random),
                    routing_head: Some(pb::msg::RoutingHead {
                        c2c: Some(pb::msg::C2c { to_uin: Some(uin) }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                long_message_flag: Some(0),
                reserved: Some(vec![0x08, 0x00]),
                sub_cmd: Some(1),
            }],
            ..Default::default()
        };
        self.uni_packet("PbMessageSvc.PbMsgWithDraw", req.to_bytes())
    }
}
