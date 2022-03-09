use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::RQResult;
use crate::{pb, RQError};

impl super::super::super::Engine {
    pub fn decode_group_try_up_ptt_resp(&self, payload: Bytes) -> RQResult<Vec<u8>> {
        let mut rsp = pb::cmd0x388::D388RspBody::from_bytes(&payload)
            .map_err(|_| RQError::Decode("D388RspBody".into()))?;
        let ptt = rsp
            .tryup_ptt_rsp
            .pop()
            .ok_or_else(|| RQError::Other("tryup_ptt_rsp is empty".into()))?;
        ptt.file_key
            .ok_or_else(|| RQError::Other("ptt file_key is empty".into()))
    }
}
