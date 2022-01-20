use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};
use tokio::sync::RwLock;

use rq_engine::command::online_push::GroupMessagePart;

use crate::engine::protocol::packet::Packet;
use crate::engine::structs::{
    AccountInfo, AddressInfo, FriendInfo, GroupInfo, GroupMemberInfo, OtherClientInfo,
};
use crate::engine::Engine;

pub mod api;
pub mod client;
pub mod handler;
pub mod net;
pub mod processor;

pub struct Client {
    handler: Box<dyn handler::Handler + Sync + Send + 'static>,

    pub engine: RwLock<Engine>,
    pub connected: AtomicBool,
    pub shutting_down: AtomicBool,
    pub heartbeat_enabled: AtomicBool,
    pub online: AtomicBool,
    pub(crate) net: net::ClientNet,

    pub out_pkt_sender: net::OutPktSender,
    pub packet_promises: RwLock<HashMap<i32, oneshot::Sender<Packet>>>,
    packet_waiters: RwLock<HashMap<String, oneshot::Sender<Packet>>>,
    receipt_waiters: Mutex<HashMap<i32, oneshot::Sender<i32>>>,

    // account info
    pub account_info: RwLock<AccountInfo>,

    // address
    pub address: RwLock<AddressInfo>,
    pub friend_list: RwLock<Vec<Arc<FriendInfo>>>,
    pub group_list: RwLock<Vec<Arc<(GroupInfo, RwLock<Vec<GroupMemberInfo>>)>>>,
    pub online_clients: RwLock<Vec<OtherClientInfo>>,

    // statics
    pub last_message_time: AtomicI64,

    /// 群消息 builder 寄存 <div_seq, parts> : parts is sorted by pkg_index
    group_message_builder: RwLock<cached::TimedCache<i32, BTreeMap<i32, GroupMessagePart>>>,
    c2c_cache: crate::cache::TimeOutCache<()>,
}
