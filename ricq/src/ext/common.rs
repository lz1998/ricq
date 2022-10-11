use std::sync::atomic::Ordering;

use crate::Client;

/// 登录后必须执行的操作
pub async fn after_login<H: crate::handler::Handler + Send>(client: &Client<H>) {
    if let Err(err) = client.register_client().await {
        tracing::error!("failed to register client: {}", err)
    }
    let (_, refresh_status) = tokio::join!(start_heartbeat(client), client.refresh_status());
    if let Err(err) = refresh_status {
        tracing::error!("failed to refresh status: {}", err)
    }
}

/// 如果当前启动心跳，开始心跳（blocking）
pub async fn start_heartbeat<H: crate::handler::Handler + Send>(client: &Client<H>) {
    if !client.heartbeat_enabled.load(Ordering::Relaxed) {
        client.do_heartbeat().await;
    }
}
