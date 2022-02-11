use std::sync::atomic::{AtomicBool, Ordering};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::sync::oneshot;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use rq_engine::protocol::version::Version;

use crate::engine::binary::{BinaryReader, BinaryWriter};
use crate::engine::protocol::{device::Device, packet::Packet};
use crate::engine::Engine;
use crate::{RQError, RQResult};

use super::Client;

impl super::Client {
    pub fn new<H>(device: Device, version: &'static Version, handler: H) -> Client
    where
        H: crate::client::handler::Handler + 'static + Sync + Send,
    {
        let (out_pkt_sender, _) = tokio::sync::broadcast::channel(1024);
        let (disconnect_signal, _) = tokio::sync::broadcast::channel(1024);

        Client {
            handler: Box::new(handler),
            engine: RwLock::new(Engine::new(device, version)),
            running: AtomicBool::new(false),
            heartbeat_enabled: AtomicBool::new(false),
            online: AtomicBool::new(false),
            out_pkt_sender,
            disconnect_signal,
            // out_going_packet_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            packet_promises: Default::default(),
            packet_waiters: Default::default(),
            receipt_waiters: Default::default(),
            account_info: Default::default(),
            address: Default::default(),
            friends: Default::default(),
            groups: Default::default(),
            online_clients: Default::default(),
            last_message_time: Default::default(),
            start_time: chrono::Utc::now().timestamp() as i32,
            group_message_builder: RwLock::new(cached::TimedCache::with_lifespan(600)),
            c2c_cache: RwLock::new(cached::TimedCache::with_lifespan(3600)),
            push_req_cache: RwLock::new(cached::TimedCache::with_lifespan(30)),
            push_trans_cache: RwLock::new(cached::TimedCache::with_lifespan(15)),
            group_sys_message_cache: RwLock::new(Default::default()),
            highway_session: RwLock::new(Default::default()),
            highway_addrs: RwLock::new(Default::default()),
        }
    }

    pub fn new_with_config<H>(config: crate::Config, handler: H) -> Self
    where
        H: crate::client::handler::Handler + 'static + Sync + Send,
    {
        Self::new(config.device, config.version, handler)
    }

    pub async fn uin(&self) -> i64 {
        return self.engine.read().await.uin.load(Ordering::Relaxed);
    }

    pub async fn send(&self, pkt: Packet) -> RQResult<usize> {
        tracing::trace!(target: "rs_qq", "sending pkt {}-{},", pkt.command_name, pkt.seq_id);
        let data = self.engine.read().await.transport.encode_packet(pkt);
        self.out_pkt_sender
            .send(data)
            .map_err(|_| RQError::Other("failed to send out_pkt".into()))
    }

    pub async fn send_and_wait(&self, pkt: Packet) -> RQResult<Packet> {
        tracing::trace!(target: "rs_qq", "send_and_waitting pkt {}-{},", pkt.command_name, pkt.seq_id);
        let seq = pkt.seq_id;
        let expect = pkt.command_name.clone();
        let data = self.engine.read().await.transport.encode_packet(pkt);
        let (sender, receiver) = oneshot::channel();
        {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.insert(seq, sender);
        }
        if let Err(_) = self.out_pkt_sender.send(data) {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.remove(&seq);
            return Err(RQError::Network);
        }
        match tokio::time::timeout(std::time::Duration::from_secs(15), receiver).await {
            Ok(p) => p.unwrap().check_command_name(&expect),
            Err(_) => {
                tracing::trace!(target: "rs_qq", "waitting pkt {}-{} timeout", expect, seq);
                self.packet_promises.write().await.remove(&seq);
                Err(RQError::Timeout)
            }
        }
    }

    pub async fn wait_packet(&self, pkt_name: &str, delay: u64) -> RQResult<Packet> {
        tracing::trace!(target: "rs_qq", "waitting pkt {}", pkt_name);
        let (tx, rx) = oneshot::channel();
        {
            self.packet_waiters
                .write()
                .await
                .insert(pkt_name.to_owned(), tx);
        }
        match tokio::time::timeout(std::time::Duration::from_secs(delay), rx).await {
            Ok(i) => Ok(i.unwrap()),
            Err(_) => {
                tracing::trace!(target: "rs_qq", "waitting pkt {} timeout", pkt_name);
                self.packet_waiters.write().await.remove(pkt_name);
                Err(RQError::Timeout)
            }
        }
    }

    pub async fn do_heartbeat(&self) {
        self.heartbeat_enabled.store(true, Ordering::SeqCst);
        let mut times = 0;
        while self.online.load(Ordering::SeqCst) {
            sleep(Duration::from_secs(30)).await;
            match self.heartbeat().await {
                Err(_) => {
                    continue;
                }
                Ok(_) => {
                    times += 1;
                    if times >= 7 {
                        if self.register_client().await.is_err() {
                            break;
                        }
                        times = 0;
                    }
                }
            }
        }
        self.heartbeat_enabled.store(false, Ordering::SeqCst);
    }

    pub async fn gen_token(&self) -> Bytes {
        let mut token = BytesMut::with_capacity(1024); //todo
        let engine = &self.engine.read().await;
        token.put_i64(self.uin().await);
        token.write_bytes_short(&engine.transport.sig.d2);
        token.write_bytes_short(&engine.transport.sig.d2key);
        token.write_bytes_short(&engine.transport.sig.tgt);
        token.write_bytes_short(&engine.transport.sig.srm_token);
        token.write_bytes_short(&engine.transport.sig.t133);
        token.write_bytes_short(&engine.transport.sig.encrypted_a1);
        token.write_bytes_short(&engine.transport.oicq_codec.wt_session_ticket_key);
        token.write_bytes_short(&engine.transport.sig.out_packet_session_id);
        token.write_bytes_short(&engine.transport.sig.tgtgt_key);
        token.freeze()
    }

    pub async fn load_token(&self, token: &mut impl Buf) {
        let mut engine = self.engine.write().await;
        engine.uin.store(token.get_i64(), Ordering::SeqCst);
        engine.transport.sig.d2 = token.read_bytes_short();
        engine.transport.sig.d2key = token.read_bytes_short();
        engine.transport.sig.tgt = token.read_bytes_short();
        engine.transport.sig.srm_token = token.read_bytes_short();
        engine.transport.sig.t133 = token.read_bytes_short();
        engine.transport.sig.encrypted_a1 = token.read_bytes_short();
        engine.transport.oicq_codec.wt_session_ticket_key = token.read_bytes_short();
        engine.transport.sig.out_packet_session_id = token.read_bytes_short();
        engine.transport.sig.tgtgt_key = token.read_bytes_short();
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        self.disconnect_signal.send(()).ok();
    }
}
