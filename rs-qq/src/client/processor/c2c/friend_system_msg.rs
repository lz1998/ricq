use crate::client::event::FriendRequestEvent;
use crate::engine::command::profile_service::FriendSystemMessages;
use crate::handler::QEvent;
use crate::Client;
use std::sync::Arc;

impl Client {
    pub(crate) async fn process_friend_system_messages(
        self: &Arc<Self>,
        msgs: FriendSystemMessages,
    ) {
        for request in msgs.requests {
            self.handler
                .handle(QEvent::FriendRequest(FriendRequestEvent {
                    client: self.clone(),
                    request,
                }))
                .await;
        }
    }
}
