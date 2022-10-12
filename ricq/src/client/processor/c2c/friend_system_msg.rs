use crate::client::event::NewFriendRequestEvent;
use crate::Client;
use ricq_core::command::profile_service::FriendSystemMessages;

impl<H: crate::handler::Handler + Send> Client<H> {
    pub(crate) async fn process_friend_system_messages(&self, msgs: FriendSystemMessages) {
        for request in msgs.requests {
            self.handler
                .handle_friend_request(NewFriendRequestEvent { 0: request })
                .await;
        }
    }
}
