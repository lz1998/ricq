use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU16, Ordering};
use std::sync::Arc;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use rand::Rng;
use tokio::sync::oneshot;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

use crate::binary::{BinaryReader, BinaryWriter};
use crate::client::income::IncomePacket;
use crate::client::outcome::OutcomePacket;
use crate::client::protocol::device::Device;
use crate::client::protocol::oicq;
use crate::client::protocol::transport::Transport;
use crate::client::protocol::version::{get_version, Protocol};
use crate::client::Password;
use crate::error::RQError;
use crate::RQResult;

use super::net;
use super::Client;

impl super::Client {
    pub async fn new<H>(uin: i64, password: Password, device: Device, handler: H) -> Client
    where
        H: crate::client::handler::Handler + 'static + Sync + Send,
    {
        let (out_pkt_sender, out_pkt_receiver) = tokio::sync::mpsc::unbounded_channel();

        let cli = Client {
            transport: RwLock::new(Transport::new(device, get_version(Protocol::IPad))),
            handler: Box::new(handler),
            seq_id: AtomicU16::new(0x3635),
            request_packet_request_id: AtomicI32::new(1921334513),
            group_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            friend_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            group_data_trans_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            highway_apply_up_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            uin: AtomicI64::new(uin),
            password_md5: password.md5(),
            connected: AtomicBool::new(false),
            shutting_down: AtomicBool::new(false),
            heartbeat_enabled: AtomicBool::new(false),
            online: AtomicBool::new(false),
            net: net::ClientNet::new(out_pkt_receiver),
            out_pkt_sender,
            random_key: Bytes::from(rand::thread_rng().gen::<[u8; 16]>().to_vec()),
            // out_going_packet_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            packet_promises: Default::default(),
            packet_waiters: Default::default(),
            oicq_codec: RwLock::new(oicq::Codec::default()),
            account_info: Default::default(),
            address: Default::default(),
            friend_list: Default::default(),
            group_list: Default::default(),
            online_clients: Default::default(),
            last_message_time: Default::default(),
            group_message_builder: RwLock::default(),
        };
        cli
    }

    pub async fn new_with_config<H>(config: crate::Config, handler: H) -> Self
    where
        H: crate::client::handler::Handler + 'static + Sync + Send,
    {
        let password = super::Password::from_str(&config.password);
        Self::new(config.uin, password, config.device, handler).await
    }

    pub async fn run(self: &Arc<Self>) -> JoinHandle<()> {
        let net = self.net.run(self).await;
        tokio::spawn(net)
    }

    pub fn next_seq(&self) -> u16 {
        self.seq_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn next_packet_seq(&self) -> i32 {
        self.request_packet_request_id
            .fetch_add(2, Ordering::Relaxed)
    }

    pub fn next_group_seq(&self) -> i32 {
        self.group_seq.fetch_add(2, Ordering::Relaxed)
    }

    pub fn next_friend_seq(&self) -> i32 {
        self.friend_seq.fetch_add(2, Ordering::Relaxed)
    }

    pub fn next_group_data_trans_seq(&self) -> i32 {
        self.group_data_trans_seq.fetch_add(2, Ordering::Relaxed)
    }

    pub fn next_highway_apply_seq(&self) -> i32 {
        self.highway_apply_up_seq.fetch_add(2, Ordering::Relaxed)
    }

    pub async fn send(&self, pkt: OutcomePacket) -> Result<(), RQError> {
        self.out_pkt_sender
            .send(pkt.bytes)
            .map_err(|_| RQError::Other("failed to send out_pkt".into()))
    }

    pub async fn send_and_wait(
        &self,
        pkt: OutcomePacket,
        expected_command_name: &str,
    ) -> Result<IncomePacket, RQError> {
        let (sender, receiver) = oneshot::channel();
        {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.insert(pkt.seq, sender);
        }
        if let Err(_) = self.out_pkt_sender.send(pkt.bytes) {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.remove(&pkt.seq);
            return Err(RQError::Network.into());
        }
        match tokio::time::timeout(std::time::Duration::from_secs(15), receiver).await {
            Ok(p) => p.unwrap().check_command_name(expected_command_name),
            Err(_) => {
                self.packet_promises.write().await.remove(&pkt.seq);
                Err(RQError::Timeout)
            }
        }
    }

    pub async fn wait_packet(&self, pkt_name: &str, delay: u64) -> RQResult<IncomePacket> {
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
            match self
                .send_and_wait(
                    self.build_heartbeat_packet().await.into(),
                    "Heartbeat.Alive",
                )
                .await
            {
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
        let sig = &self.transport.read().await.sig;
        token.put_i64(self.uin.load(Ordering::SeqCst));
        token.write_bytes_short(&sig.d2);
        token.write_bytes_short(&sig.d2key);
        token.write_bytes_short(&sig.tgt);
        token.write_bytes_short(&sig.srm_token);
        token.write_bytes_short(&sig.t133);
        token.write_bytes_short(&sig.encrypted_a1);
        token.write_bytes_short(&self.oicq_codec.read().await.wt_session_ticket_key);
        token.write_bytes_short(&sig.out_packet_session_id);
        token.write_bytes_short(&sig.tgtgt_key);
        token.freeze()
    }

    pub async fn load_token(&self, token: &mut impl Buf) {
        let sig = &mut self.transport.write().await.sig;
        self.uin.store(token.get_i64(), Ordering::SeqCst);
        sig.d2 = token.read_bytes_short();
        sig.d2key = token.read_bytes_short();
        sig.tgt = token.read_bytes_short();
        sig.srm_token = token.read_bytes_short();
        sig.t133 = token.read_bytes_short();
        sig.encrypted_a1 = token.read_bytes_short();
        self.oicq_codec.write().await.wt_session_ticket_key = token.read_bytes_short();
        sig.out_packet_session_id = token.read_bytes_short();
        sig.tgtgt_key = token.read_bytes_short();
    }
}
