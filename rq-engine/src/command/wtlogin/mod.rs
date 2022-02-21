use std::collections::HashMap;

use bytes::{Buf, Bytes};

use crate::binary::BinaryReader;
use crate::command::wtlogin::tlv_reader::*;
use crate::{RQError, RQResult};

mod builder;
mod decoder;
pub mod tlv_reader;
pub mod tlv_writer;

#[derive(Debug, Clone)]
pub enum QRCodeState {
    ImageFetch(QRCodeImageFetch),
    WaitingForScan,
    WaitingForConfirm,
    Timeout,
    Confirmed(QRCodeConfirmed),
    Canceled,
}

#[derive(Debug, Clone)]
pub struct QRCodeImageFetch {
    pub image_data: Bytes,
    pub sig: Bytes,
}

#[derive(Debug, Clone)]
pub struct QRCodeConfirmed {
    pub uin: i64,
    pub tmp_pwd: Bytes,
    pub tmp_no_pic_sig: Bytes,
    pub tgt_qr: Bytes,
    pub tgtgt_key: Bytes,
}

#[derive(Debug, Clone)]
pub struct ImageCaptcha {
    pub sign: Bytes,
    pub image: Bytes,
}

#[derive(Debug, Clone)]
pub enum LoginResponse {
    Success(LoginSuccess),
    // slider or image captcha
    NeedCaptcha(LoginNeedCaptcha),
    AccountFrozen,
    // sms or qrcode
    DeviceLocked(LoginDeviceLocked),
    TooManySMSRequest,
    // More login packet needed
    DeviceLockLogin(LoginDeviceLockLogin),
    UnknownStatus(LoginUnknownStatus),
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub rollback_sig: Option<T161>,
    pub rand_seed: Option<Bytes>,
    pub ksid: Option<Bytes>,
    pub account_info: Option<T11A>,
    pub t512: Option<T512>,
    // 不知道有没有 t402
    pub t402: Option<Bytes>,
    pub wt_session_ticket_key: Option<Bytes>,
    pub srm_token: Option<Bytes>,
    pub t133: Option<Bytes>,
    pub encrypt_a1: Option<Bytes>,
    pub tgt: Option<Bytes>,
    pub tgt_key: Option<Bytes>,
    pub user_st_key: Option<Bytes>,
    pub user_st_web_sig: Option<Bytes>,
    pub s_key: Option<Bytes>,
    pub s_key_expired_time: i64,
    pub d2: Option<Bytes>,
    pub d2key: Option<Bytes>,
    pub device_token: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct LoginNeedCaptcha {
    pub t104: Option<Bytes>,
    pub verify_url: Option<String>,
    pub image_captcha: Option<ImageCaptcha>,
}

#[derive(Debug, Clone)]
pub struct LoginDeviceLocked {
    pub t104: Option<Bytes>,
    pub t174: Option<Bytes>,
    pub t402: Option<Bytes>,
    pub sms_phone: Option<String>,
    pub verify_url: Option<String>,
    pub message: Option<String>,
    pub rand_seed: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct LoginDeviceLockLogin {
    pub t104: Option<Bytes>,
    pub t402: Option<Bytes>,
    pub rand_seed: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct LoginUnknownStatus {
    pub status: u8,
    pub tlv_map: HashMap<u16, Bytes>,
    pub message: String,
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
                    .ok_or_else(|| RQError::Decode("missing 0x119".to_string()))?;
                LoginResponse::Success(LoginSuccess {
                    rollback_sig: tlv_map.remove(&0x161).map(decode_t161),
                    rand_seed: tlv_map.remove(&0x403),
                    ksid: t119.remove(&0x108),
                    account_info: t119.remove(&0x11a).map(read_t11a),
                    t512: t119.remove(&0x512).map(read_t512),
                    t402: tlv_map.remove(&0x402),
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
                })
            }
            2 => LoginResponse::NeedCaptcha(LoginNeedCaptcha {
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
                        image: img_data,
                    }
                }),
            }),
            40 => LoginResponse::AccountFrozen,
            160 | 239 => LoginResponse::DeviceLocked(LoginDeviceLocked {
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
                t402: tlv_map.remove(&0x402),
            }),
            162 => LoginResponse::TooManySMSRequest,
            204 => LoginResponse::DeviceLockLogin(LoginDeviceLockLogin {
                t104: tlv_map.remove(&0x104),
                t402: tlv_map.remove(&0x402),
                rand_seed: tlv_map.remove(&0x403),
            }),
            _ => {
                // status=1 可能是密码错误
                let mut _title = "".into();
                let mut message = "".into();
                if let Some(mut v) = tlv_map.remove(&0x146) {
                    v.advance(4);
                    _title = v.read_string_short();
                    message = v.read_string_short();
                }
                LoginResponse::UnknownStatus(LoginUnknownStatus {
                    status,
                    tlv_map,
                    message,
                })
            }
        };
        Ok(resp)
    }
}
