use std::collections::HashMap;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use rand::Rng;
use crate::device::{DeviceInfo, Version};
use crate::encrypt::EncryptECDH;
use crate::version::{ClientProtocol, gen_version_info, VersionInfo};

pub struct Client {
    seq_id: Arc<AtomicU16>,
    pub ecdh: EncryptECDH,
    //随机16位
    pub random_key: Vec<u8>,
    pub version: VersionInfo,
    pub device_info: DeviceInfo,
    pub out_going_packet_session_id: Vec<u8>,
    pub ksid: Vec<u8>,
    pub sig_info: LoginSigInfo,
}

impl Client {
    pub fn new() -> Client {
        Client {
            seq_id: Arc::new(AtomicU16::new(0x3635)),
            ecdh: EncryptECDH::new(),
            random_key: Vec::from(rand::thread_rng().gen::<[u8; 16]>()),
            version: gen_version_info(&ClientProtocol::IPad),
            device_info: DeviceInfo::random(),
            out_going_packet_session_id: vec![0x02, 0xB0, 0x5B, 0x8B],
            ksid: vec![],
            sig_info: LoginSigInfo::default(),
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