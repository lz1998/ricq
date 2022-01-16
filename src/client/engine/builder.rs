use std::collections::HashMap;
use std::sync::atomic::Ordering;

use bytes::{BufMut, Bytes, BytesMut};
use chrono::Utc;
use jcers::JcePut;
use prost::Message;

use crate::client::outcome::PbToBytes;
use crate::client::protocol::packet::*;
use crate::jce::*;
use crate::pb;
use crate::pb::msg::SyncCookie;

fn pack_uni_request_data(data: &[u8]) -> Bytes {
    let mut r = BytesMut::new();
    r.put_slice(&[0x0A]);
    r.put_slice(data);
    r.put_slice(&[0x0B]);
    Bytes::from(r)
}

impl super::Engine {
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

    pub fn build_delete_online_push_packet(
        &self,
        uin: i64,
        svrip: i32,
        push_token: Bytes,
        seq: u16,
        del_msg: Vec<PushMessageInfo>,
    ) -> Packet {
        let mut req = SvcRespPushMsg {
            uin,
            svrip,
            push_token,
            ..Default::default()
        };
        for m in del_msg {
            req.del_infos.push(DelMsgInfo {
                from_uin: m.from_uin,
                msg_time: m.msg_time,
                msg_seq: m.msg_seq,
                msg_cookies: m.msg_cookies,
                ..Default::default()
            })
        }
        let b = pack_uni_request_data(&req.freeze());
        let buf = RequestDataVersion3 {
            map: HashMap::from([("resp".to_string(), b.into())]),
        };
        let pkt = RequestPacket {
            i_version: 3,
            i_request_id: seq as i32,
            s_servant_name: "OnlinePush".to_string(),
            s_func_name: "SvcRespPushMsg".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("OnlinePush.RespPush", pkt.freeze())
    }

    pub fn build_conf_push_resp_packet(&self, t: i32, pkt_seq: i64, jce_buf: Bytes) -> Packet {
        let mut req = jcers::JceMut::new();
        req.put_i32(t, 1);
        req.put_i64(pkt_seq, 2);
        req.put_bytes(jce_buf, 3);

        let buf = RequestDataVersion3 {
            map: HashMap::from([("PushResp".to_string(), pack_uni_request_data(&req.freeze()))]),
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "QQService.ConfigPushSvc.MainServant".to_string(),
            s_func_name: "PushResp".to_string(),
            s_buffer: buf.freeze(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        };
        self.uni_packet("ConfigPushSvc.PushResp", pkt.freeze())
    }

    pub fn build_get_offline_msg_request_packet(&self, last_message_time: i64) -> Packet {
        let transport = &self.transport;
        let reg_req = SvcReqRegisterNew {
            request_optional: 0x101C2 | 32,
            c2c_msg: SvcReqGetMsgV2 {
                uin: self.uin.load(Ordering::SeqCst),
                date_time: match last_message_time {
                    0 => 1,
                    _ => last_message_time as i32,
                },
                recive_pic: 1,
                ability: 15,
                channel: 4,
                inst: 1,
                channel_ex: 1,
                sync_cookie: transport.sig.sync_cookie.to_owned(),
                sync_flag: 0,
                ramble_flag: 0,
                general_abi: 1,
                pub_account_cookie: transport.sig.pub_account_cookie.to_owned(),
            },
            group_msg: SvcReqPullGroupMsgSeq {
                verify_type: 0,
                filter: 1,
                ..Default::default()
            },
            end_seq: Utc::now().timestamp(),
            ..Default::default()
        };
        let flag = 0; // flag := msg.SyncFlag_START
        let msg_req = pb::msg::GetMessageRequest {
            sync_flag: Some(flag),
            sync_cookie: Some(transport.sig.sync_cookie.to_vec()),
            ramble_flag: Some(0),
            context_flag: Some(1),
            online_sync_flag: Some(0),
            latest_ramble_number: Some(20),
            other_ramble_number: Some(3),
            ..Default::default()
        }
        .to_bytes();
        let mut buf = BytesMut::new();
        buf.put_slice(&[0, 0, 0, 0]);
        buf.put_slice(&msg_req);
        let buf = buf.freeze();
        let mut req = jcers::JceMut::new();
        req.put_bytes(buf, 0);
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("req_PbOffMsg".to_string(), req.freeze()),
                (
                    "req_OffMsg".to_string(),
                    pack_uni_request_data(&reg_req.freeze()),
                ),
            ]),
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "RegPrxySvc".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("RegPrxySvc.getOffMsg", pkt.freeze())
    }

    pub fn build_sync_msg_request_packet(&self, last_message_time: i64) -> Packet {
        let transport = &self.transport;
        let oidb_req = pb::oidb::D769RspBody {
            config_list: vec![
                pb::oidb::D769ConfigSeq {
                    r#type: Some(46),
                    version: Some(0),
                },
                pb::oidb::D769ConfigSeq {
                    r#type: Some(283),
                    version: Some(0),
                },
            ],
            ..Default::default()
        }
        .to_bytes();
        let reg_req = SvcReqRegisterNew {
            request_optional: 128 | 64 | 256 | 2 | 8192 | 16384 | 65536,
            dis_group_msg_filter: 1,
            c2c_msg: SvcReqGetMsgV2 {
                uin: self.uin.load(Ordering::SeqCst),
                date_time: match last_message_time {
                    0 => 1,
                    _ => last_message_time as i32,
                },
                recive_pic: 1,
                ability: 15,
                channel: 4,
                inst: 1,
                channel_ex: 1,
                sync_cookie: transport.sig.sync_cookie.to_owned(),
                sync_flag: 0, // START
                ramble_flag: 0,
                general_abi: 1,
                pub_account_cookie: transport.sig.pub_account_cookie.to_owned(),
            },
            group_mask: 2,
            end_seq: rand::random::<u32>() as i64,
            _0769_body: oidb_req,
            ..Default::default()
        };
        let flag = 0; // flag := msg.SyncFlag_START
        let mut msg_req = pb::msg::GetMessageRequest {
            sync_flag: Some(flag),
            sync_cookie: Some(transport.sig.sync_cookie.to_vec()),
            ramble_flag: Some(0),
            context_flag: Some(1),
            online_sync_flag: Some(0),
            latest_ramble_number: Some(20),
            other_ramble_number: Some(3),
            msg_req_type: Some(1),
            ..Default::default()
        };
        let off_msg = msg_req.to_bytes();
        msg_req.msg_req_type = Some(2);
        msg_req.sync_cookie = None;
        msg_req.pubaccount_cookie = Some(transport.sig.pub_account_cookie.to_vec());
        let pub_msg = msg_req.to_bytes();
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("req_PbOffMsg".to_string(), {
                    let mut w = jcers::JceMut::new();
                    w.put_bytes(
                        {
                            let mut b = BytesMut::new();
                            b.put_slice(&[0; 4]);
                            b.put_slice(&off_msg);
                            b.freeze()
                        },
                        0,
                    );
                    w.freeze()
                }),
                ("req_PbPubMsg".to_string(), {
                    let mut w = jcers::JceMut::new();
                    w.put_bytes(
                        {
                            let mut b = BytesMut::new();
                            b.put_slice(&[0; 4]);
                            b.put_slice(&pub_msg);
                            b.freeze()
                        },
                        0,
                    );
                    w.freeze()
                }),
                (
                    "req_OffMsg".to_string(),
                    pack_uni_request_data(&reg_req.freeze()),
                ),
            ]),
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "RegPrxySvc".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("RegPrxySvc.infoSync", pkt.freeze())
    }

    pub fn build_group_msg_readed_packet(&self, group_code: i64, msg_seq: i32) -> Packet {
        let req = pb::msg::PbMsgReadedReportReq {
            grp_read_report: vec![pb::msg::PbGroupReadedReportReq {
                group_code: Some(group_code as u64),
                last_read_seq: Some(msg_seq as u64),
            }],
            ..Default::default()
        };
        self.uni_packet("PbMessageSvc.PbMsgReadedReport", req.to_bytes())
    }

    pub fn build_private_msg_readed_packet(&self, uin: i64, time: i64) -> Packet {
        let transport = &self.transport;
        let req = pb::msg::PbMsgReadedReportReq {
            c2_c_read_report: Some(pb::msg::PbC2cReadedReportReq {
                pair_info: vec![pb::msg::UinPairReadInfo {
                    peer_uin: Some(uin as u64),
                    last_read_time: Some(time as u32),
                    ..Default::default()
                }],
                sync_cookie: Some(transport.sig.sync_cookie.to_vec()),
                ..Default::default()
            }),
            ..Default::default()
        };
        self.uni_packet("PbMessageSvc.PbMsgReadedReport", req.to_bytes())
    }

    pub fn build_group_info_request_packet(&self, group_code: i64) -> Packet {
        let transport = &self.transport;
        let body = pb::oidb::D88dReqBody {
            app_id: Some(transport.version.app_id),
            req_group_info: vec![pb::oidb::ReqGroupInfo {
                group_code: Some(group_code as u64),
                stgroupinfo: Some(pb::oidb::D88dGroupInfo {
                    group_owner: Some(0),
                    group_uin: Some(0),
                    group_create_time: Some(0),
                    group_flag: Some(0),
                    group_member_max_num: Some(0),
                    group_member_num: Some(0),
                    group_option: Some(0),
                    group_level: Some(0),
                    group_face: Some(0),
                    group_name: Some(vec![]),
                    group_memo: Some(vec![]),
                    group_finger_memo: Some(vec![]),
                    group_last_msg_time: Some(0),
                    group_cur_msg_seq: Some(0),
                    group_question: Some(vec![]),
                    group_answer: Some(vec![]),
                    group_grade: Some(0),
                    active_member_num: Some(0),
                    head_portrait_seq: Some(0),
                    msg_head_portrait: Some(pb::oidb::D88dGroupHeadPortrait::default()),
                    st_group_ex_info: Some(pb::oidb::D88dGroupExInfoOnly::default()),
                    group_sec_level: Some(0),
                    cmduin_privilege: Some(0),
                    no_finger_open_flag: Some(0),
                    no_code_finger_open_flag: Some(0),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            pc_client_version: Some(0),
        };
        let payload = pb::oidb::OidbssoPkg {
            command: 2189,
            bodybuffer: body.to_bytes().to_vec(),
            ..Default::default()
        };
        self.uni_packet("OidbSvc.0x88d_0", payload.to_bytes())
    }

    pub fn build_get_message_request_packet(&self, flag: i32, time: i64) -> Packet {
        let mut cook = { self.transport.sig.sync_cookie.to_vec() };
        if cook.is_empty() {
            cook = SyncCookie {
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

// #[cfg(test)]
// mod tests {
//     use bytes::BufMut;
//     use chrono::Utc;
//     use rand::distributions::Alphanumeric;
//     use rand::{Rng, thread_rng};

//     #[test]
//     fn test_read() {}
// }
