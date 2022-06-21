use crate::client::event::NewFriendRequestEvent;
use crate::handler::QEvent;
use crate::Client;
use ricq_core::command::profile_service::FriendSystemMessages;
use std::sync::Arc;

impl Client {
    pub(crate) async fn process_friend_system_messages(
        self: &Arc<Self>,
        msgs: FriendSystemMessages,
    ) {
        for request in msgs.requests {
            self.handler
                .handle(QEvent::NewFriendRequest(NewFriendRequestEvent {
                    client: self.clone(),
                    inner: request,
                }))
                .await;
        }
    }
}
