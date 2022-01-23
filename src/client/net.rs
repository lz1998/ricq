use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio_util::codec::LengthDelimitedCodec;

use crate::engine::protocol::packet::EncryptType;

use super::Client;

pub type OutPktSender = broadcast::Sender<Bytes>;
pub type OutPktReceiver = broadcast::Receiver<Bytes>;

impl crate::Client {
    pub async fn connect(self: &Arc<Self>) {
        let mut conns = self.connects.lock().unwrap();
        if conns.is_none() {
            let stream = self.connect_tcp().await;
            *conns = Some(self.net_loop(stream).await);
        }
    }

    pub fn disconnect(&self) {
        let mut conns = self.connects.lock().unwrap();
        if let Some((r, w)) = conns.take() {
            w.abort();
            r.abort();
        }
    }

    async fn connect_tcp(self: &Arc<Self>) -> TcpStream {
        match TcpStream::connect(
            "42.81.176.211:443"
                .parse::<SocketAddr>()
                .expect("failed to parse addr"),
        )
        .await
        {
            Ok(stream) => stream,
            Err(_) => {
                panic!("Tcp connect error") // todo
            }
        }
    }

    async fn net_loop(self: &Arc<Client>, stream: TcpStream) -> (JoinHandle<()>, JoinHandle<()>) {
        let (mut write_half, mut read_half) = LengthDelimitedCodec::builder()
            .length_field_length(4)
            .length_adjustment(-4)
            .new_framed(stream)
            .split();
        let cli = self.clone();
        let r = tokio::spawn(async move {
            loop {
                let cli = cli.clone();
                let mut data = read_half.next().await.unwrap().unwrap();
                let pkt = {
                    let engine = cli.engine.read().await;
                    let mut pkt = engine.transport.decode_packet(&mut data).unwrap();
                    if pkt.encrypt_type == EncryptType::EmptyKey {
                        // decrypt with ecdh
                        pkt.body = engine.oicq_codec.decode(pkt.body).unwrap().body;
                    }
                    pkt
                };
                cli.process_income_packet(pkt).await;
            }
        });
        let cli = self.clone();
        let mut rx = self.out_pkt_sender.subscribe();
        let w = tokio::spawn(async move {
            while !cli.shutting_down.load(Ordering::SeqCst) {
                let sending = rx.recv().await.unwrap();
                if write_half.send(sending).await.is_err() {
                    break;
                }
            }
            // TODO dispatch disconnect event
        });
        (r, w)
    }
}
