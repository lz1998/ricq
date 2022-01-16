use rand::Rng;
use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU16, Ordering};

use crate::client::protocol::device::Device;
use crate::client::protocol::oicq;
use crate::client::protocol::transport::Transport;
use crate::client::protocol::version::Version;

pub mod builder;
pub mod command;
pub mod decoder;

// build_packet: param -> bytes
// decode_packet: bytes -> struct
// this should be wrapped in a rwlock (readonly after login)
// TODO: build library for other language
// no async and await
pub struct Engine {
    pub uin: AtomicI64,
    pub transport: Transport,
    pub oicq_codec: oicq::Codec,
    pub seq_id: AtomicU16,
    pub request_packet_request_id: AtomicI32,
    pub group_seq: AtomicI32,
    pub friend_seq: AtomicI32,
    pub group_data_trans_seq: AtomicI32,
    pub highway_apply_up_seq: AtomicI32,
}

impl Engine {
    pub fn new(device: Device, version: &'static Version) -> Self {
        Self {
            uin: AtomicI64::new(0),
            transport: Transport::new(device, version),
            oicq_codec: oicq::Codec::default(),
            seq_id: AtomicU16::new(0x3635),
            request_packet_request_id: AtomicI32::new(1921334513),
            group_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            friend_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            group_data_trans_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
            highway_apply_up_seq: AtomicI32::new(rand::thread_rng().gen_range(0..20000)),
        }
    }

    pub fn uin(&self) -> i64 {
        self.uin.load(Ordering::Relaxed)
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
}
