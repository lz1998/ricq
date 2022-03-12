#![feature(type_alias_impl_trait)]

use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU16, Ordering};

use bytes::Bytes;
use rand::Rng;

pub use error::{RQError, RQResult};
use protocol::device::Device;
use protocol::oicq;
use protocol::transport::Transport;
use protocol::version::Version;

pub use crate::token::Token;

pub mod binary;
pub mod command;
pub mod common;
pub mod crypto;
pub mod error;
pub mod hex;
pub mod highway;
pub mod jce;
pub mod msg;
pub mod pb;
pub mod protocol;
pub mod structs;
pub mod token;
pub mod wtlogin;

// build_packet: param -> bytes
// decode_packet: bytes -> struct
// this should be wrapped in a rwlock (readonly after login)
// TODO: build library for other language
// no async and await
pub struct Engine {
    pub uin: AtomicI64,
    pub transport: Transport,
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

    pub fn gen_token(&self) -> Token {
        Token {
            uin: self.uin(),
            d2: self.transport.sig.d2.to_vec(),
            d2key: self.transport.sig.d2key.to_vec(),
            tgt: self.transport.sig.tgt.to_vec(),
            srm_token: self.transport.sig.srm_token.to_vec(),
            t133: self.transport.sig.t133.to_vec(),
            encrypted_a1: self.transport.sig.encrypted_a1.to_vec(),
            out_packet_session_id: self.transport.sig.out_packet_session_id.to_vec(),
            tgtgt_key: self.transport.sig.tgtgt_key.to_vec(),
            wt_session_ticket_key: self.transport.oicq_codec.wt_session_ticket_key.to_vec(),
        }
    }

    pub fn load_token(&mut self, token: Token) {
        self.uin.store(token.uin, Ordering::Relaxed);
        self.transport.sig.d2 = Bytes::from(token.d2);
        self.transport.sig.d2key = Bytes::from(token.d2key);
        self.transport.sig.tgt = Bytes::from(token.tgt);
        self.transport.sig.srm_token = Bytes::from(token.srm_token);
        self.transport.sig.t133 = Bytes::from(token.t133);
        self.transport.sig.encrypted_a1 = Bytes::from(token.encrypted_a1);
        self.transport.sig.out_packet_session_id = Bytes::from(token.out_packet_session_id);
        self.transport.sig.tgtgt_key = Bytes::from(token.tgtgt_key);
        self.transport.oicq_codec.wt_session_ticket_key = Bytes::from(token.wt_session_ticket_key);
    }
}
