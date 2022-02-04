use std::sync::Arc;

use crate::structs::{Group, GroupMemberInfo, GroupMessage, PrivateMessage};
use crate::Client;

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: GroupMessage,
}

impl GroupMessageEvent {
    pub async fn group(&self) -> Option<Arc<Group>> {
        self.client.find_group(self.message.group_code, true).await
    }

    pub async fn member(&self) -> Option<GroupMemberInfo> {
        let group = self.group().await?;
        let members = group.members.read().await;
        members
            .iter()
            .filter(|m| m.uin == self.message.from_uin)
            .last()
            .cloned()
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct PrivateMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: PrivateMessage,
}
