use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::sync::Arc;

use tokio::sync::{broadcast, RwLock};
use tokio::sync::{oneshot, Mutex};

use rq_engine::command::online_push::GroupMessagePart;
use rq_engine::command::profile_service::GroupSystemMessages;

use crate::engine::protocol::packet::Packet;
use crate::engine::structs::{AccountInfo, AddressInfo, FriendInfo, OtherClientInfo};
use crate::engine::Engine;
use crate::structs::Group;

mod api;
mod client;
pub mod event;
pub mod handler;
mod net;
mod processor;

pub struct Client {
    handler: Box<dyn handler::Handler + Sync + Send + 'static>,
    engine: RwLock<Engine>,

    // 是否正在运行（是否需要快速重连）
    pub running: AtomicBool,
    // 是否在线（是否可以快速重连）
    pub online: AtomicBool,
    // 停止网络
    disconnect_signal: broadcast::Sender<()>,
    pub heartbeat_enabled: AtomicBool,

    out_pkt_sender: net::OutPktSender,
    packet_promises: RwLock<HashMap<i32, oneshot::Sender<Packet>>>,
    packet_waiters: RwLock<HashMap<String, oneshot::Sender<Packet>>>,
    receipt_waiters: Mutex<HashMap<i32, oneshot::Sender<i32>>>,

    // account info
    pub account_info: RwLock<AccountInfo>,

    // address
    pub address: RwLock<AddressInfo>,
    pub friends: RwLock<HashMap<i64, Arc<FriendInfo>>>,
    pub groups: RwLock<HashMap<i64, Arc<Group>>>,
    pub online_clients: RwLock<Vec<OtherClientInfo>>,

    // statics
    pub last_message_time: AtomicI64,
    pub start_time: i32,

    /// 群消息 builder 寄存 <div_seq, parts> : parts is sorted by pkg_index
    group_message_builder: RwLock<cached::TimedCache<i32, Vec<GroupMessagePart>>>,
    /// 每个 28 Byte
    c2c_cache: RwLock<cached::TimedCache<(i64, i64, i32, i64), ()>>,
    push_req_cache: RwLock<cached::TimedCache<(i16, i64), ()>>,
    push_trans_cache: RwLock<cached::TimedCache<(i32, i64), ()>>,
    group_sys_message_cache: RwLock<GroupSystemMessages>,
}
