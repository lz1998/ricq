use bytes::Bytes;
use tokio::sync::RwLock;
use jce_struct::Jce;
use crate::client::{Client, OtherClientInfo};
use crate::jce::{RequestDataVersion2, RequestPacket, SvcRespParam};

pub fn decode_push_param_packet(payload: &[u8])->Option<Vec<OtherClientInfo>> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: RequestDataVersion2 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut req = data.map.remove("SvcRespParam").unwrap();
    let mut reader = req.remove("RegisterProxySvcPack.SvcRespParam").unwrap();
    let rsp: SvcRespParam = Jce::read_from_bytes(&mut reader);
    Some(rsp.online_infos.iter().map(|i| OtherClientInfo {
        app_id: i.instance_id as i64,
        instance_id: i.instance_id,
        sub_platform: i.sub_platform.to_owned(),
        device_kind: match i.u_client_type {
            65793 => "Windows".to_string(),
            65805 | 68104 => "aPad".to_string(),
            66818 | 66831 | 81154 => "Mac".to_string(),
            68361 | 72194 => "iPad".to_string(),
            75023 | 78082 | 78096 => "Watch".to_string(),
            77313 => "Windows TIM".to_string(),
            _ => i.sub_platform.to_string()
        },
    }).collect())
}