use std::collections::HashMap;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::Utc;
use crate::binary::BinaryReader;
use crate::client::{AccountInfo, CacheInfo};
use crate::client::errors::RQError;
use crate::client::structs::LoginSigInfo;
use crate::crypto::qqtea_decrypt;


pub fn decode_t161(data: &[u8], cache_info: &mut CacheInfo) {
    let mut reader = Bytes::from(data.to_owned());
    reader.advance(2);
    let mut m = reader.read_tlv_map(2);
    m.remove(&0x172).map(|v| cache_info.rollback_sig = v);
}

pub fn decode_t119(data: &[u8], ek: &[u8], cache_info: &mut CacheInfo, account_info: &mut AccountInfo) -> Result<(), RQError> {
    let mut reader = Bytes::from(qqtea_decrypt(data, ek).to_owned());
    reader.advance(2);
    let mut m = reader.read_tlv_map(2);
    m.remove(&0x130).map(|v| decode_t130(&v, cache_info));
    m.remove(&0x113).map(|v| decode_t113(&v));
    m.remove(&0x528).map(|v| cache_info.t528 = v);
    m.remove(&0x530).map(|v| cache_info.t530 = v);
    m.remove(&0x108).map(|v| cache_info.ksid = v);

    let mut ps_key_map: HashMap<String, Bytes> = Default::default();
    let mut pt4token_map: HashMap<String, Bytes> = Default::default();

    if m.contains_key(&0x125) {
        // read_t125(t125)
    }
    m.remove(&0x186).map(|v| decode_t186(&v, cache_info));
    m.remove(&0x11a).map(|v| {
        let (nick, age, gender) = read_t11a(&v);
        account_info.nickname = nick;
        account_info.age = age;
        account_info.gender = gender;
    });
    if m.contains_key(&0x199) {
        // read_t199(t199)
    }
    if m.contains_key(&0x200) {
        // (pf, pf_key) = read_t200(t200)
    }
    m.remove(&0x512).map(|v| {
        let (a, b) = read_t512(&v);
        ps_key_map = a;
        pt4token_map = b;
    });
    if m.contains_key(&0x531) {
        // read_t531
    }

    if m.contains_key(&0x318) {
        // read_t138 // chg time
    }

    let sig_info = LoginSigInfo {
        login_bitmap: 0,
        srm_token: select(m.get(&0x16a), &cache_info.sig_info.srm_token).into(),
        t133: select(m.get(&0x133), &cache_info.sig_info.t133),
        encrypted_a1: select(m.get(&0x106), &cache_info.sig_info.encrypted_a1),
        tgt: m.remove(&0x10a).ok_or(RQError::Decode("missing 0x10a".into()))?,
        tgt_key: m.remove(&0x10d).ok_or(RQError::Decode("missing 0x10d".into()))?,
        user_st_key: m.remove(&0x10e).ok_or(RQError::Decode("missing 0x10e".into()))?,
        user_st_web_sig: m.remove(&0x103).ok_or(RQError::Decode("missing 0x103".into()))?,
        s_key: m.remove(&0x120).ok_or(RQError::Decode("missing 0x120".into()))?,
        s_key_expired_time: Utc::now().timestamp() + 21600,
        d2: m.remove(&0x143).ok_or(RQError::Decode("missing 0x143".into()))?,
        d2key: m.remove(&0x305).ok_or(RQError::Decode("missing 0x305".into()))?,
        wt_session_ticket_key: select(m.get(&0x134), &cache_info.sig_info.wt_session_ticket_key),
        device_token: m.remove(&0x322),

        ps_key_map,
        pt4token_map,
    };
    cache_info.sig_info = sig_info;
    Ok(())
}


pub fn decode_t119r(data: &[u8], tgtgt_key: &[u8], cache_info: &mut CacheInfo, account_info: &mut AccountInfo) {
    let mut reader = Bytes::from(qqtea_decrypt(&data, tgtgt_key).to_owned());
    reader.advance(2);
    let mut m = reader.read_tlv_map(2);
    m.remove(&0x120).map(|v| {
        cache_info.sig_info.s_key = v;
        cache_info.sig_info.s_key_expired_time = Utc::now().timestamp() + 21600;
    });
    m.remove(&0x11a).map(|v| {
        let (nick, age, gender) = read_t11a(&v);
        account_info.nickname = nick;
        account_info.age = age;
        account_info.gender = gender;
    });
}

pub fn decode_t130(data: &[u8], cache_info: &mut CacheInfo) {
    let mut reader = Bytes::from(data.to_owned());
    reader.advance(2);
    cache_info.time_diff = reader.get_i32() as i64 - Utc::now().timestamp();
    cache_info.t149 = reader.copy_to_bytes(4)
}

pub fn decode_t113(data: &[u8]) {
    let mut reader = Bytes::from(data.to_owned());
    let uin = reader.get_i32();
    println!("got t133 uin: {}", uin)
}

pub fn decode_t186(data: &[u8], cache_info: &mut CacheInfo) {
    cache_info.pwd_flag = data[1] == 1;
}

// --- tlv readers ---

// not used
pub fn read_t125(data: &[u8]) -> (Bytes, Bytes) {
    let mut reader = Bytes::from(data.to_owned());
    let open_id = reader.read_bytes_short();
    let open_key = reader.read_bytes_short();
    (open_id, open_key)
}

pub fn read_t11a(data: &[u8]) -> (String, u16, u16) {
    let mut reader = Bytes::from(data.to_owned());
    reader.get_u16();
    let age = reader.get_u8() as u16;
    let gender = reader.get_u8() as u16;
    let limit = (reader.get_u8() & 0xff) as usize;
    let nick = reader.read_string_limit(limit);
    return (nick, age, gender);
}

// not used
pub fn read_t199(data: &[u8]) -> (Bytes, Bytes) {
    let mut reader = Bytes::from(data.to_owned());
    let open_id = reader.read_bytes_short();
    let pay_token = reader.read_bytes_short();
    (open_id, pay_token)
}

// not used
pub fn read_t200(data: &[u8]) -> (Bytes, Bytes) {
    let mut reader = Bytes::from(data.to_owned());
    let pf = reader.read_bytes_short();
    let pf_key = reader.read_bytes_short();
    (pf, pf_key)
}

pub fn read_t512(data: &[u8]) -> (HashMap<String, Bytes>, HashMap<String, Bytes>) {
    let mut reader = Bytes::from(data.to_owned());
    let length = reader.get_u16() as usize;

    let mut ps_key_map: HashMap<String, Bytes> = HashMap::with_capacity(length);
    let mut pt4_token_map: HashMap<String, Bytes> = HashMap::with_capacity(length);

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
pub fn read_t531(data: &[u8]) -> (Bytes, Bytes) {
    let mut reader = Bytes::from(data.to_owned());
    let mut m = reader.read_tlv_map(2);
    let mut a1 = BytesMut::new();
    let mut no_pic_sig = Bytes::new();
    if [0x103, 0x16a, 0x113, 0x10c].iter().all(|v| m.contains_key(v)) {
        a1.put_slice(&m.remove(&0x106).unwrap());
        a1.put_slice(&m.remove(&0x10c).unwrap());
        no_pic_sig = Bytes::from(m.remove(&0x16a).unwrap());
    }
    return (a1.into(), no_pic_sig);
}

pub fn select(a: Option<&Bytes>, b: &[u8]) -> Bytes {
    return match a {
        None => { Bytes::from(b.to_owned()) }
        Some(a) => { Bytes::from(a.to_vec()) }
    };
}