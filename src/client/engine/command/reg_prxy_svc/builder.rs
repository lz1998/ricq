use std::collections::HashMap;

use bytes::{BufMut, BytesMut};
use chrono::Utc;
use jcers::JcePut;

use crate::client::engine::common::pack_uni_request_data;
use crate::client::outcome::PbToBytes;
use crate::client::protocol::packet::Packet;
use crate::{jce, pb};

impl super::super::super::Engine {
    pub fn build_get_offline_msg_request_packet(&self, last_message_time: i64) -> Packet {
        let transport = &self.transport;
        let reg_req = jce::SvcReqRegisterNew {
            request_optional: 0x101C2 | 32,
            c2c_msg: jce::SvcReqGetMsgV2 {
                uin: self.uin(),
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
            group_msg: jce::SvcReqPullGroupMsgSeq {
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
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([
                ("req_PbOffMsg".to_string(), req.freeze()),
                (
                    "req_OffMsg".to_string(),
                    pack_uni_request_data(&reg_req.freeze()),
                ),
            ]),
        };
        let pkt = jce::RequestPacket {
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
        let reg_req = jce::SvcReqRegisterNew {
            request_optional: 128 | 64 | 256 | 2 | 8192 | 16384 | 65536,
            dis_group_msg_filter: 1,
            c2c_msg: jce::SvcReqGetMsgV2 {
                uin: self.uin(),
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
        let buf = jce::RequestDataVersion3 {
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
        let pkt = jce::RequestPacket {
            i_version: 3,
            s_servant_name: "RegPrxySvc".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("RegPrxySvc.infoSync", pkt.freeze())
    }
}
