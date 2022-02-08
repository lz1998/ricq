use std::sync::Arc;

use rq_engine::command::profile_service::{JoinGroupRequest, NewFriendRequest};
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
    pub async fn accept(&self) -> RQResult<()> {
        let JoinGroupRequest {
            msg_seq,
            req_uin,
            group_code,
            invitor_uin,
            ..
        } = &self.request;
        self.client
            .solve_group_add_request(
                *msg_seq,
                *req_uin,
                *group_code,
                0,
                invitor_uin.is_some(),
                true,
                false,
                "".into(),
            )
            .await
    }

    pub async fn reject(&self, reason: Option<impl Into<String>>) -> RQResult<()> {
        let JoinGroupRequest {
            msg_seq,
            req_uin,
            group_code,
            invitor_uin,
            ..
        } = &self.request;
        self.client
            .solve_group_add_request(
                *msg_seq,
                *req_uin,
                *group_code,
                0,
                invitor_uin.is_some(),
                false,
                true,
                reason
                    .and_then(|f| Some(Into::<String>::into(f)))
                    .unwrap_or_default(),
            )
            .await
    }
}

#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct FriendRequestEvent {
    #[derivative(Debug = "ignore")]
    pub client: Arc<Client>,
    pub request: NewFriendRequest,
}

impl FriendRequestEvent {
    pub async fn accept(&self) -> RQResult<()> {
        self.client
            .solve_friend_request(self.request.msg_seq, self.request.req_uin, true)
            .await
    }

    pub async fn reject(&self) -> RQResult<()> {
        self.client
            .solve_friend_request(self.request.msg_seq, self.request.req_uin, false)
            .await
    }
}
