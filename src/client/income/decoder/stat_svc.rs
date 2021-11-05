use crate::jce::*;
use bytes::{Bytes,Buf};
use jce_struct::Jce;

pub fn decode_client_register_response(payload: &[u8]) -> SvcRespRegister {
    let mut payload = Bytes::from(payload.to_owned());
    let mut request: RequestPacket = Jce::read_from_bytes(&mut payload);
    let mut data: RequestDataVersion2 = Jce::read_from_bytes(&mut request.s_buffer);
    let mut a = data.map.remove("SvcRespRegister").unwrap();
    let mut b = a.remove("QQService.SvcRespRegister").unwrap();
    b.advance(1);
    Jce::read_from_bytes(&mut b)
}