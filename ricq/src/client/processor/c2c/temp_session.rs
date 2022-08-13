use std::sync::Arc;

use ricq_core::msg::MessageChain;
use ricq_core::structs::GroupTempMessage;
use ricq_core::{pb, RQError, RQResult};

use crate::client::event::GroupTempMessageEvent;
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_temp_message(
        self: &Arc<Self>,
        msg: pb::msg::Message,
    ) -> RQResult<()> {
        let message = parse_temp_message(msg)?;
        self.handler
            .handle(QEvent::GroupTempMessage(GroupTempMessageEvent {
                client: self.clone(),
                inner: message,
            }))
            .await;
        Ok(())
    }
}

pub fn parse_temp_message(msg: pb::msg::Message) -> RQResult<GroupTempMessage> {
    let head = msg.head.unwrap();
    let tmp_head = head
        .c2c_tmp_msg_head
        .ok_or(RQError::EmptyField("c2c_tmp_msg_head"))?;

    Ok(GroupTempMessage {
        seqs: vec![head.msg_seq.unwrap_or_default()],
        rands: vec![
            if let Some(attr) = &msg.body.as_ref().unwrap().rich_text.as_ref().unwrap().attr {
                attr.random()
            } else {
                0
            },
        ],
        time: head.msg_time.unwrap(),
        from_uin: head.from_uin.unwrap_or_default(),
        from_nick: head.from_nick.unwrap_or_default(),
        elements: MessageChain::from(msg.body.unwrap().rich_text.unwrap().elems), // todo ptt_store
        group_code: tmp_head.group_code.unwrap_or_default(),
    })
}
