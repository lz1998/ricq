use crate::client::Client;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crate::binary_reader::BinaryReader;
use crate::binary_writer::BinaryWriter;
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

pub fn decode_trans_emp_response(cli:&mut Client, payload: &[u8]) -> Option<QRCodeLoginResponse> {
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


////// decodeExchangeEmpResponse


pub fn decode_exchange_emp_response(cli:&mut Client, payload: &[u8]) -> Option<QRCodeLoginResponse> {

    // reader := binary.NewReader(payload)
    let mut payload = Bytes::from(payload.to_owned());

    // cmd := reader.ReadUInt16()
    let cmd = payload.get_u16();

    // t := reader.ReadByte()
	let t = payload.get_u8();

    // reader.ReadUInt16()
    payload.get_u16();

	// m := reader.ReadTlvMap(2)
	let m = payload.read_tlv_map(2);

    // if t != 0 {
	// 	c.Error("exchange_emp error: %v", t)
	// 	return nil, errors.New("exchange_emp failed")
	// }
    if t != 0 {
        return None
    }

	// if cmd == 15 {
	// 	c.decodeT119R(m[0x119])
	// }
    if cmd == 15 {
        cli.decode_t119r(m[0x119])
    }

	// if cmd == 11 {
	// 	h := md5.Sum(c.sigInfo.d2Key)
	// 	c.decodeT119(m[0x119], h[:])
	// }
    if cmd == 11 {
        let h = md5::compute(&cli.sig_info.d2key);
        cli.decode_t119(m[0x119], &h)
    }

	// return nil, nil
    return None
}