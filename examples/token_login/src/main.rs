use std::sync::Arc;

use rand::prelude::StdRng;
use rand::SeedableRng;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ricq::client::{Connector as _, DefaultConnector, Token};
use ricq::ext::common::after_login;
use ricq::handler::DefaultHandler;
use ricq::{Client, Device, Protocol};

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
                .with_target("token_login", Level::DEBUG),
        )
        .init();

    let token = tokio::fs::read_to_string("session.token")
        .await
        .expect("failed to read token");
    let token: Token = serde_json::from_str(&token).expect("failed to parse token");
    let device = Device::random_with_rng(&mut StdRng::seed_from_u64(token.uin as u64));

    let client = Arc::new(Client::new(device, Protocol::IPad.into(), DefaultHandler));

    let handle = tokio::spawn({
        let client = client.clone();
        let stream = DefaultConnector.connect(&client).await.unwrap();
        async move { client.start(stream).await }
    });
    // 直接使用 TcpStream，目前不推荐
    // let stream = TcpStream::connect(client.get_address())
    //     .await
    //     .expect("failed to connect");
    // let c = client.clone();
    // let handle = tokio::spawn(async move { c.start(stream).await });

    tokio::task::yield_now().await; // 等一下，确保连上了
    let resp = client
        .token_login(token)
        .await
        .expect("failed to login with token");

    tracing::info!("{:?}", resp);
    after_login(&client).await;
    {
        tracing::info!("{:?}", client.get_friend_list().await);
        tracing::info!("{:?}", client.get_group_list().await);
    }
    let d = client.get_allowed_clients().await;
    tracing::info!("{:?}", d);

    handle.await.unwrap();
}
