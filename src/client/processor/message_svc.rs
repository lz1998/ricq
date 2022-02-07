use std::sync::Arc;

use cached::Cached;
use futures::{stream, StreamExt};

use rq_engine::pb;

use crate::Client;
use crate::RQResult;

impl Client {
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
                            tracing::warn!(target: "rs_qq", "failed to process private message {}",err);
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
