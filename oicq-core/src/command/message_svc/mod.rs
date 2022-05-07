use crate::pb;

pub mod builder;
pub mod decoder;

pub struct MessageSyncResponse {
    pub msg_rsp_type: i32,
    pub sync_flag: i32,
    pub sync_cookie: Option<Vec<u8>>,
    pub pub_account_cookie: Option<Vec<u8>>,
    pub msgs: Vec<pb::msg::Message>,
}
