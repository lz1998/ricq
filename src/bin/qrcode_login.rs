use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use tokio::time::{sleep, Duration};

use rs_qq::client::handler::DefaultHandler;
use rs_qq::client::income::decoder::wtlogin::{LoginResponse, QRCodeState};
use rs_qq::client::protocol::device::Device;
use rs_qq::client::protocol::version::{get_version, Protocol};
use rs_qq::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let device = match Path::new("device.json").exists() {
        true => serde_json::from_str(
            &tokio::fs::read_to_string("device.json")
                .await
                .expect("failed to read device.json"),
        )
        .expect("failed to parse device json"),
        false => Device::random(),
    };
    tokio::fs::write("device.json", serde_json::to_string(&device).unwrap())
        .await
        .expect("failed to write device.json"); //todo

    let config = rs_qq::Config::new(device, get_version(Protocol::IPad));
    let cli = Client::new_with_config(config, DefaultHandler).await;
    let client = Arc::new(cli);
    let net = client.run().await;
    tokio::spawn(async move {
        let resp = client.fetch_qrcode().await.expect("failed to fetch qrcode");

        if let QRCodeState::QRCodeImageFetch {
            ref image_data,
            ref sig,
        } = resp
        {
            tokio::fs::write("qrcode.png", &image_data)
                .await
                .expect("failed to write file");
            println!("二维码: qrcode.png");
            loop {
                sleep(Duration::from_secs(5)).await;
                let resp = client
                    .query_qrcode_result(sig)
                    .await
                    .expect("failed to query qrcode result");
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
                            .expect("failed to qrcode login");
                        if let LoginResponse::NeedDeviceLockLogin = login_resp {
                            login_resp = client
                                .device_lock_login()
                                .await
                                .expect("failed to device lock login");
                        }
                        println!("{:?}", login_resp);
                        break;
                    }
                    QRCodeState::QRCodeCanceled => {}
                }
            }
            client.register_client().await.unwrap();
            let rsp = client.group_list.read().await;
            println!("{:?}", rsp);
            let rsp = client.friend_list.read().await;
            println!("{:?}", rsp);
        } else {
            panic!("error")
        }
    });
    net.await.expect("network error"); //todo

    Ok(())
}
