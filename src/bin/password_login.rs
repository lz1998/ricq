use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

use rs_qq::client::handler::DefaultHandler;
use rs_qq::client::Client;
use rs_qq::engine::command::wtlogin::LoginResponse;
use rs_qq::engine::protocol::device::Device;
use rs_qq::engine::protocol::version::{get_version, Protocol};

#[tokio::main]
async fn main() -> Result<()> {
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

    let config = rs_qq::Config::new(device, get_version(Protocol::IPad));
    let cli = Client::new_with_config(config, DefaultHandler).await;
    let client = Arc::new(cli);
    let net = client.run().await;
    tokio::spawn(async move {
        let mut resp = client
            .password_login(uin, &password)
            .await
            .expect("failed to login with password");
        loop {
            match resp {
                LoginResponse::Success {
                    ref account_info, ..
                } => {
                    println!("login success: {:?}", account_info);
                    break;
                }
                LoginResponse::DeviceLocked {
                    ref sms_phone,
                    ref verify_url,
                    ref message,
                    ..
                } => {
                    println!("device locked: {:?}", message);
                    println!("sms_phone: {:?}", sms_phone);
                    println!("verify_url: {:?}", verify_url);
                    println!("手机打开url，处理完成后重启程序");
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
                    println!("滑块URL: {:?}", verify_url);
                    println!("请输入ticket:");
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
        println!("{:?}", resp);
        client
            .register_client()
            .await
            .expect("failed to register client");
        client
            .refresh_status()
            .await
            .expect("failed to refresh status");
        let c = client.clone();
        tokio::spawn(async move {
            c.do_heartbeat().await;
        });
        {
            client
                .reload_friend_list()
                .await
                .expect("failed to reload friend list");
            println!("{:?}", client.friend_list.read().await);
            client
                .reload_group_list()
                .await
                .expect("failed to reload group list");
            let _group_list = client.group_list.read().await;
        }
        let r = client.refresh_status().await;
        println!("{:?}", r);
        let d = client.get_allowed_clients().await;
        println!("{:?}", d);

        // client.send_group_message(335783090, vec![
        //     Msg::At { target: 875543533, display: "@lz1998".to_string() },
        //     Msg::Text { content: "xxx".to_string() },
        // ]).await;
        // let mem_info = client.get_group_member_info(335783090, 875543543).await;
        // println!("{:?}", mem_info);
        // let mem_list = client.get_group_member_list(335783090).await;
        // println!("{:?}", mem_list);
    });
    net.await.expect("network error1");

    Ok(())
}
