use std::sync::Arc;

use crate::engine::command::profile_service::GroupSystemMessages;

use crate::client::event::{GroupRequestEvent, SelfInvitedEvent};
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_group_system_messages(self: &Arc<Self>, msgs: GroupSystemMessages) {
        for request in msgs.self_invited.clone() {
            if self
                .self_invited_exists(request.msg_seq, request.msg_time)
                .await
            {
                continue;
            }
            self.handler
                .handle(QEvent::SelfInvited(SelfInvitedEvent {
                    client: self.clone(),
                    request,
                }))
                .await;
        }
        for request in msgs.join_group_requests.clone() {
            if self
                .join_group_request_exists(request.msg_seq, request.msg_time)
                .await
            {
                continue;
            }
            self.handler
                .handle(QEvent::GroupRequest(GroupRequestEvent {
                    client: self.clone(),
                    request,
                }))
                .await;
        }
        let mut cache = self.group_sys_message_cache.write().await;
        *cache = msgs
    }

    async fn self_invited_exists(&self, msg_seq: i64, msg_time: i64) -> bool {
        if self.start_time > msg_time as i32 {
            return true;
        }
        match self
            .group_sys_message_cache
            .read()
            .await
            .self_invited
            .iter()
            .find(|m| m.msg_seq == msg_seq)
        {
            None => false,
            Some(_) => true,
        }
    }

    async fn join_group_request_exists(&self, msg_seq: i64, msg_time: i64) -> bool {
        if self.start_time > msg_time as i32 {
            return true;
        }
        match self
            .group_sys_message_cache
            .read()
            .await
            .join_group_requests
            .iter()
            .find(|m| m.msg_seq == msg_seq)
        {
            None => false,
            Some(_) => true,
        }
    }
}
