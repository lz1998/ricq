use crate::client::{Client, OtherClientInfo};
use crate::RQError;

// use crate::client::income::decoder::online_push::GroupMessagePart;

impl<H: crate::handler::Handler + Send> Client<H> {
    pub(crate) async fn process_push_param(
        &self,
        other_clients: Vec<OtherClientInfo>,
    ) -> Result<(), RQError> {
        tracing::debug!("{:?}", other_clients);
        // TODO merge part and dispatch to handler
        Ok(())
    }
}
