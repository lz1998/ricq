use std::net::SocketAddr;

pub mod builder;
pub mod decoder;

#[derive(Debug, Clone)]
pub enum GroupImageStoreResp {
    Exist {
        file_id: u64,
        width: u32,
        height: u32,
    },
    NotExist {
        file_id: u64,
        upload_key: Vec<u8>,
        upload_addrs: Vec<SocketAddr>,
    },
}
