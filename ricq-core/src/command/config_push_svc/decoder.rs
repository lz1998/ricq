use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::command::config_push_svc::*;
use crate::{jce, pb, RQError, RQResult};

// TODO 还没测试
impl super::super::super::Engine {
    // ConfigPushSvc.PushReq
    pub fn decode_push_req_packet(&self, mut payload: Bytes) -> RQResult<ConfigPushReq> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::Jce)?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::Jce)?;
        let mut a = data
            .map
            .remove("PushReq")
            .ok_or_else(|| RQError::Decode("missing PushReq".into()))?;
        let mut b = a
            .remove("ConfigPush.PushReq")
            .ok_or_else(|| RQError::Decode("missing ConfigPush.PushReq".into()))?;
        let _ = b.split_to(1);
        let mut r = jcers::Jce::new(&mut b);
        let t: i32 = r.get_by_tag(1).map_err(RQError::Jce)?;
        let mut jce_buf: Bytes = r.get_by_tag(2).map_err(RQError::Jce)?;
        let seq: i64 = r.get_by_tag(3).map_err(RQError::Jce)?;
        let mut body = ConfigPushBody::Unknown;
        if !jce_buf.is_empty() {
            body = match t {
                1 => {
                    let mut sso_pkt = jcers::Jce::new(&mut jce_buf);
                    let servers: Vec<jce::SsoServerInfo> =
                        sso_pkt.get_by_tag(1).map_err(RQError::Jce)?;
                    ConfigPushBody::SsoServers { servers }
                }
                2 => {
                    let info: jce::FileStoragePushFSSvcList =
                        jcers::from_buf(&mut jce_buf).map_err(RQError::Jce)?;
                    let rsp_body = match pb::cmd0x6ff::C501RspBody::from_bytes(
                        &info.big_data_channel.pb_buf,
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
