use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use ricq::{ext::reconnect::Credential, handler::QEvent, Client};

use crate::ClientInfo;

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
