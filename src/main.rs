use std::sync::Arc;
use anyhow::Result;
use bytes::Bytes;
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use rs_qq::client::{Client, Password};
use rs_qq::client::device::DeviceInfo;
use rs_qq::client::income::{decode_login_response, decode_trans_emp_response, LoginState};
use rs_qq::client::net::ClientNet;
use rs_qq::client::outcome::OutcomePacket;

#[tokio::main]
async fn main() -> Result<()> {
    let (cli, receiver) = Client::new(
        0,
        Password::from_str(""),
        DeviceInfo::random(),
    ).await;

    let client = Arc::new(cli);
    let client_net = ClientNet::new(client.clone(), receiver);
    let stream = client_net.connect_tcp().await;
    let net = tokio::spawn(client_net.net_loop(stream));
    tokio::spawn(async move {
        let client = client.clone();
        let (seq, pkt) = client.build_qrcode_fetch_request_packet().await;
        let resp = client.send_and_wait(OutcomePacket {
            seq,
            bytes: pkt,
        }).await.unwrap();
        let resp = decode_trans_emp_response(&client, &resp.payload).await.unwrap();
        tokio::fs::write("qrcode.png", &resp.image_data).await;
        println!("{:?}", resp);
        let sig = resp.sig;
        loop {
            sleep(Duration::from_secs(1));
            let (seq, pkt) = client.build_qrcode_result_query_request_packet(&sig).await;
            let resp = client.send_and_wait(OutcomePacket { seq, bytes: pkt }).await.unwrap();
            let resp = decode_trans_emp_response(&client, &resp.payload).await.unwrap();
            println!("{:?}", &resp.state);
            match resp.state {
                LoginState::QRCodeImageFetch => {}
                LoginState::QRCodeWaitingForScan => {}
                LoginState::QRCodeWaitingForConfirm => {}
                LoginState::QRCodeTimeout => {}
                LoginState::QRCodeConfirmed => {
                    let cache_info = client.cache_info.read().await;
                    let (seq, pkt) = client.build_qrcode_login_packet(&resp.login_info.tmp_pwd, &resp.login_info.tmp_no_pic_sig, &resp.login_info.tgt_qr).await;
                    let resp = client.send_and_wait(OutcomePacket { seq, bytes: pkt }).await.unwrap();
                    let resp = decode_login_response(&client, &resp.payload).await.unwrap();
                    println!("{:?}", resp);
                }
                LoginState::QRCodeCanceled => {}
            }
        }
    });
    net.await;
    sleep(Duration::from_millis(100)).await;
    Ok(())
    // client.login().await;
}

