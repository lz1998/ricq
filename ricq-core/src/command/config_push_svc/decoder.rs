use bytes::Bytes;

use crate::command::config_push_svc::*;
use crate::{jce, pb, RQError, RQResult};
use prost::Message;

// TODO 还没测试
impl super::super::super::Engine {
    // ConfigPushSvc.PushReq
    pub fn decode_push_req_packet(&self, mut payload: Bytes) -> RQResult<ConfigPushReq> {
        let mut request: jce::RequestPacket = jcers::from_buf(&mut payload)?;
        let mut data: jce::RequestDataVersion2 = jcers::from_buf(&mut request.s_buffer)?;
        let mut a = data
            .map
            .remove("PushReq")
            .ok_or_else(|| RQError::Decode("missing PushReq".into()))?;
        let mut b = a
            .remove("ConfigPush.PushReq")
            .ok_or_else(|| RQError::Decode("missing ConfigPush.PushReq".into()))?;
        let _ = b.split_to(1);
        let mut r = jcers::Jce::new(&mut b);
        let t: i32 = r.get_by_tag(1)?;
        let mut jce_buf: Bytes = r.get_by_tag(2)?;
        let seq: i64 = r.get_by_tag(3)?;
        let mut body = ConfigPushBody::Unknown;
        if !jce_buf.is_empty() {
            body = match t {
                1 => {
                    let mut sso_pkt = jcers::Jce::new(&mut jce_buf);
                    let servers: Vec<jce::SsoServerInfo> = sso_pkt.get_by_tag(1)?;
                    ConfigPushBody::SsoServers { servers }
                }
                2 => {
                    let mut info: jce::FileStoragePushFSSvcList = jcers::from_buf(&mut jce_buf)?;
                    let rsp_body = match pb::cmd0x6ff::C501RspBody::decode(
                        &mut info.big_data_channel.pb_buf,
                    ) {
                        Ok(c501_rsp_body) => c501_rsp_body.rsp_body,
                        _ => None,
                    };
                    ConfigPushBody::FileStorageInfo { info, rsp_body }
                }
                _ => ConfigPushBody::Unknown,
            }
        }
        Ok(ConfigPushReq {
            resp: ConfigPushResp {
                t,
                pkt_seq: seq,
                jce_buf,
            },
            body,
        })
    }
}
