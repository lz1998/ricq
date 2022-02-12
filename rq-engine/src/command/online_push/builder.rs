use std::collections::HashMap;

use bytes::Bytes;
use jcers::JcePut;

use crate::command::common::pack_uni_request_data;
use crate::jce;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    // OnlinePush.RespPush
    pub fn build_delete_online_push_packet(
        &self,
        uin: i64,
        svrip: i32,
        push_token: Bytes,
        seq: u16,
        del_msg: Vec<jce::PushMessageInfo>,
    ) -> Packet {
        let req = jce::SvcRespPushMsg {
            uin,
            svrip,
            push_token,
            del_infos: del_msg
                .into_iter()
                .map(|m| jce::DelMsgInfo {
                    from_uin: m.from_uin,
                    msg_time: m.msg_time,
                    msg_seq: m.msg_seq,
                    msg_cookies: m.msg_cookies,
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        };
        let b = pack_uni_request_data(&req.freeze());
        let buf = jce::RequestDataVersion3 {
            map: HashMap::from([("resp".to_string(), b)]),
        };
        let pkt = jce::RequestPacket {
            i_version: 3,
            i_request_id: seq as i32,
            s_servant_name: "OnlinePush".to_string(),
            s_func_name: "SvcRespPushMsg".to_string(),
            s_buffer: buf.freeze(),
            ..Default::default()
        };
        self.uni_packet("OnlinePush.RespPush", pkt.freeze())
    }
}
