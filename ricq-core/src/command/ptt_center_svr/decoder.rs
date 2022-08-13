use bytes::Bytes;

use crate::pb::short_video::{ShortVideoRspBody, ShortVideoUploadRsp};
use crate::{pb, RQError, RQResult};
use prost::Message;

impl super::super::super::Engine {
    pub fn decode_group_video_store_response(
        &self,
        payload: Bytes,
    ) -> RQResult<ShortVideoUploadRsp> {
        ShortVideoRspBody::decode(&*payload)?
            .ptt_short_video_upload_rsp
            .ok_or(RQError::EmptyField("ptt_short_video_upload_rsp"))
    }

    // PttCenterSvr.pb_pttCenter_CMD_REQ_APPLY_DOWNLOAD-1200
    pub fn decode_c2c_ptt_down(&self, payload: Bytes) -> RQResult<String> {
        pb::cmd0x346::C346RspBody::decode(&*payload)?
            .apply_download_rsp
            .ok_or(RQError::EmptyField("apply_download_rsp"))?
            .download_info
            .ok_or(RQError::EmptyField("download_info"))
            .map(|info| info.download_url)
    }
}
