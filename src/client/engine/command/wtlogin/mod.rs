mod builder;
mod decoder;
pub mod tlv_reader;
pub mod tlv_writer;

use crate::client::engine::command::wtlogin::tlv_reader::*;
use crate::{RQError, RQResult};
use bytes::{Buf, Bytes};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum QRCodeState {
    QRCodeImageFetch {
        image_data: Bytes,
        sig: Bytes,
    },
    QRCodeWaitingForScan,
    QRCodeWaitingForConfirm,
    QRCodeTimeout,
    QRCodeConfirmed {
        uin: i64,
        tmp_pwd: Bytes,
        tmp_no_pic_sig: Bytes,
        tgt_qr: Bytes,
        tgtgt_key: Bytes,
    },
    QRCodeCanceled,
}
#[derive(Debug, Clone)]
pub struct ImageCaptcha {
    pub sign: Bytes,
    pub image: Bytes,
}

#[derive(Debug, Clone)]
pub enum LoginResponse {
    Success {
        rollback_sig: Option<T161>,
        rand_seed: Option<Bytes>,
        ksid: Option<Bytes>,
        account_info: Option<T11A>,
        t512: Option<T512>,
        // 不知道有没有 t402
        t402: Option<Bytes>,
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
    // slider or image captcha
    NeedCaptcha {
        t104: Option<Bytes>,
        verify_url: Option<String>,
        image_captcha: Option<ImageCaptcha>,
    },
    AccountFrozen,
    // sms or qrcode
    DeviceLocked {
        t104: Option<Bytes>,
        t174: Option<Bytes>,
        t402: Option<Bytes>,
        sms_phone: Option<String>,
        verify_url: Option<String>,
        message: Option<String>,
        rand_seed: Option<Bytes>,
    },
    TooManySMSRequest,
    // More login packet needed
    DeviceLockLogin {
        t104: Option<Bytes>,
        t402: Option<Bytes>,
        rand_seed: Option<Bytes>,
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
                        image: img_data,
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
                t402: tlv_map.remove(&0x402),
            },
            162 => LoginResponse::TooManySMSRequest,
            204 => LoginResponse::DeviceLockLogin {
                t104: tlv_map.remove(&0x104),
                t402: tlv_map.remove(&0x402),
                rand_seed: tlv_map.remove(&0x403),
            },
            _ => LoginResponse::UnknownLoginStatus { status, tlv_map },
        };
        Ok(resp)
    }
}
