use std::net::SocketAddr;
use std::sync::atomic::Ordering;

use bytes::Bytes;

use rq_engine::common::{group_code2uin, RQAddr};
use rq_engine::highway::BdhInput;
use rq_engine::structs::ForwardMessage;

use crate::engine::command::message_svc::MessageSyncResponse;
use crate::engine::command::oidb_svc::*;
use crate::engine::pb;
use crate::engine::structs::Status;
use crate::engine::structs::SummaryCardInfo;
use crate::jce::SvcDevLoginInfo;
use crate::{RQError, RQResult};

mod friend;
mod group;
mod login;

/// API
impl super::Client {
    /// 设置在线状态 TODO net_type
    pub async fn update_online_status<T>(&self, status: T) -> RQResult<()>
    where
        T: Into<Status>,
    {
        let status = status.into();
        if let Some(ref custom_status) = status.custom_status {
            if custom_status.wording.is_empty() || custom_status.wording.chars().count() > 4 {
                return Err(RQError::Other("invalid wording length".into()));
            }
        }
        let req = self.engine.read().await.build_set_online_status_packet(
            status.online_status,
            status.ext_online_status,
            status.custom_status,
        );
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 修改签名
    pub async fn update_signature(&self, signature: String) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_update_signature_packet(signature);
        let _ = self.send_and_wait(req).await?;
        Ok(())
    }

    /// 修改个人资料
    pub async fn update_profile_detail(&self, profile: ProfileDetailUpdate) -> RQResult<()> {
        let req = self
            .engine
            .read()
            .await
            .build_update_profile_detail_packet(profile);
        let _ = self.send_and_wait(req).await?;
        Ok(())
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

    /// 获取通过安全验证的设备
    pub async fn get_allowed_clients(&self) -> RQResult<Vec<SvcDevLoginInfo>> {
        let req = self.engine.read().await.build_device_list_request_packet();
        let resp = self.send_and_wait(req).await?;
        self.engine.read().await.decode_dev_list_response(resp.body)
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

    pub async fn send_like(&self, uin: i64, count: i32) -> RQResult<()> {
        let req = self.engine.read().await.build_send_like_packet(uin, count);
        let _ = self.send_and_wait(req).await?;
        Ok(())
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
        Ok(msgs)
    }

    // 获取名片信息
    pub async fn get_summary_info(&self, uin: i64) -> RQResult<SummaryCardInfo> {
        let req = self
            .engine
            .read()
            .await
            .build_summary_card_request_packet(uin);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_summary_card_response(resp.body)
    }

    // 准备上传消息，获取 ukey, resid, ip, port
    async fn multi_msg_apply_up(
        &self,
        dst_uin: i64,
        data: &[u8],
        is_long: bool,
    ) -> RQResult<pb::multimsg::MultiMsgApplyUpRsp> {
        let req = self.engine.read().await.build_multi_msg_apply_up_req(
            data.len() as i64,
            md5::compute(data).to_vec(),
            if is_long { 1 } else { 2 },
            dst_uin,
        );
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_multi_msg_apply_up_resp(resp.body)
    }

    // 上传长消息、转发消息 私聊未测试
    pub async fn upload_msgs(
        &self,
        group_code: i64,
        msgs: Vec<ForwardMessage>,
        is_long: bool,
    ) -> RQResult<String> {
        let data = self
            .engine
            .read()
            .await
            .calculate_validation_data(msgs, group_code);
        let rsp = self
            .multi_msg_apply_up(group_code2uin(group_code), &data, is_long)
            .await?;
        let resid = rsp.msg_resid;
        if self.highway_session.read().await.session_key.is_empty() {
            return Err(RQError::Other("highway_session_key is empty".into()));
        }
        let mut addrs: Vec<RQAddr> = rsp
            .uint32_up_ip
            .into_iter()
            .zip(rsp.uint32_up_port.into_iter())
            .map(|(ip, port)| RQAddr(ip as u32, port as u16))
            .collect();
        let addr = addrs
            .pop()
            .ok_or_else(|| RQError::Other("addr is none".into()))?;
        let body =
            self.engine
                .read()
                .await
                .build_long_req(group_code2uin(group_code), data, rsp.msg_ukey);
        self.highway_upload_bdh(
            addr.into(),
            BdhInput {
                command_id: 27,
                body,
                ticket: rsp.msg_sig,
                chunk_size: 8192 * 8,
                ..Default::default()
            },
        )
        .await?;
        Ok(resid)
    }

    // 获取转发消息下载地址和 key
    async fn multi_msg_apply_down(
        &self,
        res_id: String,
    ) -> RQResult<pb::multimsg::MultiMsgApplyDownRsp> {
        let req = self
            .engine
            .read()
            .await
            .build_multi_msg_apply_down_req(res_id);
        let resp = self.send_and_wait(req).await?;
        self.engine
            .read()
            .await
            .decode_multi_msg_apply_down_resp(resp.body)
    }

    pub async fn download_msgs(&self, res_id: String) -> RQResult<Vec<ForwardMessage>> {
        let mut resp = self.multi_msg_apply_down(res_id).await?;
        if resp.result != 0 {
            return Err(RQError::Other(format!(
                "multi_msg_apply_down result {}",
                resp.result
            )));
        }
        let prefix=if let Some(pb::multimsg::ExternMsg { channel_type }) = resp.msg_extern_info && channel_type == 2 {
            "https://ssl.htdata.qq.com".into()
        } else {
            let addr = SocketAddr::from(RQAddr(resp.down_ip.pop().ok_or_else(||RQError::Other("ip is empty".into()))?,resp.down_port.pop().ok_or_else(||RQError::Other("port is empty".into()))? as u16));
            format!("http://{}",addr.to_string())
        };
        let url = format!(
            "{}{}",
            prefix,
            String::from_utf8_lossy(&resp.thumb_down_para)
        );
        let encrypt_key = resp.msg_key;
        // TODO get data and decrypt
        // TODO decoder -> LongRspBody
        // TODO uncompress
        // TODO link message, convert to Vec<ForwardMessage>
        todo!()
    }
}
