use cached::Cached;
use std::sync::Arc;

use ricq_core::msg::MessageChain;
use ricq_core::structs::{FriendAudio, FriendAudioMessage, FriendMessage};
use ricq_core::{pb, RQResult};

use crate::client::event::{FriendAudioMessageEvent, FriendMessageEvent};
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_friend_message(
        self: &Arc<Self>,
        mut msg: pb::msg::Message,
    ) -> RQResult<()> {
        fn take_ptt(msg: &mut pb::msg::Message) -> Option<pb::msg::Ptt> {
            msg.body.as_mut()?.rich_text.as_mut()?.ptt.take()
        }
        if let Some(ptt) = take_ptt(&mut msg) {
            // TODO self friend audio
            self.handler
                .handle(QEvent::FriendAudioMessage(FriendAudioMessageEvent {
                    client: self.clone(),
                    inner: parse_friend_audio_message(msg, ptt)?,
                }))
                .await;
            return Ok(());
        }

        let message = parse_friend_message(msg)?;
        if message.from_uin == self.uin().await {
            if let Some(tx) = self
                .receipt_waiters
                .lock()
                .await
                .cache_remove(&message.rands.get(0).cloned().unwrap_or_default())
            {
                let _ = tx.send(message.seqs.get(0).cloned().unwrap_or_default());
                return Ok(());
            }
        }
        self.handler
            .handle(QEvent::FriendMessage(FriendMessageEvent {
                client: self.clone(),
                inner: message,
            }))
            .await;
        Ok(())
    }
}

pub fn parse_friend_message(msg: pb::msg::Message) -> RQResult<FriendMessage> {
    let head = msg.head.unwrap();
    Ok(FriendMessage {
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
        elements: MessageChain::from(msg.body.unwrap().rich_text.unwrap().elems), // todo ptt_store
    })
}

pub fn parse_friend_audio_message(
    msg: pb::msg::Message,
    ptt: pb::msg::Ptt,
) -> RQResult<FriendAudioMessage> {
    let head = msg.head.unwrap();
    Ok(FriendAudioMessage {
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
        audio: FriendAudio(ptt),
    })
}
