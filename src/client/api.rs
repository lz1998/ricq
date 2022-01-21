use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use bytes::{Buf, Bytes};
use futures::{stream, StreamExt};
use rq_engine::{pb, GroupMessageEvent};
use tokio::sync::RwLock;

use crate::engine::command::{friendlist::*, oidb_svc::*, profile_service::*, wtlogin::*};
use crate::engine::structs::{FriendInfo, GroupInfo, GroupMemberInfo};
use crate::engine::MsgElem;
use crate::jce::{SvcDevLoginInfo, SvcRespRegister};
use crate::{QEvent, RQError, RQResult};

/// 登录相关
impl super::Client {
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
    pub async fn token_login(&self, mut token: impl Buf) -> RQResult<()> {
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

    /// 注册客户端，登录后必须注册
    pub async fn register_client(&self) -> RQResult<SvcRespRegister> {
        let req = self.engine.read().await.build_client_register_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self
            .engine
            .read()
            .await
            .decode_client_register_response(resp.body)?;
        if resp.result != "" || resp.reply_code != 0 {
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

/// API
impl super::Client {
    /// 获取进群申请信息
    pub async fn get_group_system_messages(
        &self,
        suspicious: bool,
    ) -> RQResult<GroupSystemMessages> {
        let req = self
            .engine
            .read()
            .await
            .build_system_msg_new_group_packet(suspicious);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_system_msg_group_packet(resp.body)
    }

    /// 获取好友列表
    /// 第一个参数offset，从0开始；第二个参数count，150，另外两个都是0
    pub async fn get_friend_list(
        &self,
        friend_start_index: i16,
        friend_list_count: i16,
        group_start_index: i16,
        group_list_count: i16,
    ) -> RQResult<FriendListResponse> {
        let req = self
            .engine
            .read()
            .await
            .build_friend_group_list_request_packet(
                friend_start_index,
                friend_list_count,
                group_start_index,
                group_list_count,
            );
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_friend_group_list_response(resp.body)
    }

    /// 获取群列表
    /// 第一个参数offset，从0开始；第二个参数count，150，另外两个都是0
    pub async fn get_group_list(&self, vec_cookie: &[u8]) -> RQResult<GroupListResponse> {
        let req = self
            .engine
            .read()
            .await
            .build_group_list_request_packet(vec_cookie);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_group_list_response(resp.body)
    }

    /// 发送群消息 TODO 切片, At预处理Display
    pub async fn send_group_message(
        &self,
        group_code: i64,
        message_chain: Vec<MsgElem>,
    ) -> RQResult<GroupMessageEvent> {
        let elems = crate::engine::msg::into_elems(message_chain.clone());
        let r: i32 = rand::random();
        let (tx, rx) = tokio::sync::oneshot::channel();
        {
            self.receipt_waiters.lock().await.insert(r, tx);
        }
        let req = self
            .engine
            .read()
            .await
            .build_group_sending_packet(group_code, r, 1, 0, 0, false, elems);
        self.send(req).await?;
        let mut event = GroupMessageEvent {
            id: -1, // -1 for failed send
            internal_id: r,
            group_code,
            sender: rq_engine::Sender {
                uin: self.uin().await,
                nickname: "".to_owned(), //todo self.engine.read().await.nickname.clone(),
                ..Default::default()
            },
            time: chrono::Utc::now().timestamp() as i32,
            elements: message_chain,
            ..Default::default()
        };
        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(seq)) => {
                event.id = seq;
            }
            Ok(Err(_)) => {} //todo
            Err(_) => {}
        }
        Ok(event)
    }

    /// 获取群成员信息
    pub async fn get_group_member_info(
        &self,
        group_code: i64,
        uin: i64,
    ) -> RQResult<GroupMemberInfo> {
        let req = self
            .engine
            .read()
            .await
            .build_group_member_info_request_packet(group_code, uin);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_group_member_info_response(resp.body)
    }

    /// 通过群号获取群
    pub async fn find_group(
        &self,
        code: i64,
    ) -> Option<Arc<(GroupInfo, RwLock<Vec<GroupMemberInfo>>)>> {
        for g in self.group_list.read().await.iter() {
            if g.0.code == code {
                return Some(g.clone());
            }
        }
        None
    }

    /// 通过群号从服务器获取群，请先尝试 find_group
    pub async fn get_group_infos(
        &self,
        group_codes: Vec<i64>,
    ) -> RQResult<Vec<pb::oidb::RspGroupInfo>> {
        let req = self
            .engine
            .read()
            .await
            .build_group_info_request_packet(group_codes);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_group_info_response(resp.body)
    }

    /// 通过uin获取群
    pub async fn find_group_by_uin(
        &self,
        uin: i64,
    ) -> Option<Arc<(GroupInfo, RwLock<Vec<GroupMemberInfo>>)>> {
        for g in self.group_list.read().await.iter() {
            if g.0.uin == uin {
                return Some(g.clone());
            }
        }
        None
    }

    /// 刷新群列表 TODO 获取群成员列表
    pub async fn reload_group_list(self: &Arc<Self>) -> RQResult<()> {
        // 获取群列表
        let mut vec_cookie = Bytes::new();
        let mut groups = Vec::new();
        loop {
            let resp = self.get_group_list(&vec_cookie).await?;
            vec_cookie = resp.vec_cookie;
            for g in resp.groups {
                groups.push(g);
            }
            if vec_cookie.is_empty() {
                break;
            }
        }

        let mut groups = stream::iter(groups)
            .map(|g| async move {
                let mem_list = self
                    .get_group_member_list(g.code, g.uin)
                    .await
                    .unwrap_or_default();
                Arc::new((g, RwLock::new(mem_list)))
            })
            .buffered(10)
            .collect()
            .await;

        let mut group_list = self.group_list.write().await;
        group_list.clear();
        group_list.append(&mut groups);
        Ok(())
    }

    /// 刷新好友列表
    pub async fn reload_friend_list(&self) -> RQResult<()> {
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
        Ok(())
    }

    /// 根据 uin 获取好友
    pub async fn find_friend(&self, uin: i64) -> Option<Arc<FriendInfo>> {
        self.friend_list
            .read()
            .await
            .iter()
            .find(|f| f.uin == uin)
            .cloned()
    }

    /// 获取群成员列表 (low level api)
    async fn _get_group_member_list(
        &self,
        group_uin: i64,
        group_code: i64,
        next_uin: i64,
    ) -> RQResult<GroupMemberListResponse> {
        let req = self
            .engine
            .read()
            .await
            .build_group_member_list_request_packet(group_uin, group_code, next_uin);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_group_member_list_response(resp.body)
    }

    /// 获取群成员列表
    pub async fn get_group_member_list(
        &self,
        group_code: i64,
        group_uin: i64,
    ) -> RQResult<Vec<GroupMemberInfo>> {
        let mut next_uin = 0;
        let mut list = Vec::new();
        loop {
            let mut resp = self
                ._get_group_member_list(group_uin, group_code, next_uin)
                .await?;
            if resp.list.is_empty() {
                return Err(RQError::Other("member list is empty".to_string()));
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
        Ok(list)
    }

    /// 刷新客户端状态
    pub async fn refresh_status(&self) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_get_offline_msg_request_packet(self.last_message_time.load(Ordering::SeqCst));
        let _resp = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 标记群消息已读
    pub async fn mark_group_message_readed(&self, group_code: i64, seq: i32) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_msg_readed_packet(group_code, seq);
        let _resp = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 标记私聊消息已读 TODO 待测试
    pub async fn mark_private_message_readed(&self, _uin: i64, _time: i64) -> RQResult<()> {
        // let resp = self
        //     .send_and_wait(self.build_private_msg_read_packet(uin, time).await)
        //     .await?;
        // println!("{}", resp.command_name); // todo
        Ok(())
    }

    /// 获取通过安全验证的设备
    pub async fn get_allowed_clients(&self) -> RQResult<Vec<SvcDevLoginInfo>> {
        let req = self.engine.read().await.build_device_list_request_packet();
        let resp = self.send_and_wait(req).await?;
        self.engine.read().await.decode_dev_list_response(resp.body)
    }

    /// 群禁言 (解除禁言 duration=0)
    pub async fn group_mute(
        &self,
        group_code: i64,
        member_uin: i64,
        duration: std::time::Duration,
    ) -> RQResult<()> {
        let req = self.engine.read().await.build_group_mute_packet(
            group_code,
            member_uin,
            duration.as_secs() as u32,
        );
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 全员禁言
    pub async fn group_mute_all(&self, group_code: i64, mute: bool) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_mute_all_packet(group_code, mute);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 修改群名称
    pub async fn update_group_name(&self, group_code: i64, name: String) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_name_update_packet(group_code, name);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 设置群公告
    pub async fn update_group_memo(&self, group_code: i64, memo: String) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_memo_update_packet(group_code, memo);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 设置群管理员
    pub async fn group_set_admin(&self, group_code: i64, member: i64, flag: bool) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_admin_set_packet(group_code, member, flag);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 好友戳一戳
    pub async fn friend_poke(&self, target: i64) -> RQResult<()> {
        let req = self.engine.read().await.build_friend_poke_packet(target);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 群戳一戳
    pub async fn group_poke(&self, group_code: i64, target: i64) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_poke_packet(group_code, target);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 群踢人
    pub async fn group_kick(
        &self,
        group_code: i64,
        member_uin: i64,
        kick_msg: &str,
        block: bool,
    ) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_kick_packet(group_code, member_uin, kick_msg, block);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    pub async fn group_invite(&self, group_code: i64, uin: i64) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_invite_packet(group_code, uin);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 获取群 @全体成员 剩余次数
    pub async fn group_at_all_remain(&self, group_code: i64) -> RQResult<GroupAtAllRemainInfo> {
        let req = self
            .engine
            .read()
            .await
            .build_group_at_all_remain_request_packet(group_code);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_group_at_all_remain_response(resp.body)
    }

    /// 设置群头衔
    pub async fn group_edit_special_title(
        &self,
        group_code: i64,
        member_uin: i64,
        new_title: String,
    ) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_edit_special_title_packet(group_code, member_uin, new_title);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 文本翻译
    pub async fn translate(
        &self,
        src_language: String,
        dst_language: String,
        src_text_list: Vec<String>,
    ) -> RQResult<Vec<String>> {
        let req = self.engine.read().await.build_translate_request_packet(
            src_language,
            dst_language,
            src_text_list.clone(),
        );
        let resp = self.send_and_wait(req).await?;
        let translations = self
            .engine
            .read()
            .await
            .decode_translate_response(resp.body)?;
        if translations.len() != src_text_list.len() {
            return Err(RQError::Other("translate length error".into()));
        }
        Ok(translations)
    }

    /// 发送好友消息
    pub async fn send_private_message(
        &self,
        target: i64,
        message_chain: Vec<MsgElem>,
    ) -> RQResult<()> {
        let elems = crate::engine::msg::into_elems(message_chain);
        let req = self
            .engine
            .read()
            .await
            .build_friend_sending_packet(target, 495, 1, 0, 0, elems);
        self.send(req).await?;
        Ok(())
    }

    pub async fn send_like(&self, uin: i64, count: i32) -> RQResult<()> {
        let req = self.engine.read().await.build_send_like_packet(uin, count);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    // TODO 待测试
    // 撤回私聊消息
    pub async fn recall_private_message(
        &self,
        uin: i64,
        msg_id: i32,
        msg_internal_id: i32,
    ) -> RQResult<()> {
        let req =
            self.engine
                .read()
                .await
                .build_private_recall_packet(uin, msg_id, msg_internal_id);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    // 群精华消息操作
    // flag true 设置群精华消息 ｜ false 移除群精华消息
    pub async fn group_essence_operation(
        &self,
        group_code: i64,
        message_id: i32,
        msg_internal_id: i32,
        flag: bool,
    ) -> RQResult<()> {
        let req = self.engine.read().await.build_essence_msg_operate_packet(
            group_code,
            message_id,
            msg_internal_id,
            flag,
        );
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }
}
