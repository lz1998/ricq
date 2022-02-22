use std::net::SocketAddr;

mod builder;
mod decoder;

#[derive(Debug, Clone)]
pub enum OffPicUpResp {
    Fail(String),
    SubComErr(u32),
    EmptyImgVec,
    Exit(String),
    UploadRequired {
        res_id: String,
        upload_key: Vec<u8>,
        upload_addrs: Vec<SocketAddr>,
    },
}
