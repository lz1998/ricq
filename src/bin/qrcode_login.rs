use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use tokio::time::{sleep, Duration};

use rs_qq::client::handler::DefaultHandler;
use rs_qq::client::Client;
use rs_qq::engine::command::wtlogin::{LoginResponse, QRCodeState};
use rs_qq::engine::protocol::device::Device;
use rs_qq::engine::protocol::version::{get_version, Protocol};

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
    let cli = Client::new_with_config(config, DefaultHandler);
    let client = Arc::new(cli);
    let c = client.clone();
    let handle = tokio::spawn(async move {
        c.start().await.expect("failed to run client");
    });
    tokio::time::sleep(Duration::from_millis(200)).await; // 等一下，确保连上了
    let mut resp = client.fetch_qrcode().await.expect("failed to fetch qrcode");
    let mut image_sig = Bytes::new();
    loop {
        match resp {
            QRCodeState::QRCodeImageFetch {
                ref image_data,
                ref sig,
            } => {
                tokio::fs::write("qrcode.png", &image_data)
                    .await
                    .expect("failed to write file");
                image_sig = sig.clone();
                println!("二维码: qrcode.png");
            }
            QRCodeState::QRCodeWaitingForScan => {
                println!("二维码待扫描")
            }
            QRCodeState::QRCodeWaitingForConfirm => {
                println!("二维码待确认")
            }
            QRCodeState::QRCodeTimeout => {
                println!("二维码已超时，重新获取");
                if let QRCodeState::QRCodeImageFetch {
                    ref image_data,
                    ref sig,
                } = client.fetch_qrcode().await.expect("failed to fetch qrcode")
                {
                    tokio::fs::write("qrcode.png", &image_data)
                        .await
                        .expect("failed to write file");
                    image_sig = sig.clone();
                    println!("二维码: qrcode.png");
                }
            }
            QRCodeState::QRCodeConfirmed {
                ref tmp_pwd,
                ref tmp_no_pic_sig,
                ref tgt_qr,
                ..
            } => {
                println!("二维码已确认");
                let mut login_resp = client
                    .qrcode_login(tmp_pwd, tmp_no_pic_sig, tgt_qr)
                    .await
                    .expect("failed to qrcode login");
                if let LoginResponse::DeviceLockLogin { .. } = login_resp {
                    login_resp = client
                        .device_lock_login()
                        .await
                        .expect("failed to device lock login");
                }
                println!("{:?}", login_resp);
                break;
            }
            QRCodeState::QRCodeCanceled => {
                panic!("二维码已取消")
            }
        }
        sleep(Duration::from_secs(5)).await;
        resp = client
            .query_qrcode_result(&image_sig)
            .await
            .expect("failed to query qrcode result");
    }
    client.register_client().await.unwrap();
    {
        client
            .reload_friend_list()
            .await
            .expect("failed to reload friend list");
        println!("{:?}", client.friends.read().await);
        client
            .reload_groups()
            .await
            .expect("failed to reload group list");
        println!("{:?}", client.groups.read().await);
    }

    handle.await.unwrap();
    Ok(())
}
