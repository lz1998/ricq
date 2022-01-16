use std::collections::HashMap;

use bytes::Bytes;
use jcers::JcePut;

use crate::client::engine::common::pack_uni_request_data;
use crate::client::protocol::packet::Packet;
use crate::jce;

impl super::super::super::Engine {
    // ConfigPushSvc.PushResp
    pub fn build_conf_push_resp_packet(&self, t: i32, pkt_seq: i64, jce_buf: Bytes) -> Packet {
        let mut req = jcers::JceMut::new();
        req.put_i32(t, 1);
        req.put_i64(pkt_seq, 2);
        req.put_bytes(jce_buf, 3);

        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([("PushResp".to_string(), pack_uni_request_data(&req.freeze()))]),
        };
        let pkt = jce::RequestPacket {
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
}
