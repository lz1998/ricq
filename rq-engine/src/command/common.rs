use bytes::{BufMut, Bytes, BytesMut};
use prost::{DecodeError, Message};

use crate::protocol::oicq;
use crate::protocol::packet::*;
use crate::Engine;

impl Engine {
    pub fn build_oicq_request_packet(&self, uin: i64, command_id: u16, body: &[u8]) -> Bytes {
        let req = oicq::Message {
            uin: uin as u32,
            command: command_id,
            body: Bytes::from(body.to_vec()),
            encryption_method: oicq::EncryptionMethod::ECDH,
        };
        self.transport.oicq_codec.encode(req)
    }

    pub fn uni_packet_with_seq(&self, seq: i32, command: &str, body: Bytes) -> Packet {
        Packet {
            packet_type: PacketType::Simple,
            encrypt_type: EncryptType::D2Key,
            seq_id: seq,
            body,
            command_name: command.to_owned(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    pub fn uni_packet(&self, command: &str, body: Bytes) -> Packet {
        let seq = self.next_seq();
        self.uni_packet_with_seq(seq as i32, command, body)
    }
}

pub fn pack_uni_request_data(data: &[u8]) -> Bytes {
    let mut r = BytesMut::new();
    r.put_slice(&[0x0A]);
    r.put_slice(data);
    r.put_slice(&[0x0B]);
    Bytes::from(r)
}

pub trait PbToBytes<B>
where
    B: Message,
{
    fn to_bytes(&self) -> Bytes;
    fn from_bytes(buf: &[u8]) -> Result<B, DecodeError>;
}

impl<B> PbToBytes<B> for B
where
    B: Message + Default,
{
    fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        prost::Message::encode(self, &mut buf).expect("prost encode failed");
        buf.freeze()
    }
    fn from_bytes(buf: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(buf)
    }
}
