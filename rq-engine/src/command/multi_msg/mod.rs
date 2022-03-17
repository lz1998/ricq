use crate::msg::MessageChain;

pub mod builder;
pub mod decoder;

pub struct MessageNode {
    pub sender_id: i64,
    pub time: i32,
    pub sender_name: String,
    pub elements: MessageChain,
}
