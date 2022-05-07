use crate::client::{Client, OtherClientInfo};
use crate::RQError;

// use crate::client::income::decoder::online_push::GroupMessagePart;

impl Client {
    pub(crate) async fn process_push_param(
        &self,
        other_clients: Vec<OtherClientInfo>,
    ) -> Result<(), RQError> {
        tracing::debug!(target = "rs_qq", "{:?}", other_clients);
        // TODO merge part and dispatch to handler
        Ok(())
    }
}
