use std::sync::Arc;

use bytes::Buf;
use rq_engine::{RQError, RQResult};

use crate::{handler::QEvent, Client};

impl Client {
    /// token 登录
    pub async fn token_login(self: &Arc<Self>, mut token: impl Buf) -> RQResult<()> {
        self.load_token(&mut token).await;
        let req = self.engine.read().await.build_request_change_sig_packet();
        self.send_and_wait(req).await?;
        self.register_client().await?;
        let r = tokio::join! {
            self.wait_packet("StatSvc.ReqMSFOffline", 1),
            self.wait_packet("MessageSvc.PushForceOffline", 1)
        };
        if let (Err(RQError::Timeout), Err(RQError::Timeout)) = r {
            self.handler
                .handle(QEvent::LoginEvent(self.uin().await))
                .await;
            Ok(())
        } else {
            Err(RQError::TokenLoginFailed)
        }
    }
}
