use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU16, Ordering};
use bytes::Bytes;
use rand::Rng;
use tokio::sync::RwLock;
use crate::client::device::{DeviceInfo};
use crate::client::income::IncomePacket;
use crate::client::outcome::OutcomePacket;
use crate::client::version::{ClientProtocol, gen_version_info};
use tokio::sync::oneshot;
use super::Client;
use crate::client::Password;
use super::net;


impl super::Client {
    pub async fn new(
        uin: i64,
        password: Password,
        mut device_info: DeviceInfo,
    ) -> (Client, net::OutPktReceiver) {
        device_info.gen_guid();
        device_info.gen_tgtgt_key();
        let (out_pkt_sender, out_pkt_receiver) = tokio::sync::mpsc::unbounded_channel();

        let cli = Client {
            seq_id: AtomicU16::new(0x3635),
            request_packet_request_id: AtomicI32::new(1921334513),
            group_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            friend_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            group_data_trans_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            highway_apply_up_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            uin: AtomicI64::new(uin),
            password_md5: password.md5(),
            connected: AtomicBool::new(false),
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
        };
        {
            let mut cache_info = cli.cache_info.write().await;
            cache_info.ksid = Bytes::from(format!("|{}|A8.2.7.27f6ea96", cli.device_info.read().await.imei)); // TODO before connect
        }
        cli.cache_info.write().await.ksid = format!("|{}|A8.2.7.27f6ea96", cli.device_info.read().await.imei).into();
        (cli, out_pkt_receiver)
    }

    pub fn next_seq(&self) -> u16 {
        self.seq_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn next_packet_seq(&self) -> i32 {
        self.request_packet_request_id.fetch_add(2, Ordering::Relaxed)
    }

    pub fn next_group_seq(&self) -> i32 {
        self.group_seq.fetch_add(2, Ordering::Relaxed)
    }

    pub fn next_friend_seq(&self) -> i32 {
        self.friend_seq.fetch_add(2, Ordering::Relaxed)
    }

    pub fn next_group_data_trans_seq(&self) -> i32 { self.group_data_trans_seq.fetch_add(2, Ordering::Relaxed) }

    pub fn next_highway_apply_seq(&self) -> i32 { self.highway_apply_up_seq.fetch_add(2, Ordering::Relaxed) }

    pub async fn send(&self, pkt: OutcomePacket) {
        self.out_pkt_sender.send(pkt.bytes);
    }

    pub async fn send_and_wait(&self, pkt: OutcomePacket) -> Option<IncomePacket> {
        let (sender, receiver) = oneshot::channel();
        {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.insert(pkt.seq, sender);
        }
        self.out_pkt_sender.send(pkt.bytes);
        let output = if let Ok(Ok(p)) = tokio::time::timeout(std::time::Duration::from_secs(15), receiver).await
        {
            Some(p)
        } else {
            None
        };
        {
            let mut packet_promises = self.packet_promises.write().await;
            packet_promises.remove(&pkt.seq);
        }
        output
    }
}
