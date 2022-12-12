use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

use ricq_core::command::wtlogin::LoginResponse;

use crate::client::net::Connector;
use crate::client::NetworkStatus;
use crate::ext::common::after_login;
use crate::{Client, RQError, RQResult};

/// 自动重连，在掉线后使用，会阻塞到重连结束
pub async fn auto_reconnect<T: AsyncRead + AsyncWrite + 'static + Send>(
    client: Arc<Client>,
    credential: Credential,
    interval: Duration,
    max: usize,
    connector: impl Connector<T>,
) {
    let mut count = 0;
    loop {
        // 如果不是网络原因掉线，不重连（服务端强制下线/被踢下线/用户手动停止）
        if client.get_status() != (NetworkStatus::NetworkOffline as u8) {
            tracing::warn!(
                "client status: {}, auto_reconnect break",
                client.get_status()
            );
            break;
        }
        client.stop(NetworkStatus::NetworkOffline);
        tracing::error!("client will reconnect after {} seconds", interval.as_secs());
        tokio::time::sleep(interval).await;
        let stream = if let Ok(stream) = connector.connect(&client).await {
            count = 0;
            stream
        } else {
            count += 1;
            if count > max {
                tracing::error!("reconnect_count: {}, break!", count);
                break;
            }
            continue;
        };
        let c = client.clone();
        let handle = tokio::spawn(async move { c.start(stream).await });
        tokio::task::yield_now().await; // 等一下，确保连上了
        if let Err(err) = fast_login(&client, &credential).await {
            // token 可能过期了
            tracing::error!("failed to fast_login: {}", err);
            client.stop(NetworkStatus::NetworkOffline);
            count += 1;
            if count > max {
                tracing::error!("reconnect_count: {}, break!", count);
                break;
            }
            continue;
        }
        tracing::info!("succeed to reconnect");
        after_login(&client).await;
        handle.await.ok();
    }
}

pub struct Password {
    pub uin: i64,
    pub password: String,
}

pub enum Credential {
    Token(ricq_core::Token),
    Password(Password),
}

/// 用于重连
#[async_trait]
pub trait FastLogin {
    async fn fast_login(&self, client: &Arc<Client>) -> RQResult<()>;
}

#[async_trait]
impl FastLogin for ricq_core::Token {
    async fn fast_login(&self, client: &Arc<Client>) -> RQResult<()> {
        match client.token_login(self.clone()).await? {
            LoginResponse::Success(_) => Ok(()),
            other => Err(RQError::Other(format!("failed to token_login, {other:?}"))),
        }
    }
}

#[async_trait]
impl FastLogin for Password {
    async fn fast_login(&self, client: &Arc<Client>) -> RQResult<()> {
        let resp = client.password_login(self.uin, &self.password).await?;
        match resp {
            LoginResponse::Success { .. } => return Ok(()),
            LoginResponse::DeviceLockLogin { .. } => {
                return if let LoginResponse::Success { .. } = client.device_lock_login().await? {
                    Ok(())
                } else {
                    Err(RQError::Other("failed to login".into()))
                };
            }
            other => Err(RQError::Other(format!("failed to login, {other:?}"))),
        }
    }
}

/// 如果你非常确定登录过程中不会遇到验证码，可以用 fast_login
pub async fn fast_login(client: &Arc<Client>, credential: &Credential) -> RQResult<()> {
    match credential {
        Credential::Token(token) => token.fast_login(client).await,
        Credential::Password(password) => password.fast_login(client).await,
    }
}
