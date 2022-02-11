use bytes::{Buf, Bytes};
use jcers::Jce;

use crate::structs::SummaryCardInfo;
use crate::{jce, RQError, RQResult};

impl super::super::super::Engine {
    // SummaryCard.ReqSummaryCard
    pub fn decode_summary_card_response(&self, mut payload: Bytes) -> RQResult<SummaryCardInfo> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut rsp = {
            let mut tmp = data
                .map
                .remove("RespSummaryCard")
                .ok_or_else(|| RQError::Decode("missing RespSummaryCard".into()))?;
            if let Some(resp) = tmp.remove("SummaryCard.RespSummaryCard") {
                resp
            } else {
                tmp.remove("SummaryCard_Old.RespSummaryCard")
                    .ok_or_else(|| {
                        RQError::Decode("missing SummaryCard_Old.RespSummaryCard".into())
                    })?
            }
        };
        rsp.advance(1);
        let mut rsp = Jce::new(&mut rsp);

        let info = SummaryCardInfo {
            sex: rsp.get_by_tag(1).map_err(RQError::Jce)?,
            age: rsp.get_by_tag(2).map_err(RQError::Jce)?,
            nickname: rsp.get_by_tag(3).map_err(RQError::Jce)?,
            level: rsp.get_by_tag(5).map_err(RQError::Jce)?,
            city: rsp.get_by_tag(7).map_err(RQError::Jce)?,
            sign: rsp.get_by_tag(8).map_err(RQError::Jce)?,
            mobile: rsp.get_by_tag(11).map_err(RQError::Jce)?,
            uin: rsp.get_by_tag(23).map_err(RQError::Jce)?,
            login_days: rsp.get_by_tag(36).map_err(RQError::Jce)?,
            ..Default::default()
        };
        // TODO more info
        Ok(info)
    }
}
