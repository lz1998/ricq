use std::collections::HashMap;
use std::fmt::Write;

use crate::msg::MessageChain;
use crate::pb;

pub mod builder;
pub mod decoder;

pub enum ForwardMessage {
    Message(MessageNode),
    Forward(ForwardNode),
}

pub fn gen_forward_preview(messages: &[ForwardMessage]) -> String {
    let mut ret = String::new();
    for msg in messages.iter().take(4) {
        ret.push_str(r##"<title size="26" color="#777777" maxLines="4" lineSpace="12">"##);
        match msg {
            ForwardMessage::Message(v) => write!(&mut ret, "{}: {}", v.sender_name, v.elements),
            ForwardMessage::Forward(v) => write!(&mut ret, "{}: [转发消息]", v.sender_name),
        }
        .unwrap();
        ret.push_str("</title>");
    }
    ret
}

pub struct MessageNode {
    pub sender_id: i64,
    pub time: i32,
    pub sender_name: String,
    pub elements: MessageChain,
}

impl From<MessageNode> for ForwardMessage {
    fn from(n: MessageNode) -> Self {
        Self::Message(n)
    }
}

pub struct ForwardNode {
    pub sender_id: i64,
    pub time: i32,
    pub sender_name: String,
    pub nodes: Vec<ForwardMessage>,
}

impl From<ForwardNode> for ForwardMessage {
    fn from(f: ForwardNode) -> Self {
        Self::Forward(f)
    }
}

struct PackedMessage {
    pub filename: String,
    pub buffer: HashMap<String, Vec<pb::msg::Message>>,
}
