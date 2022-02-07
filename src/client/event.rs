use std::sync::Arc;

use rq_engine::command::profile_service::JoinGroupRequest;
use rq_engine::RQResult;

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

    pub async fn recall(&self) -> RQResult<()> {
        // TODO check permission
        self.client
            .recall_group_message(
                self.message.group_code,
                self.message.seqs.clone(),
                self.message.rands.clone(),
            )
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct PrivateMessageEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub message: PrivateMessage,
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct GroupRequestEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: JoinGroupRequest,
}

impl GroupRequestEvent {
    pub async fn accept(&self) {
        todo!()
    }
}
