use std::collections::HashMap;

use bytes::Bytes;
use jcers::JcePut;

use crate::command::common::pack_uni_request_data;
use crate::jce;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    // VisitorSvc.ReqFavorite
    pub fn build_send_like_packet(
        &self,
        uin: i64,
        count: i32,
        source: i32,
        cookies: Bytes,
    ) -> Packet {
        let seq = self.next_seq();
        let req = jce::ReqFavorite {
            header: jce::QQServiceReqHead {
                uin: self.uin(),
                sh_version: 1,
                seq: seq as i32,
                req_type: 1,
                triggered: 0,
                cookies,
            },
            mid: uin,
            op_type: 0,
            source,
            count,
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
