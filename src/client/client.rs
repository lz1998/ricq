use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU16, Ordering};
use std::sync::Arc;
use bytes::Bytes;
use rand::Rng;
use tokio::sync::RwLock;
use crate::client::device::{DeviceInfo, Version};
use crate::crypto::EncryptECDH;
use crate::client::{AccountInfo, CacheInfo, Client, net, Password};
use crate::client::income::IncomingPacket;
use crate::client::version::{ClientProtocol, gen_version_info, VersionInfo};



impl Client {
    pub async fn new(
        uin: i64,
        password: Password,
    ) -> (Client, net::OutPktReceiver) {
        let (out_pkt_sender, out_pkt_receiver) = tokio::sync::mpsc::unbounded_channel();

        let mut cli = Client {
            seq_id: AtomicU16::new(0x3635),
            uin: AtomicI64::new(uin),
            password_md5: password.md5(),
            connected: AtomicBool::new(false),
            out_pkt_sender,
            ecdh: EncryptECDH::default(),
            random_key: Bytes::from(rand::thread_rng().gen::<[u8; 16]>().to_vec()),
            version: gen_version_info(&ClientProtocol::IPad),
            device_info: DeviceInfo::random(),
            out_going_packet_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            account_info: RwLock::new(AccountInfo::default()),
            cache_info: RwLock::new(CacheInfo::default()),
        };
        cli.cache_info.write().await.ksid = format!("|{}|A8.2.7.27f6ea96", cli.device_info.imei).into();
        (cli, out_pkt_receiver)
    }
    pub fn next_seq(&self) -> u16 {
        self.seq_id.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn handle_income_packet(&self, pkt: IncomingPacket) {
        println!("{:?}", pkt)
    }
}
