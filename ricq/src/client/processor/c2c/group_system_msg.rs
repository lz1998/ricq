use std::sync::Arc;

use ricq_core::command::profile_service::GroupSystemMessages;

use crate::client::event::{JoinGroupRequestEvent, SelfInvitedEvent};
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
                    inner: request,
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
                .handle(QEvent::GroupRequest(JoinGroupRequestEvent {
                    client: self.clone(),
                    inner: request,
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
        self.group_sys_message_cache
            .read()
            .await
            .self_invited
            .iter()
            .any(|m| m.msg_seq == msg_seq)
    }

    async fn join_group_request_exists(&self, msg_seq: i64, msg_time: i64) -> bool {
        if self.start_time > msg_time as i32 {
            return true;
        }
        self.group_sys_message_cache
            .read()
            .await
            .join_group_requests
            .iter()
            .any(|m| m.msg_seq == msg_seq)
    }
}
