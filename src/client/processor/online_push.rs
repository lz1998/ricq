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
            let parts = builder.cache_get_or_set_with(div_seq, || Vec::new());
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
            None => todo!(), // TODO get group from server
        };
        if group.0.member_count == 0 {
            group
                .1
                .write()
                .await
                .append(&mut self.get_group_member_list(group.0.code).await?)
        }

        let anon_info = part
            .elems
            .iter()
            .find(|elem| matches!(elem.elem, Some(pb::msg::elem::Elem::AnonGroupMsg(..))))
            .map(|elem| {
                if let pb::msg::elem::Elem::AnonGroupMsg(anon_info) = elem.elem.as_ref().unwrap() {
                    Some(anon_info.clone())
                } else {
                    None
                }
            })
            .flatten();
        let sender = if let Some(anon_info) = anon_info {
            let anonymous_info: AnonymousInfo = anon_info.clone().into();
            Sender {
                uin: 80000000,
                nickname: anonymous_info.anonymous_nick.clone(),
                anonymous_info,
                is_friend: false,
                card_name: "".to_string(),
            }
        } else {
            let member = group
                .1
                .read()
                .await
                .iter()
                .find(|m| m.uin == part.from_uin)
                .unwrap()
                .clone(); // todo
            Sender {
                uin: member.uin,
                nickname: member.nickname.clone(),
                card_name: member.card_name.clone(),
                is_friend: self.find_friend(member.uin).await.is_some(),
                anonymous_info: AnonymousInfo::default(),
            }
        };

        let group_message = GroupMessageEvent {
            id: part.seq,
            group_code: group.0.code,
            group_name: group.0.name.clone(),
            sender,
            time: part.time,
            original_obj: part.clone(),
            elements: parse_elems(part.elems),
            internal_id: part.rand,
        };
        //todo extInfo
        //todo group_card_update
        //todo ptt
        return Ok(group_message);
    }
}
