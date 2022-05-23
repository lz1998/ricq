use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use rand::prelude::StdRng;
use rand::SeedableRng;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, LinesCodec};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ricq::device::Device;
use ricq::ext::common::after_login;
use ricq::handler::DefaultHandler;
use ricq::msg::MessageChain;
use ricq::structs::ExtOnlineStatus;
use ricq::version::{get_version, Protocol};
use ricq::{Client, LoginDeviceLocked, LoginNeedCaptcha, LoginSuccess};
use ricq::{LoginResponse, LoginUnknownStatus};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
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

    let device = Device::random_with_rng(&mut StdRng::seed_from_u64(uin as u64));

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
        client
            .reload_friends()
            .await
            .expect("failed to reload friend list");
        tracing::info!("{:?}", client.friends.read().await);
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

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    let video_data = tokio::fs::read("test.mp4").await.unwrap();
    let thumb_data = tokio::fs::read("test.png").await.unwrap();
    let video = client
        .upload_group_short_video(982166018, video_data, thumb_data)
        .await
        .unwrap();
    let mut chain = MessageChain::default();
    chain.push(video);
    client.send_group_message(982166018, chain).await;

    // client.delete_essence_message(1095020555, 8114, 2107692422).await
    // let mem_info = client.get_group_member_info(335783090, 875543543).await;
    // println!("{:?}", mem_info);
    // let mem_list = client.get_group_member_list(335783090).await;
    // println!("{:?}", mem_list);
    handle.await.unwrap();
    Ok(())
}
