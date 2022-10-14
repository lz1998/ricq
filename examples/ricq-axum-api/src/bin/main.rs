#![feature(async_closure)]

use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    routing::{get, get_service, post},
    Extension, Router,
};
use dashmap::DashMap;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ricq::client::event::{FriendMessageEvent, GroupMessageEvent};
use ricq::client::{DefaultConnector, NetworkStatus};
use ricq::ext::common::after_login;
use ricq::ext::reconnect::{auto_reconnect, Credential};
use ricq::handler::QEvent;
use ricq::Client;
use ricq_axum_api::handler::{bot, password, qrcode};
use ricq_axum_api::processor::Processor;
use ricq_axum_api::u8_protocol::U8Protocol;
use ricq_axum_api::{ClientInfo, RicqAxumApi};

// 默认处理器
struct ClientProcessor(DashMap<(i64, u8), Arc<Client>>);

#[async_trait::async_trait]
impl Processor for ClientProcessor {
    async fn on_login_success(
        &self,
        client: Arc<Client>,
        mut event_receiver: broadcast::Receiver<QEvent>,
        credential: Credential,
        network_join_handle: JoinHandle<()>,
    ) {
        let uin = client.uin().await;
        let protocol = client.version().await.protocol.to_u8();
        self.0.insert((uin, protocol), client.clone());
        after_login(&client).await;

        tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                match event {
                    QEvent::GroupMessage(e) => {
                        let GroupMessageEvent {
                            inner: message,
                            client,
                        } = e;
                        tracing::info!(
                            "GROUP_MSG, code: {}, content: {}",
                            message.group_code,
                            message.elements.to_string()
                        );
                        client
                            .send_group_message(message.group_code, message.elements)
                            .await
                            .ok();
                    }
                    QEvent::FriendMessage(e) => {
                        let FriendMessageEvent {
                            inner: message,
                            client,
                        } = e;
                        tracing::info!(
                            "FRIEND_MSG, code: {}, content: {}",
                            message.from_uin,
                            message.elements.to_string()
                        );
                        client
                            .send_friend_message(message.from_uin, message.elements)
                            .await
                            .ok();
                    }
                    other => {
                        tracing::info!("{:?}", other)
                    }
                }
            }
        });

        // DONT BLOCK
        tokio::spawn(async move {
            network_join_handle.await.ok();
            auto_reconnect(
                client,
                credential,
                Duration::from_secs(10),
                10,
                DefaultConnector,
            )
            .await;
        });
    }

    async fn list_client(&self) -> Vec<ClientInfo> {
        let mut infos = Vec::new();
        for cli in self.0.iter() {
            let (uin, protocol) = cli.key();
            let client = cli.value();
            infos.push(ClientInfo {
                uin: *uin,
                nick: client.account_info.read().await.nickname.clone(),
                status: client.get_status(),
                protocol: *protocol,
            });
        }
        infos
    }

    async fn delete_client(&self, uin: i64, protocol: u8) {
        if let Some((_, client)) = self.0.remove(&(uin, protocol)) {
            client.stop(NetworkStatus::Stop);
        }
    }
}

#[tokio::main]
async fn main() {
    // 初始化日志
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
                .with_target("main", Level::DEBUG)
                .with_target("ricq", Level::DEBUG)
                .with_target("ricq_axum_api", Level::DEBUG),
        )
        .init();

    let processor = ClientProcessor(Default::default());
    let ricq_axum_api = Arc::new(RicqAxumApi::new(processor));

    let app = Router::new()
        .route("/ping", get(async move || "pong"))
        .nest(
            "/login",
            Router::new()
                .nest(
                    "/qrcode",
                    Router::new()
                        .route("/create", post(qrcode::create::<ClientProcessor>))
                        .route("/list", get(qrcode::list::<ClientProcessor>))
                        .route("/delete", post(qrcode::delete::<ClientProcessor>))
                        .route("/query", post(qrcode::query::<ClientProcessor>)),
                )
                .nest(
                    "/password",
                    Router::new()
                        .route("/create", post(password::login::<ClientProcessor>))
                        .route(
                            "/request_sms",
                            post(password::request_sms::<ClientProcessor>),
                        )
                        .route("/submit_sms", post(password::submit_sms::<ClientProcessor>))
                        .route(
                            "/submit_ticket",
                            post(password::submit_ticket::<ClientProcessor>),
                        )
                        .route("/list", get(password::list::<ClientProcessor>))
                        .route("/delete", post(password::delete::<ClientProcessor>)),
                ),
        )
        .nest(
            "/bot",
            Router::new()
                .route("/list", get(bot::list::<ClientProcessor>))
                .route("/delete", post(bot::delete::<ClientProcessor>)),
        )
        .fallback(get_service(ServeDir::new("static")).handle_error(handle_error))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(ricq_axum_api))
                .into_inner(),
        );
    let addr = SocketAddr::from_str("0.0.0.0:9000").expect("failed to parse bind_addr");
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_: std::io::Error) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Something went wrong...")
}
