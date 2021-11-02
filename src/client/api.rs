use std::sync::atomic::Ordering;
use jce_struct::Jce;
use crate::client::income::{decode_client_register_response, decode_login_response, decode_trans_emp_response, LoginResponse, QRCodeState};
use crate::client::outcome::OutcomePacket;
use crate::jce::{RequestDataVersion2, RequestPacket, SvcRespRegister};
use bytes::{Buf, Bytes};

impl super::Client {
    pub async fn fetch_qrcode(&self) -> Option<QRCodeState> {
        let resp = self.send_and_wait(self.build_qrcode_fetch_request_packet().await.into()).await?;
        if &resp.command_name != "wtlogin.trans_emp" {
            return None;
        }
        decode_trans_emp_response(self, &resp.payload).await
    }

    pub async fn query_qrcode_result(&self, sig: &[u8]) -> Option<QRCodeState> {
        let resp = self.send_and_wait(self.build_qrcode_result_query_request_packet(sig).await.into()).await?;
        if &resp.command_name != "wtlogin.trans_emp" {
            return None;
        }
        decode_trans_emp_response(self, &resp.payload).await
    }

    pub async fn qrcode_login(&self, tmp_pwd: &[u8], tmp_no_pic_sig: &[u8], tgt_qr: &[u8]) -> Option<LoginResponse> {
        let resp = self.send_and_wait(self.build_qrcode_login_packet(tmp_pwd, tmp_no_pic_sig, tgt_qr).await.into()).await?;
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }

    pub async fn register_client(&self) -> Option<SvcRespRegister> {
        let mut resp = self.send_and_wait(self.build_client_register_packet().await.into()).await?;
        if &resp.command_name != "StatSvc.register" {
            return None;
        }
        let resp = decode_client_register_response(&resp.payload);
        // println!("{:?}",svc_rsp.result);
        if resp.result != "" || resp.reply_code != 0 {
            return None;
        }
        self.online.store(true, Ordering::SeqCst);
        Some(resp)
    }
}