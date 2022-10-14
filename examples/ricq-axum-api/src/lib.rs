use bytes::Bytes;
use dashmap::DashMap;
use ricq::ext::reconnect::Credential;
use ricq::handler::QEvent;
use ricq::{Client, LoginResponse, QRCodeState};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

pub mod handler;
pub mod processor;
pub mod u8_protocol;
use serde::{Deserialize, Serialize};

use crate::processor::Processor;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClientInfo {
    pub uin: i64,
    pub nick: String,
    pub status: u8,
    pub protocol: u8,
}

pub struct PasswordClient {
    pub client: Arc<Client>,
    pub login_response: LoginResponse,
    pub event_receiver: broadcast::Receiver<QEvent>,
    pub network_join_handle: JoinHandle<()>,
    pub credential: Credential,
}

pub struct QRCodeClient {
    pub sig: Vec<u8>,
    pub image: Vec<u8>,
    pub state: QRCodeState,
    pub client: Arc<Client>,
    pub event_receiver: broadcast::Receiver<QEvent>,
    pub network_join_handle: JoinHandle<()>,
}

pub struct RicqAxumApi<P: Processor> {
    // key: uin+protocol
    password_clients: DashMap<(i64, u8), PasswordClient>,

    // key: sig
    qrcode_clients: DashMap<Bytes, QRCodeClient>,

    // 仅负责登录后的逻辑
    processor: P,
}

impl<P: Processor> RicqAxumApi<P> {
    pub fn new(processor: P) -> Self {
        Self {
            password_clients: Default::default(),
            qrcode_clients: Default::default(),
            processor,
        }
    }
}
