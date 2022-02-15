use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, LinesCodec};

use rs_qq::device::Device;
use rs_qq::ext::common::after_login;
use rs_qq::handler::DefaultHandler;
use rs_qq::version::{get_version, Protocol};
use rs_qq::Client;
use rs_qq::LoginResponse;

#[tokio::main]
async fn main() -> Result<()> {
    let env = tracing_subscriber::EnvFilter::from("rs_qq=debug,sled=warn,info");
    tracing_subscriber::fmt()
        .with_env_filter(env)
        .without_time()
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
        false => Device::random(),
    };
    tokio::fs::write("device.json", serde_json::to_string(&device).unwrap())
        .await
        .expect("failed to write device info to file");

    let client = Arc::new(Client::new(
        device,
        get_version(Protocol::IPad),
        DefaultHandler,
    ));
    let stream = TcpStream::connect(client.get_address())
        .await
        .expect("failed to connect");
    let c = client.clone();
    let handle = tokio::spawn(async move { c.start_with_stream(stream).await });
    tokio::task::yield_now().await; // 等一下，确保连上了
    let mut resp = client
        .password_login(uin, &password)
        .await
        .expect("failed to login with password");
    loop {
        match resp {
            LoginResponse::Success {
                ref account_info, ..
            } => {
                tracing::info!(target = "rs_qq", "login success: {:?}", account_info);
                break;
            }
            LoginResponse::DeviceLocked {
                ref sms_phone,
                ref verify_url,
                ref message,
                ..
            } => {
                tracing::info!(target = "rs_qq", "device locked: {:?}", message);
                tracing::info!(target = "rs_qq", "sms_phone: {:?}", sms_phone);
                tracing::info!(target = "rs_qq", "verify_url: {:?}", verify_url);
                tracing::info!(target = "rs_qq", "手机打开url，处理完成后重启程序");
                std::process::exit(0);
                //也可以走短信验证
                // resp = client.request_sms().await.expect("failed to request sms");
            }
            LoginResponse::NeedCaptcha {
                ref verify_url,
                // 图片应该没了
                image_captcha: ref _image_captcha,
                ..
            } => {
                tracing::info!(target = "rs_qq", "滑块URL: {:?}", verify_url);
                tracing::info!(target = "rs_qq", "请输入ticket:");
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
            LoginResponse::UnknownLoginStatus {
                ref status,
                ref tlv_map,
            } => {
                panic!("unknown login status: {:?}, {:?}", status, tlv_map);
            }
        }
    }
    tracing::info!(target = "rs_qq", "{:?}", resp);
    after_login(&client).await;
    {
        client
            .reload_friends()
            .await
            .expect("failed to reload friend list");
        tracing::info!(target = "rs_qq", "{:?}", client.friends.read().await);
        client
            .reload_groups()
            .await
            .expect("failed to reload group list");
        let group_list = client.groups.read().await;
        tracing::info!(target = "rs_qq", "{:?}", group_list);
    }
    let d = client.get_allowed_clients().await;
    tracing::info!(target = "rs_qq", "{:?}", d);

    // client.delete_essence_message(1095020555, 8114, 2107692422).await
    // let mem_info = client.get_group_member_info(335783090, 875543543).await;
    // println!("{:?}", mem_info);
    // let mem_list = client.get_group_member_list(335783090).await;
    // println!("{:?}", mem_list);
    handle.await.unwrap();
    Ok(())
}
