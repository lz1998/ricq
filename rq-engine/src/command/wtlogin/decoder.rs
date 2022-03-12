use bytes::{Buf, Bytes};

use crate::binary::BinaryReader;
use crate::command::wtlogin::{LoginResponse, QRCodeConfirmed, QRCodeImageFetch, QRCodeState};
use crate::{RQError, RQResult};

impl super::super::super::Engine {
    pub fn decode_trans_emp_response(&self, mut payload: Bytes) -> RQResult<QRCodeState> {
        if payload.len() < 48 {
            return Err(RQError::Decode("invalid payload length".into()));
        }
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
        let mut body = payload.copy_to_bytes(len);
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
                return Ok(QRCodeState::ImageFetch(QRCodeImageFetch {
                    image_data: m
                        .remove(&0x17)
                        .ok_or_else(|| RQError::Decode("missing 0x17".into()))?,
                    sig,
                }));
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
                    0x30 => Ok(QRCodeState::WaitingForScan),
                    0x35 => Ok(QRCodeState::WaitingForConfirm),
                    0x36 => Ok(QRCodeState::Canceled),
                    0x11 => Ok(QRCodeState::Timeout),
                    _ => Err(RQError::Decode("invalid body code".to_string())),
                };
            }
            let uin = body.get_i64();
            body.get_i32(); // sig create time
            body.get_u16();
            let mut m = body.read_tlv_map(2);
            return Ok(QRCodeState::Confirmed(QRCodeConfirmed {
                uin,
                tmp_pwd: m
                    .remove(&0x18)
                    .ok_or_else(|| RQError::Decode("missing 0x18".into()))?,
                tmp_no_pic_sig: m
                    .remove(&0x19)
                    .ok_or_else(|| RQError::Decode("missing 0x19".into()))?,
                tgt_qr: m
                    .remove(&0x65)
                    .ok_or_else(|| RQError::Decode("missing 0x65".into()))?,
                tgtgt_key: m
                    .remove(&0x1e)
                    .ok_or_else(|| RQError::Decode("missing 0x1e".into()))?,
            }));
        }
        Err(RQError::Decode(
            "decode_trans_emp_response unknown error".to_string(),
        ))
    }

    pub fn decode_login_response(&self, mut reader: Bytes) -> RQResult<LoginResponse> {
        let _sub_command = reader.get_u16(); // sub command
        let status = reader.get_u8();
        // TODO status=213 不能执行下面的步骤 panic
        reader.get_u16();
        let tlv_map = reader.read_tlv_map(2);
        LoginResponse::decode(status, tlv_map, &self.transport.sig.tgtgt_key)
    }

    pub fn decode_exchange_emp_response(&self, mut payload: Bytes) -> RQResult<LoginResponse> {
        let sub_command = payload.get_u16();
        let status = payload.get_u8();
        payload.get_u16();
        let tlv_map = payload.read_tlv_map(2);
        if status != 0 {
            return Err(RQError::Decode(
                "decode_exchange_emp_response status != 0".to_string(),
            ));
        }
        let encrypt_key = if sub_command == 11 {
            md5::compute(&self.transport.sig.d2key).to_vec()
        } else {
            self.transport.sig.tgtgt_key.to_vec()
        };

        LoginResponse::decode(status, tlv_map, &encrypt_key)
    }
}
