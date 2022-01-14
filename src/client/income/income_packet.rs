use std::io::Read;

use bytes::{Buf, Bytes};
use flate2::read::ZlibDecoder;

use crate::binary::BinaryReader;
use crate::crypto::qqtea_decrypt;
use crate::{RQError, RQResult};

#[derive(Default, Debug)]
pub struct IncomePacket {
    pub seq_id: u16,
    pub flag1: i32,
    pub flag2: u8,
    pub flag3: u8,
    pub uin_string: String,
    pub command_name: String,
    pub session_id: Bytes,
    pub payload: Bytes,
}

impl IncomePacket {
    pub fn decrypt_payload(
        &mut self,
        ecdh_share_key: &[u8],
        _random: &[u8],
        session_key: &[u8],
    ) -> Result<(), RQError> {
        let mut payload = Bytes::from(self.payload.to_owned());
        if payload.get_u8() != 2 {
            // err
            return Err(RQError::Other("unknown flag".into()));
        }
        payload.advance(2);
        payload.advance(2);
        payload.get_u16();
        payload.get_u16();
        payload.get_i32();
        let encrypt_type = payload.get_u16();
        payload.get_u8();
        if encrypt_type == 0 {
            let len = payload.remaining() - 1;
            let data = payload.copy_to_bytes(len);
            self.payload = Bytes::from(qqtea_decrypt(&data, ecdh_share_key));
            return Ok(());
        }
        if encrypt_type == 3 {
            let len = payload.remaining() - 1;
            let data = payload.copy_to_bytes(len);
            self.payload = Bytes::from(qqtea_decrypt(&data, session_key));
            return Ok(());
        }
        return Err(RQError::Other("unknown encrypt type".into()));
        //return error
    }

    pub(crate) fn check_command_name(self, command_name: &str) -> RQResult<Self> {
        if !(&self.command_name == command_name) {
            Err(RQError::CommandNameMismatch(
                command_name.to_owned(),
                self.command_name.clone(),
            ))
        } else {
            Ok(self)
        }
    }
}

impl super::super::Client {
    pub async fn parse_incoming_packet(
        &self,
        payload: &mut Bytes,
    ) -> Result<IncomePacket, RQError> {
        if payload.len() < 6 {
            return Err(RQError::Decode("invalid  incoming packet length".into()));
        }
        let mut pkt = IncomePacket::default();
        pkt.flag1 = payload.get_i32();
        pkt.flag2 = payload.get_u8();
        pkt.flag3 = payload.get_u8();
        pkt.uin_string = payload.read_string();
        pkt.payload = match pkt.flag2 {
            0 => Bytes::from(payload.chunk().to_owned()),
            1 => Bytes::from(qqtea_decrypt(
                payload.chunk(),
                &self.transport.read().await.sig.d2key,
            )),
            2 => Bytes::from(qqtea_decrypt(payload.chunk(), &[0; 16])),
            _ => Bytes::new(),
        };
        if pkt.payload.len() == 0 {
            return Err(RQError::Decode("payload length==0".into()));
        }
        if pkt.flag1 != 0x0A && pkt.flag1 != 0x0B {
            return Err(RQError::Decode("flag1 error".into()));
        }
        self.parse_sso_frame(&mut pkt).await?;
        if pkt.flag2 == 2 {
            let oicq_codec = self.oicq_codec.read().await;
            let decrypted_message = oicq_codec.decode(pkt.payload)?;
            pkt.payload = decrypted_message.body;
        }
        Ok(pkt)
    }

    pub async fn parse_sso_frame(&self, pkt: &mut IncomePacket) -> Result<(), RQError> {
        let mut payload = Bytes::from(pkt.payload.to_owned());
        let head_len = payload.get_i32() as usize - 4;
        if payload.remaining() < head_len {
            return Err(RQError::Decode("remaining<len".into()));
        }
        let mut head = payload.copy_to_bytes(head_len);
        pkt.seq_id = head.get_i32() as u16;
        let ret_code = head.get_i32();
        if ret_code != 0 {
            if ret_code == -10008 {
                return Err(RQError::Decode("ErrSessionExpired".into())); //ErrSessionExpired
            }
            return Err(RQError::Decode("unsuccessful".into())); //unsuccessful
        }

        // extra data
        let len = head.get_i32() as usize - 4;
        head.advance(len);

        pkt.command_name = head.read_string();

        let len = head.get_i32() as usize - 4;
        pkt.session_id = head.copy_to_bytes(len);
        if pkt.command_name == "Heartbeat.Alive" {
            return Ok(());
        }
        let compressed_flag = head.get_i32();

        let body_len = payload.get_i32() as usize - 4;
        let packet = match compressed_flag {
            0 => Bytes::from(payload.copy_to_bytes(body_len)),
            1 => {
                let mut uncompressed = Vec::new();
                ZlibDecoder::new(payload.copy_to_bytes(body_len).chunk())
                    .read_to_end(&mut uncompressed)
                    .map_err(|_| RQError::Other("failed to decode zlib".into()))?;
                Bytes::from(uncompressed)
            }
            8 => Bytes::from(payload),
            _ => Bytes::new(),
        };
        pkt.payload = packet;
        return Ok(());
    }
}
