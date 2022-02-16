use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use tokio::net::TcpStream;

use rq_engine::command::wtlogin::LoginResponse;

use crate::ext::common::after_login;
use crate::{Client, RQError, RQResult};

#[async_trait]
pub trait Connector {
    async fn connect(&self, client: &Arc<Client>) -> std::io::Result<TcpStream> {
        TcpStream::connect(client.get_address()).await
    }
}

pub struct DefaultConnector;

impl Connector for DefaultConnector {}

/// 自动重连，在掉线后使用，会阻塞到重连结束
pub async fn auto_reconnect<C: Connector + Sync>(
    client: Arc<Client>,
    credential: Credential,
    interval: Duration,
    max: usize,
    connector: C,
) {
    let mut count = 0;
    loop {
        client.stop();
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
            tracing::error!("failed to fast_login: {}, break!", err);
            break;
        }
        tracing::info!("succeed to reconnect");
        after_login(&client).await;
        handle.await.ok();
    }
}

pub struct Token(pub Bytes);
pub struct Password {
    pub uin: i64,
    pub password: String,
}

pub enum Credential {
    Token(Token),
    Password(Password),
    Both(Token, Password),
}

/// 用于重连
#[async_trait]
pub trait FastLogin {
    async fn fast_login(&self, client: &Arc<Client>) -> RQResult<()>;
}

#[async_trait]
impl FastLogin for Token {
    async fn fast_login(&self, client: &Arc<Client>) -> RQResult<()> {
        client.token_login(self.0.clone()).await
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
            _ => return Err(RQError::Other("failed to login".into())),
        }
    }
}

/// 如果你非常确定登录过程中不会遇到验证码，可以用 fast_login
pub async fn fast_login(client: &Arc<Client>, credential: &Credential) -> RQResult<()> {
    return match credential {
        Credential::Token(token) => token.fast_login(client).await,
        Credential::Password(password) => password.fast_login(client).await,
        Credential::Both(token, password) => match token.fast_login(client).await {
            Ok(_) => Ok(()),
            Err(_) => password.fast_login(client).await,
        },
    };
}
