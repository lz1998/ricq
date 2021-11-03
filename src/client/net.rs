use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpSocket, TcpStream};
use tokio::sync::mpsc;
use super::Client;
use std::io::Result as IoResult;


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

    pub async fn read_from_tcp_stream(stream: &mut TcpStream) -> IoResult<Bytes> {
        stream.readable().await?;
        let len = stream.read_i32().await?;
        let mut data = vec![0; len as usize - 4];
        stream.read_exact(&mut data).await?;
        Ok(Bytes::from(data))
    }

    pub async fn net_loop(mut self, mut stream: TcpStream) -> IoResult<()> {
        loop {
            tokio::select! {
                bytes_result = Self::read_from_tcp_stream(&mut stream) => {
                    match bytes_result {
                        Ok(mut b)=>{
                            let pkt=self.client.parse_incoming_packet(&mut b).await.unwrap();
                            self.client.handle_income_packet(pkt).await;
                        }
                        Err(_)=>{}
                    };
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