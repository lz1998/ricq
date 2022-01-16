use bytes::{Buf, Bytes};

use crate::client::OtherClientInfo;
use crate::{jce, RQError, RQResult};

impl super::super::super::Engine {
    // RegPrxySvc.PushParam
    pub fn decode_push_param_packet(&self, payload: &[u8]) -> RQResult<Vec<OtherClientInfo>> {
        let mut payload = Bytes::from(payload.to_owned());
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(|e| RQError::from(e))?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::from(e))?;
        let mut req = data
            .map
            .remove("SvcRespParam")
            .ok_or(RQError::Decode("SvcRespParam is none".to_string()))?;
        let mut reader = req
            .remove("RegisterProxySvcPack.SvcRespParam")
            .ok_or(RQError::Decode(
                "RegisterProxySvcPack.SvcRespParam is none".to_string(),
            ))?;
        reader.advance(1);
        let rsp: jce::SvcRespParam = jcers::from_buf(&mut reader).map_err(|e| RQError::from(e))?;
        Ok(rsp
            .online_infos
            .iter()
            .map(|i| OtherClientInfo {
                app_id: i.instance_id as i64,
                instance_id: i.instance_id,
                sub_platform: String::from_utf8_lossy(&i.sub_platform).to_string(),
                device_kind: match i.u_client_type {
                    65793 => "Windows".to_string(),
                    65805 | 68104 => "aPad".to_string(),
                    66818 | 66831 | 81154 => "Mac".to_string(),
                    68361 | 72194 => "iPad".to_string(),
                    75023 | 78082 | 78096 => "Watch".to_string(),
                    77313 => "Windows TIM".to_string(),
                    _ => String::from_utf8_lossy(&i.sub_platform).to_string(),
                },
            })
            .collect())
    }
}
