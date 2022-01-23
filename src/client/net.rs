use std::io;
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
pub type Connection = JoinHandle<()>;

impl crate::Client {
    pub async fn run(self: &Arc<Self>) -> io::Result<()> {
        let addr = "42.81.176.211:443"
            .parse::<SocketAddr>()
            .expect("failed to parse addr");

        let mut conn = self.connection.lock().unwrap();
        *conn = Some(self.connect(&addr).await?);
        Ok(())
    }

    pub fn disconnect(&self) {
        let mut conns = self.connection.lock().unwrap();
        if let Some(conn) = conns.take() {
            conn.abort()
        }
    }

    async fn connect(self: &Arc<Self>, addr: &SocketAddr) -> io::Result<Connection> {
        let stream = TcpStream::connect(&addr).await?;
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
        while !cli.shutting_down.load(Ordering::Relaxed) {
            let cli = cli.clone();
            tokio::select! {
                input = read_half.next() => {
                    if let Some(Ok(mut input)) = input {
                        let pkt = {
                            let engine = cli.engine.read().await;
                            let mut pkt = engine.transport.decode_packet(&mut input).unwrap();
                            if pkt.encrypt_type == EncryptType::EmptyKey {
                                // decrypt with ecdh
                                pkt.body = engine.oicq_codec.decode(pkt.body).unwrap().body;
                            }
                            pkt
                        };
                        cli.process_income_packet(pkt).await;
                    }
                }
                output = rx.recv() => {
                    if let Ok(output) = output {
                        if let Err(_)=write_half.send(output).await{
                            break;
                        }
                    }
                }
            }
        }
    }
}
