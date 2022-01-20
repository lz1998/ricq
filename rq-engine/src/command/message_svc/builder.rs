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
            sync_cookie: Some(Vec::new()),
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

    // MessageSvc.PbGetMsg
    pub fn build_get_message_request_packet(&self, flag: i32) -> Packet {
        // Strat = 0, continue = 1, stop = 2
        let mut cook = { self.transport.sig.sync_cookie.to_vec() };
        let time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if cook.is_empty() {
            cook = pb::msg::SyncCookie {
                time: Some(time as i64),
                time1: None,
                ran1: Some(758330138),
                ran2: Some(2480149246),
                const1: Some(1167238020),
                const2: Some(3913056418),
                const3: Some(0x1D),
                const4: None,
                last_sync_time: None,
            }
            .encode_to_vec();
        }
        let req = pb::msg::GetMessageRequest {
            sync_flag: Some(flag),
            sync_cookie: Some(cook),
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

    pub fn build_delete_message_request_packet(&self, msgs: Vec<pb::msg::Message>) -> Packet {
        let mut msg_items = vec![];
        msgs.into_iter().for_each(|msg| {
            let head = msg.head.unwrap();
            msg_items.push(pb::MessageItem {
                from_uin: head.from_uin.unwrap(),
                to_uin: head.to_uin.unwrap(),
                msg_type: head.msg_type.unwrap(),
                msg_seq: head.msg_seq.unwrap(),
                msg_uid: head.msg_uid.unwrap(),
                sig: vec![],
            })
        });
        let body = pb::DeleteMessageRequest { items: msg_items }.to_bytes();
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
        let mut cookie = { self.transport.sig.sync_cookie.to_vec() };
        let time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if cookie.is_empty() {
            cookie = pb::msg::SyncCookie {
                time1: None,
                time: Some(time as i64),
                ran1: Some(884121413),
                ran2: Some(852218874),
                const1: Some(390939176),
                const2: Some(315764147),
                const3: Some(0x1d),
                ..Default::default()
            }
            .encode_to_vec();
        }

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
            sync_cookie: Some(cookie),
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
}
