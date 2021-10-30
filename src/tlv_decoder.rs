use std::collections::HashMap;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::Utc;
use crate::binary_reader::BinaryReader;
use crate::client::{Client, LoginSigInfo};
use crate::tea::qqtea_decrypt;

pub trait TlvDecoder {
    fn decode_t161(&mut self, data: &[u8]);
    fn decode_t119(&mut self, data: &[u8], ek: &[u8]);
    fn decode_t119r(&mut self, data: &[u8]);
    fn decode_t130(&mut self, data: &[u8]);
    fn decode_t113(&mut self, data: &[u8]);
    fn decode_t186(&mut self, data: &[u8]);
}

impl TlvDecoder for Client {
    fn decode_t161(&mut self, data: &[u8]) {
        let mut reader = Bytes::from(data.to_owned());
        reader.advance(2);
        let mut m = reader.read_tlv_map(2);
        if m.contains_key(&0x172) {
            self.rollback_sig = m.remove(&0x172).unwrap();
        }
    }

    fn decode_t119(&mut self, data: &[u8], ek: &[u8]) {
        let mut reader = Bytes::from(qqtea_decrypt(data, ek).to_owned());
        reader.advance(2);
        let mut m = reader.read_tlv_map(2);
        if m.contains_key(&0x130) {
            self.decode_t130(&m.remove(&0x130).unwrap())
        }
        if m.contains_key(&0x113) {
            self.decode_t113(&m.remove(&0x113).unwrap())
        }
        if m.contains_key(&0x528) {
            self.t528 = m.remove(&0x528).unwrap()
        }
        if m.contains_key(&0x530) {
            self.t530 = m.remove(&0x530).unwrap()
        }
        if m.contains_key(&0x108) {
            self.ksid = m.remove(&0x108).unwrap()
        }


        let mut ps_key_map: HashMap<String, Vec<u8>> = Default::default();
        let mut pt4token_map: HashMap<String, Vec<u8>> = Default::default();

        if m.contains_key(&0x125) {
            // read_t125(t125)
        }
        if m.contains_key(&0x186) {
            self.decode_t186(&m.remove(&0x186).unwrap())
        }
        if m.contains_key(&0x11a) {
            let (nick, age, gender) = read_t11a(&m.remove(&0x11a).unwrap());
            self.nickname = nick;
            self.age = age;
            self.gender = gender;
        }
        if m.contains_key(&0x199) {
            // read_t199(t199)
        }
        if m.contains_key(&0x200) {
            // (pf, pf_key) = read_t200(t200)
        }
        if m.contains_key(&0x512) {
            let (a, b) = read_t512(&m.remove(&0x512).unwrap());
            ps_key_map = a;
            pt4token_map = b;
        }
        if m.contains_key(&0x531) {
            // read_t531
        }

        if m.contains_key(&0x318) {
            // read_t138 // chg time
        }

        self.sig_info = LoginSigInfo {
            login_bitmap: 0,
            srm_token: select(m.get(&0x16a), &self.sig_info.srm_token),
            t133: select(m.get(&0x133), &self.sig_info.t133),
            encrypted_a1: select(m.get(&0x106), &self.sig_info.encrypted_a1),
            tgt: m.remove(&0x10a).unwrap(),
            tgt_key: m.remove(&0x10d).unwrap(),
            user_st_key: m.remove(&0x10e).unwrap(),
            user_st_web_sig: m.remove(&0x103).unwrap(),
            s_key: m.remove(&0x120).unwrap(),
            s_key_expired_time: Utc::now().timestamp() + 21600,
            d2: m.remove(&0x143).unwrap(),
            d2key: m.remove(&0x305).unwrap(),
            wt_session_ticket_key: select(m.get(&0x134), &self.sig_info.wt_session_ticket_key),
            device_token: m.remove(&0x322),

            ps_key_map,
            pt4token_map,
        };
    }

    fn decode_t119r(&mut self, data: &[u8]) {
        let mut reader = Bytes::from(qqtea_decrypt(&data, &self.device_info.tgtgt_key).to_owned());
        reader.advance(2);
        let mut m = reader.read_tlv_map(2);
        if m.contains_key(&0x120) {
            self.sig_info.s_key = m.remove(&0x120).unwrap();
            self.sig_info.s_key_expired_time = Utc::now().timestamp() + 21600;
        }
        if m.contains_key(&0x11a) {
            let (nick, age, gender)=read_t11a(&m.remove(&0x11a).unwrap());
            self.nickname = nick;
            self.age = age;
            self.gender = gender;
        }
    }

    fn decode_t130(&mut self, data: &[u8]) {
        let mut reader = Bytes::from(data.to_owned());
        reader.advance(2);
        self.time_diff = reader.get_i32() as i64 - Utc::now().timestamp();
        self.t149 = reader.copy_to_bytes(4).to_vec()
    }

    fn decode_t113(&mut self, data: &[u8]) {
        let mut reader = Bytes::from(data.to_owned());
        let uin = reader.get_i32();
        println!("got t133 uin: {}", uin)
    }

    fn decode_t186(&mut self, data: &[u8]) {
        self.pwd_flag = data[1] == 1
    }
}

// --- tlv readers ---

// not used
fn read_t125(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut reader = Bytes::from(data.to_owned());
    let open_id = reader.read_bytes_short();
    let open_key = reader.read_bytes_short();
    (open_id, open_key)
}

fn read_t11a(data: &[u8]) -> (String, u16, u16) {
    let mut reader = Bytes::from(data.to_owned());
    reader.get_u16();
    let age = reader.get_u8() as u16;
    let gender = reader.get_u8() as u16;
    let limit = (reader.get_u8() & 0xff) as usize;
    let nick = reader.read_string_limit(limit);
    return (nick, age, gender);
}

// not used
fn read_t199(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut reader = Bytes::from(data.to_owned());
    let open_id = reader.read_bytes_short();
    let pay_token = reader.read_bytes_short();
    (open_id, pay_token)
}

// not used
fn read_t200(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut reader = Bytes::from(data.to_owned());
    let pf = reader.read_bytes_short();
    let pf_key = reader.read_bytes_short();
    (pf, pf_key)
}

fn read_t512(data: &[u8]) -> (HashMap<String, Vec<u8>>, HashMap<String, Vec<u8>>) {
    let mut reader = Bytes::from(data.to_owned());
    let length = reader.get_u16() as usize;

    let mut ps_key_map: HashMap<String, Vec<u8>> = HashMap::with_capacity(length);
    let mut pt4_token_map: HashMap<String, Vec<u8>> = HashMap::with_capacity(length);

    for _ in 0..length {
        let domain = reader.read_string_short();
        let ps_key = reader.read_bytes_short();
        let ps4_token = reader.read_bytes_short();

        if ps_key.len() > 0 {
            ps_key_map.insert(domain.clone(), ps_key);
        }

        if ps4_token.len() > 0 {
            pt4_token_map.insert(domain, ps4_token);
        }
    }

    (ps_key_map, pt4_token_map)
}

// not used
fn read_t531(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut reader = Bytes::from(data.to_owned());
    let mut m = reader.read_tlv_map(2);
    let mut a1=Vec::new();
    let mut no_pic_sig =Vec::new();
    if m.contains_key(&0x103) && m.contains_key(&0x16a) && m.contains_key(&0x113) && m.contains_key(&0x10c) {
        a1.put_slice(&m.remove(&0x106).unwrap());
        a1.put_slice(&m.remove(&0x10c).unwrap());
        no_pic_sig = m.remove(&0x16a).unwrap();
    }
    return (a1,no_pic_sig);
}

fn select(a: Option<&Vec<u8>>, b: &[u8]) -> Vec<u8> {
    return match a {
        None => { b.to_vec() }
        Some(a) => { a.to_vec() }
    };
}