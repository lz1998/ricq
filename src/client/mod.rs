use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU16};

use bytes::Bytes;
use tokio::sync::RwLock;
use crate::client::device::DeviceInfo;
use crate::client::structs::{FriendInfo, GroupInfo, LoginSigInfo, OtherClientInfo};
use crate::client::version::VersionInfo;

use crate::crypto::EncryptECDH;
use tokio::sync::oneshot;
use crate::client::income::decoder::profile_service::GroupSystemMessages;
use crate::client::income::IncomePacket;
use crate::jce::{BigDataChannel, FileStoragePushFSSvcList};

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

    // address
    pub address: RwLock<AddressInfo>,

    // message state
    pub message_state: RwLock<MessageStateInfo>,

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
    // tlv cache
    pub t104: Bytes,
    pub t174: Bytes,
    pub g: Bytes,
    pub t402: Bytes,
    pub t150: Bytes,
    pub t149: Bytes,
    pub t528: Bytes,
    pub t530: Bytes,
    pub rand_seed: Bytes, // t403
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

#[derive(Default)]
pub struct AccountInfo {
    pub nickname: String,
    pub age: u16,
    pub gender: u16,
    pub friend_list: Vec<FriendInfo>,
    pub group_list: Vec<GroupInfo>,
    pub online_clients: Vec<OtherClientInfo>,
}

pub struct AddressInfo {
    pub srv_sso_addrs: Vec<String>,
    pub other_srv_addrs: Vec<String>,
    pub file_storage_info: FileStoragePushFSSvcList,
}

impl Default for AddressInfo {
    fn default() -> Self {
        AddressInfo {
            srv_sso_addrs: vec![],
            other_srv_addrs: vec![],
            file_storage_info: FileStoragePushFSSvcList {
                upload_list: vec![],
                pic_download_list: vec![],
                g_pic_download_list: vec![],
                q_zone_proxy_service_list: vec![],
                url_encode_service_list: vec![],
                big_data_channel: BigDataChannel {
                    ip_list: vec![],
                    sig_session: Default::default(),
                    key_session: Default::default(),
                    sig_uin: 0,
                    connect_flag: 0,
                    pb_buf: Default::default()
                },
                vip_emotion_list: vec![],
                c2c_pic_down_list: vec![],
                ptt_list: Default::default()
            }
        }
    }
}

#[derive(Default)]
pub struct MessageStateInfo {
    pub last_message_seq: i32,
    // pub msg_svc_cache: *utils.Cache,
    pub last_c2c_msg_time: i64,
    // pub trans_cache: *utils.Cache,
    pub last_lost_msg: String,
    pub group_sys_msg_cache: GroupSystemMessages,
    // pub groupMsgBuilders       :sync.Map,
    // pub online_push_cache:*utils.Cache,
    pub request_packet_request_id: i32,
    pub group_seq: i32,
    pub friend_seq: i32,
    pub heartbeat_enabled: bool,
    pub group_data_trans_seq: i32,
    pub highway_apply_up_seq: i32,
    // pub event_handlers: *eventHandlers,
}
