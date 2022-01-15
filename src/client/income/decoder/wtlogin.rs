use std::sync::atomic::Ordering;

use bytes::{Buf, BufMut, Bytes};

use crate::binary::BinaryReader;
use crate::client::income::decoder::tlv::*;
use crate::client::protocol::device::random_string;
use crate::client::Client;
use crate::{QEvent, RQError};

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
pub enum LoginResponse {
    Success,
    SliderNeededError {
        verify_url: String,
    },
    NeedCaptcha {
        captcha_sign: Bytes,
        captcha_image: Bytes,
    },
    UnknownLoginError {
        error_message: String,
    },
    SMSOrVerifyNeededError {
        verify_url: String,
        sms_phone: String,
        error_message: String,
    },
    SMSNeededError {
        sms_phone: String,
        error_message: String,
    },
    UnsafeDeviceError {
        verify_url: String,
    },
    TooManySMSRequestError,
    OtherLoginError {
        error_message: String,
    },
    NeedDeviceLockLogin,
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
        cli.handler
            .handle(QEvent::UinChanged(cli.uin.load(Ordering::SeqCst)))
            .await;
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

pub async fn decode_login_response(cli: &Client, payload: &[u8]) -> Result<LoginResponse, RQError> {
    let mut transport = cli.transport.write().await;
    let mut reader = Bytes::from(payload.to_owned());
    reader.get_u16(); // sub command
    let t = reader.get_u8();
    reader.get_u16();
    let mut m = reader.read_tlv_map(2);
    if m.contains_key(&0x402) {
        transport.sig.dpwd = random_string(16).into();
        transport.sig.t402 = m
            .remove(&0x402)
            .ok_or(RQError::Decode("missing 0x402".to_string()))?;
        let mut v = Vec::new();
        v.put_slice(&transport.sig.guid);
        v.put_slice(&transport.sig.dpwd);
        v.put_slice(&transport.sig.t402);
        transport.sig.g = md5::compute(&v).to_vec().into();
    }
    if t == 0 {
        let mut account_info = cli.account_info.write().await;
        let mut codec = cli.oicq_codec.write().await;
        // m.remove(&0x150).map(|v| transport.sig.t150 = v);
        m.remove(&0x161).map(|v| decode_t161(&v));
        m.remove(&0x403).map(|v| transport.sig.rand_seed = v);
        decode_t119(
            &m.remove(&0x119)
                .ok_or(RQError::Decode("missing 0x119".to_string()))?,
            &transport.sig.tgtgt_key.clone(),
            &mut transport,
            &mut account_info,
            &mut codec,
        )
        .await?;
        return Ok(LoginResponse::Success);
    }
    if t == 2 {
        transport.sig.t104 = m
            .remove(&0x104)
            .ok_or(RQError::Decode("missing 0x104".to_string()))?;
        if let Some(v) = m.remove(&0x192) {
            return Ok(LoginResponse::SliderNeededError {
                verify_url: String::from_utf8_lossy(&v).to_string(),
            });
        }
        if m.contains_key(&0x165) {
            let mut img_data = Bytes::from(
                m.remove(&0x105)
                    .ok_or(RQError::Decode("missing 0x105".to_string()))?,
            );
            let sign_len = img_data.get_u16();
            img_data.get_u16();
            let sign = img_data.copy_to_bytes(sign_len as usize);
            return Ok(LoginResponse::NeedCaptcha {
                captcha_sign: sign,
                captcha_image: img_data,
            });
        } else {
            return Ok(LoginResponse::UnknownLoginError {
                error_message: "".to_string(),
            });
        }
    } // need captcha

    if t == 40 {
        return Ok(LoginResponse::UnknownLoginError {
            error_message: "账号被冻结".to_string(),
        });
    }

    if t == 160 || t == 239 {
        if m.contains_key(&0x174) {
            transport.sig.t174 = m
                .remove(&0x174)
                .ok_or(RQError::Decode("missing 0x174".to_string()))?;
            transport.sig.t104 = m
                .remove(&0x104)
                .ok_or(RQError::Decode("missing 0x104".to_string()))?;
            transport.sig.rand_seed = m
                .remove(&0x403)
                .ok_or(RQError::Decode("missing 0x403".to_string()))?;
            let phone = {
                // let mut r = Bytes::from(m.remove(&0x178).unwrap());
                // let len = r.get_i32() as usize;
                // r.read_string_limit(len)
                "phone_num".to_string() // 这里有问题
            };
            if let Some(v) = m.get(&0x204) {
                return Ok(LoginResponse::SMSOrVerifyNeededError {
                    verify_url: String::from_utf8_lossy(v).to_string(),
                    sms_phone: phone,
                    error_message: String::from_utf8_lossy(
                        m.get(&0x17e)
                            .ok_or(RQError::Decode("missing 0x17e".to_string()))?,
                    )
                    .to_string(),
                });
            }
            return Ok(LoginResponse::SMSNeededError {
                sms_phone: phone,
                error_message: String::from_utf8_lossy(
                    m.get(&0x17e)
                        .ok_or(RQError::Decode("missing 0x17e".to_string()))?,
                )
                .to_string(),
            });
        }

        if m.contains_key(&0x17b) {
            transport.sig.t104 = m
                .remove(&0x104)
                .ok_or(RQError::Decode("missing 0x104".to_string()))?;
            return Ok(LoginResponse::SMSNeededError {
                sms_phone: "".to_string(),
                error_message: "".to_string(),
            });
        }
        if let Some(t204) = m.remove(&0x204) {
            return Ok(LoginResponse::UnsafeDeviceError {
                verify_url: String::from_utf8_lossy(&t204).to_string(),
            });
        }
    }

    if t == 162 {
        return Ok(LoginResponse::TooManySMSRequestError);
    }

    if t == 204 {
        {
            transport.sig.t104 = m
                .remove(&0x104)
                .ok_or(RQError::Decode("missing 0x104".to_string()))?;
            transport.sig.rand_seed = m
                .remove(&0x403)
                .ok_or(RQError::Decode("missing 0x403".to_string()))?;
        }
        return Ok(LoginResponse::NeedDeviceLockLogin);
    } // drive lock

    if let Some(mut t149r) = m.remove(&0x149) {
        t149r.advance(2);
        t149r.read_string_short(); //title
        return Ok(LoginResponse::OtherLoginError {
            error_message: t149r.read_string_short(),
        });
    }
    if let Some(mut t146r) = m.remove(&0x146) {
        t146r.advance(4); // ver and code
        t146r.read_string_short(); // title
        return Ok(LoginResponse::OtherLoginError {
            error_message: t146r.read_string_short(),
        });
    }
    return Err(RQError::Decode(
        "decode_login_response unknown error".to_string(),
    ));
}

pub async fn decode_exchange_emp_response(cli: &mut Client, payload: &[u8]) -> Result<(), RQError> {
    let mut transport = cli.transport.write().await;
    let mut payload = Bytes::from(payload.to_owned());
    let cmd = payload.get_u16();
    let t = payload.get_u8();
    payload.get_u16();
    let m = payload.read_tlv_map(2);
    if t != 0 {
        return Err(RQError::Decode(
            "decode_exchange_emp_response t != 0".to_string(),
        ));
    }
    if cmd == 15 {
        let mut account_info = cli.account_info.write().await;
        decode_t119r(
            m.get(&0x119)
                .ok_or(RQError::Decode("missing 0x119".to_string()))?,
            &transport.sig.tgtgt_key.clone(),
            &mut transport,
            &mut account_info,
        )
        .await;
    }
    if cmd == 11 {
        let mut account_info = cli.account_info.write().await;
        let mut codec = cli.oicq_codec.write().await;
        let h = md5::compute(&transport.sig.d2key).to_vec();
        decode_t119(
            m.get(&0x119)
                .ok_or(RQError::Decode("missing 0x119".to_string()))?,
            &h,
            &mut transport,
            &mut account_info,
            &mut codec,
        )
        .await?;
    }
    return Ok(());
}
