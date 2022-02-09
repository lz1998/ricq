use std::sync::Arc;

use rq_engine::command::profile_service::GroupSystemMessages;

use crate::client::event::{GroupRequestEvent, SelfInvitedEvent};
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub async fn process_group_system_messages(self: &Arc<Self>, msgs: GroupSystemMessages) {
        for request in msgs.self_invited {
            self.handler
                .handle(QEvent::SelfInvited(SelfInvitedEvent {
                    client: self.clone(),
                    request,
                }))
                .await;
        }
        for request in msgs.join_group_requests {
            self.handler
                .handle(QEvent::GroupRequest(GroupRequestEvent {
                    client: self.clone(),
                    request,
                }))
                .await;
        }
    }
}
