use cached::Cached;

use rq_engine::command::message_svc::MessageSyncResponse;

use crate::engine::*;
use crate::Client;
use crate::QEvent;
use crate::RQResult;

impl Client {
    pub(crate) async fn process_message_sync(&self, resp: MessageSyncResponse) -> RQResult<()> {
        {
            let mut engine = self.engine.write().await;
            engine.transport.sig.sync_cookie = resp.sync_cookie;
            engine.transport.sig.pub_account_cookie = resp.pub_account_cookie;
        }
        for msg in &resp.msgs {
            let head = msg.head.as_ref().unwrap();

            {
                // 消息去重
                let mut c2c_cache = self.c2c_cache.write().await;
                if let Some(_) = c2c_cache.cache_set(
                    (
                        head.from_uin(),
                        head.to_uin(),
                        head.msg_seq(),
                        head.msg_uid(),
                    ),
                    (),
                ) {
                    break;
                }
                if c2c_cache.cache_misses().unwrap_or_default() > 100 {
                    c2c_cache.flush();
                    c2c_cache.cache_reset_metrics();
                }
            }

            //todo

            match msg.head.as_ref().unwrap().msg_type() {
                9 | 10 | 31 | 79 | 97 | 120 | 132 | 133 | 166 | 167 => {
                    let private_message = self.parse_private_message(msg.clone()).await?;
                    self.handler
                        .handle(QEvent::PrivateMessage(private_message))
                        .await;
                }
                _ => tracing::warn!("unhandled sync message type"),
            }
        }
        let engine = self.engine.read().await;
        let pkt = engine.build_delete_message_request_packet(resp.msgs);
        let _ = self.send_and_wait(pkt).await?; // delete message
        if resp.sync_flag != 2 {
            tracing::debug!("continue sync with flag: {}", resp.sync_flag);
            let pkt = engine.build_get_message_request_packet(resp.sync_flag);
            let _ = self.send_and_wait(pkt).await?; // continue sync message
        }
        Ok(())
    }

    pub async fn parse_private_message(
        &self,
        msg: pb::msg::Message,
    ) -> RQResult<PrivateMessageEvent> {
        let head = msg.head.unwrap();
        let sender = match self.find_friend(head.from_uin.unwrap()).await {
            Some(friend) => Sender {
                uin: friend.uin,
                nickname: friend.nick.clone(),
                ..Default::default()
            },
            None => Sender {
                uin: head.from_uin.unwrap(),
                nickname: head.from_nick.as_ref().unwrap().clone(),
                ..Default::default()
            },
        };
        Ok(PrivateMessageEvent {
            id: head.msg_seq(),
            target: head.to_uin.unwrap(),
            time: head.msg_time.unwrap(),
            sender,
            self_id: self.uin().await,
            internal_id: if let Some(attr) =
                &msg.body.as_ref().unwrap().rich_text.as_ref().unwrap().attr
            {
                attr.random()
            } else {
                0
            },
            elements: parse_elems(msg.body.unwrap().rich_text.unwrap().elems), //ptt todo
        })
    }
}
