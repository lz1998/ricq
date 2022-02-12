use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use rq_engine::RQError;

use crate::client::highway::HighwayFrame;

pub struct HighwayCodec;

impl Encoder<HighwayFrame> for HighwayCodec {
    type Error = RQError;

    fn encode(&mut self, item: HighwayFrame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u8(40);
        dst.put_u32(item.head.len() as u32);
        dst.put_u32(item.body.len() as u32);
        dst.put_slice(&item.head);
        dst.put_slice(&item.body);
        dst.put_u8(41);
        Ok(())
    }
}

impl Decoder for HighwayCodec {
    type Item = HighwayFrame;
    type Error = RQError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 10 {
            return Ok(None);
        }
        src.get_u8();
        let head_length = src.get_u32() as usize;
        let body_length = src.get_u32() as usize;
        if head_length + body_length + 1 > src.remaining() {
            return Ok(None);
        }
        let head = src.copy_to_bytes(head_length);
        let body = src.copy_to_bytes(body_length);
        src.get_u8();
        Ok(Some(Self::Item { head, body }))
    }
}
