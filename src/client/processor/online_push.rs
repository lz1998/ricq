use crate::client::handler::Msgs;
use crate::client::income::{builder::GroupMessageBuilder, decoder::online_push::GroupMessagePart};
use crate::client::messages::GroupMessage;
use crate::client::Client;
use std::sync::atomic::Ordering;

impl Client {
    pub async fn process_group_message_part(&self, group_message_part: GroupMessagePart) {
        println!("{:?}", group_message_part);
        self.mark_group_message_readed(group_message_part.group_code, group_message_part.seq)
            .await
            .unwrap(); //todo

        // receipt message
        if group_message_part.from_uin == self.uin.load(Ordering::SeqCst) {
            //todo
            todo!();
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
                .handle(Msgs::GroupMessage(
                    self.parse_group_message(group_msg).await,
                ))
                .await
                .unwrap(); //todo
        }
    }

    pub(crate) async fn parse_group_message(&self, part: GroupMessagePart) -> GroupMessage {
        let group = match self.find_group(part.group_code).await {
            Some(group) => group,
            None => {
                // self.get_group_info(part.group_code).await;
                todo!();
            }
        };
        if group.member_count == 0 {
            todo!();
        }

        todo!()
    }
}
