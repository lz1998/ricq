use bytes::Bytes;

pub struct OutcomePacket {
    pub bytes: Bytes,
    pub seq: u16,
}

impl From<(u16, Bytes)> for OutcomePacket {
    fn from(out: (u16, Bytes)) -> Self {
        Self {
            seq: out.0,
            bytes: out.1,
        }
    }
}
