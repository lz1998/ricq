pub mod token;
pub mod device_lock;
use std::sync::atomic::Ordering;

use crate::RQError;
use rq_engine::{jce::SvcRespRegister, RQResult};

use crate::Client;

pub mod pwd;
pub mod qr_code;

impl Client {
    /// 注册客户端，登录后必须注册
    pub async fn register_client(&self) -> RQResult<SvcRespRegister> {
        let req = self.engine.read().await.build_client_register_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self
            .engine
            .read()
            .await
            .decode_client_register_response(resp.body)?;
        if !resp.result.is_empty() || resp.reply_code != 0 {
            return Err(RQError::Other(resp.result + &resp.reply_code.to_string()));
        }
        self.online.store(true, Ordering::SeqCst);
        Ok(resp)
    }

    pub async fn heartbeat(&self) -> RQResult<()> {
        let req = self.engine.read().await.build_heartbeat_packet();
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }
}


