use crate::msg::{MessageChainBuilder, MessageElem, PushBuilder};
use crate::pb::msg;
use crate::pb::msg::AnonymousGroupMessage;

#[derive(Default, Debug, Clone)]
pub struct Anonymous {
    // 用于禁言
    pub anon_id: Vec<u8>,
    pub nick: String,
    pub portrait_index: i32,
    pub bubble_index: i32,
    pub expire_time: i32,
    pub color: String,
}

impl From<Anonymous> for MessageElem {
    fn from(e: Anonymous) -> Self {
        MessageElem::AnonGroupMsg(msg::AnonymousGroupMessage {
            flags: Some(2),
            anon_id: None,
            anon_nick: Some(e.nick.into_bytes()),
            head_portrait: Some(e.portrait_index),
            expire_time: Some(e.expire_time),
            bubble_id: Some(e.bubble_index),
            rank_color: Some(e.color.into_bytes()),
        })
    }
}

impl PushBuilder for Anonymous {
    fn push_builder(elem: Self, builder: &mut MessageChainBuilder) {
        builder.elems.insert(0, elem.into());
    }
}

impl From<msg::AnonymousGroupMessage> for Anonymous {
    fn from(e: AnonymousGroupMessage) -> Self {
        Self {
            anon_id: e.anon_id.unwrap_or_default(),
            nick: String::from_utf8_lossy(&e.anon_nick.unwrap_or_default()).into_owned(),
            portrait_index: e.head_portrait.unwrap_or_default(),
            bubble_index: e.bubble_id.unwrap_or_default(),
            expire_time: e.expire_time.unwrap_or_default(),
            color: String::from_utf8_lossy(&e.rank_color.unwrap_or_default()).into_owned(),
        }
    }
}
