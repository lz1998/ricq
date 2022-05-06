use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::command::video_store::GroupVideoStoreResp;
use crate::common::RQAddr;
use crate::{pb, RQError, RQResult};

impl super::super::super::Engine {
    pub fn decode_group_video_store_response(
        &self,
        payload: Bytes,
    ) -> RQResult<GroupVideoStoreResp> {
        // std::process::exit(0);
        let mut rsp = pb::short_video::ShortVideoRspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("ShortVideoRspBody".into()))?;
        let rsp = rsp
            .ptt_short_video_upload_rsp
            .ok_or_else(|| RQError::Other("ptt_short_video_upload_rsp not found".into()))?;
        // todo
        panic!("not impl")
    }
}
