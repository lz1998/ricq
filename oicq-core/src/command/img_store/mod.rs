use crate::common::RQAddr;

pub mod builder;
pub mod decoder;

#[derive(Debug, Clone)]
pub enum GroupImageStoreResp {
    Exist {
        file_id: u64,
        addrs: Vec<RQAddr>,
    },
    NotExist {
        file_id: u64,
        upload_key: Vec<u8>,
        upload_addrs: Vec<RQAddr>,
    },
}
