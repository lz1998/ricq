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
            .ok_or(RQError::EmptyField("tryup_ptt_rsp"))?;
        ptt.file_key.ok_or(RQError::EmptyField("file_key"))
    }

    pub fn decode_friend_try_up_ptt_resp(&self, payload: Bytes) -> RQResult<Vec<u8>> {
        pb::cmd0x346::C346RspBody::decode(&*payload)?
            .apply_upload_rsp
            .map(|r| r.uuid)
            .ok_or(RQError::EmptyField("apply_upload_rsp"))
    }

    pub fn decode_group_ptt_down(&self, payload: Bytes) -> RQResult<String> {
        let mut rsp = pb::cmd0x388::D388RspBody::decode(&*payload)?;
        let ptt = rsp
            .getptt_url_rsp
            .pop()
            .ok_or(RQError::EmptyField("getptt_url_rsp"))?;
        Ok(format!(
            "http://{}{}",
            ptt.domain.ok_or(RQError::EmptyField("ptt_domain"))?,
            String::from_utf8_lossy(&ptt.down_para.ok_or(RQError::EmptyField("ptt_down_para"))?)
        ))
    }
}
