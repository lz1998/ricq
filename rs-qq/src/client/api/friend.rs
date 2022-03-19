use std::sync::Arc;
use std::time::Duration;

use bytes::BufMut;

use crate::engine::command::long_conn::OffPicUpResp;
use crate::engine::command::oidb_svc::music::{MusicShare, MusicType, SendMusicTarget};
use crate::engine::command::{friendlist::*, profile_service::*};
use crate::engine::hex::encode_hex;
use crate::engine::highway::BdhInput;
use crate::engine::msg::elem::FriendImage;
use crate::engine::msg::MessageChain;
use crate::engine::pb;
use crate::engine::structs::FriendAudio;
use crate::engine::structs::{FriendInfo, MessageReceipt};
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
        let time = chrono::Utc::now().timestamp();
        let seq = self.engine.read().await.next_friend_seq();
        let ran = (rand::random::<u32>() >> 1) as i32;
        let req = self.engine.read().await.build_friend_sending_packet(
            target,
            message_chain.into(),
            ptt,
            seq,
            ran,
            time,
            1,
            0,
            0,
        );
        let _ = self.send_and_wait(req).await?;
        Ok(MessageReceipt {
            seqs: vec![seq],
            rands: vec![ran],
            time,
        })
    }

    pub async fn upload_friend_image(&self, target: i64, data: Vec<u8>) -> RQResult<FriendImage> {
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
                let addr = upload_addrs
                    .pop()
                    .ok_or_else(|| RQError::Other("addrs is empty".into()))?;
                self._upload_friend_image(upload_key, addr.clone().into(), data)
                    .await?;
                image_info.into_friend_image(res_id, uuid)
            }
        };
        Ok(friend_image)
    }

    pub async fn _upload_friend_image(
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
                command_id: 1,
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
        data: Vec<u8>,
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
                    command_id: 26,
                    body: data,
                    ticket,
                    ext: ext.to_vec(),
                    encrypt: false,
                    chunk_size: 256 * 1024,
                    send_echo: true,
                },
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
            audio
                .0
                .file_uuid
                .ok_or_else(|| RQError::Other("file_uuid is none".into()))?,
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
}
