use std::fmt;

use crate::msg::{MessageChainBuilder, MessageElem, PushBuilder};
use crate::pb::msg;

use super::super::MessageChain;

#[derive(Default, Debug, Clone)]
pub struct Reply {
    pub reply_seq: i32,
    pub sender: i64,
    pub time: i32,
    pub elements: MessageChain,
}

impl From<Reply> for MessageElem {
    fn from(e: Reply) -> Self {
        MessageElem::SrcMsg(msg::SourceMsg {
            orig_seqs: vec![e.reply_seq],
            sender_uin: Some(e.sender),
            time: Some(e.time),
            flag: Some(1),
            elems: e.elements.into(),
            rich_msg: Some(vec![]),
            pb_reserve: Some(vec![]),
            src_msg: Some(vec![]),
            troop_name: Some(vec![]),
            ..Default::default()
        })
    }
}

impl PushBuilder for Reply {
    fn push_builder(elem: Self, builder: &mut MessageChainBuilder) {
        let index = if let Some(MessageElem::AnonGroupMsg(..)) = builder.elems.get(0) {
            1
        } else {
            0
        };
        builder.elems.insert(index, elem.into());
    }
}

impl From<msg::SourceMsg> for Reply {
    fn from(e: msg::SourceMsg) -> Self {
        Self {
            reply_seq: e.orig_seqs[0],
            time: e.time(),
            sender: e.sender_uin(),
            elements: MessageChain::from(e.elems),
        }
    }
}

impl fmt::Display for Reply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Reply: {}]", self.reply_seq)
    }
}
