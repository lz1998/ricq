use std::collections::HashMap;
use std::sync::atomic::Ordering;

use bytes::{BufMut, Bytes, BytesMut};
use chrono::Utc;
use jcers::JcePut;

use crate::client::outcome::PbToBytes;
use crate::client::protocol::packet::*;
use crate::jce::*;
use crate::pb;

fn pack_uni_request_data(data: &[u8]) -> Bytes {
    let mut r = BytesMut::new();
    r.put_slice(&[0x0A]);
    r.put_slice(data);
    r.put_slice(&[0x0B]);
    Bytes::from(r)
}

impl super::Engine {
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
