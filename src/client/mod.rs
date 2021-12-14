use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU16};
use std::sync::Arc;

use crate::client::device::DeviceInfo;
use crate::client::structs::{FriendInfo, GroupInfo, LoginSigInfo};
use crate::client::version::VersionInfo;
use bytes::Bytes;
use tokio::sync::RwLock;

use crate::client::income::IncomePacket;
use crate::crypto::EncryptECDH;
use crate::jce::FileStoragePushFSSvcList;
use tokio::sync::oneshot;

pub mod api;
pub mod client;
pub mod device;
pub mod errors;
pub mod handler;
pub mod income;
pub mod messages;
pub mod msg;
pub mod net;
pub mod outcome;
pub mod processor;
pub mod structs;
pub mod version;

pub struct Client {
    handler: Box<dyn handler::Handler + Sync + Send + 'static>,
    seq_id: AtomicU16,
    request_packet_request_id: AtomicI32,
    group_seq: AtomicI32,
    friend_seq: AtomicI32,
    group_data_trans_seq: AtomicI32,
    highway_apply_up_seq: AtomicI32,

    pub uin: AtomicI64,
    pub password_md5: Bytes,
    pub ecdh: EncryptECDH,
    pub connected: AtomicBool,
    pub shutting_down: AtomicBool,
    pub heartbeat_enabled: AtomicBool,
    pub online: AtomicBool,
    pub(crate) net: net::ClientNet,

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

    // address
    pub address: RwLock<AddressInfo>,
    pub friend_list: RwLock<Vec<Arc<FriendInfo>>>,
    pub group_list: RwLock<Vec<Arc<GroupInfo>>>,
    pub online_clients: RwLock<Vec<OtherClientInfo>>,

    // statics
    pub last_message_time: AtomicI64,

    /// 群消息 builder 寄存
    pub group_message_builder: RwLock<HashMap<i32, income::builder::GroupMessageBuilder>>,
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

#[derive(Default, Debug)]
pub struct CacheInfo {
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
    pub rollback_sig: Bytes,

    // sync info
    pub sync_cookie: Bytes,
    pub pub_account_cookie: Bytes,
    pub msg_ctrl_buf: Bytes,
    pub ksid: Bytes,

    // session info
    pub sig_info: LoginSigInfo,
    pub dpwd: Bytes,
    pub time_diff: i64,
    pub pwd_flag: bool,
}

#[derive(Default, Debug)]
pub struct AccountInfo {
    pub nickname: String,
    pub age: u16,
    pub gender: u16,
}

#[derive(Default, Debug)]
pub struct AddressInfo {
    pub srv_sso_addrs: Vec<String>,
    pub other_srv_addrs: Vec<String>,
    pub file_storage_info: FileStoragePushFSSvcList,
}

#[derive(Debug, Default)]
pub struct OtherClientInfo {
    pub app_id: i64,
    pub instance_id: i32,
    pub sub_platform: String,
    pub device_kind: String,
}
