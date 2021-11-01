use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU16, Ordering};
use std::sync::Arc;
use bytes::Bytes;
use rand::Rng;
use tokio::sync::RwLock;
use crate::client_packet::IncomingPacket;
use crate::device::{DeviceInfo, Version};
use crate::encrypt::EncryptECDH;
use crate::net;
use crate::version::{ClientProtocol, gen_version_info, VersionInfo};

pub struct Client {
    seq_id: AtomicU16,
    pub uin: AtomicI64,
    pub password_md5: Bytes,
    pub ecdh: EncryptECDH,
    pub connected: AtomicBool,

    pub out_pkt_sender: net::OutPktSender,
    //随机16位
    pub random_key: Bytes,
    pub version: VersionInfo,
    pub device_info: DeviceInfo,
    pub out_going_packet_session_id: RwLock<Bytes>,

    // account info
    pub account_info: RwLock<AccountInfo>,

    pub cache_info: RwLock<CacheInfo>,
}

/// Password enum
pub enum Password {
    String(String),
    /// [u8; 16]
    Md5(Bytes),
}

impl Password {
    /// compute password md5(do nothing if already md5)
    pub fn md5(&self) -> Bytes {
        match self {
            Self::String(s) => Bytes::copy_from_slice(&md5::compute(s).0),
            Self::Md5(m) => m.clone(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        Self::String(s.to_owned())
    }
}

#[derive(Default)]
pub struct CacheInfo {
    pub ksid: Bytes,
    // tlv cache
    pub t104: Bytes,
    pub t174: Bytes,
    pub g: Bytes,
    pub t402: Bytes,
    pub t150: Bytes,
    pub t149: Bytes,
    pub t528: Bytes,
    pub t530: Bytes,
    pub rand_seed: Bytes,
    // t403
    pub rollback_sig: Bytes,
    // session info
    pub sig_info: LoginSigInfo,
    pub dpwd: Bytes,
    pub time_diff: i64,
    pub pwd_flag: bool,
}

#[derive(Default)]
pub struct AccountInfo {
    pub nickname: String,
    pub age: u16,
    pub gender: u16,
}


impl Client {
    pub async fn new(
        uin: i64,
        password: Password,
    ) -> (Client, net::OutPktReceiver) {
        let (out_pkt_sender, out_pkt_receiver) = tokio::sync::mpsc::unbounded_channel();

        let mut cli = Client {
            seq_id: AtomicU16::new(0x3635),
            uin: AtomicI64::new(uin),
            password_md5: password.md5(),
            connected: AtomicBool::new(false),
            out_pkt_sender,
            ecdh: EncryptECDH::default(),
            random_key: Bytes::from(rand::thread_rng().gen::<[u8; 16]>().to_vec()),
            version: gen_version_info(&ClientProtocol::IPad),
            device_info: DeviceInfo::random(),
            out_going_packet_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            account_info: RwLock::new(AccountInfo::default()),
            cache_info: RwLock::new(CacheInfo::default()),
        };
        cli.cache_info.write().await.ksid = format!("|{}|A8.2.7.27f6ea96", cli.device_info.imei).into();
        (cli, out_pkt_receiver)
    }
    pub fn next_seq(&self) -> u16 {
        self.seq_id.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn handle_income_packet(&self, pkt: IncomingPacket) {
        println!("{:?}", pkt)
    }
}

#[derive(Default)]
pub struct LoginSigInfo {
    pub login_bitmap: u64,
    pub tgt: Bytes,
    pub tgt_key: Bytes,
    // study room manager | 0x16a
    pub srm_token: Bytes,
    pub t133: Bytes,
    pub encrypted_a1: Bytes,
    pub user_st_key: Bytes,
    pub user_st_web_sig: Bytes,
    pub s_key: Bytes,
    pub s_key_expired_time: i64,
    pub d2: Bytes,
    pub d2key: Bytes,
    pub wt_session_ticket_key: Bytes,
    // TODO 是不是可能None？
    pub device_token: Option<Bytes>,
    pub ps_key_map: HashMap<String, Bytes>,
    pub pt4token_map: HashMap<String, Bytes>,
}

pub struct QiDianAccountInfo {
    pub master_uin: i64,
    pub ext_name: String,
    pub create_time: i64,

    pub big_data_req_addrs: Vec<String>,
    pub big_data_req_session: BigDataReqSessionInfo,
}

pub struct BigDataReqSessionInfo {
    pub sig_session: Bytes,
    pub session_key: Bytes,
}