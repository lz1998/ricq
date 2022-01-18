use rq_engine::{command::message_svc::MessageSyncResponse, RQResult};

use crate::Client;

impl Client {
    pub(crate) async fn process_message_sync(&self, resp: MessageSyncResponse) -> RQResult<()> {
        {
            let mut engine = self.engine.write().await;
            engine.transport.sig.sync_cookie = resp.sync_cookie;
            engine.transport.sig.pub_account_cookie = resp.pub_account_cookie;
        }
        for msg in &resp.msgs {
            let head = msg.head.as_ref().unwrap();
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
        let engine = self.engine.read().await;
        let pkt = engine.build_delete_message_request_packet(resp.msgs);
        let _ = self.send_and_wait(pkt).await?; // delete message
        if resp.sync_flag != 2 {
            tracing::debug!("continue sync with flag: {}", resp.sync_flag);
            let pkt = engine.build_get_message_request_packet(resp.sync_flag);
            let _ = self.send_and_wait(pkt).await?; // continue sync message
        }
        Ok(())
    }
}
