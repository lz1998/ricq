use std::collections::HashMap;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use rand::Rng;
use crate::device::{DeviceInfo, Version};
use crate::encrypt::EncryptECDH;
use crate::version::{ClientProtocol, gen_version_info, VersionInfo};

pub struct Client {
    seq_id: Arc<AtomicU16>,
    pub uin: i64,
    pub ecdh: EncryptECDH,
    //随机16位
    pub random_key: Vec<u8>,
    pub version: VersionInfo,
    pub device_info: DeviceInfo,
    pub out_going_packet_session_id: Vec<u8>,
    pub ksid: Vec<u8>,

    // account info
    pub nickname: String,
    pub age: u16,
    pub gender: u16,

    // tlv cache
    pub t104: Vec<u8>,
    pub t174: Vec<u8>,
    pub g: Vec<u8>,
    pub t402: Vec<u8>,
    pub t150: Vec<u8>,
    pub t149: Vec<u8>,
    pub t528: Vec<u8>,
    pub t530: Vec<u8>,
    pub rand_seed: Vec<u8>, // t403
    pub rollback_sig: Vec<u8>,

    // session info
    pub sig_info: LoginSigInfo,
    pub dpwd: Vec<u8>,
    pub time_diff: i64,
    pub pwd_flag: bool,

}

impl Client {
    pub fn new() -> Client {
        Client {
            seq_id: Arc::new(AtomicU16::new(0x3635)),
            uin: 0,
            ecdh: EncryptECDH::new(),
            random_key: Vec::from(rand::thread_rng().gen::<[u8; 16]>()),
            version: gen_version_info(&ClientProtocol::IPad),
            device_info: DeviceInfo::random(),
            out_going_packet_session_id: vec![0x02, 0xB0, 0x5B, 0x8B],
            ksid: vec![],
            nickname: "".to_string(),
            age: 0,
            gender: 0,
            t104: vec![],
            t174: vec![],
            g: vec![],
            t402: vec![],
            t150: vec![],
            t149: vec![],
            t528: vec![],
            t530: vec![],
            rand_seed: vec![],
            rollback_sig: vec![],
            sig_info: LoginSigInfo::default(),
            dpwd: vec![],
            time_diff: 0,
            pwd_flag: false
        }
    }
    pub fn next_seq(&mut self) -> u16 {
        self.seq_id.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Default)]
pub struct LoginSigInfo {
    pub login_bitmap: u64,
    pub tgt: Vec<u8>,
    pub tgt_key: Vec<u8>,
    // study room manager | 0x16a
    pub srm_token: Vec<u8>,
    pub t133: Vec<u8>,
    pub encrypted_a1: Vec<u8>,
    pub user_st_key: Vec<u8>,
    pub user_st_web_sig: Vec<u8>,
    pub s_key: Vec<u8>,
    pub s_key_expired_time: i64,
    pub d2: Vec<u8>,
    pub d2key: Vec<u8>,
    pub wt_session_ticket_key: Vec<u8>,
    pub device_token: Vec<u8>,
    pub ps_key_map: HashMap<String, Vec<u8>>,
    pub pt4token_map: HashMap<String, Vec<u8>>,
}

pub struct QiDianAccountInfo {
    pub master_uin: i64,
    pub ext_name: String,
    pub create_time: i64,

    pub big_data_req_addrs: Vec<String>,
    pub big_data_req_session: BigDataReqSessionInfo
}

pub struct BigDataReqSessionInfo {
    pub sig_session: Vec<u8>,
    pub session_key: Vec<u8>
}