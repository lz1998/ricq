use std::sync::Arc;

use rq_engine::command::profile_service::GroupSystemMessages;

use crate::client::event::GroupRequestEvent;
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub async fn process_group_system_messages(self: &Arc<Self>, msgs: GroupSystemMessages) {
        let join_group_requests = msgs.join_group_requests;
        let self_invited = msgs.self_invited;
        for request in join_group_requests {
            self.handler
                .handle(QEvent::GroupRequest(GroupRequestEvent {
                    client: self.clone(),
                    request,
                }))
                .await;
        }
        // TODO dispatch self invited event
    }
}
