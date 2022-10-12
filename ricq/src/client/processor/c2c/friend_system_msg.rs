use crate::client::event::NewFriendRequestEvent;
use crate::handler::RawHandler;
use crate::Client;
use ricq_core::command::profile_service::FriendSystemMessages;

impl<H: RawHandler> Client<H> {
    pub(crate) async fn process_friend_system_messages(&self, msgs: FriendSystemMessages) {
        for request in msgs.requests {
            self.handler
                .handle_friend_request(request)
                .await;
        }
    }
}
