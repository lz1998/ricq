use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU16};

use bytes::Bytes;
use tokio::sync::RwLock;
use crate::client::device::DeviceInfo;
use crate::client::structs::LoginSigInfo;
use crate::client::version::VersionInfo;

use crate::crypto::EncryptECDH;
use tokio::sync::oneshot;
use crate::client::income::IncomePacket;

pub mod client;
pub mod structs;
pub mod income;
pub mod outcome;
pub mod net;
pub mod version;
pub mod device;
pub mod api;

pub struct Client {
    seq_id: AtomicU16,
    request_packet_request_id: AtomicI32,
    group_seq: AtomicI32,

    pub uin: AtomicI64,
    pub password_md5: Bytes,
    pub ecdh: EncryptECDH,
    pub connected: AtomicBool,
    pub online: AtomicBool,

    pub out_pkt_sender: net::OutPktSender,
    pub packet_promises: RwLock<HashMap<u16, oneshot::Sender<IncomePacket>>>,
    //随机16位
    pub random_key: Bytes,
    pub version: VersionInfo,
    pub device_info: RwLock<DeviceInfo>,
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
