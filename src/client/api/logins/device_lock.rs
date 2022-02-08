use std::sync::Arc;

use rq_engine::{command::wtlogin::LoginResponse, RQResult};

use crate::Client;

impl Client {
    /// 设备锁登录 - 二维码、密码登录都需要
    pub async fn device_lock_login(self: &Arc<Self>) -> RQResult<LoginResponse> {
        let req = self.engine.read().await.build_device_lock_login_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }
}
