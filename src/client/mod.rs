use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::sync::Arc;

use bytes::Bytes;
use tokio::sync::oneshot;
use tokio::sync::RwLock;

use crate::client::engine::Engine;
use crate::client::protocol::packet::Packet;
use crate::client::structs::{FriendInfo, GroupInfo};
use crate::jce::FileStoragePushFSSvcList;

pub mod api;
pub mod client;
pub mod engine;
pub mod handler;
pub mod income;
pub mod msg;
pub mod net;
pub mod processor;
pub mod protocol;
pub mod structs;

pub struct Client {
    handler: Box<dyn handler::Handler + Sync + Send + 'static>,

    pub uin: AtomicI64,
    pub engine: RwLock<Engine>,
    pub connected: AtomicBool,
    pub shutting_down: AtomicBool,
    pub heartbeat_enabled: AtomicBool,
    pub online: AtomicBool,
    pub(crate) net: net::ClientNet,

    pub out_pkt_sender: net::OutPktSender,
    pub packet_promises: RwLock<HashMap<i32, oneshot::Sender<Packet>>>,
    packet_waiters: RwLock<HashMap<String, oneshot::Sender<Packet>>>,

    // account info
    pub account_info: RwLock<AccountInfo>,

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
pub struct AccountInfo {
    pub nickname: String,
    pub age: u8,
    pub gender: u8,
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
