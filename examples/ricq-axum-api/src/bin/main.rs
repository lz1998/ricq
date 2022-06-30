#![feature(async_closure)]

use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use axum::{
    routing::{get, get_service, post},
    Extension, Router,
};
use dashmap::DashMap;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use ricq::Client;
use ricq_axum_api::handler::{bot, password, qrcode};
use ricq_axum_api::RicqAxumApi;

type ClientProcessor = DashMap<(i64, u8), Arc<Client>>;

#[tokio::main]
async fn main() {
    // 默认处理器，登录后什么也不做，仅作为容器
    let processor = ClientProcessor::default();
    let ricq_axum_api = Arc::new(RicqAxumApi::new(processor));

    let app = Router::new()
        .route("/ping", get(async move || "pong"))
        .nest(
            "/login",
            Router::new()
                .nest(
                    "/qrcode",
                    Router::new()
                        .route("/create", post(qrcode::create))
                        .route("/list", get(qrcode::list))
                        .route("/delete", post(qrcode::delete))
                        .route("/query", post(qrcode::query)),
                )
                .nest(
                    "/password",
                    Router::new()
                        .route("/create", post(password::login))
                        .route("/request_sms", post(password::request_sms))
                        .route("/submit_sms", post(password::submit_sms))
                        .route("/submit_ticket", post(password::submit_ticket))
                        .route("/list", get(password::list))
                        .route("/delete", post(password::delete)),
                ),
        )
        .nest(
            "/bot",
            Router::new()
                .route("/list", get(bot::list))
                .route("/delete", post(bot::delete)),
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
