use std::sync::Arc;

use rq_engine::msg::MessageChain;
use rq_engine::structs::{PrivateAudio, PrivateAudioMessage, PrivateMessage};
use rq_engine::{pb, RQResult};

use crate::client::event::{PrivateAudioMessageEvent, PrivateMessageEvent};
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_private_message(
        self: &Arc<Self>,
        mut msg: pb::msg::Message,
    ) -> RQResult<()> {
        fn take_ptt(msg: &mut pb::msg::Message) -> Option<pb::msg::Ptt> {
            msg.body.as_mut()?.rich_text.as_mut()?.ptt.take()
        }
        if let Some(ptt) = take_ptt(&mut msg) {
            self.handler
                .handle(QEvent::PrivateAudioMessage(PrivateAudioMessageEvent {
                    client: self.clone(),
                    message: parse_private_audio_message(msg, ptt)?,
                }))
                .await;
            return Ok(());
        }

        let private_message = parse_private_message(msg)?;
        if private_message.from_uin == self.uin().await {
            // TODO dispatch self private message event
            // TODO swap friend seq
            return Ok(());
        }
        self.handler
            .handle(QEvent::PrivateMessage(PrivateMessageEvent {
                client: self.clone(),
                message: private_message,
            }))
            .await;
        Ok(())
    }
}

pub fn parse_private_message(msg: pb::msg::Message) -> RQResult<PrivateMessage> {
    let head = msg.head.unwrap();
    Ok(PrivateMessage {
        seqs: vec![head.msg_seq()],
        target: head.to_uin.unwrap(),
        time: head.msg_time.unwrap(),
        from_uin: head.from_uin.unwrap_or_default(),
        from_nick: head.from_nick.unwrap_or_default(),
        rands: vec![
            if let Some(attr) = &msg.body.as_ref().unwrap().rich_text.as_ref().unwrap().attr {
                attr.random()
            } else {
                0
            },
        ],
        elements: MessageChain::from(msg.body.unwrap().rich_text.unwrap().elems), // todo ptt
    })
}

pub fn parse_private_audio_message(
    msg: pb::msg::Message,
    ptt: pb::msg::Ptt,
) -> RQResult<PrivateAudioMessage> {
    let head = msg.head.unwrap();
    Ok(PrivateAudioMessage {
        seqs: vec![head.msg_seq()],
        target: head.to_uin.unwrap(),
        time: head.msg_time.unwrap(),
        from_uin: head.from_uin.unwrap_or_default(),
        from_nick: head.from_nick.unwrap_or_default(),
        rands: vec![
            if let Some(attr) = &msg.body.as_ref().unwrap().rich_text.as_ref().unwrap().attr {
                attr.random()
            } else {
                0
            },
        ],
        audio: PrivateAudio(ptt),
    })
}
