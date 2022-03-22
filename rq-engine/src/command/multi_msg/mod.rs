use std::collections::HashMap;

use crate::msg::MessageChain;
use crate::pb;

pub mod builder;
pub mod decoder;

pub enum ForwardMessage {
    Message(MessageNode),
    Forward(ForwardNode),
}

pub fn gen_forward_preview(messages: &Vec<ForwardMessage>) -> String {
    messages
        .iter()
        .take(4)
        .map(|n| match n {
            ForwardMessage::Message(message) => {
                format!(
                    r##"<title size="26" color="#777777" maxLines="4" lineSpace="12">{}: {}</title>"##,
                    message.sender_name,
                    message.elements.to_string()
                )
            }
            ForwardMessage::Forward(forward) => {
                format!(
                    r##"<title size="26" color="#777777" maxLines="4" lineSpace="12">{}: [转发消息]</title>"##,
                    forward.sender_name
                )
            }
        })
        .collect::<Vec<String>>()
        .join("")
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
