use anyhow::Result;
use futures::StreamExt;
use rs_qq::client::device::DeviceInfo;
use rs_qq::client::handler::DefaultHandler;
use rs_qq::client::income::decoder::wtlogin::LoginResponse;
use rs_qq::client::net::ClientNet;
use rs_qq::client::{Client, Password};
use std::path::Path;
use std::sync::Arc;
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() -> Result<()> {
    // load uin and password from env
    let uin: i64 = std::env::var("UIN").expect("failed to read UIN from env").parse().expect("failed to parse UIN");
    let password = std::env::var("PASSWORD").expect("failed to read PASSWORD from env");

    let device_info = match Path::new("device.json").exists() {
        true => {
            DeviceInfo::from_json(&tokio::fs::read_to_string("device.json").await.expect("failed to read device.json")).expect("failed to parse device info")
        }
        false => DeviceInfo::random(),
    };
    tokio::fs::write("device.json", device_info.to_json())
        .await
        .expect("failed to write device info to file");

    let (cli, receiver) = Client::new(
        uin,
        Password::from_str(&password),
        device_info,
        DefaultHandler,
    )
        .await;
    let client = Arc::new(cli);
    let client_net = ClientNet::new(client.clone(), receiver);
    let stream = client_net.connect_tcp().await;
    let net = tokio::spawn(client_net.net_loop(stream));
    tokio::spawn(async move {
        let mut resp = client.password_login().await.expect("failed to login with password");
        loop {
            match resp {
                LoginResponse::Success => {
                    break;
                }
                LoginResponse::SMSOrVerifyNeededError {
                    ref verify_url,
                    ref sms_phone,
                    ref error_message,
                } => {
                    println!("{}", error_message);
                    println!("{}", sms_phone);
                    println!("手机打开url，处理完成后重启程序");
                    println!("{}", verify_url);
                    std::process::exit(0);

                    // 也可以走短信验证
                    // resp = client.request_sms().await.expect("failed to request sms");
                }
                LoginResponse::SliderNeededError { ref verify_url } => {
                    println!("滑块URL: {}", verify_url);
                    println!("请输入ticket:");
                    let mut reader = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
                    let ticket = reader.next().await.transpose().expect("failed to read ticket").expect("failed to read ticket");
                    resp = client.submit_ticket(&ticket).await.expect("failed to submit ticket");
                }
                LoginResponse::SMSNeededError {
                    ref sms_phone,
                    ref error_message,
                } => {
                    println!("{}", sms_phone);
                    println!("{}", error_message);
                    println!("请输入短信验证码:");
                    let mut reader = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
                    let sms_code = reader.next().await.transpose().expect("failed to read sms_code").expect("failed to read sms_code");
                    resp = client.submit_sms_code(&sms_code).await.expect("failed to submit sms_code");
                }
                LoginResponse::NeedDeviceLockLogin => {
                    resp = client.device_lock_login().await.expect("failed to login with device lock");
                }
                LoginResponse::NeedCaptcha { .. } => {}
                LoginResponse::UnsafeDeviceError { ref verify_url } => {
                    println!("手机打开url，处理完成后重启程序");
                    println!("{}", verify_url);
                    std::process::exit(0);
                }
                LoginResponse::TooManySMSRequestError => {}
                LoginResponse::OtherLoginError { .. } => {}
                LoginResponse::UnknownLoginError { .. } => {}
            }
        }
        println!("{:?}", resp);
        client.register_client().await.expect("failed to register client");
        client.refresh_status().await.expect("failed to refresh status");
        let c = client.clone();
        tokio::spawn(async move {
            c.do_heartbeat().await;
        });
        {
            client.reload_friend_list().await.expect("failed to reload friend list");
            println!("{:?}", client.friend_list.read().await);
            client.reload_group_list().await.expect("failed to reload group list");
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
    net.await.expect("network error1").expect("network error2");

    Ok(())
}
