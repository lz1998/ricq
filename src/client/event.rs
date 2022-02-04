use std::sync::Arc;

use rq_engine::structs::{GroupMessage, PrivateMessage};

use crate::Client;

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: GroupMessage,
}

impl GroupMessageEvent {
    pub fn group_name(&self) -> String {
        // lazy load
        todo!()
    }
    pub fn sender_nick(&self) -> String {
        // lazy load
        todo!()
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct PrivateMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: PrivateMessage,
}
