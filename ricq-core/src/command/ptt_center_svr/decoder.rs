use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::pb::short_video::{ShortVideoRspBody, ShortVideoUploadRsp};
use crate::{pb, RQError, RQResult};

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

    // PttCenterSvr.pb_pttCenter_CMD_REQ_APPLY_DOWNLOAD-1200
    pub fn decode_c2c_ptt_down(&self, payload: Bytes) -> RQResult<String> {
        pb::cmd0x346::C346RspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("C346RspBody".into()))?
            .apply_download_rsp
            .ok_or_else(|| RQError::Other("apply_download_rsp is empty".into()))?
            .download_info
            .ok_or_else(|| RQError::Other("download_info is empty".into()))
            .map(|info| info.download_url)
    }
}
