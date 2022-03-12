use std::io::Read;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use flate2::read::ZlibDecoder;

use crate::binary::{BinaryReader, BinaryWriter};
use crate::command::common::PbToBytes;
use crate::crypto::{qqtea_decrypt, qqtea_encrypt};
use crate::protocol::{
    device::Device,
    packet::{EncryptType, Packet, PacketType},
    sig::Sig,
    version::Version,
};
use crate::{oicq, pb, RQError, RQResult};

pub struct Transport {
    pub sig: Sig,
    pub device: Device,
    pub version: &'static Version,
    pub oicq_codec: oicq::Codec,
}

impl Transport {
    pub fn new(device: Device, version: &'static Version) -> Self {
        Self {
            sig: Sig::new(&device),
            device,
            version,
            oicq_codec: Default::default(),
        }
    }
}

impl Transport {
    pub fn encode_packet(&self, mut pkt: Packet) -> Bytes {
        if self.sig.d2.is_empty() {
            pkt.encrypt_type = EncryptType::EmptyKey
        }

        let mut w = BytesMut::new();
        // let pos = w.len();
        // w.put_u32(0);

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
                body = Bytes::from(qqtea_encrypt(&body, &self.sig.d2key));
            }
            EncryptType::EmptyKey => {
                body = Bytes::from(qqtea_encrypt(&body, &[0; 16]));
            }
            EncryptType::NoEncrypt => {}
        }
        w.put_slice(&body);

        // let len = w.len();
        // w[pos..pos + 4].as_mut().put_u32(len as u32);
        w.freeze()
    }

    pub fn decode_packet<B>(&self, mut r: B) -> RQResult<Packet>
    where
        B: Buf,
    {
        let mut pkt = Packet {
            packet_type: PacketType::from_i32(r.get_i32())?,
            encrypt_type: EncryptType::from_u8(r.get_u8())?,
            ..Default::default()
        };
        r.get_u8(); // 0x00

        pkt.uin = r.read_string().parse().unwrap_or_default();

        let mut body = Bytes::from(r.chunk().to_owned());
        match pkt.encrypt_type {
            EncryptType::NoEncrypt => {}
            EncryptType::D2Key => body = Bytes::from(qqtea_decrypt(&body, &self.sig.d2key)),
            EncryptType::EmptyKey => body = Bytes::from(qqtea_decrypt(&body, &[0; 16])),
        }

        self.decode_sso_frame(&mut pkt, body)?;
        if pkt.encrypt_type == EncryptType::EmptyKey {
            // decrypt oicq_codec
            pkt.body = self.oicq_codec.decode(pkt.body)?.body;
        }
        Ok(pkt)
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
            if tgt.is_empty() {
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
            w.put_u16(self.device.ksid().len() as u16 + 2);
            w.put_slice(&self.device.ksid());
        }
        w.put_u32(0x04);

        // write len
        let len = w.len() - pos;
        w[pos..pos + 4].as_mut().put_u32(len as u32);

        w.put_u32(pkt.body.len() as u32 + 4);
        w.put_slice(&pkt.body);
    }

    fn decode_sso_frame<B>(&self, pkt: &mut Packet, mut r: B) -> RQResult<()>
    where
        B: Buf,
    {
        let head_len = r.get_i32() as usize;
        if head_len - 4 > r.remaining() {
            return Err(RQError::PacketDropped);
        }

        let mut head = r.copy_to_bytes(head_len - 4);
        pkt.seq_id = head.get_i32();

        let ret_code = head.get_i32();
        match ret_code {
            0 => {}
            -10008 => return Err(RQError::SessionExpired),
            _ => return Err(RQError::UnsuccessfulRetCode(ret_code)),
        }
        pkt.message = head.read_string();
        pkt.command_name = head.read_string();
        if &pkt.command_name == "Heartbeat.Alive" {
            return Ok(());
        }

        let session_id_len = head.get_i32() as usize - 4;
        let _ = head.copy_to_bytes(session_id_len);

        let compress_flag = head.get_i32();

        let mut body_len = r.get_i32() as usize - 4;
        body_len = if body_len > 0 && body_len <= r.remaining() {
            body_len
        } else {
            r.remaining()
        };
        let mut body = r.copy_to_bytes(body_len);

        if compress_flag == 1 {
            let mut uncompressed = Vec::new();
            ZlibDecoder::new(body.chunk())
                .read_to_end(&mut uncompressed)
                .map_err(|_| RQError::Other("failed to decode zlib".into()))?;
            body = Bytes::from(uncompressed)
        }

        pkt.body = body;
        Ok(())
    }

    pub fn encode_oidb_packet(&self, cmd: i32, service_type: i32, body: Bytes) -> Bytes {
        pb::oidb::OidbssoPkg {
            command: cmd,
            service_type,
            bodybuffer: body.to_vec(),
            client_version: format!("Android {}", self.version.sort_version_name),
            ..Default::default()
        }
        .to_bytes()
    }
}
