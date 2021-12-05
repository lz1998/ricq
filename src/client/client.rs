use super::errors::RQError;
use super::net;
use super::Client;
use crate::client::device::DeviceInfo;
use crate::client::income::IncomePacket;
use crate::client::outcome::OutcomePacket;
use crate::client::version::{gen_version_info, ClientProtocol};
use crate::client::Password;
use bytes::Bytes;
use rand::Rng;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU16, Ordering};
use tokio::sync::oneshot;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

impl super::Client {
    pub async fn new<H>(
        uin: i64,
        password: Password,
        mut device_info: DeviceInfo,
        handler: H,
    ) -> (Client, net::OutPktReceiver)
        where
            H: crate::client::handler::Handler + 'static + Sync + Send,
    {
        device_info.gen_guid();
        device_info.gen_tgtgt_key();
        let (out_pkt_sender, out_pkt_receiver) = tokio::sync::mpsc::unbounded_channel();

        let cli = Client {
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
            heartbeat_enabled: AtomicBool::new(false),
            online: AtomicBool::new(false),
            out_pkt_sender,
            random_key: Bytes::from(rand::thread_rng().gen::<[u8; 16]>().to_vec()),
            version: gen_version_info(&ClientProtocol::IPad),
            device_info: RwLock::new(device_info),
            out_going_packet_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            packet_promises: Default::default(),
            ecdh: Default::default(),
            account_info: Default::default(),
            cache_info: Default::default(),
            address: Default::default(),
            friend_list: Default::default(),
            group_list: Default::default(),
            online_clients: Default::default(),
            last_message_time: Default::default(),
            group_message_builder: RwLock::default(),
        };
        {
            let mut cache_info = cli.cache_info.write().await;
            cache_info.ksid = Bytes::from(format!(
                "|{}|A8.2.7.27f6ea96",
                cli.device_info.read().await.imei
            )); // TODO before connect
        }
        cli.cache_info.write().await.ksid =
            format!("|{}|A8.2.7.27f6ea96", cli.device_info.read().await.imei).into();
        (cli, out_pkt_receiver)
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
        self.out_pkt_sender.send(pkt.bytes).map_err(|_| RQError::Other("failed to send out_pkt".into()))
    }

    pub async fn send_and_wait(&self, pkt: OutcomePacket) -> Result<IncomePacket, RQError> {
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
        let output = if let Ok(Ok(p)) = tokio::time::timeout(std::time::Duration::from_secs(15), receiver).await {
            Ok(p)
        } else {
            Err(RQError::Timeout)
        };
        {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.remove(&pkt.seq);
        }
        output
    }

    pub async fn do_heartbeat(&self) {
        self.heartbeat_enabled.store(true, Ordering::SeqCst);
        let mut times = 0;
        while self.online.load(Ordering::SeqCst) {
            sleep(Duration::from_secs(30)).await;
            match self
                .send_and_wait(self.build_heartbeat_packet().await.into())
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
}
