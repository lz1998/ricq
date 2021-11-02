use std::sync::Arc;
use std::sync::atomic::Ordering;
use anyhow::Result;
use tokio::time::{Duration, sleep};
use rs_qq::client::{Client, Password};
use rs_qq::client::device::DeviceInfo;
use rs_qq::client::income::QRCodeState;
use rs_qq::client::net::ClientNet;

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
        let resp = client.fetch_qrcode().await.unwrap();
        if let QRCodeState::QRCodeImageFetch { ref image_data, ref sig } = resp {
            tokio::fs::write("qrcode.png", &image_data).await;
            println!("{:?}", &resp);
            loop {
                sleep(Duration::from_secs(5)).await;
                let resp = client.query_qrcode_result(&sig).await.unwrap();
                println!("{:?}", &resp);
                match resp {
                    QRCodeState::QRCodeImageFetch { .. } => {}
                    QRCodeState::QRCodeWaitingForScan => {}
                    QRCodeState::QRCodeWaitingForConfirm => {}
                    QRCodeState::QRCodeTimeout => {}
                    QRCodeState::QRCodeConfirmed { tmp_pwd, tmp_no_pic_sig, tgt_qr } => {
                        let resp = client.qrcode_login(&tmp_pwd, &tmp_no_pic_sig, &tgt_qr).await.unwrap();
                        println!("{:?}", resp);
                        println!("{}",client.uin.load(Ordering::SeqCst));
                        let resp=client.register_client().await.unwrap();
                        println!("{:?}", resp);

                        break;
                    }
                    QRCodeState::QRCodeCanceled => {}
                }
            }
        }
    });
    net.await;
    sleep(Duration::from_millis(100)).await;
    Ok(())
    // client.login().await;
}

