use std::future::Future;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use bytes::Bytes;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};

use crate::client::protocol::packet::EncryptType;

use super::Client;

pub type OutPktSender = mpsc::UnboundedSender<Bytes>;
pub type OutPktReceiver = mpsc::UnboundedReceiver<Bytes>;

pub struct ClientNet {
    // client: Arc<Client>,
    receiver: Arc<RwLock<OutPktReceiver>>,
}

impl ClientNet {
    pub fn new(receiver: OutPktReceiver) -> Self {
        Self {
            receiver: Arc::new(RwLock::new(receiver)),
        }
    }

    pub async fn run(&self, client: &Arc<Client>) -> impl Future<Output = ()> {
        let stream = self.connect_tcp(client).await;
        self.net_loop(client, stream)
    }

    pub async fn connect_tcp(&self, client: &Arc<Client>) -> TcpStream {
        match TcpStream::connect(
            "42.81.176.211:443"
                .parse::<SocketAddr>()
                .expect("failed to parse addr"),
        )
        .await
        {
            Ok(stream) => {
                client.connected.swap(true, Ordering::SeqCst);
                stream
            }
            Err(_) => {
                panic!("Tcp connect error") // todo
            }
        }
    }

    pub fn net_loop(&self, client: &Arc<Client>, stream: TcpStream) -> impl Future<Output = ()> {
        let (mut read_half, mut write_half) = stream.into_split();
        let cli = client.clone();
        let a = tokio::spawn(async move {
            loop {
                let cli = cli.clone();
                let len = read_half.read_i32().await.unwrap();
                if len - 4 < 0 {
                    panic!("invalid packet length: {}", len);
                }
                let mut data = vec![0; len as usize - 4];
                read_half.read_exact(&mut data).await.unwrap();
                let mut data = Bytes::from(data);

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
        let cli = client.clone();
        let rx = self.receiver.clone();
        async move {
            while !cli.shutting_down.load(Ordering::SeqCst) {
                let sending = rx.write().await.recv().await.unwrap();
                if write_half.write_all(&sending).await.is_err() {
                    break;
                }
            }
            // TODO dispatch disconnect event
            a.abort();
        }
    }
}
