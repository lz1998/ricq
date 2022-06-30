#![allow(clippy::large_enum_variant)]
use bytes::Bytes;

use crate::{jce, pb};

pub mod builder;
pub mod decoder;

#[derive(Default, Debug)]
pub struct ConfigPushReq {
    pub resp: ConfigPushResp,
    pub body: ConfigPushBody,
}

#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
pub enum ConfigPushBody {
    #[derivative(Default)]
    Unknown,
    SsoServers {
        servers: Vec<jce::SsoServerInfo>,
    },
    FileStorageInfo {
        info: jce::FileStoragePushFSSvcList,
        rsp_body: Option<pb::cmd0x6ff::SubCmd0x501RspBody>,
    },
}

#[derive(Default, Debug)]
pub struct ConfigPushResp {
    pub t: i32,
    pub pkt_seq: i64,
    pub jce_buf: Bytes,
}
