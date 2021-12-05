use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use bytes::Bytes;
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
        match connect("42.81.176.211:443".parse().expect("failed to parse addr")).await {
            Ok(stream) => {
                self.client.connected.swap(true, Ordering::SeqCst);
                stream
            }
            Err(_) => {
                panic!("Tcp connect error") // todo
            }
        }
    }

    pub async fn net_loop(mut self, stream: TcpStream) -> IoResult<()> {
        let (mut read_half, mut write_half) = stream.into_split();
        let cli = self.client.clone();
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
                let pkt = cli.parse_incoming_packet(&mut data).await.unwrap();
                cli.process_income_packet(pkt).await;
            }
        });
        loop {
            let sending = self.receiver.recv().await.unwrap();
            if write_half.write_all(&sending).await.is_err() {
                break;
            }
        }
        // TODO dispatch disconnect event
        a.abort();
        Ok(())
    }
}

async fn connect(addr: SocketAddr) -> IoResult<TcpStream> {
    let tcp_connect = TcpSocket::new_v4()?;
    let stream: TcpStream = tcp_connect.connect(addr).await?;
    Ok(stream)
}