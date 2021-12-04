use bytes::{Buf, Bytes};
use jce_struct::Jce;
use crate::client::{Client, OtherClientInfo};
use crate::jce::{RequestDataVersion2, RequestPacket, SvcRespParam};
use crate::client::errors::RQError;

pub fn decode_push_param_packet(payload: &[u8]) -> Result<Vec<OtherClientInfo>, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: RequestDataVersion2 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut req = data.map.remove("SvcRespParam")
        .ok_or(RQError::Decode("SvcRespParam is none".to_string()))?;
    let mut reader = req.remove("RegisterProxySvcPack.SvcRespParam").unwrap();
    reader.advance(1);
    let rsp: SvcRespParam = Jce::read_from_bytes(&mut reader);
    Ok(rsp.online_infos.iter().map(|i| OtherClientInfo {
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
            _ => String::from_utf8_lossy(&i.sub_platform).to_string()
        },
    }).collect())
}