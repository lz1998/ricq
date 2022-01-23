use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio_util::codec::LengthDelimitedCodec;

use rq_engine::{RQError, RQResult};

use super::Client;

pub type OutPktSender = broadcast::Sender<Bytes>;
pub type OutPktReceiver = broadcast::Receiver<Bytes>;
pub type Connection = JoinHandle<()>;

impl crate::Client {
    pub async fn start(self: &Arc<Self>) -> RQResult<()> {
        self.running.store(true, Ordering::Relaxed);
        let addr = "42.81.176.211:443"
            .parse::<SocketAddr>()
            .expect("failed to parse addr");

        let conn = self.connect(&addr).await?;
        conn.await.unwrap();
        while self.running.load(Ordering::Relaxed) {
            if self.online.load(Ordering::Relaxed) {
                // 登录过，快速重连，恢复登录
                if let Err(_) = self.quick_reconnect(&addr).await {
                    self.online.store(false, Ordering::Relaxed);
                    // TODO dispatch offline event
                    // break;
                }
            } else {
                // 没登录过，重连
                self.reconnect(&addr).await?;
            }
        }
        self.disconnect();
        Ok(())
    }

    pub fn stop(self: &Arc<Self>) {
        self.running.store(false, Ordering::Relaxed);
        self.disconnect();
    }

    fn disconnect(&self) {
        // TODO dispatch disconnect event
        // don't unwrap (Err means there is no receiver.)
        self.disconnect_signal.send(()).ok();
    }

    async fn connect(self: &Arc<Self>, addr: &SocketAddr) -> RQResult<Connection> {
        let stream = TcpStream::connect(&addr).await.map_err(RQError::IO)?;
        let cli = self.clone();
        Ok(tokio::spawn(async move { cli.net_loop(stream).await }))
    }

    async fn net_loop(self: &Arc<Client>, stream: TcpStream) {
        let (mut write_half, mut read_half) = LengthDelimitedCodec::builder()
            .length_field_length(4)
            .length_adjustment(-4)
            .new_framed(stream)
            .split();
        let cli = self.clone();
        let mut rx = self.out_pkt_sender.subscribe();
        let mut disconnect_signal = self.disconnect_signal.subscribe();
        loop {
            tokio::select! {
                input = read_half.next() => {
                    if let Some(Ok(mut input)) = input {
                        if let Ok(pkt)=cli.engine.read().await.transport.decode_packet(&mut input){
                            cli.process_income_packet(pkt).await;
                        }else {
                            break;
                        }
                    }else {
                        break;
                    }
                }
                output = rx.recv() => {
                    if let Ok(output) = output {
                        if let Err(_)=write_half.send(output).await{
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

    async fn quick_reconnect(self: &Arc<Self>, addr: &SocketAddr) -> RQResult<Connection> {
        self.disconnect();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        let conn = self.connect(addr).await?;
        self.register_client().await?;
        Ok(conn)
    }

    async fn reconnect(self: &Arc<Self>, addr: &SocketAddr) -> RQResult<Connection> {
        self.disconnect();
        self.connect(addr).await
    }
}
