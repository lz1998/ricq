use std::sync::Arc;
use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use rs_qq::client::{Client, Password};
use rs_qq::net::ClientNet;

#[tokio::main]
async fn main() -> Result<()> {
    // let stream = TcpStream::connect("127.0.0.1:9527").await?;
    // let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    // stream.send(Bytes::from("hello world")).await?;
    //
    // // 接收从服务器返回的数据
    // if let Some(Ok(data)) = stream.next().await {
    //     println!("Got: {:?}", String::from_utf8_lossy(&data));
    // }
    //
    // Ok(())

    let (cli, receiver) = Client::new(
        0,
        Password::from_str(""),
    ).await;

    let client = Arc::new(cli);
    let client_net = ClientNet::new(client.clone(), receiver);
    let stream = client_net.connect_tcp().await;
    let net = tokio::spawn(client_net.net_loop(stream));
    let (seq, pkt) = client.build_qrcode_fetch_request_packet();

    // client.login().await;
}

