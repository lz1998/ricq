use bytes::{BufMut, Bytes, BytesMut};

use crate::binary::BinaryWriter;
use crate::client::protocol::packet::{EncryptType, PacketType};
use crate::client::protocol::{device::Device, packet::Packet, sig::Sig, version::Version};
use crate::crypto::qqtea_encrypt;

pub struct Transport {
    sig: Sig,
    device: Device,
    version: Version,
}

impl Transport {
    pub fn encode_packet(&self, mut pkt: Packet) -> Bytes {
        if self.sig.d2.len() == 0 {
            pkt.encrypt_type = EncryptType::EmptyKey
        }

        let mut w = BytesMut::new();
        let pos = w.len();
        w.put_u32(0);

        // vvv w.Write(head) vvv
        w.put_u32(pkt.packet_type.value());
        w.put_u8(pkt.encrypt_type.value() as u8);
        match pkt.packet_type {
            PacketType::Simple => w.put_u32(pkt.seq_id as u32),
            PacketType::Login => match pkt.encrypt_type {
                EncryptType::D2Key => {
                    w.put_u32(self.sig.d2.len() as u32 + 4);
                    w.put_slice(&self.sig.d2);
                }
                _ => w.put_u32(4),
            },
        }
        w.put_u8(0x00);
        w.write_string(&pkt.uin.to_string());
        // ^^^ w.Write(head) ^^^

        let mut w2 = BytesMut::new();
        self.encode_body(&pkt, &mut w2);
        let mut body = w2.freeze();
        match pkt.encrypt_type {
            EncryptType::D2Key => {
                body = Bytes::from(qqtea_encrypt(&body, &self.sig.d2));
            }
            EncryptType::EmptyKey => {
                body = Bytes::from(qqtea_encrypt(&body, &[0; 16]));
            }
            EncryptType::NoEncrypt => {}
        }
        w.put_slice(&body);

        let len = w.len();
        w[pos..pos + 4].as_mut().put_u32(len as u32);
        w.freeze()
    }
    pub fn decode_packet(&self, payload: &[u8]) -> Packet {
        todo!()
    }
    fn encode_body(&self, pkt: &Packet, w: &mut BytesMut) {
        let pos = w.len();
        w.put_u32(0); // len

        if pkt.packet_type == PacketType::Login {
            w.put_u32(pkt.seq_id as u32);
            w.put_u32(self.version.app_id);
            w.put_u32(self.version.sub_app_id);
            w.put_slice(&[
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            ]);
            let tgt = &self.sig.tgt;
            if tgt.len() == 0 {
                w.put_u32(0x04);
            } else {
                w.put_u32(tgt.len() as u32 + 4);
                w.put_slice(tgt);
            }
        }
        w.write_string(&pkt.command_name);

        w.put_u32(self.sig.out_packet_session_id.len() as u32 + 4);
        w.put_slice(&self.sig.out_packet_session_id);
        if pkt.packet_type == PacketType::Login {
            w.write_string(&self.device.imei);
            w.put_u32(0x04);
            w.put_u16(self.sig.ksid.len() as u16 + 2);
            w.put_slice(&self.sig.ksid);
        }
        w.put_u32(0x04);

        // write len
        let len = w.len() - pos;
        w[pos..pos + 4].as_mut().put_u32(len as u32);

        w.put_u32(pkt.body.len() as u32 + 4);
        w.put_slice(&pkt.body);
    }
}
