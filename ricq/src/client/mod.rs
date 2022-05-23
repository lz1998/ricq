use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64};

use tokio::sync::{broadcast, RwLock};
use tokio::sync::{oneshot, Mutex};

use ricq_core::command::online_push::GroupMessagePart;
use ricq_core::command::profile_service::GroupSystemMessages;
use ricq_core::common::RQAddr;
use ricq_core::protocol::packet::Packet;
use ricq_core::structs::{AccountInfo, AddressInfo, OtherClientInfo};
use ricq_core::Engine;
pub use ricq_core::Token;

mod api;
mod client;
pub mod event;
pub mod handler;
mod highway;
mod net;
mod processor;

pub struct Client {
    handler: Box<dyn handler::Handler + Sync + Send + 'static>,
    engine: RwLock<Engine>,

    // 是否正在运行（可用于判断是否需要重连）
    // 手动断开/服务端要求下线/其他客户端上线 -> running=false
    // 网络原因/其他意外 -> running=true
    pub running: AtomicBool,
    // 是否在线
    pub online: AtomicBool,
    // 停止网络
    disconnect_signal: broadcast::Sender<()>,
    pub heartbeat_enabled: AtomicBool,

    out_pkt_sender: net::OutPktSender,
    packet_promises: RwLock<HashMap<i32, oneshot::Sender<Packet>>>,
    receipt_waiters: Mutex<HashMap<i32, oneshot::Sender<i32>>>,

    // account info
    pub account_info: RwLock<AccountInfo>,

    // address
    pub address: RwLock<AddressInfo>,
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

    highway_session: RwLock<ricq_core::highway::Session>,
    highway_addrs: RwLock<Vec<RQAddr>>,
}
