use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};
use jcers::JcePut;

use crate::command::common::{pack_uni_request_data, PbToBytes};
use crate::protocol::packet::*;
use crate::{jce, pb};

impl super::super::super::Engine {
    // SummaryCard.ReqSummaryCard
    pub fn build_summary_card_request_packet(&self, target: i64) -> Packet {
        let seq = self.next_seq();
        let gate = pb::profilecard::GateVaProfileGateReq {
            u_cmd: Some(3),
            st_privilege_req: Some(pb::profilecard::GatePrivilegeBaseInfoReq {
                u_req_uin: Some(target),
            }),
            st_gift_req: Some(pb::profilecard::GateGetGiftListReq {
                uin: Some(target as i32),
            }),
            st_vip_care: Some(pb::profilecard::GateGetVipCareReq { uin: Some(target) }),
            oidb_flag: vec![
                pb::profilecard::GateOidbFlagInfo {
                    fieled: Some(42334),
                    byets_value: None,
                },
                pb::profilecard::GateOidbFlagInfo {
                    fieled: Some(42340),
                    byets_value: None,
                },
                pb::profilecard::GateOidbFlagInfo {
                    fieled: Some(42344),
                    byets_value: None,
                },
                pb::profilecard::GateOidbFlagInfo {
                    fieled: Some(42354),
                    byets_value: None,
                },
            ],
            ..Default::default()
        }
        .to_bytes();
        let business_buf = {
            let mut w = BytesMut::new();
            let comm = pb::profilecard::BusiComm {
                ver: Some(1),
                seq: Some(seq as i32),
                fromuin: Some(self.uin()),
                touin: Some(target),
                service: Some(16),
                platform: Some(2),
                qqver: Some(self.transport.version.build_ver.into()),
                build: Some(4945),
                ..Default::default()
            }
            .to_bytes();
            w.put_u8(40);
            w.put_u32(comm.len() as u32);
            w.put_u32(gate.len() as u32);
            w.put_slice(&comm);
            w.put_slice(&gate);
            w.put_u8(42);
            w.freeze()
        };

        let req = jce::SummaryCardReq {
            uin: target,
            come_from: 31,
            get_control: 69181,
            add_friend_source: 3001,
            secure_sig: Bytes::from(vec![0]),
            req_medal_wall_info: 0,
            req_0x5eb_field_id: vec![
                27225, 27224, 42122, 42121, 27236, 27238, 42167, 42172, 40324, 42284, 42326, 42325,
                42356, 42363, 42361, 42367, 42377, 42425, 42505, 42488,
            ],
            req_services: vec![business_buf],
            req_nearby_god_info: 1,
            req_extend_card: 1,
            ..Default::default()
        };
        let mut head = jcers::JceMut::new();
        head.put_i32(2, 0);
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([
                ("ReqHead".to_string(), pack_uni_request_data(&head.freeze())),
                (
                    "ReqSummaryCard".to_string(),
                    pack_uni_request_data(&req.freeze()),
                ),
            ]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            s_servant_name: "SummaryCardServantObj".to_string(),
            s_func_name: "ReqSummaryCard".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("SummaryCard.ReqSummaryCard", pkt.freeze())
    }
}
