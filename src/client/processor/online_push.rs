use crate::client::income::decoder::online_push::GroupMessagePart;
use crate::client::Client;

impl Client {
    pub async fn process_group_message_part(&self, group_message_part: GroupMessagePart) {
        println!("{:?}", group_message_part);
        self.mark_group_message_readed(group_message_part.group_code, group_message_part.seq)
            .await
            .unwrap(); //todo
                       // TODO merge part and dispatch to handler
        self.handler
            .handle(crate::client::handler::Msgs::GroupMessage(
                group_message_part,
            ))
            .await
            .unwrap(); //todo
    }
}
