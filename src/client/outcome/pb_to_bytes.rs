use bytes::{Bytes, BytesMut};
use prost::{DecodeError, Message};

pub trait PbToBytes<B>
    where B: Message {
    fn to_bytes(&self) -> Bytes;
    fn from_bytes(buf: &[u8]) -> Result<B, DecodeError>;
}

impl<B> PbToBytes<B> for B
    where B: Message+Default
{
    fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        prost::Message::encode(self, &mut buf).unwrap();
        buf.freeze()
    }
    fn from_bytes(buf: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(buf)
    }
}