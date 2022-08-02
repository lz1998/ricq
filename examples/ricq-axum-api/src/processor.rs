use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::u8_protocol::U8Protocol;
use crate::ClientInfo;
use ricq::client::{DefaultConnector, NetworkStatus};
use ricq::ext::reconnect::auto_reconnect;
use ricq::{ext::reconnect::Credential, handler::QEvent, Client};

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

#[async_trait::async_trait]
impl Processor for DashMap<(i64, u8), Arc<Client>> {
    async fn on_login_success(
        &self,
        client: Arc<Client>,
        _event_receiver: broadcast::Receiver<QEvent>,
        credential: Credential,
        network_join_handle: JoinHandle<()>,
    ) {
        let uin = client.uin().await;
        let protocol = client.version().await.protocol.to_u8();
        self.insert((uin, protocol), client.clone());
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
        for cli in self.iter() {
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
        if let Some((_, client)) = self.remove(&(uin, protocol)) {
            client.stop(NetworkStatus::Stop);
        }
    }
}
