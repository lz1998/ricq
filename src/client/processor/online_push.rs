use crate::client::handler::QEvent;
use crate::client::income::builder::GroupMessageBuilder;
use crate::client::msg::*;
use crate::client::Client;
use crate::engine::command::online_push::GroupMessagePart;
use crate::{RQError, RQResult};

impl Client {
    pub async fn process_group_message_part(
        &self,
        group_message_part: GroupMessagePart,
    ) -> Result<(), RQError> {
        // self.mark_group_message_readed(group_message_part.group_code, group_message_part.seq).await;

        // receipt message
        if group_message_part.from_uin == self.uin().await {
            self.handler
                .handle(QEvent::GroupMessageReceipt(GroupMessageReceiptEvent {
                    rand: group_message_part.rand,
                    seq: group_message_part.seq,
                    msg_event: self.parse_group_message(group_message_part).await?,
                }))
                .await;
            return Ok(());
        }

        // merge parts
        let div_seq = group_message_part.div_seq;
        let group_msg = if group_message_part.pkg_num > 1 {
            // muti-part
            let mut map = self.group_message_builder.write().await;
            let build_result = match map.remove(&div_seq) {
                Some(builder) => builder.join(group_message_part), // have previous part
                None => Err(GroupMessageBuilder::new(group_message_part)), // the first part
            };
            match build_result {
                Ok(group_message) => Some(group_message), // message is finish
                Err(builder) => {
                    // message is not finish
                    map.insert(div_seq, builder);
                    None
                }
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
            None => self.get_group(part.group_code).await.unwrap(), // TODO remove unwrap
        };
        if group.0.member_count == 0 {
            group.1.write().await.append(
                &mut self
                    .get_group_member_list(group.0.code, group.0.uin)
                    .await?,
            )
        }

        let anon_info = part
            .elems
            .iter()
            .find(|elem| elem.anon_group_msg.is_some())
            .map(|e| e.anon_group_msg.as_ref().unwrap());
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
