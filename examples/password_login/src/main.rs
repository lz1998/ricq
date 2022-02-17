use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, LinesCodec};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rs_qq::device::Device;
use rs_qq::ext::common::after_login;
use rs_qq::handler::DefaultHandler;
use rs_qq::version::{get_version, Protocol};
use rs_qq::{Client, LoginDeviceLocked, LoginNeedCaptcha, LoginSuccess};
use rs_qq::{LoginResponse, LoginUnknownStatus};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("rs_qq", Level::DEBUG)
                .with_target("password_login", Level::DEBUG),
        )
        .init();

    // load uin and password from env
    let uin: i64 = std::env::var("UIN")
        .expect("failed to read UIN from env")
        .parse()
        .expect("failed to parse UIN");
    let password = std::env::var("PASSWORD").expect("failed to read PASSWORD from env");

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
        get_version(Protocol::IPad),
        DefaultHandler,
    ));
    let stream = TcpStream::connect(client.get_address())
        .await
        .expect("failed to connect");
    let c = client.clone();
    let handle = tokio::spawn(async move { c.start(stream).await });
    tokio::task::yield_now().await; // 等一下，确保连上了
    let mut resp = client
        .password_login(uin, &password)
        .await
        .expect("failed to login with password");
    loop {
        match resp {
            LoginResponse::Success(LoginSuccess {
                ref account_info, ..
            }) => {
                tracing::info!("login success: {:?}", account_info);
                break;
            }
            LoginResponse::DeviceLocked(LoginDeviceLocked {
                ref sms_phone,
                ref verify_url,
                ref message,
                ..
            }) => {
                tracing::info!("device locked: {:?}", message);
                tracing::info!("sms_phone: {:?}", sms_phone);
                tracing::info!("verify_url: {:?}", verify_url);
                tracing::info!("手机打开url，处理完成后重启程序");
                std::process::exit(0);
                //也可以走短信验证
                // resp = client.request_sms().await.expect("failed to request sms");
            }
            LoginResponse::NeedCaptcha(LoginNeedCaptcha {
                ref verify_url,
                // 图片应该没了
                image_captcha: ref _image_captcha,
                ..
            }) => {
                tracing::info!("滑块URL: {:?}", verify_url);
                tracing::info!("请输入ticket:");
                let mut reader = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
                let ticket = reader
                    .next()
                    .await
                    .transpose()
                    .expect("failed to read ticket")
                    .expect("failed to read ticket");
                resp = client
                    .submit_ticket(&ticket)
                    .await
                    .expect("failed to submit ticket");
            }
            LoginResponse::DeviceLockLogin { .. } => {
                resp = client
                    .device_lock_login()
                    .await
                    .expect("failed to login with device lock");
            }
            LoginResponse::AccountFrozen => {
                panic!("account frozen");
            }
            LoginResponse::TooManySMSRequest => {
                panic!("too many sms request");
            }
            LoginResponse::UnknownStatus(LoginUnknownStatus {
                ref status,
                ref tlv_map,
            }) => {
                panic!("unknown login status: {:?}, {:?}", status, tlv_map);
            }
        }
    }
    tracing::info!("{:?}", resp);
    after_login(&client).await;
    {
        client
            .reload_friends()
            .await
            .expect("failed to reload friend list");
        tracing::info!("{:?}", client.friends.read().await);
        client
            .reload_groups()
            .await
            .expect("failed to reload group list");
        let group_list = client.groups.read().await;
        tracing::info!("{:?}", group_list);
    }
    let d = client.get_allowed_clients().await;
    tracing::info!("{:?}", d);

    // client.delete_essence_message(1095020555, 8114, 2107692422).await
    // let mem_info = client.get_group_member_info(335783090, 875543543).await;
    // println!("{:?}", mem_info);
    // let mem_list = client.get_group_member_list(335783090).await;
    // println!("{:?}", mem_list);
    handle.await.unwrap();
    Ok(())
}
