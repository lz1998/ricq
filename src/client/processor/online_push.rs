use crate::client::Client;
use crate::client::income::decoder::online_push::GroupMessagePart;

impl Client {
    pub async fn process_group_message_part(&self, group_message_part: GroupMessagePart) {
        println!("{:?}", group_message_part);
        self.mark_group_message_readed(group_message_part.group_code, group_message_part.seq).await;
        // TODO merge part and dispatch to handler
    }
}