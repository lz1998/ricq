use std::net::IpAddr;
use std::sync::atomic::{AtomicI32, Ordering};

use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::{pb, RQError, RQResult};

#[derive(Default)]
pub struct Session {
    pub uin: i64,
    pub app_id: i32,
    pub sig_session: Bytes,
    pub session_key: Bytes,
    pub sso_addr: Vec<IpAddr>,
    pub seq: AtomicI32,
}

pub struct BdhInput {
    pub command_id: i32,
    pub body: Vec<u8>,
    pub ticket: Vec<u8>,
    pub ext: Vec<u8>,
    pub encrypt: bool,
    pub size: usize,
    pub md5: Vec<u8>,
}

impl Session {
    fn next_seq(&self) -> i32 {
        self.seq.fetch_add(2, Ordering::Relaxed)
    }
    pub fn build_bdh_head(
        &self,
        command_id: i32,
        filesize: i64,
        chunk: Vec<u8>,
        dataoffset: i64,
        ticket: Vec<u8>,
        file_md5: Vec<u8>,
        ext: Vec<u8>,
    ) -> Bytes {
        let chunk_md5 = md5::compute(&chunk).to_vec();
        let chunk_length = chunk.len();
        pb::ReqDataHighwayHead {
            msg_basehead: Some(pb::DataHighwayHead {
                version: 1,
                uin: self.uin.to_string(),
                command: "PicUp.DataUp".into(),
                seq: self.next_seq(),
                appid: self.app_id,
                dataflag: 4096,
                command_id,
                locale_id: 2052,
                ..Default::default()
            }),
            msg_seghead: Some(pb::SegHead {
                serviceid: 0,
                filesize,
                dataoffset,
                datalength: chunk_length as i32,
                rtcode: 0,
                serviceticket: ticket,
                flag: 0,
                md5: chunk_md5,
                file_md5,
                ..Default::default()
            }),
            req_extendinfo: ext.to_vec(),
            ..Default::default()
        }
        .to_bytes()
    }

    pub fn decode_rsp_head(&self, payload: Bytes) -> RQResult<pb::RspDataHighwayHead> {
        pb::RspDataHighwayHead::from_bytes(&payload)
            .map_err(|_| RQError::Other("RspDataHighwayHead".into()))
    }

    pub fn build_heartbreak(&self) -> Bytes {
        pb::ReqDataHighwayHead {
            msg_basehead: Some(pb::DataHighwayHead {
                version: 1,
                uin: self.uin.to_string(),
                command: "PicUp.Echo".into(),
                seq: self.next_seq(),
                appid: self.app_id,
                dataflag: 4096,
                command_id: 0,
                locale_id: 2052,
                ..Default::default()
            }),
            ..Default::default()
        }
        .to_bytes()
    }
}
