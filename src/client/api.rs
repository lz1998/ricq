use std::sync::Arc;
use std::sync::atomic::Ordering;
use bytes::Bytes;
use crate::jce::SvcRespRegister;
use crate::client::income::decoder::{friendlist::*, profile_service::*, stat_svc::*, wtlogin::*};
use crate::client::income::decoder::group_member_card::decode_group_member_info_response;
use crate::client::msg::Msg;
use crate::client::structs::{GroupInfo, GroupMemberInfo};

/// 登录相关
impl super::Client {
    /// 二维码登录 - 获取二维码
    pub async fn fetch_qrcode(&self) -> Option<QRCodeState> {
        let resp = self.send_and_wait(self.build_qrcode_fetch_request_packet().await.into()).await?;
        if &resp.command_name != "wtlogin.trans_emp" {
            return None;
        }
        decode_trans_emp_response(self, &resp.payload).await
    }

    /// 二维码登录 - 查询二维码状态
    pub async fn query_qrcode_result(&self, sig: &[u8]) -> Option<QRCodeState> {
        let resp = self.send_and_wait(self.build_qrcode_result_query_request_packet(sig).await.into()).await?;
        if &resp.command_name != "wtlogin.trans_emp" {
            return None;
        }
        decode_trans_emp_response(self, &resp.payload).await
    }

    /// 二维码登录 - 登录 ( 可能还需要 device_lock_login )
    pub async fn qrcode_login(&self, tmp_pwd: &[u8], tmp_no_pic_sig: &[u8], tgt_qr: &[u8]) -> Option<LoginResponse> {
        let resp = self.send_and_wait(self.build_qrcode_login_packet(tmp_pwd, tmp_no_pic_sig, tgt_qr).await.into()).await?;
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }

    /// 密码登录 - 提交密码
    pub async fn password_login(&self) -> Option<LoginResponse> {
        let resp = self.send_and_wait(self.build_login_packet(true).await.into()).await.unwrap();
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }

    /// 密码登录 - 请求短信验证码
    pub async fn request_sms(&self) -> Option<LoginResponse> {
        let resp = self.send_and_wait(self.build_sms_request_packet().await.into()).await.unwrap();
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }

    /// 密码登录 - 提交短信验证码
    pub async fn submit_sms_code(&self, code: &str) -> Option<LoginResponse> {
        let resp = self.send_and_wait(self.build_sms_code_submit_packet(code.trim()).await.into()).await.unwrap();
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }

    /// 密码登录 - 提交滑块ticket
    pub async fn submit_ticket(&self, ticket: &str) -> Option<LoginResponse> {
        let resp = self.send_and_wait(self.build_ticket_submit_packet(ticket).await.into()).await.unwrap();
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }

    /// 设备锁登录 - 二维码、密码登录都需要
    pub async fn device_lock_login(&self) -> Option<LoginResponse> {
        let resp = self.send_and_wait(self.build_device_lock_login_packet().await.into()).await.unwrap();
        if &resp.command_name != "wtlogin.login" {
            return None;
        }
        decode_login_response(self, &resp.payload).await
    }

    /// 注册客户端，登录后必须注册
    pub async fn register_client(&self) -> Option<SvcRespRegister> {
        let resp = self.send_and_wait(self.build_client_register_packet().await.into()).await?;
        if &resp.command_name != "StatSvc.register" {
            return None;
        }
        let resp = decode_client_register_response(&resp.payload);
        if resp.result != "" || resp.reply_code != 0 {
            return None;
        }
        self.online.store(true, Ordering::SeqCst);
        Some(resp)
    }
}

/// API
impl super::Client {
    /// 获取进群申请信息
    pub async fn get_group_system_messages(&self, suspicious: bool) -> Option<GroupSystemMessages> {
        let resp = self.send_and_wait(self.build_system_msg_new_group_packet(suspicious).await.into()).await?;
        if &resp.command_name != "ProfileService.Pb.ReqSystemMsgNew.Group" {
            return None;
        }
        decode_system_msg_group_packet(&resp.payload)
    }

    /// 获取好友列表
    /// 第一个参数offset，从0开始；第二个参数count，150，另外两个都是0
    pub async fn get_friend_list(&self, friend_start_index: i16, friend_list_count: i16, group_start_index: i16, group_list_count: i16) -> Option<FriendListResponse> {
        let resp = self.send_and_wait(self.build_friend_group_list_request_packet(friend_start_index, friend_list_count, group_start_index, group_list_count).await.into()).await?;
        if &resp.command_name != "friendlist.getFriendGroupList" {
            return None;
        }
        decode_friend_group_list_response(&resp.payload)
    }

    /// 获取群列表
    /// 第一个参数offset，从0开始；第二个参数count，150，另外两个都是0
    pub async fn get_group_list(&self, vec_cookie: &[u8]) -> Option<GroupListResponse> {
        let resp = self.send_and_wait(self.build_group_list_request_packet(vec_cookie).await.into()).await?;
        if &resp.command_name != "friendlist.GetTroopListReqV2" {
            return None;
        }
        decode_group_list_response(&resp.payload)
    }

    /// 发送群消息 TODO 切片, At预处理Display
    pub async fn send_group_message(&self, group_code: i64, message_chain: Vec<Msg>) {
        let mut elems = Vec::new();
        for message in message_chain.iter() {
            elems.append(&mut message.pack());
        }
        let (_, packet) = self.build_group_sending_packet(group_code, 383, 1, 0, 0, false, elems).await;
        self.out_pkt_sender.send(packet);
    }

    /// 获取群成员信息
    pub async fn get_group_member_info(&self, group_code: i64, uin: i64) -> Option<GroupMemberInfo> {
        let resp = self.send_and_wait(self.build_group_member_info_request_packet(group_code, uin).await.into()).await?;
        if &resp.command_name != "group_member_card.get_group_member_card_info" {
            return None;
        }
        decode_group_member_info_response(&resp.payload)
    }

    /// 通过群号获取群
    pub async fn find_group(&self, code: i64) -> Option<Arc<GroupInfo>> {
        for g in self.group_list.read().await.iter() {
            if g.code == code {
                return Some(g.clone());
            }
        }
        None
    }

    /// 通过uin获取群
    pub async fn find_group_by_uin(&self, uin: i64) -> Option<Arc<GroupInfo>> {
        for g in self.group_list.read().await.iter() {
            if g.uin == uin {
                return Some(g.clone());
            }
        }
        None
    }

    /// 刷新群列表 TODO 获取群成员列表
    pub async fn reload_group_list(self: &Arc<Self>) -> Option<()> {
        // 获取群列表
        let mut vec_cookie = Bytes::new();
        let mut groups = Vec::new();
        loop {
            let resp = self.get_group_list(&vec_cookie).await?;
            vec_cookie = resp.vec_cookie;
            for g in resp.groups {
                groups.push(Arc::new(g));
            }
            if vec_cookie.is_empty() {
                break;
            }
        }

        // 对于每个群，获取群成员列表（最多10个群并发执行）
        let semaphore = Arc::new(tokio::sync::Semaphore::new(50));
        let mut handles = Vec::new();

        for g in groups.iter_mut() {
            let cli = self.clone();
            let group = g.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            handles.push(tokio::spawn(async move {
                let mut mem_list = cli.get_group_member_list(group.code, group.uin).await?;
                let mut members = group.members.write().await;
                members.append(&mut mem_list);
                drop(permit);
                Some(())
            }));
        }
        for h in handles {
            h.await;
        }

        let mut group_list = self.group_list.write().await;
        group_list.clear();
        group_list.append(&mut groups); // TODO 不知道会不会复制大量内存
        Some(())
    }

    /// 刷新好友列表
    pub async fn reload_friend_list(&self) -> Option<()> {
        let mut cur_friend_count = 0;
        let mut friends = Vec::new();
        loop {
            let resp = self.get_friend_list(cur_friend_count, 150, 0, 0).await?;
            cur_friend_count += resp.list.len() as i16;
            for f in resp.list {
                friends.push(Arc::new(f));
            }
            if cur_friend_count >= resp.total_count {
                break;
            }
        }
        let mut friend_list = self.friend_list.write().await;
        friend_list.clear();
        friend_list.append(&mut friends);
        Some(())
    }

    /// 获取群成员列表 (low level api)
    async fn _get_group_member_list(&self, group_uin: i64, group_code: i64, next_uin: i64) -> Option<GroupMemberListResponse> {
        let resp = self.send_and_wait(self.build_group_member_list_request_packet(group_uin, group_code, next_uin).await.into()).await?;
        if resp.command_name != "friendlist.GetTroopMemberListReq" {
            return None;
        }
        decode_group_member_list_response(&resp.payload)
    }

    /// 获取群成员列表
    pub async fn get_group_member_list(&self, group_code: i64, group_uin: i64) -> Option<Vec<GroupMemberInfo>> {
        let mut next_uin = 0;
        let mut list = Vec::new();
        loop {
            let mut resp = self._get_group_member_list(group_uin, group_code, next_uin).await?;
            if resp.list.is_empty() {
                return None;
            }
            for m in resp.list.iter_mut() {
                m.group_code = group_code;
            }
            list.append(&mut resp.list);
            next_uin = resp.next_uin;
            if next_uin == 0 {
                break;
            }
        }
        Some(list)
    }

    /// 刷新客户端状态
    pub async fn refresh_status(&self) {
        self.send_and_wait(self.build_get_offline_msg_request_packet()).await;
    }
}
