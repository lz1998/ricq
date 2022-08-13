use bytes::{Buf, Bytes};

use crate::jce::{RespSummaryCard, RespSummaryCardHead};
use crate::structs::SummaryCardInfo;
use crate::{jce, RQError, RQResult};

impl super::super::super::Engine {
    // SummaryCard.ReqSummaryCard
    pub fn decode_summary_card_response(&self, mut payload: Bytes) -> RQResult<SummaryCardInfo> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut head = data
            .map
            .remove("RespHead")
            .ok_or_else(|| RQError::Decode("missing RespHead".into()))?
            .remove("SummaryCard.RespHead")
            .ok_or_else(|| RQError::Decode("missing SummaryCard.RespHead".into()))?;
        head.advance(1);
        let head: RespSummaryCardHead = jcers::from_buf(&mut head)?;
        let mut rsp = data
            .map
            .remove("RespSummaryCard")
            .ok_or_else(|| RQError::Decode("missing RespSummaryCard".into()))?
            .remove("SummaryCard.RespSummaryCard")
            .ok_or_else(|| RQError::Decode("missing SummaryCard.RespSummaryCard".into()))?;
        rsp.advance(1);
        let rsp: RespSummaryCard = jcers::from_buf(&mut rsp)?;
        let info = SummaryCardInfo {
            sex: rsp.sex,
            age: rsp.age,
            nickname: rsp.nickname,
            level: rsp.level,
            city: rsp.city,
            sign: rsp.sign,
            mobile: rsp.mobile,
            uin: rsp.uin,
            login_days: rsp.login_days,
            cookie: head.cookie,
        };
        // TODO more info
        Ok(info)
    }
}
