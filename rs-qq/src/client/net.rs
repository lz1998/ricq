use std::net::{Ipv4Addr, SocketAddr};
use std::sync::atomic::Ordering;
use std::sync::Arc;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::broadcast;
use tokio_util::codec::LengthDelimitedCodec;

use super::Client;

pub type OutPktSender = broadcast::Sender<Bytes>;

impl crate::Client {
    pub fn get_address(&self) -> SocketAddr {
        // TODO 选择最快地址
        SocketAddr::new(Ipv4Addr::new(114, 221, 144, 215).into(), 80)
    }

    // 开始处理流数据
    pub async fn start<S: AsyncRead + AsyncWrite>(self: &Arc<Self>, stream: S) {
        self.running.store(true, Ordering::Relaxed);
        self.net_loop(stream).await; // 阻塞到断开
        self.disconnect();
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
        self.disconnect();
    }

    fn disconnect(&self) {
        // TODO dispatch disconnect event
        // don't unwrap (Err means there is no receiver.)
        self.disconnect_signal.send(()).ok();
    }

    async fn net_loop<S: AsyncRead + AsyncWrite>(self: &Arc<Client>, stream: S) {
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
                        if write_half.send(output).await.is_err(){
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
