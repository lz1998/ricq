# ricq-axum-api

1. 下载 UI 文件，放在工作目录 static 文件夹

```bash
 wget https://github.com/lz1998/ricq-react-ui/releases/latest/download/static.zip && unzip static.zip && rm static.zip
```

2. 实现 Processor trait（仅包含登录后处理逻辑）

> 参考 [processor.rs](https://github.com/lz1998/ricq/blob/master/examples/ricq-axum-api/src/processor.rs#L28)

```rust
#[async_trait::async_trait]
pub trait Processor {
    async fn on_login_success(
        &self,
        client: Arc<Client>,
        event_receiver: broadcast::Receiver<QEvent>,
        credential: Credential,
        network_join_handle: JoinHandle<()>,
    );
    async fn list_client(&self) -> Vec<ClientInfo>;
    async fn delete_client(&self, uin: i64, protocol: u8);
} 
```

3. 创建 RicqAxumApi，并启动 axum 服务器

```rust
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
```

4. 访问 `http://localhost:9000`
