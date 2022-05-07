use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::Client;

/// 登录后必须执行的操作
pub async fn after_login(client: &Arc<Client>) {
    if let Err(err) = client.register_client().await {
        tracing::error!("failed to register client: {}", err)
    }
    start_heartbeat(client.clone()).await;
    if let Err(err) = client.refresh_status().await {
        tracing::error!("failed to refresh status: {}", err)
    }
}

/// 如果当前启动心跳，spawn 开始心跳
pub async fn start_heartbeat(client: Arc<Client>) {
    if !client.heartbeat_enabled.load(Ordering::Relaxed) {
        tokio::spawn(async move {
            client.do_heartbeat().await;
        });
    }
}
