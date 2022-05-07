use std::collections::HashMap;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::binary::BinaryReader;
use crate::crypto::qqtea_decrypt;

#[derive(Debug, Clone)]
pub struct T161 {
    // 172
    pub rollback_sig: Option<Bytes>,
    // 173
    // 17f
}
#[derive(Debug, Clone)]
pub struct T113 {
    pub uin: i32,
}
#[derive(Debug, Clone)]
pub struct T125 {
    pub open_id: Bytes,
    pub open_key: Bytes,
}
#[derive(Debug, Clone, Default)]
pub struct T11A {
    pub face: u16,
    pub gender: u8,
    pub age: u8,
    pub nick: String,
}
#[derive(Debug, Clone)]
pub struct T199 {
    pub open_id: Bytes,
    pub pay_token: Bytes,
}
#[derive(Debug, Clone)]
pub struct T200 {
    pub pf: Bytes,
    pub pf_key: Bytes,
}
#[derive(Debug, Clone)]
pub struct T512 {
    pub ps_key_map: HashMap<String, Bytes>,
    pub pt4_token_map: HashMap<String, Bytes>,
}
#[derive(Debug, Clone)]
pub struct T531 {
    pub a1: Bytes,
    pub no_pic_sig: Bytes,
}

pub fn decode_t161(mut data: Bytes) -> T161 {
    data.advance(2);
    let mut m = data.read_tlv_map(2);
    T161 {
        rollback_sig: m.remove(&0x172),
    }
}

pub fn decode_t119(data: &[u8], ek: &[u8]) -> HashMap<u16, Bytes> {
    let mut reader = Bytes::from(qqtea_decrypt(data, ek));
    reader.advance(2);
    reader.read_tlv_map(2)
}

pub fn decode_t113(mut data: Bytes) -> T113 {
    T113 {
        uin: data.get_i32(),
    }
}

pub fn decode_t186(_: &[u8]) {}

// not used
pub fn read_t125(data: &[u8]) -> T125 {
    let mut reader = Bytes::from(data.to_owned());
    let open_id = reader.read_bytes_short();
    let open_key = reader.read_bytes_short();
    T125 { open_id, open_key }
}

pub fn read_t11a(mut data: Bytes) -> T11A {
    let face = data.get_u16();
    let age = data.get_u8();
    let gender = data.get_u8();
    let limit = (data.get_u8() & 0xff) as usize;
    let nick = data.read_string_limit(limit);
    T11A {
        face,
        age,
        gender,
        nick,
    }
}

pub fn read_t199(mut data: Bytes) -> T199 {
    let open_id = data.read_bytes_short();
    let pay_token = data.read_bytes_short();
    T199 { open_id, pay_token }
}

pub fn read_t200(mut data: Bytes) -> T200 {
    let pf = data.read_bytes_short();
    let pf_key = data.read_bytes_short();
    T200 { pf, pf_key }
}

pub fn read_t512(mut reader: Bytes) -> T512 {
    let length = reader.get_u16() as usize;

    let mut ps_key_map: HashMap<String, Bytes> = HashMap::with_capacity(length);
    let mut pt4_token_map: HashMap<String, Bytes> = HashMap::with_capacity(length);

    for _ in 0..length {
        let domain = reader.read_string_short();
        let ps_key = reader.read_bytes_short();
        let ps4_token = reader.read_bytes_short();

        if !ps_key.is_empty() {
            ps_key_map.insert(domain.clone(), ps_key);
        }

        if !ps4_token.is_empty() {
            pt4_token_map.insert(domain, ps4_token);
        }
    }
    T512 {
        ps_key_map,
        pt4_token_map,
    }
}

pub fn read_t531(mut data: Bytes) -> T531 {
    let mut m = data.read_tlv_map(2);
    let mut a1 = BytesMut::new();
    let mut no_pic_sig = Bytes::new();
    if [0x16a, 0x16a, 0x10c].iter().all(|v| m.contains_key(v)) {
        a1.put_slice(&m.remove(&0x106).unwrap());
        a1.put_slice(&m.remove(&0x10c).unwrap());
        no_pic_sig = m.remove(&0x16a).unwrap();
    }
    T531 {
        a1: a1.freeze(),
        no_pic_sig,
    }
}

pub fn select(a: Option<&Bytes>, b: &[u8]) -> Bytes {
    match a {
        None => Bytes::from(b.to_owned()),
        Some(a) => Bytes::from(a.to_vec()),
    }
}
