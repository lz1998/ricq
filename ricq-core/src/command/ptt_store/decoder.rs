use bytes::Bytes;

use crate::RQResult;
use crate::{pb, RQError};
use prost::Message;

impl super::super::super::Engine {
    pub fn decode_group_try_up_ptt_resp(&self, payload: Bytes) -> RQResult<Vec<u8>> {
        let mut rsp = pb::cmd0x388::D388RspBody::decode(&*payload)?;
        let ptt = rsp
            .tryup_ptt_rsp
            .pop()
            .ok_or_else(|| RQError::Other("tryup_ptt_rsp is empty".into()))?;
        ptt.file_key
            .ok_or_else(|| RQError::Other("ptt_store file_key is empty".into()))
    }

    pub fn decode_friend_try_up_ptt_resp(&self, payload: Bytes) -> RQResult<Vec<u8>> {
        pb::cmd0x346::C346RspBody::decode(&*payload)?
            .apply_upload_rsp
            .map(|r| r.uuid)
            .ok_or_else(|| RQError::Other("apply_upload_rsp is none".into()))
    }

    pub fn decode_group_ptt_down(&self, payload: Bytes) -> RQResult<String> {
        let mut rsp = pb::cmd0x388::D388RspBody::decode(&*payload)?;
        let ptt = rsp
            .getptt_url_rsp
            .pop()
            .ok_or_else(|| RQError::Other("getptt_url_rsp is empty".into()))?;
        Ok(format!(
            "http://{}{}",
            ptt.domain
                .ok_or_else(|| RQError::Other("ptt_domain is none".into()))?,
            String::from_utf8_lossy(
                &ptt.down_para
                    .ok_or_else(|| RQError::Other("ptt_down_para is none".into()))?
            )
        ))
    }
}
