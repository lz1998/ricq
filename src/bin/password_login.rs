use std::path::Path;
use std::sync::Arc;
use rs_qq::client::device::DeviceInfo;
use anyhow::Result;
use futures::StreamExt;
use tokio::io::AsyncReadExt;
use tokio_util::codec::{FramedRead, LinesCodec};
use rs_qq::client::{Client, Password};
use rs_qq::client::income::decoder::wtlogin::LoginResponse;
use rs_qq::client::net::ClientNet;

#[tokio::main]
async fn main() -> Result<()> {
    let uin = 0;
    let password = "";

    let mut device_info = match Path::new("device.json").exists() {
        true => {
            DeviceInfo::from_json(&tokio::fs::read_to_string("device.json").await.unwrap()).unwrap()
        }
        false => {
            DeviceInfo::random()
        }
    };
    tokio::fs::write("device.json", device_info.to_json()).await;

    let (cli, receiver) = Client::new(
        uin,
        Password::from_str(password),
        device_info,
    ).await;
    let client = Arc::new(cli);
    let client_net = ClientNet::new(client.clone(), receiver);
    let stream = client_net.connect_tcp().await;
    let net = tokio::spawn(client_net.net_loop(stream));
    tokio::spawn(async move {
        let mut resp = client.password_login().await.unwrap();
        loop {
            match resp {
                LoginResponse::Success => { break; }
                LoginResponse::SMSOrVerifyNeededError { ref verify_url, ref sms_phone, ref error_message } => {
                    println!("手机打开url，处理完成后重启程序");
                    println!("{}", verify_url);
                    std::process::exit(0);

                    // 也可以走短信验证
                    // resp = client.request_sms().await.unwrap();
                }
                LoginResponse::SliderNeededError { .. } => {
                    println!("请输入ticket:");
                    let mut reader = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
                    let ticket = reader.next().await.transpose().unwrap().unwrap();
                    resp = client.submit_ticket(&ticket).await.unwrap();
                }
                LoginResponse::SMSNeededError { .. } => {
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
        let rsp = client.group_list(&[]).await;
        println!("{:?}", rsp);
        let rsp = client.friend_group_list(0, 150, 0, 0).await;
        println!("{:?}", rsp);
    });
    net.await;

    Ok(())
}