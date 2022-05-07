use crate::command::common::pack_uni_request_data;
use crate::hex::decode_hex;
use crate::jce;
use crate::protocol::packet::Packet;
use bytes::Bytes;
use jcers::JcePut;
use std::collections::HashMap;

impl super::super::super::Engine {
    // VisitorSvc.ReqFavorite
    pub fn build_send_like_packet(&self, uin: i64, count: i32) -> Packet {
        let seq = self.next_seq();
        let req = jce::ReqFavorite {
            st_header: jce::QQServiceReqHead {
                l_uin: self.uin(),
                sh_version: 1,
                i_seq: seq as i32,
                b_req_type: 1,
                b_triggered: 0,
                v_cookies: Bytes::from(decode_hex("0C180001060131160131").unwrap()),
            },
            l_mid: uin,
            c_op_type: 0,
            em_source: 1,
            i_count: count,
        };
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([(
                "ReqFavorite".to_string(),
                pack_uni_request_data(&req.freeze()),
            )]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            s_servant_name: "VisitorSvc".to_string(),
            s_func_name: "ReqFavorite".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("VisitorSvc.ReqFavorite", pkt.freeze())
    }
}
