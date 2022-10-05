use std::path::Path;
use std::sync::Arc;

use bytes::Bytes;
use tokio::time::{sleep, Duration};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ricq::client::{Connector as _, DefaultConnector};
use ricq::ext::common::after_login;
use ricq::handler::DefaultHandler;
use ricq::{Client, Device, Protocol};
use ricq::{LoginResponse, QRCodeConfirmed, QRCodeImageFetch, QRCodeState};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", Level::DEBUG)
                .with_target("qrcode_login", Level::DEBUG),
        )
        .init();

    let device = match Path::new("device.json").exists() {
        true => serde_json::from_str(
            &tokio::fs::read_to_string("device.json")
                .await
                .expect("failed to read device.json"),
        )
        .expect("failed to parse device info"),
        false => {
            let d = Device::random();
            tokio::fs::write("device.json", serde_json::to_string(&d).unwrap())
                .await
                .expect("failed to write device info to file");
            d
        }
    };

    let client = Arc::new(Client::new(
        device,
        Protocol::AndroidWatch.into(),
        DefaultHandler,
    ));
    let handle = tokio::spawn({
        let client = client.clone();
        let stream = DefaultConnector.connect(&client).await.unwrap();
        async move { client.start(stream).await }
    });
    tokio::task::yield_now().await; // 等一下，确保连上了
    let mut resp = client.fetch_qrcode().await.expect("failed to fetch qrcode");

    // // vvv 如果不关心二维码状态，可以用这个替换下面的 vvv
    // use ricq::ext::login::auto_query_qrcode;
    // match resp {
    //     QRCodeState::QRCodeImageFetch {
    //         ref image_data,
    //         ref sig,
    //     } => {
    //         tokio::fs::write("qrcode.png", &image_data).await.expect("failed to write file");
    //         if let Err(err) = auto_query_qrcode(&client, sig).await {
    //             panic!("登录失败 {}", err)
    //         };
    //     }
    //     _ => {
    //         panic!("resp error")
    //     }
    // }
    // // ^^^ 如果不关心二维码状态，可以用这个替换下面的 ^^^

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

    // vvv 如果关心二维码状态，可以用这个 vvv
    let mut image_sig = Bytes::new();
    loop {
        match resp {
            QRCodeState::ImageFetch(QRCodeImageFetch {
                ref image_data,
                ref sig,
            }) => {
                tokio::fs::write("qrcode.png", &image_data)
                    .await
                    .expect("failed to write file");
                image_sig = sig.clone();
                tracing::info!("二维码: qrcode.png");
            }
            QRCodeState::WaitingForScan => {
                tracing::info!("二维码待扫描")
            }
            QRCodeState::WaitingForConfirm => {
                tracing::info!("二维码待确认")
            }
            QRCodeState::Timeout => {
                tracing::info!("二维码已超时，重新获取");
                if let QRCodeState::ImageFetch(QRCodeImageFetch {
                    ref image_data,
                    ref sig,
                }) = client.fetch_qrcode().await.expect("failed to fetch qrcode")
                {
                    tokio::fs::write("qrcode.png", &image_data)
                        .await
                        .expect("failed to write file");
                    image_sig = sig.clone();
                    tracing::info!("二维码: qrcode.png");
                }
            }
            QRCodeState::Confirmed(QRCodeConfirmed {
                ref tmp_pwd,
                ref tmp_no_pic_sig,
                ref tgt_qr,
                ..
            }) => {
                tracing::info!("二维码已确认");
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
                tracing::info!("{:?}", login_resp);
                break;
            }
            QRCodeState::Canceled => {
                panic!("二维码已取消")
            }
        }
        sleep(Duration::from_secs(5)).await;
        resp = client
            .query_qrcode_result(&image_sig)
            .await
            .expect("failed to query qrcode result");
    }
    // ^^^ 如果不关心二维码状态，可以用这个 ^^^

    after_login(&client).await;
    {
        tracing::info!("{:?}", client.get_friend_list().await);
        tracing::info!("{:?}", client.get_group_list().await);
    }

    handle.await.unwrap();
}
