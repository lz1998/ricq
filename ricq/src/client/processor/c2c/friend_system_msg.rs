use crate::client::event::NewFriendRequestEvent;
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
                .handle_friend_request(NewFriendRequestEvent {
                    client: self.clone(),
                    inner: request,
                })
                .await;
        }
    }
}
