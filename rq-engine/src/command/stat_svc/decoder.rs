use bytes::{Buf, Bytes};
use jcers::Jce;

use crate::{jce, RQError, RQResult};

impl super::super::super::Engine {
    // StatSvc.register
    pub fn decode_client_register_response(
        &self,
        mut payload: Bytes,
    ) -> RQResult<jce::SvcRespRegister> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut a = data
            .map
            .remove("SvcRespRegister")
            .ok_or_else(|| RQError::Decode("missing SvcRespRegister".into()))?;
        let mut b = a
            .remove("QQService.SvcRespRegister")
            .ok_or_else(|| RQError::Decode("missing QQService.SvcRespRegister".into()))?;
        b.advance(1);
        jcers::from_buf(&mut b).map_err(RQError::from)
    }

    // StatSvc.GetDevLoginInfo
    pub fn decode_dev_list_response(
        &self,
        mut payload: Bytes,
    ) -> RQResult<Vec<jce::SvcDevLoginInfo>> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut req = data
            .map
            .remove("SvcRspGetDevLoginInfo")
            .ok_or_else(|| RQError::Decode("missing SvcRspGetDevLoginInfo".into()))?;
        let mut msg = req
            .remove("QQService.SvcRspGetDevLoginInfo")
            .ok_or_else(|| RQError::Decode("missing QQService.SvcRspGetDevLoginInfo".into()))?;
        msg.advance(1);
        let mut rsp = Jce::new(&mut msg);
        let d: Vec<jce::SvcDevLoginInfo> = rsp.get_by_tag(4).map_err(RQError::from)?;
        if !d.is_empty() {
            return Ok(d);
        }
        let d: Vec<jce::SvcDevLoginInfo> = rsp.get_by_tag(5).map_err(RQError::from)?;
        if !d.is_empty() {
            return Ok(d);
        }
        let d: Vec<jce::SvcDevLoginInfo> = rsp.get_by_tag(6).map_err(RQError::from)?;
        if !d.is_empty() {
            return Ok(d);
        }
        Err(RQError::Decode("decode_dev_list_response".into()))
    }

    // StatSvc.ReqMSFOffline
    pub fn decode_msf_force_offline(
        &self,
        mut payload: Bytes,
    ) -> RQResult<jce::RequestMSFForceOffline> {
        let mut request: jce::RequestPacket =
            jcers::from_buf(&mut payload).map_err(RQError::from)?;
        let mut data: jce::RequestDataVersion2 =
            jcers::from_buf(&mut request.s_buffer).map_err(RQError::from)?;
        let mut data = data
            .map
            .remove("RequestMSFForceOffline")
            .ok_or_else(|| RQError::Decode("missing RequestMSFForceOffline".into()))?
            .remove("QQService.RequestMSFForceOffline")
            .ok_or_else(|| RQError::Decode("missing QQService.RequestMSFForceOffline".into()))?;
        jcers::from_buf(&mut data).map_err(RQError::from)
    }
}
