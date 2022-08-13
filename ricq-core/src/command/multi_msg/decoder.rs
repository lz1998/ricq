use bytes::Bytes;

use crate::{pb, RQError, RQResult};
use prost::Message;

impl super::super::super::Engine {
    pub fn decode_multi_msg_apply_down_resp(
        &self,
        payload: Bytes,
    ) -> RQResult<pb::multimsg::MultiMsgApplyDownRsp> {
        pb::multimsg::MultiRspBody::decode(&*payload)?
            .multimsg_applydown_rsp
            .pop()
            .ok_or_else(|| RQError::Other("multimsg_applydown_rsp is none".into()))
    }

    pub fn decode_multi_msg_apply_up_resp(
        &self,
        payload: Bytes,
    ) -> RQResult<pb::multimsg::MultiMsgApplyUpRsp> {
        pb::multimsg::MultiRspBody::decode(&*payload)?
            .multimsg_applyup_rsp
            .pop()
            .ok_or_else(|| RQError::Other("multimsg_applyup_rsp is none".into()))
    }
}
