use crate::client::Client;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::binary_reader::BinaryReader;
use crate::binary_writer::BinaryWriter;
use crate::device::random_string;
use crate::tlv_decoder::TlvDecoder;

#[derive(Debug)]
pub enum LoginState {
    QRCodeImageFetch,
    QRCodeWaitingForScan,
    QRCodeWaitingForConfirm,
    QRCodeTimeout,
    QRCodeConfirmed,
    QRCodeCanceled,
}

#[derive(Debug, Default)]
pub struct QRCodeLoginInfo {
    pub tmp_pwd: Vec<u8>,
    pub tmp_no_pic_sig: Vec<u8>,
    pub tgt_qr: Vec<u8>,
}

#[derive(Debug)]
pub struct QRCodeLoginResponse {
    pub image_data: Vec<u8>,
    pub sig: Vec<u8>,
    pub state: LoginState,
    pub login_info: QRCodeLoginInfo,
}

#[derive(Debug)]
pub enum LoginResponse {
    Success,
    SliderNeededError {
        verify_url: String,
    },
    NeedCaptcha {
        captcha_sign: Vec<u8>,
        captcha_image: Vec<u8>,
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
        verify_url: String
    },
    TooManySMSRequestError,
    OtherLoginError {
        error_message: String
    },
}


pub fn decode_trans_emp_response(cli: &mut Client, payload: &[u8]) -> Option<QRCodeLoginResponse> {
    if payload.len() < 48 {
        return None;
    }
    let mut payload = Bytes::from(payload.to_owned());
    payload.advance(5);// trans req head
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
            return None;
        }
        let sig = body.read_bytes_short();
        body.get_u16();
        let mut m = body.read_tlv_map(2);
        if m.contains_key(&0x17) {
            return Some(QRCodeLoginResponse {
                image_data: m.remove(&0x17).unwrap(),
                sig,
                state: LoginState::QRCodeImageFetch,
                login_info: QRCodeLoginInfo::default(),
            });
        }
    }
    if cmd == 0x12 {
        let mut a_var_len = body.get_u16();
        if a_var_len != 0 {
            a_var_len -= 1; // 阴间的位移操作
            if body.get_u8() == 2 {
                body.get_i64();//uin?
                a_var_len -= 8;
            }
        }
        if a_var_len > 0 {
            body.advance(a_var_len as usize);
        }
        body.get_i32();
        let code = body.get_u8();
        if code != 0 {
            match code {
                0x30 => {
                    return Some(QRCodeLoginResponse { image_data: vec![], sig: vec![], state: LoginState::QRCodeWaitingForScan, login_info: Default::default() });
                }
                0x35 => {
                    return Some(QRCodeLoginResponse { image_data: vec![], sig: vec![], state: LoginState::QRCodeWaitingForConfirm, login_info: Default::default() });
                }
                0x36 => {
                    return Some(QRCodeLoginResponse { image_data: vec![], sig: vec![], state: LoginState::QRCodeCanceled, login_info: Default::default() });
                }
                0x11 => {
                    return Some(QRCodeLoginResponse { image_data: vec![], sig: vec![], state: LoginState::QRCodeTimeout, login_info: Default::default() });
                }
                _ => { return None; }
            }
        }
        cli.uin = body.get_i64();
        body.get_i32(); // sig create time
        body.get_u16();
        let mut m = body.read_tlv_map(2);
        if !m.contains_key(&0x18) || !m.contains_key(&0x1e) || !m.contains_key(&0x19) {
            return None;
        }
        cli.device_info.tgtgt_key = m.remove(&0x1e).unwrap();
        return Some(QRCodeLoginResponse {
            image_data: vec![],
            sig: vec![],
            state: LoginState::QRCodeConfirmed,
            login_info: QRCodeLoginInfo {
                tmp_pwd: m.remove(&0x18).unwrap(),
                tmp_no_pic_sig: m.remove(&0x19).unwrap(),
                tgt_qr: m.remove(&0x65).unwrap(),
            },
        });
    }
    return None;
}

pub fn decode_login_response(cli: &mut Client, payload: &[u8]) -> Option<LoginResponse> {
    let mut reader = Bytes::from(payload.to_owned());
    reader.get_u16(); // sub command
    let t = reader.get_u8();
    reader.get_u16();
    let mut m = reader.read_tlv_map(2);
    if m.contains_key(&0x402) {
        cli.dpwd = random_string(16).into_bytes();
        cli.t402 = m.remove(&0x402).unwrap();
        let mut v: Vec<u8> = Vec::new();
        v.put_slice(&cli.device_info.guid);
        v.put_slice(&cli.dpwd);
        v.put_slice(&cli.t402);
        cli.g = md5::compute(&v).to_vec();
    }
    if t == 0 {
        if m.contains_key(&0x150) {
            cli.t150 = m.remove(&0x150).unwrap();
        }
        if m.contains_key(&0x161) {
            cli.decode_t161(&m.remove(&0x161).unwrap())
        }
        if m.contains_key(&0x403) {
            cli.rand_seed = m.remove(&0x403).unwrap();
        }
        // TODO
        cli.decode_t119(&m.get(&0x119).unwrap(), &cli.device_info.tgtgt_key.clone());
        return Some(LoginResponse::Success);
    }
    if t == 2 {
        cli.t104 = m.remove(&0x104).unwrap();
        if m.contains_key(&0x192) {
            return Some(LoginResponse::SliderNeededError {
                verify_url: String::from_utf8(m.remove(&0x192).unwrap()).unwrap(),
            });
        }
        if m.contains_key(&0x165) {
            let mut img_data = Bytes::from(m.remove(&0x105).unwrap());
            let sign_len = img_data.get_u16();
            img_data.get_u16();
            let sign = img_data.copy_to_bytes(sign_len as usize).to_vec();
            return Some(LoginResponse::NeedCaptcha {
                captcha_sign: sign,
                captcha_image: img_data.chunk().to_vec(),
            });
        } else {
            return Some(LoginResponse::UnknownLoginError {
                error_message: "".to_string()
            });
        }
    } // need captcha

    if t == 40 {
        return Some(LoginResponse::UnknownLoginError {
            error_message: "账号被冻结".to_string(),
        });
    }

    if t == 160 || t == 239 {
        if m.contains_key(&0x174) {
            cli.t174 = m.remove(&0x147).unwrap();
            cli.t104 = m.remove(&0x104).unwrap();
            cli.rand_seed = m.remove(&0x403).unwrap();
            let phone = {
                let mut r = Bytes::from(m.remove(&0x178).unwrap());
                let len = r.get_i32() as usize;
                r.read_string_limit(len)
            };
            if m.contains_key(&0x204) {
                return Some(LoginResponse::SMSOrVerifyNeededError {
                    verify_url: String::from_utf8(m.remove(&0x204).unwrap()).unwrap(),
                    sms_phone: phone,
                    error_message: String::from_utf8(m.remove(&0x17e).unwrap()).unwrap(),
                });
            }
            return Some(LoginResponse::SMSNeededError {
                sms_phone: phone,
                error_message: String::from_utf8(m.remove(&0x17e).unwrap()).unwrap(),
            });
        }

        if m.contains_key(&0x17b) {
            cli.t104 = m.remove(&0x104).unwrap();
            return Some(LoginResponse::SMSNeededError {
                sms_phone: "".to_string(),
                error_message: "".to_string(),
            });
        }

        if m.contains_key(&0x204) {
            return Some(LoginResponse::UnsafeDeviceError {
                verify_url: String::from_utf8(m.remove(&0x204).unwrap()).unwrap(),
            });
        }
    }

    if t == 162 {
        return Some(LoginResponse::TooManySMSRequestError);
    }

    if t == 204 {
        cli.t104 = m.remove(&0x104).unwrap();
        cli.rand_seed = m.remove(&0x403).unwrap();
        // TODO c.sendAndWait(c.buildDeviceLockLoginPacket())
        return None;
    } // drive lock

    if m.contains_key(&0x149) {
        let mut t149r = Bytes::from(m.remove(&0x149).unwrap());
        t149r.advance(2);
        t149r.read_string_short();//title
        return Some(LoginResponse::OtherLoginError {
            error_message: t149r.read_string_short(),
        });
    }

    if m.contains_key(&0x146) {
        let mut t146r = Bytes::from(m.remove(&0x146).unwrap());
        t146r.advance(4); // ver and code
        t146r.read_string_short(); // title
        return Some(LoginResponse::OtherLoginError {
            error_message: t146r.read_string_short(),
        });
    }
    return None;
}


pub fn decode_exchange_emp_response(cli: &mut Client, payload: &[u8]) -> Option<QRCodeLoginResponse> {
    let mut payload = Bytes::from(payload.to_owned());
    let cmd = payload.get_u16();
    let t = payload.get_u8();
    payload.get_u16();
    let m = payload.read_tlv_map(2);
    if t != 0 {
        return None;
    }
    if cmd == 15 {
        cli.decode_t119r(m.get(&0x119).unwrap())
    }
    if cmd == 11 {
        let h = md5::compute(&cli.sig_info.d2key).to_vec();
        cli.decode_t119(m.get(&0x119).unwrap(), &h)
    }
    return None;
}