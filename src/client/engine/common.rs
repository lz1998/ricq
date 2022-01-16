use crate::client::protocol::oicq;
use crate::client::protocol::packet::*;
use bytes::{BufMut, Bytes, BytesMut};

impl super::Engine {
    pub fn build_oicq_request_packet(&self, uin: i64, command_id: u16, body: &[u8]) -> Bytes {
        let req = oicq::Message {
            uin: uin as u32,
            command: command_id,
            body: Bytes::from(body.to_vec()),
            encryption_method: oicq::EncryptionMethod::ECDH,
        };
        self.oicq_codec.encode(req)
    }

    pub fn uni_packet_with_seq(&self, seq: u16, command: &str, body: Bytes) -> Packet {
        Packet {
            packet_type: PacketType::Simple,
            encrypt_type: EncryptType::D2Key,
            seq_id: seq as i32,
            body,
            command_name: command.to_owned(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    pub fn uni_packet(&self, command: &str, body: Bytes) -> Packet {
        let seq = self.next_seq();
        self.uni_packet_with_seq(seq, command, body)
    }
}

pub fn pack_uni_request_data(data: &[u8]) -> Bytes {
    let mut r = BytesMut::new();
    r.put_slice(&[0x0A]);
    r.put_slice(data);
    r.put_slice(&[0x0B]);
    Bytes::from(r)
}
