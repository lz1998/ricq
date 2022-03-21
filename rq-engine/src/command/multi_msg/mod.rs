use crate::msg::MessageChain;

pub mod builder;
pub mod decoder;

pub enum ForwardMessage {
    Message(MessageNode),
    Forward(ForwardNode),
}

pub struct MessageNode {
    pub sender_id: i64,
    pub time: i32,
    pub sender_name: String,
    pub elements: MessageChain,
}

pub struct ForwardNode {
    pub sender_id: i64,
    pub time: i32,
    pub sender_name: String,
    pub nodes: Vec<ForwardMessage>,
}
