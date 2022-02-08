use std::sync::Arc;

use rq_engine::{
    command::wtlogin::{LoginResponse, QRCodeState},
    RQResult,
};

use crate::Client;

impl Client {
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
        self: &Arc<Self>,
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
}
