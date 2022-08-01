use std::net::{Ipv4Addr, SocketAddr};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{self};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_util::codec::LengthDelimitedCodec;

use crate::client::tcp::tcp_connect_fastest;
use crate::client::NetworkStatus;

use super::Client;

pub type OutPktSender = broadcast::Sender<Bytes>;

#[async_trait]
pub trait Connector {
    async fn stream(&self) -> io::Result<TcpStream>;
}

pub struct DefaultConnector;

#[async_trait]
impl Connector for DefaultConnector {
    async fn stream(&self) -> io::Result<TcpStream> {
        const SERVERS: &[([u8; 4], u16)] = &[
            ([42, 81, 172, 81], 80),
            ([114, 221, 148, 59], 14000),
            ([42, 81, 172, 147], 443),
            ([125, 94, 60, 146], 80),
            ([114, 221, 144, 215], 80),
            ([42, 81, 172, 22], 80),
        ];
        tcp_connect_fastest(
            SERVERS.iter().map(|v| SocketAddr::from(*v)).collect(),
            Duration::from_secs(5),
        )
        .await
    }
}

impl crate::Client {
    pub fn get_address(&self) -> SocketAddr {
        // TODO 选择最快地址
        SocketAddr::new(Ipv4Addr::new(114, 221, 144, 215).into(), 80)
    }

    pub fn get_status(&self) -> u8 {
        self.status.load(Ordering::Relaxed)
    }

    /// 开始处理流数据
    ///
    ///**Notice: 该方法仅开始处理包，需要手动登录并开始心跳包**
    pub async fn start(self: &Arc<Self>, stream: TcpStream) {
        self.status
            .store(NetworkStatus::Running as u8, Ordering::Relaxed);
        self.net_loop(stream).await; // 阻塞到断开
        self.disconnect();
        if self.get_status() == (NetworkStatus::Running as u8) {
            self.status
                .store(NetworkStatus::NetworkOffline as u8, Ordering::Relaxed);
        }
    }

    pub fn stop(&self, status: NetworkStatus) {
        self.disconnect();
        self.status.store(status as u8, Ordering::Relaxed);
        self.online.store(false, Ordering::Relaxed);
    }

    fn disconnect(&self) {
        // TODO dispatch disconnect event
        // don't unwrap (Err means there is no receiver.)
        self.disconnect_signal.send(()).ok();
    }

    async fn net_loop(self: &Arc<Client>, stream: TcpStream) {
        let (mut write_half, mut read_half) = LengthDelimitedCodec::builder()
            .length_field_length(4)
            .length_adjustment(-4)
            .new_framed(stream)
            .split();
        let cli = self.clone();
        // 外发包 Channel Receiver
        let mut rx = self.out_pkt_sender.subscribe();
        let mut disconnect_signal = self.disconnect_signal.subscribe();
        loop {
            tokio::select! {
                input = read_half.next() => {
                    if let Some(Ok(mut input)) = input {
                        if let Ok(pkt) = cli.engine.read().await.transport.decode_packet(&mut input) {
                            cli.process_income_packet(pkt).await;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                output = rx.recv() => {
                    if let Ok(output) = output {
                        if write_half.send(output).await.is_err() {
                            break;
                        }
                    }
                }
                _ = disconnect_signal.recv() => {
                    break;
                }
            }
        }
    }
}
