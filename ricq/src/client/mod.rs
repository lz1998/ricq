use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, Ordering};
use std::time::UNIX_EPOCH;

use cached::Cached;
use tokio::sync::{broadcast, RwLock};
use tokio::sync::{oneshot, Mutex};
use tokio::time::{sleep, Duration};

pub use net::{Connector, DefaultConnector};
use ricq_core::command::online_push::GroupMessagePart;
use ricq_core::command::profile_service::GroupSystemMessages;
use ricq_core::common::RQAddr;
use ricq_core::protocol::version::Version;
use ricq_core::protocol::{device::Device, packet::Packet};
use ricq_core::structs::{AccountInfo, AddressInfo, OtherClientInfo};
use ricq_core::Engine;
pub use ricq_core::Token;

use crate::{RQError, RQResult};

mod api;
pub mod event;
pub mod handler;
mod highway;
pub(crate) mod net;
mod processor;
mod tcp;

pub struct Client {
    /// QEvent Handler 调用 handle 方法外发 QEvent
    handler: Box<dyn handler::Handler + Sync + Send + 'static>,
    pub engine: RwLock<Engine>,

    // 状态相关
    /// 网络状态
    status: AtomicU8,
    /// 停止网络信号 Sender
    disconnect_signal: broadcast::Sender<()>,
    /// 是否在线
    pub online: AtomicBool,
    /// 心跳包是否已启用
    pub heartbeat_enabled: AtomicBool,

    // 包相关
    /// 外发包 Sender
    out_pkt_sender: net::OutPktSender,
    /// send_and_wait WaitMap
    packet_promises: RwLock<HashMap<i32, oneshot::Sender<Packet>>>,
    /// 当前客户端发送消息后使用 cache 避免上报自身消息事件
    receipt_waiters: Mutex<cached::TimedCache<i32, oneshot::Sender<i32>>>,

    // account info
    pub account_info: RwLock<AccountInfo>,

    // address
    pub address: RwLock<AddressInfo>,
    /// 其他同时在线客户端
    pub online_clients: RwLock<Vec<OtherClientInfo>>,

    // statics
    pub last_message_time: AtomicI64,
    /// 调用 new 方法时的时间戳
    pub start_time: i32,

    /// 群消息 builder 寄存 <div_seq, parts> : parts is sorted by pkg_index
    group_message_builder: RwLock<cached::TimedCache<i32, Vec<GroupMessagePart>>>,
    /// 每个 28 Byte
    c2c_cache: RwLock<cached::TimedCache<(i64, i64, i32, i64), ()>>,
    push_req_cache: RwLock<cached::TimedCache<(i16, i64), ()>>,
    push_trans_cache: RwLock<cached::TimedCache<(i32, i64), ()>>,
    group_sys_message_cache: RwLock<GroupSystemMessages>,

    pub highway_session: RwLock<ricq_core::highway::Session>,
    pub highway_addrs: RwLock<Vec<RQAddr>>,

    packet_handler: RwLock<HashMap<String, broadcast::Sender<Packet>>>,
}

impl super::Client {
    /// 新建 Clinet
    ///
    /// **Notice: 该方法仅新建 Client 需要调用 start 方法连接到服务器**
    pub fn new<H>(device: Device, version: Version, handler: H) -> Client
    where
        H: crate::client::handler::Handler + 'static + Sync + Send,
    {
        let (out_pkt_sender, _) = tokio::sync::broadcast::channel(1024);
        let (disconnect_signal, _) = tokio::sync::broadcast::channel(8);

        Client {
            handler: Box::new(handler),
            engine: RwLock::new(Engine::new(device, version)),
            status: AtomicU8::new(NetworkStatus::Unknown as u8),
            heartbeat_enabled: AtomicBool::new(false),
            online: AtomicBool::new(false),
            out_pkt_sender,
            disconnect_signal,
            // out_going_packet_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            packet_promises: Default::default(),
            receipt_waiters: Mutex::new(cached::TimedCache::with_lifespan(60)),
            account_info: Default::default(),
            address: Default::default(),
            online_clients: Default::default(),
            last_message_time: Default::default(),
            start_time: UNIX_EPOCH.elapsed().unwrap().as_secs() as i32,
            group_message_builder: RwLock::new(cached::TimedCache::with_lifespan(600)),
            c2c_cache: RwLock::new(cached::TimedCache::with_lifespan(3600)),
            push_req_cache: RwLock::new(cached::TimedCache::with_lifespan(30)),
            push_trans_cache: RwLock::new(cached::TimedCache::with_lifespan(15)),
            group_sys_message_cache: RwLock::new(Default::default()),
            highway_session: RwLock::new(Default::default()),
            highway_addrs: RwLock::new(Default::default()),
            packet_handler: Default::default(),
        }
    }

    /// 新建 Clinet
    ///
    /// **Notice: 该方法仅新建 Client 需要调用 start 方法连接到服务器**
    pub fn new_with_config<H>(config: crate::Config, handler: H) -> Self
    where
        H: crate::client::handler::Handler + 'static + Sync + Send,
    {
        Self::new(config.device, config.version, handler)
    }

    /// 获取当前 Client uin
    pub async fn uin(&self) -> i64 {
        self.engine.read().await.uin.load(Ordering::Relaxed)
    }

    /// 向服务器发包
    pub async fn send(&self, pkt: Packet) -> RQResult<usize> {
        tracing::trace!("sending pkt {}-{},", pkt.command_name, pkt.seq_id);
        let data = self.engine.read().await.transport.encode_packet(pkt);
        self.out_pkt_sender
            .send(data)
            .map_err(|_| RQError::Other("failed to send out_pkt".into()))
    }

    /// 向服务器发包并等待接收返回的包，15 秒后超时返回 `Err(RQError::Timeout)`
    pub async fn send_and_wait(&self, pkt: Packet) -> RQResult<Packet> {
        tracing::trace!("send_and_waitting pkt {}-{},", pkt.command_name, pkt.seq_id);
        let seq = pkt.seq_id;
        let expect = pkt.command_name.clone();
        let data = self.engine.read().await.transport.encode_packet(pkt);
        let (sender, receiver) = oneshot::channel();
        {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.insert(seq, sender);
        }
        if self.out_pkt_sender.send(data).is_err() {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.remove(&seq);
            return Err(RQError::Network);
        }
        match tokio::time::timeout(std::time::Duration::from_secs(15), receiver).await {
            Ok(p) => p.unwrap().check_command_name(&expect),
            Err(_) => {
                tracing::trace!("waiting pkt {}-{} timeout", expect, seq);
                self.packet_promises.write().await.remove(&seq);
                Err(RQError::Timeout)
            }
        }
    }

    /// 向服务器发送心跳包，并自动注册客户端
    ///
    /// 该方法会阻塞当前协程，通常 spawn 使用
    pub async fn do_heartbeat(&self) {
        self.heartbeat_enabled.store(true, Ordering::SeqCst);
        let mut times = 0;
        while self.online.load(Ordering::SeqCst) {
            sleep(Duration::from_secs(30)).await;
            if self.heartbeat().await.is_ok() {
                times += 1;
                if times >= 7 {
                    if self.register_client().await.is_err() {
                        break;
                    }
                    times = 0;
                }
            }
        }
        self.heartbeat_enabled.store(false, Ordering::SeqCst);
    }

    /// 生成 token
    pub async fn gen_token(&self) -> Token {
        self.engine.read().await.gen_token()
    }

    /// 从 token 恢复
    pub async fn load_token(&self, token: Token) {
        self.engine.write().await.load_token(token)
    }

    pub async fn device(&self) -> Device {
        self.engine.read().await.transport.device.clone()
    }

    pub async fn version(&self) -> Version {
        self.engine.read().await.transport.version.clone()
    }

    pub async fn get_highway_session_key(&self) -> Vec<u8> {
        self.highway_session.read().await.session_key.to_vec()
    }

    /// 监听指定 command 数据包
    pub async fn listen_command<S: ToString>(&self, command: S) -> broadcast::Receiver<Packet> {
        self.packet_handler
            .write()
            .await
            .cache_get_or_set_with(command.to_string(), || broadcast::channel(10).0)
            .subscribe()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop(NetworkStatus::Drop);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum NetworkStatus {
    // 未启动
    Unknown = 0,
    // 运行中
    Running = 1,
    // 用户手动停止
    Stop = 2,
    // 内存释放
    Drop = 3,
    // 网络原因掉线
    NetworkOffline = 4,
    // 其他客户端踢下线
    KickedOffline = 5,
    // 服务端强制下线
    MsfOffline = 6,
}
