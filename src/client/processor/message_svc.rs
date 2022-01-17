use rq_engine::{command::message_svc::MessageSyncResponse, RQResult};

use crate::Client;

impl Client {
    pub(crate) async fn process_message_sync(&self, resp: MessageSyncResponse) -> RQResult<()> {
        for msg in resp.msgs {
            let head = msg.head.unwrap();
            let str_msg = format!(
                "{}{}{}{}",
                head.from_uin.unwrap(),
                head.to_uin.unwrap(),
                head.msg_seq(),
                head.msg_uid()
            );
            if self.c2c_cache.update(&str_msg, 60 * 60).await {
                break;
            } else {
                self.c2c_cache.insert(str_msg, (), 60 * 60).await;
            }
            //todo
        }
        Ok(())
    }
}
