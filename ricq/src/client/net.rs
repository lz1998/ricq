use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::client::event::{ClientDisconnect, DisconnectReason};
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_util::codec::LengthDelimitedCodec;

use crate::client::tcp::tcp_connect_fastest;
use crate::client::NetworkStatus;
use crate::handler::QEvent;

use super::Client;

pub type OutPktSender = broadcast::Sender<Bytes>;

#[async_trait]
pub trait Connector<T: AsyncRead + AsyncWrite> {
    async fn connect(&self, client: &Client) -> io::Result<T>;
}

pub struct DefaultConnector;

#[async_trait]
impl Connector<TcpStream> for DefaultConnector {
    async fn connect(&self, client: &Client) -> io::Result<TcpStream> {
        tcp_connect_fastest(client.get_address_list().await, Duration::from_secs(5)).await
    }
}

impl crate::Client {
    /// 获取服务器地址
    pub async fn get_address_list(&self) -> Vec<SocketAddr> {
        const BUILD_IN: [([u8; 4], u16); 6] = [
            ([42, 81, 172, 81], 80),
            ([114, 221, 148, 59], 14000),
            ([42, 81, 172, 147], 443),
            ([125, 94, 60, 146], 80),
            ([114, 221, 144, 215], 80),
            ([42, 81, 172, 22], 80),
        ];
        let mut addrs: Vec<_> = BUILD_IN.into_iter().map(SocketAddr::from).collect();
        if let Ok(res) = tokio::net::lookup_host(("msfwifi.3g.qq.com", 8080)).await {
            addrs.extend(res);
        }
        // TODO: src/client/processor/config_push_svc.rs
        addrs
    }

    /// 获取网络状态
    pub fn get_status(&self) -> u8 {
        self.status.load(Ordering::Relaxed)
    }

    /// 开始处理流数据，阻塞当前 Task。该方法返回即为断线。
    ///
    /// **Notice: 该方法仅开始处理包，需要手动登录并开始心跳包**
    pub async fn start(self: &Arc<Self>, stream: impl AsyncRead + AsyncWrite) {
        self.status
            .store(NetworkStatus::Running as u8, Ordering::Relaxed);
        self.net_loop(stream).await; // 阻塞到断开
        self.disconnect();
        self.online.store(false, Ordering::Relaxed);

        match self.status.compare_exchange(
            NetworkStatus::Running as u8,
            NetworkStatus::NetworkOffline as u8,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                self.handler
                    .handle(QEvent::ClientDisconnect(ClientDisconnect {
                        client: Arc::clone(self),
                        inner: DisconnectReason::Network,
                    }))
                    .await;
            }
            Err(status) => {
                self.handler
                    .handle(QEvent::ClientDisconnect(ClientDisconnect {
                        client: Arc::clone(self),
                        inner: {
                            let network = match status {
                                0 => NetworkStatus::Unknown,
                                1 => NetworkStatus::Running,
                                2 => NetworkStatus::Stop,
                                3 => NetworkStatus::Drop,
                                5 => NetworkStatus::KickedOffline,
                                6 => NetworkStatus::MsfOffline,
                                _ => NetworkStatus::Unknown,
                            };

                            DisconnectReason::Actively(network)
                        },
                    }))
                    .await;
            }
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

    async fn net_loop(self: &Arc<Client>, stream: impl AsyncRead + AsyncWrite) {
        let (mut write_half, mut read_half) = LengthDelimitedCodec::builder()
            .length_field_length(4)
            .length_adjustment(-4)
            .new_framed(stream)
            .split();
        // 外发包 Channel Receiver
        let mut rx = self.out_pkt_sender.subscribe();
        let mut disconnect_signal = self.disconnect_signal.subscribe();
        loop {
            tokio::select! {
                input = read_half.next() => {
                    if let Some(Ok(mut input)) = input {
                        if let Ok(pkt) = self.engine.read().await.transport.decode_packet(&mut input) {
                            self.process_income_packet(pkt).await;
                        } else {
                            self.status.store(NetworkStatus::MsfOffline as u8, Ordering::Relaxed);
                            break;
                        }
                    } else {
                        break;
                    }
                }
                output = rx.recv() => {
                    if let Ok(output) = output && write_half.send(output).await.is_err() {
                        break;
                    }
                }
                _ = disconnect_signal.recv() => {
                    break;
                }
            }
        }
    }
}
