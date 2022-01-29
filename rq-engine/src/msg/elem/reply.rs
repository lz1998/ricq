use super::super::MessageChain;
use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct Reply {
    pub reply_seq: i32,
    pub sender: i64,
    pub group_id: i64,
    pub time: i32,
    pub elements: MessageChain,
}

impl Into<msg::Elem> for Reply {
    fn into(self) -> msg::Elem {
        msg::Elem {
            elem: Some(msg::elem::Elem::SrcMsg(msg::SourceMsg {
                orig_seqs: vec![self.reply_seq],
                sender_uin: Some(self.sender),
                time: Some(self.time),
                flag: Some(1),
                elems: self.elements.0,
                rich_msg: Some(vec![]),
                pb_reserve: Some(vec![]),
                src_msg: Some(vec![]),
                troop_name: Some(vec![]),
                ..Default::default()
            })),
        }
    }
}

impl From<msg::SourceMsg> for Reply {
    fn from(e: msg::SourceMsg) -> Self {
        Self {
            reply_seq: e.orig_seqs[0],
            time: e.time(),
            sender: e.sender_uin(),
            group_id: e.to_uin(),
            elements: MessageChain(e.elems),
        }
    }
}
