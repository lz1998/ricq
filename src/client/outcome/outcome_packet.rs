use bytes::Bytes;

pub struct OutcomePacket {
    pub bytes: Bytes,
    pub seq: u16,
}