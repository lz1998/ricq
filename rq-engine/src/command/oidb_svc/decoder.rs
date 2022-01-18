use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::command::oidb_svc::GroupAtAllRemainInfo;
use crate::{pb, RQError, RQResult};

impl super::super::super::Engine {
    pub fn decode_group_at_all_remain_response(
        &self,
        payload: Bytes,
    ) -> RQResult<GroupAtAllRemainInfo> {
        let pkg = pb::oidb::OidbssoPkg::from_bytes(&payload)
            .map_err(|_| RQError::Decode("OidbssoPkg".into()))?;
        let rsp = pb::oidb::D8a7RspBody::from_bytes(&pkg.bodybuffer)
            .map_err(|_| RQError::Decode("D8a7RspBody".into()))?;
        Ok(GroupAtAllRemainInfo {
            can_at_all: rsp.can_at_all(),
            remain_at_all_count_for_group: rsp.remain_at_all_count_for_group(),
            remain_at_all_count_for_uin: rsp.remain_at_all_count_for_uin(),
        })
    }
}
