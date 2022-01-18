use crate::pb;
use bytes::Bytes;

pub mod builder;
pub mod decoder;

pub struct MessageSyncResponse {
    pub sync_flag: i32,
    pub sync_cookie: Bytes,
    pub pub_account_cookie: Bytes,
    pub msgs: Vec<pb::msg::Message>,
}
