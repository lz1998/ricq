use std::time::Duration;

use bytes::BufMut;

use ricq_core::command::long_conn::OffPicUpResp;
use ricq_core::command::oidb_svc::{LinkShare, MusicShare, MusicVersion, ShareTarget};
use ricq_core::command::{friendlist::*, profile_service::*};
use ricq_core::hex::encode_hex;
use ricq_core::highway::BdhInput;
use ricq_core::msg::elem::FriendImage;
use ricq_core::msg::MessageChain;
use ricq_core::pb;
use ricq_core::pb::msg::routing_head::RoutingHead;
use ricq_core::structs::FriendAudio;
use ricq_core::structs::MessageReceipt;

use crate::structs::ImageInfo;
use crate::{RQError, RQResult};

impl super::super::Client {
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
    pub async fn _get_friend_list(
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

    /// 刷新好友列表
    pub async fn get_friend_list(&self) -> RQResult<FriendListResponse> {
        let mut output = FriendListResponse::default();
        loop {
            let resp = self
                ._get_friend_list(output.friends.len() as i16, 150, 0, 0)
                .await?;
            output.friend_groups.extend(resp.friend_groups);
            output.friends.extend(resp.friends);
            output.total_count = resp.total_count;
            if output.friends.len() as i16 >= resp.total_count {
                break;
            }
        }
        Ok(output)
    }

    /// 好友列表-添加好友分组
    pub async fn friend_list_add_group(&self, sort_id: u8, group_name: String) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_friend_list_add_group_req_packet(sort_id, &group_name);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 好友列表-重命名好友分组
    pub async fn friend_list_rename_group(&self, group_id: u8, group_name: String) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_friend_list_rename_group_req_packet(group_id, &group_name);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 好友列表-删除好友分组
    pub async fn friend_list_del_group(&self, group_id: u8) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_friend_list_del_group_req_packet(group_id);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 好友戳一戳
    pub async fn friend_poke(&self, target: i64) -> RQResult<()> {
        let req = self.engine.read().await.build_friend_poke_packet(target);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 发送好友消息
    pub async fn send_friend_message(
        &self,
        target: i64,
        message_chain: MessageChain,
    ) -> RQResult<MessageReceipt> {
        self._send_friend_message(target, message_chain, None).await
    }

    /// 发送好友语音
    pub async fn send_friend_audio(
        &self,
        target: i64,
        audio: FriendAudio,
    ) -> RQResult<MessageReceipt> {
        self._send_friend_message(target, MessageChain::default(), Some(audio.0))
            .await
    }

    async fn _send_friend_message(
        &self,
        target: i64,
        message_chain: MessageChain,
        ptt: Option<pb::msg::Ptt>,
    ) -> RQResult<MessageReceipt> {
        self.send_message(
            RoutingHead::C2c(pb::msg::C2c {
                to_uin: Some(target),
            }),
            message_chain,
            ptt,
        )
        .await
    }

    pub async fn upload_friend_image(&self, target: i64, data: &[u8]) -> RQResult<FriendImage> {
        let image_info = ImageInfo::try_new(&data)?;
        let image_store = self.get_off_pic_store(target, &image_info).await?;

        let friend_image = match image_store {
            OffPicUpResp::Exist { res_id, uuid } => image_info.into_friend_image(res_id, uuid),
            OffPicUpResp::UploadRequired {
                res_id,
                uuid,
                upload_key,
                mut upload_addrs,
            } => {
                let addr = match self.highway_addrs.read().await.first() {
                    Some(addr) => *addr,
                    None => upload_addrs
                        .pop()
                        .ok_or(RQError::EmptyField("upload_addrs"))?,
                };
                self.highway_upload_bdh(
                    addr.into(),
                    BdhInput {
                        command_id: 1,
                        ticket: upload_key,
                        ext: vec![],
                        encrypt: false,
                        chunk_size: 256 * 1024,
                        send_echo: true,
                    },
                    data,
                )
                .await?;
                image_info.into_friend_image(res_id, uuid)
            }
        };
        Ok(friend_image)
    }

    pub async fn get_off_pic_store(
        &self,
        target: i64,
        image_info: &ImageInfo,
    ) -> RQResult<OffPicUpResp> {
        let req = self.engine.read().await.build_off_pic_up_packet(
            target,
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
            .decode_off_pic_up_response(resp.body)
    }

    /// 分享好友音乐
    pub async fn send_friend_music_share(
        &self,
        uin: i64,
        music_share: MusicShare,
        music_version: MusicVersion,
    ) -> RQResult<()> {
        let req = self.engine.read().await.build_share_music_request_packet(
            ShareTarget::Friend(uin),
            music_share,
            music_version,
        );
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 分享链接
    pub async fn send_friend_link_share(&self, uin: i64, link_share: LinkShare) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_share_link_request_packet(ShareTarget::Friend(uin), link_share);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    // 撤回好友消息
    pub async fn recall_friend_message(
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
            .build_friend_recall_packet(uin, msg_time, seqs, rands);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    pub async fn upload_friend_audio(
        &self,
        target: i64,
        data: &[u8],
        audio_duration: Duration,
    ) -> RQResult<FriendAudio> {
        let md5 = md5::compute(&data).to_vec();
        let size = data.len();
        let ext = self.engine.read().await.build_friend_try_up_ptt_req(
            target,
            md5.clone(),
            size as i64,
            size as i32,
        );
        let addr = self
            .highway_addrs
            .read()
            .await
            .first()
            .copied()
            .ok_or(RQError::EmptyField("highway_addrs"))?;
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
                    command_id: 26,
                    ticket,
                    ext: ext.to_vec(),
                    encrypt: false,
                    chunk_size: 256 * 1024,
                    send_echo: true,
                },
                data,
            )
            .await?;
        let uuid = self
            .engine
            .read()
            .await
            .decode_friend_try_up_ptt_resp(resp)?;
        Ok(FriendAudio(pb::msg::Ptt {
            file_type: Some(4),
            src_uin: Some(self.uin().await),
            file_uuid: Some(uuid),
            file_name: Some(format!("{}.amr", encode_hex(&md5))),
            file_md5: Some(md5),
            file_size: Some(size as i32),
            reserve: Some({
                let mut w = Vec::new();
                w.put_u8(3); // tlv count
                {
                    w.put_u8(8);
                    w.put_u16(4);
                    w.put_u32(1); // codec
                }
                {
                    w.put_u8(9);
                    w.put_u16(4);
                    w.put_u32(audio_duration.as_secs() as u32); // voiceLength
                }
                {
                    w.put_u8(10);
                    w.put_u16(6);
                    w.put_slice(&[0x08, 0x00, 0x28, 0x00, 0x38, 0x00]); // change_voice+redpack_type+autototext_voice
                }
                w
            }),
            bool_valid: Some(true),
            ..Default::default()
        }))
    }

    pub async fn get_friend_audio_url(
        &self,
        sender_uin: i64,
        audio: FriendAudio,
    ) -> RQResult<String> {
        let req = self.engine.read().await.build_c2c_ptt_down_req(
            sender_uin,
            audio.0.file_uuid.ok_or(RQError::EmptyField("file_uuid"))?,
        );
        let resp = self.send_and_wait(req).await?;
        self.engine.read().await.decode_c2c_ptt_down(resp.body)
    }

    /// 标记私聊消息已读 TODO 待测试
    pub async fn mark_friend_message_readed(&self, uin: i64, time: i64) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_friend_msg_readed_packet(uin, time);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 获取好友个性签名
    pub async fn get_friend_rich_sig(&self, user_ids: Vec<i64>) -> RQResult<Vec<RichSigInfo>> {
        let req = self
            .engine
            .read()
            .await
            .build_get_rich_sig_request_packet(user_ids);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_get_rich_sig_response_packet(resp.body)
    }
}
