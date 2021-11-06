use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU16, Ordering};
use bytes::Bytes;
use chrono::Utc;
use rand::Rng;
use tokio::sync::RwLock;
use crate::client::device::{DeviceInfo};
use crate::crypto::EncryptECDH;
use crate::client::{AccountInfo, AddressInfo, CacheInfo, Client, MessageStateInfo, net, Password};
use crate::client::income::IncomePacket;
use crate::client::outcome::OutcomePacket;
use crate::client::version::{ClientProtocol, gen_version_info};
use tokio::sync::oneshot;
use crate::client::income::decoder::online_push::decode_group_message_packet;
use crate::client::structs::GroupInfo;


impl Client {
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
            uin: AtomicI64::new(uin),
            password_md5: password.md5(),
            connected: AtomicBool::new(false),
            online: AtomicBool::new(false),
            out_pkt_sender,
            packet_promises: RwLock::new(HashMap::new()),
            ecdh: EncryptECDH::default(),
            random_key: Bytes::from(rand::thread_rng().gen::<[u8; 16]>().to_vec()),
            version: gen_version_info(&ClientProtocol::IPad),
            device_info: RwLock::new(device_info),
            out_going_packet_session_id: RwLock::new(Bytes::from_static(&[0x02, 0xb0, 0x5b, 0x8b])),
            account_info: RwLock::new(AccountInfo::default()),
            cache_info: RwLock::new(CacheInfo::default()),
            address: RwLock::new(AddressInfo::default()),
            message_state: RwLock::new(MessageStateInfo::default()),
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
        self.request_packet_request_id.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn next_group_seq(&self) -> i32 {
        self.message_state.read().await.group_seq + 2
    }

    pub async fn next_friend_seq(&self) -> i32 {
        self.message_state.read().await.friend_seq + 1
    }

    pub async fn next_group_data_trans_seq(&self) -> i32 {
        self.message_state.read().await.group_data_trans_seq + 2
    }

    pub async fn next_highway_apply_seq(&self) -> i32 {
        self.message_state.read().await.highway_apply_up_seq + 2
    }

    pub async fn find_group(&self, code: i64) -> Option<GroupInfo> {
        for g in &self.account_info.read().await.group_list {
            if g.code == code {
                return Some(g.clone())
            }
        }
        None
    }

    pub async fn find_group_by_uin(&self, uin: i64) -> Option<GroupInfo> {
        for g in &self.account_info.read().await.group_list {
            if g.uin == uin {
                return Some(g.clone())
            }
        }
        None
    }

    pub async fn get_s_key(&self) -> String {
        if self.cache_info.read().await.sig_info.s_key_expired_time < Utc::now().timestamp() && self.cache_info.read().await.g.len() > 0 {
            println!("skey expired. refresh...");
            self.send_and_wait(self.build_request_tgtgt_no_pic_sig_packet().await.into());
        }
        String::from_utf8(self.cache_info.read().await.sig_info.s_key.to_vec()).unwrap()
    }

    pub async fn get_csrf_token(&self) -> isize {
        let mut accu = 5381;
        for b in self.get_s_key().await.bytes() {
            accu = accu + (accu << 5) + b as isize
        }
        2147483647 & accu
    }

    pub async fn handle_income_packet(&self, pkt: IncomePacket) {
        // response
        if let Some(sender) = self.packet_promises.write().await.remove(&pkt.seq_id) {
            sender.send(pkt); // response
            return;
        }
        // TODO decoders -> default handlers
        if pkt.command_name == "OnlinePush.PbPushGroupMsg" {
            let msg = decode_group_message_packet(&pkt.payload).unwrap();
            println!("{:?}", msg);
            return;
        }
        println!("unhandled pkt: {}", &pkt.command_name);
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
