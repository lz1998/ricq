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
    let uin = 0;
    let password = "";

    let device_info = match Path::new("device.json").exists() {
        true => {
            DeviceInfo::from_json(&tokio::fs::read_to_string("device.json").await.unwrap()).unwrap()
        }
        false => DeviceInfo::random(),
    };
    tokio::fs::write("device.json", device_info.to_json()).await.unwrap();

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
        let mut resp = client.password_login().await.unwrap();
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
                    // resp = client.request_sms().await.unwrap();
                }
                LoginResponse::SliderNeededError { ref verify_url } => {
                    println!("滑块URL: {}", verify_url);
                    println!("请输入ticket:");
                    let mut reader = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
                    let ticket = reader.next().await.transpose().unwrap().unwrap();
                    resp = client.submit_ticket(&ticket).await.unwrap();
                }
                LoginResponse::SMSNeededError {
                    ref sms_phone,
                    ref error_message,
                } => {
                    println!("{}", sms_phone);
                    println!("{}", error_message);
                    println!("请输入短信验证码:");
                    let mut reader = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
                    let sms_code = reader.next().await.transpose().unwrap().unwrap();
                    resp = client.submit_sms_code(&sms_code).await.unwrap();
                }
                LoginResponse::NeedDeviceLockLogin => {
                    resp = client.device_lock_login().await.unwrap();
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
        client.register_client().await;
        client.refresh_status().await;
        let c = client.clone();
        tokio::spawn(async move {
            c.do_heartbeat().await;
        });
        {
            client.reload_friend_list().await;
            println!("{:?}", client.friend_list.read().await);
            client.reload_group_list().await;
            let group_list = client.group_list.read().await;
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
    net.await;

    Ok(())
}
