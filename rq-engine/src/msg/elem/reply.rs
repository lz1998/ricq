use std::fmt;

use crate::pb::msg;

use super::super::MessageChain;

#[derive(Default, Debug, Clone)]
pub struct Reply {
    pub reply_seq: i32,
    pub sender: i64,
    pub group_id: i64,
    pub time: i32,
    pub elements: MessageChain,
}

impl Into<Vec<msg::elem::Elem>> for Reply {
    fn into(self) -> Vec<msg::elem::Elem> {
        vec![msg::elem::Elem::SrcMsg(msg::SourceMsg {
            orig_seqs: vec![self.reply_seq],
            sender_uin: Some(self.sender),
            time: Some(self.time),
            flag: Some(1),
            elems: self.elements.into(),
            rich_msg: Some(vec![]),
            pb_reserve: Some(vec![]),
            src_msg: Some(vec![]),
            troop_name: Some(vec![]),
            ..Default::default()
        })]
    }
}

impl From<msg::SourceMsg> for Reply {
    fn from(e: msg::SourceMsg) -> Self {
        Self {
            reply_seq: e.orig_seqs[0],
            time: e.time(),
            sender: e.sender_uin(),
            group_id: e.to_uin(),
            elements: MessageChain::from(e.elems),
        }
    }
}

impl fmt::Display for Reply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Reply: {}]", self.reply_seq)
    }
}
