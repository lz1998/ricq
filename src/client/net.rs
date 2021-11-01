use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpSocket, TcpStream};
use tokio::sync::mpsc;
use super::Client;
use std::io::Result as IoResult;
use crate::client::income::IncomePacket;


pub type OutPktSender = mpsc::UnboundedSender<Bytes>;
pub type OutPktReceiver = mpsc::UnboundedReceiver<Bytes>;

pub struct ClientNet {
    client: Arc<Client>,
    receiver: OutPktReceiver,
}

impl ClientNet {
    pub fn new(client: Arc<Client>, receiver: OutPktReceiver) -> Self {
        Self { client, receiver }
    }
    pub async fn connect_tcp(&self) -> TcpStream {
        match connect("42.81.172.81:80".parse().unwrap()).await {
            Ok(stream) => {
                self.client.connected.swap(true, Ordering::SeqCst);
                stream
            }
            Err(_) => {
                panic!("Tcp connect error") // todo
            }
        }
    }
    pub async fn send_bytes_to_tcp_stream(
        &mut self,
        stream: &mut TcpStream,
        bytes: Bytes,
    ) -> IoResult<()> {
        stream.writable().await?;
        stream.write(&bytes).await?;
        Ok(())
    }

    pub async fn read_from_tcp_stream(&self, stream: &mut TcpStream) -> IoResult<Bytes> {
        stream.readable().await?;
        let len = stream.read_i32().await?;
        let mut data = BytesMut::with_capacity(len as usize - 4);
        stream.read_buf(&mut data).await?;
        Ok(data.freeze())
    }

    pub async fn read_and_parse(&self, stream: &mut TcpStream) -> IoResult<IncomePacket> {
        let mut in_bytes = self.read_from_tcp_stream(stream).await?;
        match self.client.parse_incoming_packet(&mut in_bytes).await {
            Ok(pkt) => {
                Ok(pkt)
            }
            Err(err) => {
                panic!(err)
            }
        }
    }
    pub async fn net_loop(mut self, mut stream: TcpStream) -> IoResult<()> {
        loop {
            tokio::select! {
                _ = stream.readable() => {
                    let pkt = self.read_and_parse(&mut stream).await?;
                    self.client.handle_income_packet(pkt).await;
                }
                bytes_option = self.receiver.recv() => {
                    if let Some(bytes) = bytes_option {
                        self.send_bytes_to_tcp_stream(&mut stream, bytes).await?;
                    }
                }
            }
        }
    }
}

async fn connect(addr: SocketAddr) -> IoResult<TcpStream> {
    let tcp_connect = TcpSocket::new_v4()?;
    let stream: TcpStream = tcp_connect.connect(addr).await?;
    Ok(stream)
}