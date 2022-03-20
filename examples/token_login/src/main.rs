use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use tokio::net::TcpStream;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rs_qq::client::Token;
use rs_qq::device::Device;
use rs_qq::ext::common::after_login;
use rs_qq::handler::DefaultHandler;
use rs_qq::version::{get_version, Protocol};
use rs_qq::Client;

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
                .with_target("rs_qq", Level::DEBUG)
                .with_target("token_login", Level::DEBUG),
        )
        .init();

    let token = tokio::fs::read_to_string("session.token")
        .await
        .expect("failed to read token");
    let token: Token = serde_json::from_str(&token).expect("failed to parse token");

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
    let resp = client
        .token_login(token)
        .await
        .expect("failed to login with token");

    tracing::info!("{:?}", resp);
    after_login(&client).await;
    {
        client
            .reload_friends()
            .await
            .expect("failed to reload friend list");
        tracing::info!("{:?}", client.friends.read().await);
        client
            .reload_groups(50)
            .await
            .expect("failed to reload group list");
        let group_list = client.groups.read().await;
        tracing::info!("{:?}", group_list);
    }
    let d = client.get_allowed_clients().await;
    tracing::info!("{:?}", d);

    handle.await.unwrap();
    Ok(())
}
