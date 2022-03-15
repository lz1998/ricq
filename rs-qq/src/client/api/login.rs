use std::sync::atomic::Ordering;

use crate::engine::command::wtlogin::*;
use crate::engine::token::Token;
use crate::jce::SvcRespRegister;
use crate::{RQError, RQResult};

/// 登录相关
impl super::super::Client {
    /// 二维码登录 - 获取二维码
    pub async fn fetch_qrcode(&self) -> RQResult<QRCodeState> {
        let req = self.engine.read().await.build_qrcode_fetch_request_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self
            .engine
            .read()
            .await
            .decode_trans_emp_response(resp.body)?;
        self.process_trans_emp_response(resp.clone()).await;
        Ok(resp)
    }

    /// 二维码登录 - 查询二维码状态
    pub async fn query_qrcode_result(&self, sig: &[u8]) -> RQResult<QRCodeState> {
        let req = self
            .engine
            .read()
            .await
            .build_qrcode_result_query_request_packet(sig);
        let resp = self.send_and_wait(req).await?;
        let resp = self
            .engine
            .read()
            .await
            .decode_trans_emp_response(resp.body)?;
        self.process_trans_emp_response(resp.clone()).await;
        Ok(resp)
    }

    /// 二维码登录 - 登录 ( 可能还需要 device_lock_login )
    pub async fn qrcode_login(
        &self,
        tmp_pwd: &[u8],
        tmp_no_pic_sig: &[u8],
        tgt_qr: &[u8],
    ) -> RQResult<LoginResponse> {
        let req =
            self.engine
                .read()
                .await
                .build_qrcode_login_packet(tmp_pwd, tmp_no_pic_sig, tgt_qr);
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    /// 密码登录 - 提交密码md5
    pub async fn password_md5_login(
        &self,
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

    pub async fn password_login(&self, uin: i64, password: &str) -> RQResult<LoginResponse> {
        self.password_md5_login(uin, &md5::compute(password).to_vec())
            .await
    }

    /// 密码登录 - 请求短信验证码
    pub async fn request_sms(&self) -> RQResult<LoginResponse> {
        let req = self.engine.read().await.build_sms_request_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    /// 密码登录 - 提交短信验证码
    pub async fn submit_sms_code(&self, code: &str) -> RQResult<LoginResponse> {
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
    pub async fn submit_ticket(&self, ticket: &str) -> RQResult<LoginResponse> {
        let req = self.engine.read().await.build_ticket_submit_packet(ticket);
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    /// 设备锁登录 - 二维码、密码登录都需要
    pub async fn device_lock_login(&self) -> RQResult<LoginResponse> {
        let req = self.engine.read().await.build_device_lock_login_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    /// token 登录
    pub async fn token_login(&self, token: Token) -> RQResult<LoginResponse> {
        self.load_token(token).await;
        self.request_change_sig(None).await
    }

    pub(crate) async fn request_change_sig(
        &self,
        main_sig_map: Option<u32>,
    ) -> RQResult<LoginResponse> {
        let req = self
            .engine
            .read()
            .await
            .build_request_change_sig_packet(main_sig_map);
        let resp = self.send_and_wait(req).await?;
        let resp = self
            .engine
            .read()
            .await
            .decode_exchange_emp_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

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

    // 系统强制下线 response
    pub(crate) async fn send_msg_offline_rsp(&self, uin: i64, seq_no: i64) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_msf_force_offline_rsp(uin, seq_no);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    pub(crate) async fn send_sid_ticket_expired_response(&self, seq: i32) -> RQResult<()> {
        let rsp = self
            .engine
            .read()
            .await
            .build_sid_ticket_expired_response(seq);
        self.send(rsp).await?;
        Ok(())
    }
}
