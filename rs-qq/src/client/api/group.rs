use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use futures::{stream, StreamExt};
use tokio::sync::RwLock;

use rq_engine::command::multi_msg::gen_forward_preview;
use rq_engine::msg::elem::RichMsg;
use rq_engine::structs::{ForwardMessage, MessageNode};

use crate::client::Group;
use crate::engine::command::img_store::GroupImageStoreResp;
use crate::engine::command::oidb_svc::music::{MusicShare, MusicType, SendMusicTarget};
use crate::engine::command::{friendlist::*, oidb_svc::*, profile_service::*};
use crate::engine::common::group_code2uin;
use crate::engine::hex::encode_hex;
use crate::engine::highway::BdhInput;
use crate::engine::msg::elem::{Anonymous, GroupImage};
use crate::engine::msg::MessageChain;
use crate::engine::pb;
use crate::engine::structs::GroupAudio;
use crate::engine::structs::{GroupInfo, GroupMemberInfo, MessageReceipt};
use crate::structs::ImageInfo;
use crate::{RQError, RQResult};

impl super::super::Client {
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

    /// 发送群消息
    pub async fn send_group_message(
        &self,
        group_code: i64,
        message_chain: MessageChain,
    ) -> RQResult<MessageReceipt> {
        self._send_group_message(group_code, message_chain.into(), None)
            .await
    }

    /// 发送群语音
    pub async fn send_group_audio(
        &self,
        group_code: i64,
        group_audio: GroupAudio,
    ) -> RQResult<MessageReceipt> {
        self._send_group_message(group_code, vec![], Some(group_audio.0))
            .await
    }

    async fn _send_group_message(
        &self,
        group_code: i64,
        elems: Vec<pb::msg::Elem>,
        ptt: Option<pb::msg::Ptt>,
    ) -> RQResult<MessageReceipt> {
        let time = chrono::Utc::now().timestamp();
        let ran = (rand::random::<u32>() >> 1) as i32;
        let (tx, rx) = tokio::sync::oneshot::channel();
        {
            self.receipt_waiters.lock().await.insert(ran, tx);
        }
        let req = self
            .engine
            .read()
            .await
            .build_group_sending_packet(group_code, elems, ptt, ran, time, 1, 0, 0, false);
        let _ = self.send_and_wait(req).await?;
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
        let members = self
            .get_group_member_list(group_code, group_info.owner_uin)
            .await?;
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
    pub async fn reload_groups(&self, buffered: usize) -> RQResult<()> {
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
                let mem_list = self
                    .get_group_member_list(g.code, g.owner_uin)
                    .await
                    .unwrap_or_default();
                (
                    g.code,
                    Arc::new(Group {
                        info: g,
                        members: RwLock::new(mem_list),
                    }),
                )
            })
            .buffered(buffered)
            .collect()
            .await;

        let mut groups = self.groups.write().await;
        groups.clear();
        groups.extend(group_list);
        Ok(())
    }

    /// 获取群成员列表 (low level api)
    async fn _get_group_member_list(
        &self,
        group_code: i64,
        next_uin: i64,
        group_owner_uin: i64,
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
            .decode_group_member_list_response(resp.body, group_owner_uin)
    }

    /// 获取群成员列表
    pub async fn get_group_member_list(
        &self,
        group_code: i64,
        group_owner_uin: i64,
    ) -> RQResult<Vec<GroupMemberInfo>> {
        let mut next_uin = 0;
        let mut list = Vec::new();
        loop {
            let mut resp = self
                ._get_group_member_list(group_code, next_uin, group_owner_uin)
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

    /// 标记群消息已读
    pub async fn mark_group_message_readed(&self, group_code: i64, seq: i32) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_msg_readed_packet(group_code, seq);
        let _ = self.send_and_wait(req).await?;
        Ok(())
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
    ///
    /// flag: true 设置管理员 false 取消管理员
    pub async fn group_set_admin(&self, group_code: i64, member: i64, flag: bool) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_group_admin_set_packet(group_code, member, flag);
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

    pub async fn group_quit(&self, group_code: i64) -> RQResult<()> {
        let req = self.engine.read().await.build_quit_group_packet(group_code);
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

    // 用 highway 上传群图片之前调用，获取 upload_key
    pub async fn get_group_image_store(
        &self,
        group_code: i64,
        image_info: &ImageInfo,
    ) -> RQResult<GroupImageStoreResp> {
        let req = self.engine.read().await.build_group_image_store_packet(
            group_code,
            image_info.filename.clone(),
            image_info.md5.clone(),
            image_info.size as u64,
            image_info.width,
            image_info.height,
            image_info.image_type as u32,
        );
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_group_image_store_response(resp.body)
    }

    pub async fn _upload_group_image(
        &self,
        upload_key: Vec<u8>,
        addr: std::net::SocketAddr,
        data: Vec<u8>,
    ) -> RQResult<()> {
        if self.highway_session.read().await.session_key.is_empty() {
            return Err(RQError::Other("highway_session_key is empty".into()));
        }
        self.highway_upload_bdh(
            addr,
            BdhInput {
                command_id: 2,
                body: data,
                ticket: upload_key,
                ext: vec![],
                encrypt: false,
                chunk_size: 256 * 1024,
                send_echo: true,
            },
        )
        .await?;
        Ok(())
    }

    /// 上传群图片
    pub async fn upload_group_image(&self, group_code: i64, data: Vec<u8>) -> RQResult<GroupImage> {
        let image_info = ImageInfo::try_new(&data)?;

        let image_store = self.get_group_image_store(group_code, &image_info).await?;
        let signature = self.highway_session.read().await.session_key.to_vec();
        let group_image = match image_store {
            GroupImageStoreResp::Exist { file_id, addrs } => image_info.into_group_image(
                file_id,
                addrs.first().cloned().unwrap_or_default(),
                signature,
            ),
            GroupImageStoreResp::NotExist {
                file_id,
                upload_key,
                mut upload_addrs,
            } => {
                let addr = upload_addrs
                    .pop()
                    .ok_or_else(|| RQError::Other("addrs is empty".into()))?;
                self._upload_group_image(upload_key, addr.clone().into(), data)
                    .await?;
                image_info.into_group_image(file_id, addr, signature)
            }
        };
        Ok(group_image)
    }

    /// 上传群音频 codec: 0-amr, 1-silk
    pub async fn upload_group_audio(
        &self,
        group_code: i64,
        data: Vec<u8>,
        codec: u32,
    ) -> RQResult<GroupAudio> {
        let md5 = md5::compute(&data).to_vec();
        let size = data.len();
        let ext = self.engine.read().await.build_group_try_up_ptt_req(
            group_code,
            md5.clone(),
            size as u64,
            codec,
            size as u32,
        );
        let addr = self
            .highway_addrs
            .read()
            .await
            .first()
            .cloned()
            .ok_or(RQError::Other("highway_addrs is empty".into()))?;
        let ticket = self
            .highway_session
            .read()
            .await
            .sig_session
            .clone()
            .to_vec();
        let resp = self
            .highway_upload_bdh(
                addr.into(),
                BdhInput {
                    command_id: 29,
                    body: data,
                    ticket,
                    ext: ext.to_vec(),
                    encrypt: false,
                    chunk_size: 256 * 1024,
                    send_echo: true,
                },
            )
            .await?;
        let file_key = self
            .engine
            .read()
            .await
            .decode_group_try_up_ptt_resp(resp)?;
        Ok(GroupAudio(pb::msg::Ptt {
            file_type: Some(4),
            src_uin: Some(self.uin().await),
            file_name: Some(format!("{}.amr", encode_hex(&md5))),
            file_md5: Some(md5),
            file_size: Some(size as i32),
            bool_valid: Some(true),
            pb_reserve: Some(vec![8, 0, 40, 0, 56, 0]),
            group_file_key: Some(file_key),
            ..Default::default()
        }))
    }

    pub async fn get_group_audio_url(
        &self,
        group_code: i64,
        audio: GroupAudio,
    ) -> RQResult<String> {
        let req = self.engine.read().await.build_group_ptt_down_req(
            group_code,
            audio
                .0
                .file_md5
                .ok_or_else(|| RQError::Other("file_md5 is none".into()))?,
        );
        let resp = self.send_and_wait(req).await?;
        self.engine.read().await.decode_group_ptt_down(resp.body)
    }

    /// 设置群精华消息
    pub async fn operate_group_essence(
        &self,
        group_code: i64,
        msg_seq: i32,
        msg_rand: i32,
        flag: bool,
    ) -> RQResult<pb::oidb::EacRspBody> {
        let req = self
            .engine
            .read()
            .await
            .build_essence_msg_operate_packet(group_code, msg_seq, msg_rand, flag);
        let resp = self.send_and_wait(req).await?;
        let decode = self
            .engine
            .read()
            .await
            .decode_essence_msg_response(resp.body)?;
        Ok(decode)
    }

    /// 发送群消息
    /// 仅在多张图片时需要，发送文字不需要
    pub async fn send_group_long_message(
        &self,
        group_code: i64,
        message_chain: MessageChain,
    ) -> RQResult<MessageReceipt> {
        let brief = "[图片][图片][图片]"; // TODO brief
        let res_id = self
            .upload_msgs(
                group_code,
                vec![MessageNode {
                    sender_id: self.uin().await,
                    time: chrono::Utc::now().timestamp() as i32,
                    sender_name: self.account_info.read().await.nickname.clone(),
                    elements: message_chain,
                }
                .into()],
                true,
            )
            .await?;
        let template=format!(
            "<?xml version='1.0' encoding='UTF-8' standalone='yes' ?><msg serviceID=\"35\" templateID=\"1\" action=\"viewMultiMsg\" brief=\"{}\" m_resid=\"{}\" m_fileName=\"{}\" sourceMsgId=\"0\" url=\"\" flag=\"3\" adverSign=\"0\" multiMsgFlag=\"1\"><item layout=\"1\"><title>{}</title><hr hidden=\"false\" style=\"0\" /><summary>点击查看完整消息</summary></item><source name=\"聊天记录\" icon=\"\" action=\"\" appid=\"-1\" /></msg>",
            brief,
            res_id,
            chrono::Utc::now().timestamp_millis(),
            brief);
        let mut chain = MessageChain::default();
        chain.push(RichMsg {
            service_id: 35,
            template1: template,
        });
        chain.0.extend(vec![
            pb::msg::elem::Elem::Text(pb::msg::Text {
                str: Some("你的QQ暂不支持查看[转发多条消息]，请期待后续版本。".into()),
                ..Default::default()
            }),
            pb::msg::elem::Elem::GeneralFlags(pb::msg::GeneralFlags {
                long_text_flag: Some(1),
                long_text_resid: Some(res_id),
                pendant_id: Some(0),
                pb_reserve: Some(vec![0x78, 0x00, 0xF8, 0x01, 0x00, 0xC8, 0x02, 0x00]), // TODO 15=73255?
                ..Default::default()
            }),
        ]);
        self._send_group_message(group_code, chain.into(), None)
            .await
    }

    /// 发送转发消息
    pub async fn send_group_forward_message(
        &self,
        group_code: i64,
        msgs: Vec<ForwardMessage>,
    ) -> RQResult<MessageReceipt> {
        let t_sum = msgs.len();
        let preview = gen_forward_preview(&msgs);
        let res_id = self.upload_msgs(group_code, msgs, false).await?;
        // TODO friend template?
        let template = format!(
            r##"<?xml version='1.0' encoding='UTF-8' standalone='yes' ?><msg serviceID="35" templateID="1" action="viewMultiMsg" brief="[聊天记录]" m_resid="{}" m_fileName="{}" tSum="{}" sourceMsgId="0" url="" flag="3" adverSign="0" multiMsgFlag="0"><item layout="1" advertiser_id="0" aid="0"><title size="34" maxLines="2" lineSpace="12">群聊的聊天记录</title>{}<hr hidden="false" style="0" /><summary size="26" color="#777777">查看{}条转发消息</summary></item><source name="聊天记录" icon="" action="" appid="-1" /></msg>"##,
            res_id,
            chrono::Utc::now().timestamp_millis(), // TODO m_filename?
            t_sum,
            preview,
            t_sum
        );
        let mut chain = MessageChain::default();
        chain.push(RichMsg {
            service_id: 35,
            template1: template,
        });
        chain
            .0
            .push(pb::msg::elem::Elem::GeneralFlags(pb::msg::GeneralFlags {
                pendant_id: Some(0),
                pb_reserve: Some(vec![0x78, 0x00, 0xF8, 0x01, 0x00, 0xC8, 0x02, 0x00]),
                ..Default::default()
            }));
        self._send_group_message(group_code, chain.into(), None)
            .await
    }
}
