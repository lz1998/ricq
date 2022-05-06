use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::common::RQAddr;
use crate::pb::short_video::{ShortVideoRspBody, ShortVideoUploadRsp};
use crate::{RQError, RQResult};

impl super::super::super::Engine {
    pub fn decode_group_video_store_response(
        &self,
        payload: Bytes,
    ) -> RQResult<ShortVideoUploadRsp> {
        ShortVideoRspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("ShortVideoRspBody".into()))?
            .ptt_short_video_upload_rsp
            .ok_or_else(|| RQError::Other("ptt_short_video_upload_rsp not found".into()))
    }
}
