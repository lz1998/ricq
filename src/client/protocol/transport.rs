use bytes::Bytes;

use crate::client::protocol::{device::Device, packet::Packet, sig::Sig, version::Version};

pub struct Transport {
    sig: Sig,
    device: Device,
    version: Version,
}

impl Transport {
    pub fn encode_packet(&self, pkt: Packet) -> Bytes {
        todo!()
    }
    pub fn decode_packet(&self, payload: &[u8]) -> Packet {
        todo!()
    }
    fn encode_body() {
        todo!()
    }
}
