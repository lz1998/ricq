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
        elems: Vec<pb::msg::Elem>,
        ptt: Option<pb::msg::Ptt>,
        ran: i32,
        time: i64,
        pkg_num: i32,
        pkg_index: i32,
        pkg_div: i32,
        forward: bool,
    ) -> Packet {
        let sync_cookie = self.sync_cookie(time);
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
                    ptt,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            msg_seq: Some(self.next_group_seq()),
            msg_rand: Some(ran),
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
    fn sync_cookie(&self, time: i64) -> Vec<u8> {
        if !self.transport.sig.sync_cookie.is_empty() {
            return self.transport.sig.sync_cookie.to_vec();
        }
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
    pub fn build_get_message_request_packet(&self, flag: i32, time: i64) -> Packet {
        // start = 0, continue = 1, stop = 2
        let sync_cookie = self.sync_cookie(time);
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
        elems: Vec<pb::msg::Elem>,
        ptt: Option<pb::msg::Ptt>,
        seq: i32,
        ran: i32,
        time: i64,
        pkg_num: i32,
        pkg_index: i32,
        pkg_div: i32,
    ) -> Packet {
        let sync_cookie = self.sync_cookie(time);

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
                    ptt,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            msg_seq: Some(seq),
            msg_rand: Some(ran),
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
        elems: Vec<pb::msg::Elem>,
        seq: i32,
        ran: i32,
        time: i64,
    ) -> Packet {
        let sync_cookie = self.sync_cookie(time);
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
            msg_seq: Some(seq),
            msg_rand: Some(ran),
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

    pub fn build_friend_recall_packet(
        &self,
        uin: i64,
        msg_time: i64,
        seqs: Vec<i32>,
        rands: Vec<i32>,
    ) -> Packet {
        let req = pb::msg::MsgWithDrawReq {
            c2c_with_draw: vec![pb::msg::C2cMsgWithDrawReq {
                msg_info: seqs
                    .into_iter()
                    .zip(rands.into_iter())
                    .map(|(seq, ran)| pb::msg::C2cMsgInfo {
                        from_uin: Some(self.uin()),
                        to_uin: Some(uin),
                        msg_time: Some(msg_time),
                        msg_uid: Some(0x0100_0000_0000_0000 | (ran as i64 & 0xFFFFFFFF)),
                        msg_seq: Some(seq),
                        msg_random: Some(ran),
                        routing_head: Some(pb::msg::RoutingHead {
                            c2c: Some(pb::msg::C2c { to_uin: Some(uin) }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                    .collect(),
                long_message_flag: Some(0),
                reserved: Some(vec![0x08, 0x00]),
                sub_cmd: Some(1),
            }],
            ..Default::default()
        };
        self.uni_packet("PbMessageSvc.PbMsgWithDraw", req.to_bytes())
    }

    pub fn build_group_recall_packet(
        &self,
        group_code: i64,
        seqs: Vec<i32>,
        rands: Vec<i32>,
    ) -> Packet {
        let req = pb::msg::MsgWithDrawReq {
            group_with_draw: vec![pb::msg::GroupMsgWithDrawReq {
                sub_cmd: Some(1),
                group_code: Some(group_code),
                user_def: Some(vec![0x08, 0x00]),
                msg_list: seqs
                    .into_iter()
                    .zip(rands.into_iter())
                    .map(|(seq, ran)| pb::msg::GroupMsgInfo {
                        msg_seq: Some(seq),
                        msg_random: Some(ran),
                        msg_type: Some(0),
                    })
                    .collect(),
                group_type: None,
            }],
            ..Default::default()
        };
        self.uni_packet("PbMessageSvc.PbMsgWithDraw", req.to_bytes())
    }
}
