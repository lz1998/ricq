use crate::client::errors::RQError;
use crate::jce::*;
use bytes::{Buf, Bytes};
use jcers::Jce;

pub fn decode_client_register_response(payload: &[u8]) -> Result<SvcRespRegister, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: RequestPacket = jcers::from_buf(&mut payload).map_err(|e| RQError::from(e))?;
    let mut data: RequestDataVersion2 =
        jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::from(e))?;
    let mut a = data
        .map
        .remove("SvcRespRegister")
        .ok_or(RQError::Decode("missing SvcRespRegister".into()))?;
    let mut b = a
        .remove("QQService.SvcRespRegister")
        .ok_or(RQError::Decode("missing QQService.SvcRespRegister".into()))?;
    b.advance(1);
    jcers::from_buf(&mut b).map_err(|e| RQError::from(e))
}

pub fn decode_dev_list_response(payload: &[u8]) -> Result<Vec<SvcDevLoginInfo>, RQError> {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: RequestPacket = jcers::from_buf(&mut payload).map_err(|e| RQError::from(e))?;
    let mut data: RequestDataVersion2 =
        jcers::from_buf(&mut request.s_buffer).map_err(|e| RQError::from(e))?;
    let mut req = data
        .map
        .remove("SvcRspGetDevLoginInfo")
        .ok_or(RQError::Decode("missing SvcRspGetDevLoginInfo".into()))?;
    let mut msg = req
        .remove("QQService.SvcRspGetDevLoginInfo")
        .ok_or(RQError::Decode(
            "missing QQService.SvcRspGetDevLoginInfo".into(),
        ))?;
    msg.advance(1);
    let mut rsp = Jce::new(&mut msg);
    let d: Vec<SvcDevLoginInfo> = rsp.get_by_tag(4).map_err(|e| RQError::from(e))?;
    if d.len() > 0 {
        return Ok(d);
    }
    let d: Vec<SvcDevLoginInfo> = rsp.get_by_tag(5).map_err(|e| RQError::from(e))?;
    if d.len() > 0 {
        return Ok(d);
    }
    let d: Vec<SvcDevLoginInfo> = rsp.get_by_tag(6).map_err(|e| RQError::from(e))?;
    if d.len() > 0 {
        return Ok(d);
    }
    return Err(RQError::Decode("decode_dev_list_response".to_string()));
}
