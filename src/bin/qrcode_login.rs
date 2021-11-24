use anyhow::Result;
use rs_qq::client::device::DeviceInfo;
use rs_qq::client::handler::DefaultHandler;
use rs_qq::client::income::decoder::wtlogin::{LoginResponse, QRCodeState};
use rs_qq::client::net::ClientNet;
use rs_qq::client::{Client, Password};
use std::path::Path;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    let uin = 0;
    let password = "";

    let device_info = match Path::new("device.json").exists() {
        true => {
            DeviceInfo::from_json(&tokio::fs::read_to_string("device.json").await.unwrap()).unwrap()
        }
        false => DeviceInfo::random(),
    };
    tokio::fs::write("device.json", device_info.to_json())
        .await
        .unwrap(); //todo

    let (cli, receiver) = Client::new(
        uin,
        Password::from_str(password),
        device_info,
        DefaultHandler,
    )
    .await;
    let client = Arc::new(cli);
    let client_net = ClientNet::new(client.clone(), receiver);
    let stream = client_net.connect_tcp().await;
    let net = tokio::spawn(client_net.net_loop(stream));
    tokio::spawn(async move {
        let resp = client.fetch_qrcode().await.unwrap();

        if let QRCodeState::QRCodeImageFetch {
            ref image_data,
            ref sig,
        } = resp
        {
            tokio::fs::write("qrcode.png", &image_data).await.unwrap(); //todo
            println!("二维码: qrcode.png");
            loop {
                sleep(Duration::from_secs(5)).await;
                let resp = client.query_qrcode_result(sig).await.unwrap();
                match resp {
                    QRCodeState::QRCodeImageFetch { .. } => {}
                    QRCodeState::QRCodeWaitingForScan => {
                        println!("二维码待扫描")
                    }
                    QRCodeState::QRCodeWaitingForConfirm => {
                        println!("二维码待确认")
                    }
                    QRCodeState::QRCodeTimeout => {
                        println!("二维码超时")
                    }
                    QRCodeState::QRCodeConfirmed {
                        ref tmp_pwd,
                        ref tmp_no_pic_sig,
                        ref tgt_qr,
                    } => {
                        println!("二维码已确认");
                        let mut login_resp = client
                            .qrcode_login(tmp_pwd, tmp_no_pic_sig, tgt_qr)
                            .await
                            .unwrap();
                        if let LoginResponse::NeedDeviceLockLogin = login_resp {
                            login_resp = client.device_lock_login().await.unwrap();
                        }
                        println!("{:?}", login_resp);
                        break;
                    }
                    QRCodeState::QRCodeCanceled => {}
                }
            }
            client.register_client().await.unwrap(); //todo
            let rsp = client.group_list.read().await;
            println!("{:?}", rsp);
            let rsp = client.friend_list.read().await;
            println!("{:?}", rsp);
        } else {
            panic!("error")
        }
    });
    net.await.unwrap().unwrap(); //todo

    Ok(())
}
