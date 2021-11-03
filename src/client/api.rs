use std::sync::atomic::Ordering;
use jce_struct::Jce;
use crate::client::income::{decode_client_register_response, decode_friend_group_list_response, decode_login_response, decode_system_msg_group_packet, decode_trans_emp_response, FriendListResponse, GroupSystemMessages, LoginResponse, QRCodeState};
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

    pub async fn get_group_system_messages(&self, suspicious: bool) -> Option<GroupSystemMessages> {
        let mut resp = self.send_and_wait(self.build_system_msg_new_group_packet(suspicious).await.into()).await?;
        if &resp.command_name != "ProfileService.Pb.ReqSystemMsgNew.Group" {
            return None;
        }
        let resp = decode_system_msg_group_packet(&resp.payload)?;
        Some(resp)
    }

    // 第一个参数offset，从0开始；第二个参数count，150，另外两个都是0
    pub async fn friend_group_list(&self, friend_start_index: i16, friend_list_count: i16, group_start_index: i16, group_list_count: i16) -> Option<FriendListResponse> {
        let mut resp = self.send_and_wait(self.build_friend_group_list_request_packet(friend_start_index, friend_list_count, group_start_index, group_list_count).await.into()).await?;
        if &resp.command_name != "friendlist.getFriendGroupList" {
            return None;
        }
        let resp = decode_friend_group_list_response(&resp.payload)?;
        Some(resp)
    }
}