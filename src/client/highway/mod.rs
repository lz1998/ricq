use bytes::Bytes;

mod codec;
mod net;

pub struct HighwayFrame {
    pub head: Bytes,
    pub body: Bytes,
}
