use crate::client::{Client, OtherClientInfo};
use crate::client::errors::RQError;
// use crate::client::income::decoder::online_push::GroupMessagePart;

impl Client {
    pub async fn process_push_param(&self, other_clients: Vec<OtherClientInfo>) -> Result<(), RQError> {
        println!("{:?}", other_clients);
        // TODO merge part and dispatch to handler
        Ok(())
    }
}