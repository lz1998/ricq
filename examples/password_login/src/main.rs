use std::sync::Arc;

use futures_util::StreamExt;
use rand::prelude::StdRng;
use rand::SeedableRng;
use tokio_util::codec::{FramedRead, LinesCodec};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ricq::client::{Connector as _, DefaultConnector};
use ricq::ext::common::after_login;
use ricq::handler::DefaultHandler;
use ricq::structs::ExtOnlineStatus;
use ricq::{Client, Device, Protocol};
use ricq::{LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess, LoginUnknownStatus};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
                    time::UtcOffset::__from_hms_unchecked(8, 0, 0),
                    time::macros::format_description!(
                        "[year repr:last_two]-[month]-[day] [hour]:[minute]:[second]"
                    ),
                )),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", Level::DEBUG)
                .with_target("password_login", Level::DEBUG),
        )
        .init();

    // load uin and password from env
    let uin: i64 = std::env::var("UIN")
        .expect("failed to read UIN from env")
        .parse()
        .expect("failed to parse UIN");
    let password = std::env::var("PASSWORD").expect("failed to read PASSWORD from env");

    let mut seed = StdRng::seed_from_u64(uin as u64);
    let device = Device::random_with_rng(&mut seed);

    let client = Arc::new(Client::new(device, Protocol::IPad.into(), DefaultHandler));
    let handle = tokio::spawn({
        let client = client.clone();
        // 连接所有服务器，哪个最快用哪个，可以使用 TcpStream::connect 代替
        let stream = DefaultConnector.connect(&client).await.unwrap();
        async move { client.start(stream).await }
    });

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
                ref message,
            }) => {
                panic!(
                    "unknown login status: {:?}, {:?}, {:?}",
                    message, status, tlv_map
                );
            }
        }
    }
    tracing::info!("{:?}", resp);
    after_login(&client).await;
    {
        tracing::info!("{:?}", client.get_friend_list().await);
        tracing::info!("{:?}", client.get_group_list().await);
    }
    let d = client.get_allowed_clients().await;
    tracing::info!("{:?}", d);

    // 等一下，收到 ConfigPushSvc.PushReq 才可以发
    // use ricq::msg::MessageChain;
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // let img_bytes = tokio::fs::read("test.png").await.unwrap();
    // let group_image = client
    //     .upload_group_image(982166018, img_bytes)
    //     .await
    //     .unwrap();
    // let mut chain = MessageChain::default();
    // chain.push(group_image);
    // client.send_group_message(982166018, chain).await.ok();
    let aaa = client
        .update_online_status(ExtOnlineStatus::StudyOnline)
        .await;
    println!("{:?}", aaa);

    // client.delete_essence_message(1095020555, 8114, 2107692422).await
    // let mem_info = client.get_group_member_info(335783090, 875543543).await;
    // println!("{:?}", mem_info);
    // let mem_list = client.get_group_member_list(335783090).await;
    // println!("{:?}", mem_list);
    handle.await.unwrap();
}
