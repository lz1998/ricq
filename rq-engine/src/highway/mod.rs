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

#[derive(Default, Debug, Clone)]
pub struct BdhInput {
    // 1-friend, 2-group, 299-groupPtt
    pub command_id: i32,
    pub body: Vec<u8>,
    pub ticket: Vec<u8>,
    pub ext: Vec<u8>,
    pub encrypt: bool,
    pub chunk_size: usize,
    pub send_echo: bool,
}

impl Session {
    fn next_seq(&self) -> i32 {
        self.seq.fetch_add(2, Ordering::Relaxed)
    }

    pub fn build_basehead(
        &self,
        command: String,
        dataflag: i32,
        command_id: i32,
        locale_id: i32,
    ) -> pb::DataHighwayHead {
        pb::DataHighwayHead {
            version: 1,
            uin: self.uin.to_string(),
            command,
            seq: self.next_seq(),
            appid: self.app_id,
            dataflag,
            command_id,
            locale_id,
            ..Default::default()
        }
    }

    pub fn build_seghead(
        &self,
        filesize: i64,
        dataoffset: i64,
        chunk: &[u8],
        ticket: Vec<u8>,
        file_md5: Vec<u8>,
    ) -> pb::SegHead {
        pb::SegHead {
            filesize,
            dataoffset,
            datalength: chunk.len() as i32,
            serviceticket: ticket,
            md5: md5::compute(chunk).to_vec(),
            file_md5,
            ..Default::default()
        }
    }

    pub fn build_bdh_head(
        &self,
        command_id: i32,
        filesize: i64,
        chunk: &[u8],
        dataoffset: i64,
        ticket: Vec<u8>,
        file_md5: Vec<u8>,
    ) -> Bytes {
        pb::ReqDataHighwayHead {
            msg_basehead: Some(self.build_basehead("PicUp.DataUp".into(), 4096, command_id, 2052)),
            msg_seghead: Some(pb::SegHead {
                filesize,
                dataoffset,
                datalength: chunk.len() as i32,
                serviceticket: ticket,
                md5: md5::compute(chunk).to_vec(),
                file_md5,
                ..Default::default()
            }),
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
            msg_basehead: Some(self.build_basehead("PicUp.Echo".into(), 4096, 0, 2052)),
            ..Default::default()
        }
        .to_bytes()
    }
}
