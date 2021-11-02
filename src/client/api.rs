use crate::client::income::{decode_login_response, decode_trans_emp_response, LoginResponse, QRCodeState};
use crate::client::outcome::OutcomePacket;

impl super::Client {
    pub async fn fetch_qrcode(&self) -> Option<QRCodeState> {
        let (seq, req) = self.build_qrcode_fetch_request_packet().await;
        let resp = self.send_and_wait(OutcomePacket {
            seq,
            bytes: req,
        }).await?;
        if &resp.command_name != "wtlogin.trans_emp" {
            return None;
        }
        decode_trans_emp_response(self, &resp.payload).await
    }

    pub async fn query_qrcode_result(&self, sig: &[u8]) -> Option<QRCodeState> {
        let (seq, req) = self.build_qrcode_result_query_request_packet(sig).await;
        let resp = self.send_and_wait(OutcomePacket {
            seq,
            bytes: req,
        }).await?;
        if &resp.command_name != "wtlogin.trans_emp" {
            return None;
        }
        decode_trans_emp_response(self, &resp.payload).await
    }

    pub async fn qrcode_login(&self, tmp_pwd: &[u8], tmp_no_pic_sig: &[u8], tgt_qr: &[u8]) -> Option<LoginResponse> {
        let (seq, req) = self.build_qrcode_login_packet(tmp_pwd, tmp_no_pic_sig, tgt_qr).await;
        let resp = self.send_and_wait(OutcomePacket {
            seq,
            bytes: req,
        }).await?;
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }
}