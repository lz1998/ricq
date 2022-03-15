use std::sync::Arc;

use crate::client::event::TempMessageEvent;
use crate::engine::msg::MessageChain;
use crate::engine::structs::TempMessage;
use crate::engine::{pb, RQError, RQResult};
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_temp_message(
        self: &Arc<Self>,
        msg: pb::msg::Message,
    ) -> RQResult<()> {
        let message = parse_temp_message(msg)?;
        if message.from_uin == self.uin().await {
            // TODO dispatch self temp message event
            // TODO swap friend seq
            return Ok(());
        }
        self.handler
            .handle(QEvent::TempMessage(TempMessageEvent {
                client: self.clone(),
                message,
            }))
            .await;
        Ok(())
    }
}

pub fn parse_temp_message(msg: pb::msg::Message) -> RQResult<TempMessage> {
    let head = msg.head.unwrap();
    let tmp_head = head
        .c2c_tmp_msg_head
        .ok_or_else(|| RQError::Other("tmp head is none".into()))?;

    Ok(TempMessage {
        seqs: vec![head.msg_seq.unwrap_or_default()],
        time: head.msg_time.unwrap(),
        from_uin: head.from_uin.unwrap_or_default(),
        from_nick: head.from_nick.unwrap_or_default(),
        elements: MessageChain::from(msg.body.unwrap().rich_text.unwrap().elems), // todo ptt
        group_code: tmp_head.group_code,
        sig: tmp_head.sig,
        service_type: tmp_head.service_type.unwrap_or_default(),
    })
}
