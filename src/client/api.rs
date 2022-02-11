use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use bytes::{Buf, Bytes};
use futures::{stream, StreamExt};
use tokio::sync::RwLock;

use rq_engine::command::message_svc::MessageSyncResponse;
use rq_engine::command::oidb_svc::music::{MusicShare, MusicType, SendMusicTarget};
use rq_engine::common::group_code2uin;
use rq_engine::msg::elem::Anonymous;
use rq_engine::msg::MessageChain;
use rq_engine::pb;

use crate::client::Group;
use crate::engine::command::{friendlist::*, oidb_svc::*, profile_service::*, wtlogin::*};
use crate::engine::structs::{FriendInfo, GroupInfo, GroupMemberInfo, MessageReceipt};
use crate::handler::QEvent;
use crate::jce::{SvcDevLoginInfo, SvcRespRegister};
use crate::{RQError, RQResult};

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

    /// 设备锁登录 - 二维码、密码登录都需要
    pub async fn device_lock_login(self: &Arc<Self>) -> RQResult<LoginResponse> {
        let req = self.engine.read().await.build_device_lock_login_packet();
        let resp = self.send_and_wait(req).await?;
        let resp = self.engine.read().await.decode_login_response(resp.body)?;
        self.process_login_response(resp.clone()).await;
        Ok(resp)
    }

    /// token 登录
    pub async fn token_login(self: &Arc<Self>, mut token: impl Buf) -> RQResult<()> {
        self.load_token(&mut token).await;
        let req = self.engine.read().await.build_request_change_sig_packet();
        self.send_and_wait(req).await?;
        self.register_client().await?;
        let r = tokio::join! {
            self.wait_packet("StatSvc.ReqMSFOffline", 1),
            self.wait_packet("MessageSvc.PushForceOffline", 1)
        };
        if let (Err(RQError::Timeout), Err(RQError::Timeout)) = r {
            self.handler.handle(QEvent::Login(self.uin().await)).await;
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

/// API
impl super::Client {
    /// 获取进群申请信息
    async fn get_group_system_messages(&self, suspicious: bool) -> RQResult<GroupSystemMessages> {
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

    /// 获取所有进群请求
    pub async fn get_all_group_system_messages(&self) -> RQResult<GroupSystemMessages> {
        let mut resp = self.get_group_system_messages(false).await?;
        let risk_resp = self.get_group_system_messages(true).await?;
        resp.join_group_requests
            .extend(risk_resp.join_group_requests);
        resp.self_invited.extend(risk_resp.self_invited);
        Ok(resp)
    }

    /// 处理加群申请
    pub async fn solve_group_system_message(
        &self,
        msg_seq: i64,
        req_uin: i64,
        group_code: i64,
        suspicious: bool,
        is_invite: bool,
        accept: bool,
        block: bool,
        reason: String,
    ) -> RQResult<()> {
        let pkt = self
            .engine
            .read()
            .await
            .build_system_msg_group_action_packet(
                msg_seq,
                req_uin,
                group_code,
                if suspicious { 2 } else { 1 },
                is_invite,
                accept,
                block,
                reason,
            );
        self.send_and_wait(pkt).await?;

        Ok(())
    }

    /// 获取好友请求
    pub async fn get_friend_system_messages(&self) -> RQResult<FriendSystemMessages> {
        let req = self
            .engine
            .read()
            .await
            .build_system_msg_new_friend_packet();
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_system_msg_friend_packet(resp.body)
    }

    /// 处理好友申请
    pub async fn solve_friend_system_message(
        &self,
        msg_seq: i64,
        req_uin: i64,
        accept: bool,
    ) -> RQResult<()> {
        let pkt = self
            .engine
            .read()
            .await
            .build_system_msg_friend_action_packet(msg_seq, req_uin, accept);
        self.send_and_wait(pkt).await?;
        Ok(())
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

    /// 删除好友
    /// ## Args
    /// - `del_uin` 为要删除的好友QQid
    ///
    /// ## Return
    /// - 如果删除好友成功 返回 Ok(())
    /// - 如果删除好友失败 返回 Err(RQError::Other)
    /// - 其他异常 返回 Err(..)
    pub async fn delete_friend(&self, del_uin: i64) -> RQResult<()> {
        let req = self.engine.read().await.build_delete_friend_packet(del_uin);

        let resp = self.send_and_wait(req).await?;

        let resp = self.engine.read().await.decode_remove_friend(resp.body)?;
        if resp.error_code != 0 {
            Err(RQError::Other(format!(
                "Delete Friend Failure : code = {}",
                resp.error_code
            )))
        } else {
            Ok(())
        }
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
        message_chain: MessageChain,
    ) -> RQResult<MessageReceipt> {
        let time = chrono::Utc::now().timestamp();
        let ran = (rand::random::<u32>() >> 1) as i32;
        let (tx, rx) = tokio::sync::oneshot::channel();
        {
            self.receipt_waiters.lock().await.insert(ran, tx);
        }
        let req = self.engine.read().await.build_group_sending_packet(
            group_code,
            message_chain.into(),
            ran,
            time,
            1,
            0,
            0,
            false,
        );
        self.send(req).await?;
        let mut receipt = MessageReceipt {
            seqs: vec![0],
            rands: vec![ran],
            time,
        };
        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(seq)) => {
                if let Some(s) = receipt.seqs.first_mut() {
                    *s = seq;
                }
            }
            Ok(Err(_)) => {} //todo
            Err(_) => {}
        }
        Ok(receipt)
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
    pub async fn find_group(&self, code: i64, auto_reload: bool) -> Option<Arc<Group>> {
        let group = self.groups.read().await.get(&code).cloned();
        if group.is_some() {
            return group;
        }
        if auto_reload {
            self.reload_group(code).await.ok();
        }
        self.groups.read().await.get(&code).cloned()
    }

    /// 批量获取群信息
    pub async fn get_group_infos(&self, group_codes: Vec<i64>) -> RQResult<Vec<GroupInfo>> {
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

    /// 获取群信息，请先尝试 find_group
    pub async fn get_group_info(&self, group_code: i64) -> RQResult<Option<GroupInfo>> {
        Ok(self.get_group_infos(vec![group_code]).await?.pop())
    }

    /// 刷新单个群信息
    pub async fn reload_group(&self, group_code: i64) -> RQResult<()> {
        let group_info = self
            .get_group_info(group_code)
            .await?
            .ok_or(RQError::Other("failed to get group".into()))?;
        let members = self.get_group_member_list(group_code).await?;
        let mut groups = self.groups.write().await;
        groups.insert(
            group_info.code,
            Arc::new(Group {
                info: group_info,
                members: RwLock::new(members),
            }),
        );
        Ok(())
    }

    /// 刷新群列表
    pub async fn reload_groups(self: &Arc<Self>) -> RQResult<()> {
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

        let group_list: Vec<(i64, Arc<Group>)> = stream::iter(groups)
            .map(|g| async move {
                let mem_list = self.get_group_member_list(g.code).await.unwrap_or_default();
                (
                    g.code,
                    Arc::new(Group {
                        info: g,
                        members: RwLock::new(mem_list),
                    }),
                )
            })
            .buffered(10)
            .collect()
            .await;

        let mut groups = self.groups.write().await;
        groups.clear();
        groups.extend(group_list);
        Ok(())
    }

    /// 刷新好友列表
    pub async fn reload_friends(&self) -> RQResult<()> {
        let mut cur_friend_count = 0;
        let mut friend_list = Vec::new();
        loop {
            let resp = self.get_friend_list(cur_friend_count, 150, 0, 0).await?;
            cur_friend_count += resp.list.len() as i16;
            for f in resp.list {
                friend_list.push((f.uin, Arc::new(f)));
            }
            if cur_friend_count >= resp.total_count {
                break;
            }
        }
        let mut friends = self.friends.write().await;
        friends.clear();
        friends.extend(friend_list);
        Ok(())
    }

    /// 根据 uin 获取好友
    pub async fn find_friend(&self, uin: i64) -> Option<Arc<FriendInfo>> {
        self.friends.read().await.get(&uin).cloned()
    }

    /// 获取群成员列表 (low level api)
    async fn _get_group_member_list(
        &self,
        group_code: i64,
        next_uin: i64,
    ) -> RQResult<GroupMemberListResponse> {
        let req = self
            .engine
            .read()
            .await
            .build_group_member_list_request_packet(group_code, next_uin);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_group_member_list_response(resp.body)
    }

    /// 获取群成员列表
    pub async fn get_group_member_list(&self, group_code: i64) -> RQResult<Vec<GroupMemberInfo>> {
        let mut next_uin = 0;
        let mut list = Vec::new();
        loop {
            let mut resp = self._get_group_member_list(group_code, next_uin).await?;
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
        member_uins: Vec<i64>,
        kick_msg: &str,
        block: bool,
    ) -> RQResult<()> {
        let req = self.engine.read().await.build_group_kick_packet(
            group_code,
            member_uins,
            kick_msg,
            block,
        );
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
        message_chain: MessageChain,
    ) -> RQResult<MessageReceipt> {
        let time = chrono::Utc::now().timestamp();
        let seq = self.engine.read().await.next_friend_seq();
        let ran = (rand::random::<u32>() >> 1) as i32;
        let req = self.engine.read().await.build_friend_sending_packet(
            target,
            message_chain.into(),
            seq,
            ran,
            time,
            1,
            0,
            0,
        );
        self.send(req).await?;
        Ok(MessageReceipt {
            seqs: vec![seq],
            rands: vec![ran],
            time,
        })
    }

    pub async fn send_like(&self, uin: i64, count: i32) -> RQResult<()> {
        let req = self.engine.read().await.build_send_like_packet(uin, count);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    // 撤回私聊消息
    pub async fn recall_private_message(
        &self,
        uin: i64,
        msg_time: i64,
        seqs: Vec<i32>,
        rands: Vec<i32>,
    ) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_private_recall_packet(uin, msg_time, seqs, rands);
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
    ) -> RQResult<pb::oidb::EacRspBody> {
        let req = self.engine.read().await.build_essence_msg_operate_packet(
            group_code,
            message_id,
            msg_internal_id,
            flag,
        );
        let resp = self.send_and_wait(req).await?;
        let decode = self
            .engine
            .read()
            .await
            .decode_essence_msg_response(resp.body)?;
        Ok(decode)
    }

    // TODO 待完善
    // 图片 OCR
    pub async fn image_ocr(
        &self,
        img_url: String,
        md5: String,
        size: i32,
        wight: i32,
        height: i32,
    ) -> RQResult<OcrResponse> {
        let req = self
            .engine
            .read()
            .await
            .build_image_ocr_request_packet(img_url, md5, size, wight, height);
        let resp = self.send_and_wait(req).await?;

        let decode = self
            .engine
            .read()
            .await
            .decode_image_ocr_response(resp.body)?;
        Ok(decode)
    }

    // 标记消息已收到，server 不再重复推送
    pub async fn delete_message(&self, items: Vec<pb::MessageItem>) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_delete_message_request_packet(items);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    // sync message
    async fn sync_message(&self, sync_flag: i32) -> RQResult<MessageSyncResponse> {
        let time = chrono::Utc::now().timestamp();
        let req = self
            .engine
            .read()
            .await
            .build_get_message_request_packet(sync_flag, time);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_message_svc_packet(resp.body)
    }

    // 从服务端拉取通知
    pub(crate) async fn sync_all_message(&self) -> RQResult<Vec<pb::msg::Message>> {
        const SYNC_START: i32 = 0;
        const _SYNC_CONTINUE: i32 = 1;
        const SYNC_STOP: i32 = 2;

        let mut sync_flag = SYNC_START;
        let mut msgs = Vec::new();
        loop {
            let resp = match self.sync_message(sync_flag).await {
                Ok(resp) => resp,
                Err(_) => {
                    tracing::warn!(target: "rs_qq", "failed to sync_message");
                    break;
                }
            };
            if let Err(err) = self
                .delete_message(
                    resp.msgs
                        .iter()
                        .map(|m| {
                            let head = m.head.as_ref().unwrap();
                            pb::MessageItem {
                                from_uin: head.from_uin(),
                                to_uin: head.to_uin(),
                                msg_type: head.msg_type(),
                                msg_seq: head.msg_seq(),
                                msg_uid: head.msg_uid(),
                                ..Default::default()
                            }
                        })
                        .collect(),
                )
                .await
            {
                tracing::warn!(target: "rs_qq", "failed to delete_message: {}",err);
                break;
            }
            match resp.msg_rsp_type {
                0 => {
                    let mut engine = self.engine.write().await;
                    if let Some(sync_cookie) = resp.sync_cookie {
                        engine.transport.sig.sync_cookie = Bytes::from(sync_cookie)
                    }
                    if let Some(pub_account_cookie) = resp.pub_account_cookie {
                        engine.transport.sig.pub_account_cookie = Bytes::from(pub_account_cookie)
                    }
                }
                1 => {
                    let mut engine = self.engine.write().await;
                    if let Some(sync_cookie) = resp.sync_cookie {
                        engine.transport.sig.sync_cookie = Bytes::from(sync_cookie)
                    }
                }
                2 => {
                    let mut engine = self.engine.write().await;
                    if let Some(pub_account_cookie) = resp.pub_account_cookie {
                        engine.transport.sig.pub_account_cookie = Bytes::from(pub_account_cookie)
                    }
                }
                _ => {}
            }
            msgs.extend(resp.msgs);
            sync_flag = resp.sync_flag;
            if sync_flag == SYNC_STOP {
                break;
            }
        }
        return Ok(msgs);
    }

    /// 获取自己的匿名信息（用于发送群消息）
    pub async fn get_anony_info(&self, group_code: i64) -> RQResult<Option<Anonymous>> {
        let req = self
            .engine
            .read()
            .await
            .build_get_anony_info_request(group_code);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_get_anony_info_response(resp.body)
    }

    /// 分享群音乐
    pub async fn send_group_music_share(
        &self,
        group_code: i64,
        music_share: MusicShare,
        music_type: MusicType,
    ) -> RQResult<()> {
        let req = self.engine.read().await.build_share_music_request_packet(
            SendMusicTarget::Group(group_code),
            music_share,
            music_type.version(),
        );
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 分享好友音乐
    pub async fn send_friend_music_share(
        &self,
        uin: i64,
        music_share: MusicShare,
        music_type: MusicType,
    ) -> RQResult<()> {
        let req = self.engine.read().await.build_share_music_request_packet(
            SendMusicTarget::Friend(uin),
            music_share,
            music_type.version(),
        );
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 发送临时消息
    pub async fn send_temp_message(
        &self,
        group_code: i64,
        user_uin: i64,
        message_chain: MessageChain,
    ) -> RQResult<()> {
        let time = chrono::Utc::now().timestamp();
        let seq = self.engine.read().await.next_friend_seq();
        let ran = (rand::random::<u32>() >> 1) as i32;
        let req = self.engine.read().await.build_temp_sending_packet(
            group_code2uin(group_code),
            user_uin,
            message_chain.into(),
            seq,
            ran,
            time,
        );
        self.send(req).await?;
        Ok(())
    }

    /// 修改群名片
    pub async fn edit_group_member_card(
        &self,
        group_code: i64,
        member_uin: i64,
        card: String,
    ) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_edit_group_tag_packet(group_code, member_uin, card);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 撤回群消息
    pub async fn recall_group_message(
        &self,
        group_code: i64,
        seqs: Vec<i32>,
        rands: Vec<i32>,
    ) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_recall_packet(group_code, seqs, rands);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }
}
