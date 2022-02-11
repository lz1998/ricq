use std::sync::Arc;

use cached::Cached;
use futures::{stream, StreamExt};

use rq_engine::{jce, pb};

use crate::Client;
use crate::RQResult;

impl Client {
    pub(crate) async fn process_push_notify(
        self: &Arc<Self>,
        notify: Option<jce::RequestPushNotify>,
    ) {
        if let Some(notify) = notify {
            match notify.msg_type {
                35 | 36 | 37 | 45 | 46 | 84 | 85 | 86 | 87 => {
                    // pull group system msg(group request), then process
                    match self.get_all_group_system_messages().await {
                        Ok(msgs) => {
                            self.process_group_system_messages(msgs).await;
                        }
                        Err(err) => {
                            tracing::warn!("failed to get group system message {}", err);
                        }
                    }
                }
                187 | 188 | 189 | 190 | 191 => {
                    // pull friend system msg(friend request), then process
                    match self.get_friend_system_messages().await {
                        Ok(msgs) => {
                            self.process_friend_system_messages(msgs).await;
                        }
                        Err(err) => {
                            tracing::warn!("failed to get friend system message {}", err);
                        }
                    }
                }
                _ => {
                    // TODO tracing.warn!()
                }
            }
        }
        // pull private msg and other, then process
        let all_message = self.sync_all_message().await;
        match all_message {
            Ok(msgs) => {
                if let Err(err) = self.process_message_sync(msgs).await {
                    tracing::error!(target: "rs_qq", "process message sync error: {:?}",err);
                }
            }
            Err(err) => {
                tracing::warn!("failed to sync message {}", err);
            }
        }
    }

    pub(crate) async fn process_message_sync(
        self: &Arc<Self>,
        msgs: Vec<pb::msg::Message>,
    ) -> RQResult<()> {
        stream::iter(msgs)
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
                        if let Err(err)=self.process_private_message(msg).await{
                            tracing::error!(target: "rs_qq", "failed to process private message {}",err);
                        }
                    }
                    33 => {
                        if let Err(err)=self.process_join_group(msg).await{
                            tracing::error!(target: "rs_qq", "failed to process join group {}",err);
                        }
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
}
