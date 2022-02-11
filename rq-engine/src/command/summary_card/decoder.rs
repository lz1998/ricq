use bytes::{Buf, Bytes};
use jcers::Jce;

use crate::structs::SummaryCardInfo;
use crate::{jce, RQError, RQResult};

impl super::super::super::Engine {
    // SummaryCard.ReqSummaryCard
    pub fn decode_summary_card_response(&self, mut payload: Bytes) /*-> RQResult<SummaryCardInfo> */
    {
        // let mut request: jce::RequestPacket =
        //     jcers::from_buf(&mut payload).map_err(RQError::from)?;
        // let mut data: jce::RequestDataVersion2 =
        //     jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        // let mut req = data
        //     .map
        //     .remove("RespSummaryCard")
        //     .ok_or_else(|| RQError::Decode("missing RespSummaryCard".into()))?;
        // let mut msg = req
        //     .remove("SummaryCard.RespSummaryCard")
        //     .ok_or_else(|| RQError::Decode("missing SummaryCard.RespSummaryCard".into()))?;
        // msg.advance(1);
        // let mut rsp = Jce::new(&mut msg);
        // TODO
        let info = SummaryCardInfo {
            ..Default::default()
        };
    }
}
