use std::sync::{atomic::Ordering, Arc};

use rq_engine::{command::wtlogin::LoginResponse, RQResult};

use crate::Client;

impl Client {
    /// 密码登录 - 提交密码md5
    pub async fn password_md5_login(
        self: &Arc<Self>,
        uin: i64,
        password_md5: &[u8],
    ) -> RQResult<LoginResponse> {
        self.engine.read().await.uin.store(uin, Ordering::Relaxed);
        let req = self
            .engine
            .read()
            .await
            .build_login_packet(password_md5, true);
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    pub async fn password_login(
        self: &Arc<Self>,
        uin: i64,
        password: &str,
    ) -> RQResult<LoginResponse> {
        self.password_md5_login(uin, &md5::compute(password).to_vec())
            .await
    }

    /// 密码登录 - 请求短信验证码
    pub async fn request_sms(self: &Arc<Self>) -> RQResult<LoginResponse> {
        let req = self.engine.read().await.build_sms_request_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    /// 密码登录 - 提交短信验证码
    pub async fn submit_sms_code(self: &Arc<Self>, code: &str) -> RQResult<LoginResponse> {
        let req = self
            .engine
            .read()
            .await
            .build_sms_code_submit_packet(code.trim());
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    /// 密码登录 - 提交滑块ticket
    pub async fn submit_ticket(self: &Arc<Self>, ticket: &str) -> RQResult<LoginResponse> {
        let req = self.engine.read().await.build_ticket_submit_packet(ticket);
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }
}
