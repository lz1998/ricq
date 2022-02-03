use std::sync::Arc;

use cached::Cached;

use crate::client::handler::QEvent;
use crate::client::Client;
use crate::engine::command::online_push::GroupMessagePart;
use crate::engine::*;
use crate::{RQError, RQResult};

impl Client {
    pub async fn process_group_message_part(
        self: &Arc<Self>,
        group_message_part: GroupMessagePart,
    ) -> Result<(), RQError> {
        // self.mark_group_message_readed(group_message_part.group_code, group_message_part.seq).await;

        // receipt message
        if group_message_part.from_uin == self.uin().await {
            if let Some(tx) = self
                .receipt_waiters
                .lock()
                .await
                .remove(&group_message_part.seq)
            {
                let _ = tx.send(group_message_part.seq);
            }
            return Ok(());
        }

        // merge parts
        let pkg_num = group_message_part.pkg_num;
        let group_msg = if pkg_num > 1 {
            let mut builder = self.group_message_builder.write().await;
            if builder.cache_misses().unwrap_or_default() > 100 {
                builder.flush();
                builder.cache_reset_metrics();
            }
            // muti-part
            let div_seq = group_message_part.div_seq;
            let parts = builder.cache_get_or_set_with(div_seq, Vec::new);
            parts.push(group_message_part);
            if parts.len() < pkg_num as usize {
                // wait for more parts
                None
            } else {
                let mut parts = builder.cache_remove(&div_seq).unwrap_or_default();
                parts.sort_by(|a, b| a.pkg_index.cmp(&b.pkg_index));
                let mut merged = parts.remove(0);
                for part in parts.into_iter() {
                    merged.elems.extend(part.elems);
                }
                Some(merged)
            }
        } else {
            // single-part
            Some(group_message_part)
        };

        // handle message
        if let Some(group_msg) = group_msg {
            // message is finish
            self.handler
                .handle(QEvent::GroupMessage(
                    self.clone(),
                    self.parse_group_message(group_msg).await?,
                ))
                .await; //todo
        }
        Ok(())
    }

    pub(crate) async fn parse_group_message(
        &self,
        part: GroupMessagePart,
    ) -> RQResult<GroupMessageEvent> {
        let group = match self.find_group(part.group_code).await {
            Some(group) => group,
            None => return Err(RQError::Other("TODO: load group from server".into())), // TODO get group from server
        };
        if group.info.member_count == 0 {
            group
                .members
                .write()
                .await
                .append(&mut self.get_group_member_list(group.info.code).await?)
        }

        let group_message = GroupMessageEvent {
            id: part.seq,
            group_code: group.info.code,
            group_name: group.info.name.clone(),
            from_uin: part.from_uin,
            time: part.time,
            original_obj: part.clone(),
            elements: MessageChain::from(part.elems),
            internal_id: part.rand,
        };
        //todo extInfo
        //todo group_card_update
        //todo ptt
        Ok(group_message)
    }
}
