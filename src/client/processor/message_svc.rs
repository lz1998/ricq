use std::sync::Arc;

use cached::Cached;
use futures::{stream, StreamExt};

use rq_engine::command::message_svc::MessageSyncResponse;
use rq_engine::msg::MessageChain;
use rq_engine::pb;
use rq_engine::structs::PrivateMessage;

use crate::client::event::PrivateMessageEvent;
use crate::handler::QEvent;
use crate::Client;
use crate::RQResult;

impl Client {
    pub(crate) async fn process_message_sync(
        self: &Arc<Self>,
        resp: MessageSyncResponse,
    ) -> RQResult<()> {
        {
            let mut engine = self.engine.write().await;
            engine.transport.sig.sync_cookie = resp.sync_cookie;
            engine.transport.sig.pub_account_cookie = resp.pub_account_cookie;
        };

        self.delete_message(
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
        .await?;

        stream::iter(resp.msgs)
            .filter_map(|msg| async {
                let head = msg.head.clone().unwrap();
                if self.msg_exists(&head).await {
                    None
                } else {
                    Some(msg)
                }
            })
            .for_each(|msg| async {
                match msg.head.as_ref().unwrap().msg_type() {
                    9 | 10 | 31 | 79 | 97 | 120 | 132 | 133 | 166 | 167 => {
                        if let Ok(private_message) = self.parse_private_message(msg).await {
                            self.handler
                                .handle(QEvent::PrivateMessage(PrivateMessageEvent {
                                    client: self.clone(),
                                    message: private_message,
                                }))
                                .await
                        }
                    }
                    33 => {
                        // troop add member broadcast
                    }
                    140 | 141 => {
                        // temp session
                    }
                    208 => {
                        // private ptt
                    }
                    _ => tracing::warn!("unhandled sync message type"),
                }
            })
            .await;

        if resp.sync_flag != 2 {
            self.get_sync_message(resp.sync_flag).await?;
        }
        Ok(())
    }

    async fn msg_exists(&self, head: &pb::msg::MessageHead) -> bool {
        let now = chrono::Utc::now().timestamp() as i32;
        let msg_time = head.msg_time.unwrap_or_default();
        if now - msg_time > 60 || self.start_time > msg_time {
            return true;
        }
        let mut c2c_cache = self.c2c_cache.write().await;
        let key = (
            head.from_uin(),
            head.to_uin(),
            head.msg_seq(),
            head.msg_uid(),
        );
        if c2c_cache.cache_get(&key).is_some() {
            return true;
        }
        c2c_cache.cache_set(key, ());
        if c2c_cache.cache_misses().unwrap_or_default() > 100 {
            c2c_cache.flush();
            c2c_cache.cache_reset_metrics();
        }
        false
    }

    pub async fn parse_private_message(&self, msg: pb::msg::Message) -> RQResult<PrivateMessage> {
        let head = msg.head.unwrap();
        Ok(PrivateMessage {
            seqs: vec![head.msg_seq()],
            target: head.to_uin.unwrap(),
            time: head.msg_time.unwrap(),
            from_uin: head.from_uin.unwrap_or_default(),
            from_nick: head.from_nick.unwrap_or_default(),
            rands: vec![if let Some(attr) =
                &msg.body.as_ref().unwrap().rich_text.as_ref().unwrap().attr
            {
                attr.random()
            } else {
                0
            }],
            elements: MessageChain::from(msg.body.unwrap().rich_text.unwrap().elems), // todo ptt
        })
    }
}
