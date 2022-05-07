use crate::common::RQAddr;

mod builder;
mod decoder;

#[derive(Debug, Clone)]
pub enum OffPicUpResp {
    Exist {
        res_id: String,
        uuid: String,
    },
    UploadRequired {
        res_id: String,
        uuid: String,
        upload_key: Vec<u8>,
        upload_addrs: Vec<RQAddr>,
    },
}
