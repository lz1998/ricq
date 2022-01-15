use std::collections::HashMap;
use std::sync::atomic::Ordering;

use bytes::{Buf, BufMut, Bytes};
use jcers::JcePut;

use crate::binary::BinaryReader;
use crate::client::income::decoder::tlv::*;
use crate::client::protocol::device::random_string;
use crate::client::Client;
use crate::{RQError, RQResult};

#[derive(Debug)]
pub enum QRCodeState {
    QRCodeImageFetch {
        image_data: Bytes,
        sig: Bytes,
    },
    QRCodeWaitingForScan,
    QRCodeWaitingForConfirm,
    QRCodeTimeout,
    QRCodeConfirmed {
        tmp_pwd: Bytes,
        tmp_no_pic_sig: Bytes,
        tgt_qr: Bytes,
    },
    QRCodeCanceled,
}
#[derive(Debug)]
pub struct ImageCaptcha {
    pub sign: Bytes,
    pub image: Bytes,
}

#[derive(Debug)]
pub enum LoginResponse {
    Success {
        rollback_sig: Option<T161>,
        rand_seed: Option<Bytes>,
        ksid: Option<Bytes>,
        account_info: Option<T11A>,
        t512: Option<T512>,
        wt_session_ticket_key: Option<Bytes>,
        srm_token: Option<Bytes>,
        t133: Option<Bytes>,
        encrypt_a1: Option<Bytes>,
        tgt: Option<Bytes>,
        tgt_key: Option<Bytes>,
        user_st_key: Option<Bytes>,
        user_st_web_sig: Option<Bytes>,
        s_key: Option<Bytes>,
        s_key_expired_time: i64,
        d2: Option<Bytes>,
        d2key: Option<Bytes>,
        device_token: Option<Bytes>,
    },
    NeedCaptcha {
        t104: Option<Bytes>,
        verify_url: Option<String>,
        image_captcha: Option<ImageCaptcha>,
    },
    AccountFrozen,
    DeviceLocked {
        sms_phone: Option<String>,
        verify_url: Option<String>,
        message: Option<String>,
        rand_seed: Option<Bytes>,
        t104: Option<Bytes>,
        t174: Option<Bytes>,
    },
    TooManySMSRequest,
    DeviceLockLogin {
        rand_seed: Option<Bytes>,
        t104: Option<Bytes>,
    },
    UnknownLoginStatus {
        status: u8,
        tlv_map: HashMap<u16, Bytes>,
    },
}

impl LoginResponse {
    pub fn decode(
        status: u8,
        mut tlv_map: HashMap<u16, Bytes>,
        encrypt_key: &[u8],
    ) -> RQResult<Self> {
        let resp = match status {
            0 => {
                let mut t119 = tlv_map
                    .remove(&0x119)
                    .map(|v| decode_t119(&v, encrypt_key))
                    .ok_or(RQError::Decode("missing 0x119".to_string()))?;
                LoginResponse::Success {
                    rollback_sig: tlv_map.remove(&0x161).map(decode_t161),
                    rand_seed: tlv_map.remove(&0x403),
                    ksid: t119.remove(&0x108),
                    account_info: t119.remove(&0x11a).map(read_t11a),
                    t512: t119.remove(&0x512).map(read_t512),
                    wt_session_ticket_key: t119.remove(&0x134),
                    srm_token: t119.remove(&0x16a),
                    t133: t119.remove(&0x133),
                    encrypt_a1: t119.remove(&0x106),
                    tgt: t119.remove(&0x10a),
                    tgt_key: t119.remove(&0x10d),
                    user_st_key: t119.remove(&0x10e),
                    user_st_web_sig: t119.remove(&0x103),
                    s_key: t119.remove(&0x120),
                    s_key_expired_time: chrono::Utc::now().timestamp() + 21600,
                    d2: t119.remove(&0x143),
                    d2key: t119.remove(&0x305),
                    device_token: t119.remove(&0x322),
                }
            }
            2 => LoginResponse::NeedCaptcha {
                t104: tlv_map.remove(&0x104),
                verify_url: tlv_map
                    .remove(&0x192)
                    .map(|v| String::from_utf8_lossy(&v).to_string()),
                image_captcha: tlv_map.remove(&0x165).map(|mut img_data| {
                    let sign_len = img_data.get_u16();
                    img_data.get_u16();
                    let image_sign = img_data.copy_to_bytes(sign_len as usize);
                    ImageCaptcha {
                        sign: image_sign,
                        image: img_data.freeze(),
                    }
                }),
            },
            40 => LoginResponse::AccountFrozen,
            160 | 239 => LoginResponse::DeviceLocked {
                // TODO?
                sms_phone: tlv_map.remove(&0x178).map(|_| "todo".into()),
                verify_url: tlv_map
                    .remove(&0x204)
                    .map(|v| String::from_utf8_lossy(&v).to_string()),
                message: tlv_map
                    .remove(&0x17e)
                    .map(|v| String::from_utf8_lossy(&v).to_string()),
                rand_seed: tlv_map.remove(&0x403),
                t104: tlv_map.remove(&0x104),
                t174: tlv_map.remove(&0x174),
            },
            162 => LoginResponse::TooManySMSRequest,
            204 => LoginResponse::DeviceLockLogin {
                t104: tlv_map.remove(&0x104),
                rand_seed: tlv_map.remove(&0x403),
            },
            _ => LoginResponse::UnknownLoginStatus { status, tlv_map },
        };
        Ok(resp)
    }
}

pub async fn decode_trans_emp_response(
    cli: &Client,
    payload: &[u8],
) -> Result<QRCodeState, RQError> {
    if payload.len() < 48 {
        return Err(RQError::Decode("invalid payload length".to_string()).into());
    }
    let mut payload = Bytes::from(payload.to_owned());
    payload.advance(5); // trans req head
    payload.get_u8();
    payload.get_u16();
    let cmd = payload.get_u16();
    payload.advance(21);
    payload.get_u8();
    payload.get_u16();
    payload.get_u16();
    payload.get_i32();
    payload.get_i64();
    let len = payload.remaining() - 1;
    let mut body = Bytes::from(payload.copy_to_bytes(len));
    if cmd == 0x31 {
        body.get_u16();
        body.get_i32();
        let code = body.get_u8();
        if code != 0 {
            return Err(RQError::Decode("body code != 0".into()));
        }
        let sig = body.read_bytes_short();
        body.get_u16();
        let mut m = body.read_tlv_map(2);
        if m.contains_key(&0x17) {
            return Ok(QRCodeState::QRCodeImageFetch {
                image_data: m
                    .remove(&0x17)
                    .ok_or(RQError::Decode("missing 0x17".into()))?,
                sig,
            });
        }
    }
    if cmd == 0x12 {
        let mut a_var_len = body.get_u16();
        if a_var_len != 0 {
            a_var_len -= 1; // 阴间的位移操作
            if body.get_u8() == 2 {
                body.get_i64(); //uin?
                a_var_len -= 8;
            }
        }
        if a_var_len > 0 {
            body.advance(a_var_len as usize);
        }
        body.get_i32();
        let code = body.get_u8();
        if code != 0 {
            return match code {
                0x30 => Ok(QRCodeState::QRCodeWaitingForScan),
                0x35 => Ok(QRCodeState::QRCodeWaitingForConfirm),
                0x36 => Ok(QRCodeState::QRCodeCanceled),
                0x11 => Ok(QRCodeState::QRCodeTimeout),
                _ => Err(RQError::Decode("invalid body code".to_string())),
            };
        }
        cli.uin.store(body.get_i64(), Ordering::SeqCst);
        body.get_i32(); // sig create time
        body.get_u16();
        let mut m = body.read_tlv_map(2);
        if !m.contains_key(&0x18) || !m.contains_key(&0x1e) || !m.contains_key(&0x19) {
            return Err(RQError::Decode("invalid tlv map".to_string()));
        }
        {
            let mut transport = cli.transport.write().await;
            transport.sig.tgtgt_key = m
                .remove(&0x1e)
                .ok_or(RQError::Decode("missing 0x1e".into()))?;
        }
        return Ok(QRCodeState::QRCodeConfirmed {
            tmp_pwd: m
                .remove(&0x18)
                .ok_or(RQError::Decode("missing 0x18".into()))?,
            tmp_no_pic_sig: m
                .remove(&0x19)
                .ok_or(RQError::Decode("missing 0x19".into()))?,
            tgt_qr: m
                .remove(&0x65)
                .ok_or(RQError::Decode("missing 0x65".into()))?,
        });
    }
    return Err(RQError::Decode(
        "decode_trans_emp_response unknown error".to_string(),
    ));
}

pub async fn decode_login_response(cli: &Client, payload: &[u8]) -> RQResult<LoginResponse> {
    let mut transport = cli.transport.write().await;
    let mut reader = Bytes::from(payload.to_owned());
    let _sub_command = reader.get_u16(); // sub command
    let status = reader.get_u8();
    reader.get_u16();
    let mut tlv_map = reader.read_tlv_map(2);
    if tlv_map.contains_key(&0x402) {
        transport.sig.dpwd = random_string(16).into();
        transport.sig.t402 = tlv_map
            .remove(&0x402)
            .ok_or(RQError::Decode("missing 0x402".to_string()))?;
        let mut v = Vec::new();
        v.put_slice(&transport.sig.guid);
        v.put_slice(&transport.sig.dpwd);
        v.put_slice(&transport.sig.t402);
        transport.sig.g = md5::compute(&v).to_vec().into();
    }
    LoginResponse::decode(status, tlv_map, &transport.sig.tgtgt_key)
}

pub async fn decode_exchange_emp_response(
    cli: &mut Client,
    payload: &[u8],
) -> RQResult<LoginResponse> {
    let transport = cli.transport.write().await;
    let mut payload = Bytes::from(payload.to_owned());
    let sub_command = payload.get_u16();
    let status = payload.get_u8();
    payload.get_u16();
    let tlv_map = payload.read_tlv_map(2);
    if status != 0 {
        return Err(RQError::Decode(
            "decode_exchange_emp_response t != 0".to_string(),
        ));
    }
    let encrypt_key = if sub_command == 11 {
        Bytes::from(md5::compute(&transport.sig.d2key).to_vec())
    } else {
        transport.sig.tgtgt_key.clone()
    };

    LoginResponse::decode(status, tlv_map, &encrypt_key)
}
