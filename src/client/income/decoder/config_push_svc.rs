use bytes::{Buf, Bytes};
use jce_struct::Jce;
use crate::client::structs::BigDataReqSessionInfo;
use crate::jce;
use crate::pb;
use crate::client::outcome::PbToBytes;

pub struct ConfigPushReq {
    pub resp: ConfigPushResp,
    pub body: ConfigPushBody,
}

pub enum ConfigPushBody {
    Unknown,
    SsoServers {
        servers: Vec<jce::SsoServerInfo>
    },
    FileStorageInfo {
        info: jce::FileStoragePushFSSvcList,
        rsp_body: Option<pb::cmd0x6ff::SubCmd0x501RspBody>,
    },
}

pub struct ConfigPushResp {
    pub t: i32,
    pub pkt_seq: i64,
    pub jce_buf: Bytes,
}

// TODO 还没测试
pub fn decode_push_req_packet(payload: &[u8]) -> Option<ConfigPushReq> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: jce::RequestDataVersion2 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut a = data.map.remove("PushReq").unwrap();
    let mut b = a.remove("ConfigPush.PushReq").unwrap();
    let mut r = Jce::new(&mut b);
    let t: i32 = r.get_by_tag(1);
    let mut jce_buf: Bytes = r.get_by_tag(2);
    let seq: i64 = r.get_by_tag(3);
    let mut body = ConfigPushBody::Unknown;
    if jce_buf.len() > 0 {
        body = match t {
            1 => {
                let mut sso_pkt = Jce::new(&mut jce_buf);
                let servers: Vec<jce::SsoServerInfo> = sso_pkt.get_by_tag(1);
                ConfigPushBody::SsoServers { servers }
            }
            2 => {
                let info: jce::FileStoragePushFSSvcList = Jce::read_from_bytes(&mut jce_buf);
                let rsp_body = match pb::cmd0x6ff::C501RspBody::from_bytes(&info.big_data_channel.pb_buf) {
                    Ok(c501_rsp_body) => {
                        c501_rsp_body.rsp_body
                    }
                    _ => None
                };
                ConfigPushBody::FileStorageInfo {
                    info,
                    rsp_body,
                }
            }
            _ => ConfigPushBody::Unknown
        }
    }
    Some(ConfigPushReq {
        resp: ConfigPushResp {
            t,
            pkt_seq: seq,
            jce_buf,
        },
        body,
    })
}