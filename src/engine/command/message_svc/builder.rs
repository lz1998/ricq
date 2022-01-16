use prost::Message;

use crate::engine::command::common::PbToBytes;
use crate::engine::protocol::packet::Packet;
use crate::pb;

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
                c2c: None,
                grp: Some(pb::msg::Grp {
                    group_code: Some(group_code),
                }),
                grp_tmp: None,
                wpa_tmp: None,
            }),
            content_head: Some(pb::msg::ContentHead {
                pkg_num: Some(pkg_num),
                pkg_index: Some(pkg_index),
                div_seq: Some(pkg_div),
                auto_reply: None,
            }),
            msg_body: Some(pb::msg::MessageBody {
                rich_text: Some(pb::msg::RichText {
                    elems,
                    attr: None,
                    not_online_file: None,
                    ptt: None,
                }),
                msg_content: None,
                msg_encrypt_content: None,
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
            data_statist: None,
            multi_send_seq: None,
        };
        self.uni_packet("MessageSvc.PbSendMsg", req.to_bytes())
    }

    // MessageSvc.PbGetMsg
    pub fn build_get_message_request_packet(&self, flag: i32, time: i64) -> Packet {
        let mut cook = { self.transport.sig.sync_cookie.to_vec() };
        if cook.is_empty() {
            cook = pb::msg::SyncCookie {
                time: Some(time),
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
}
