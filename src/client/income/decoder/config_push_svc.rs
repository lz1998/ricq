use crate::client::errors::RQError;
use crate::client::outcome::PbToBytes;
use crate::jce;
use crate::pb;
use bytes::Bytes;
// use jce_struct::Jce;

#[derive(Default, Debug)]
pub struct ConfigPushReq {
    pub resp: ConfigPushResp,
    pub body: ConfigPushBody,
}

#[derive(Debug)]
pub enum ConfigPushBody {
    Unknown,
    SsoServers {
        servers: Vec<jce::SsoServerInfo>,
    },
    FileStorageInfo {
        info: jce::FileStoragePushFSSvcList,
        rsp_body: Option<pb::cmd0x6ff::SubCmd0x501RspBody>,
    },
}

impl Default for ConfigPushBody {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Default, Debug)]
pub struct ConfigPushResp {
    pub t: i32,
    pub pkt_seq: i64,
    pub jce_buf: Bytes,
}

// TODO 还没测试
pub fn decode_push_req_packet(payload: &[u8]) -> Result<ConfigPushReq, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: jce::RequestPacket =
        jcers::from_buf(&mut payload).map_err(|e| RQError::Jce(e))?;
    let mut data: jce::RequestDataVersion2 =
        jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::Jce(e))?;
    let mut a = data
        .map
        .remove("PushReq")
        .ok_or(RQError::Decode("missing PushReq".into()))?;
    let mut b = a
        .remove("ConfigPush.PushReq")
        .ok_or(RQError::Decode("missing ConfigPush.PushReq".into()))?;
    let _ = b.split_to(1);
    let mut r = jcers::Jce::new(&mut b);
    let t: i32 = r.get_by_tag(1).map_err(|e| RQError::Jce(e))?;
    let mut jce_buf: Bytes = r.get_by_tag(2).map_err(|e| RQError::Jce(e))?;
    let seq: i64 = r.get_by_tag(3).map_err(|e| RQError::Jce(e))?;
    let mut body = ConfigPushBody::Unknown;
    if jce_buf.len() > 0 {
        body = match t {
            1 => {
                let mut sso_pkt = jcers::Jce::new(&mut jce_buf);
                let servers: Vec<jce::SsoServerInfo> =
                    sso_pkt.get_by_tag(1).map_err(|e| RQError::Jce(e))?;
                ConfigPushBody::SsoServers { servers }
            }
            2 => {
                let info: jce::FileStoragePushFSSvcList =
                    jcers::from_buf(&mut jce_buf).map_err(|e| RQError::Jce(e))?;
                let rsp_body =
                    match pb::cmd0x6ff::C501RspBody::from_bytes(&info.big_data_channel.pb_buf) {
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
