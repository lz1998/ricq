use crate::binary::BinaryReader;
use crate::crypto::qqtea_decrypt;
use bytes::{Buf, Bytes};
use flate2::read::ZlibDecoder;
use std::io::Read;

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
    pub fn decrypt_payload(&mut self, ecdh_share_key: &[u8], _random: &[u8], session_key: &[u8]) {
        let mut payload = Bytes::from(self.payload.to_owned());
        if payload.get_u8() != 2 {
            // err
            return;
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
        }
        if encrypt_type == 3 {
            let len = payload.remaining() - 1;
            let data = payload.copy_to_bytes(len);
            self.payload = Bytes::from(qqtea_decrypt(&data, session_key));
        }
        //return error
    }
}

impl super::super::Client {
    pub async fn parse_incoming_packet(&self, payload: &mut Bytes) -> Result<IncomePacket, String> {
        if payload.len() < 6 {
            return Err("invalid  incoming packet length".to_string());
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
                &self.cache_info.read().await.sig_info.d2key,
            )),
            2 => Bytes::from(qqtea_decrypt(payload.chunk(), &[0; 16])),
            _ => Bytes::new(),
        };
        if pkt.payload.len() == 0 {
            return Err("payload length==0".to_string());
        }
        if pkt.flag1 != 0x0A && pkt.flag1 != 0x0B {
            return Err("flag1 error".to_string());
        }
        self.parse_sso_frame(&mut pkt).await?;
        if pkt.flag2 == 2 {
            pkt.decrypt_payload(
                &self.ecdh.initial_share_key,
                &self.random_key,
                &self.cache_info.read().await.sig_info.wt_session_ticket_key,
            )
        }
        Ok(pkt)
    }

    pub async fn parse_sso_frame(&self, pkt: &mut IncomePacket) -> Result<(), String> {
        let mut payload = Bytes::from(pkt.payload.to_owned());
        let len = payload.get_i32() as usize - 4;
        if payload.remaining() < len {
            return Err("remaining<len".to_string());
        }
        pkt.seq_id = payload.get_i32() as u16;
        let ret_code = payload.get_i32();
        if ret_code != 0 {
            if ret_code == -10008 {
                return Err("ErrSessionExpired".to_string()); //ErrSessionExpired
            }
            return Err("unsuccessful".to_string()); //unsuccessful
        }

        // extra data
        let len = payload.get_i32() as usize - 4;
        payload.advance(len);

        pkt.command_name = payload.read_string();

        let len = payload.get_i32() as usize - 4;
        pkt.session_id = payload.copy_to_bytes(len);
        if pkt.command_name == "Heartbeat.Alive" {
            return Ok(());
        }
        let compressed_flag = payload.get_i32();
        let packet = match compressed_flag {
            0 => {
                let _ = (payload.get_i32() as u64) & 0xffffffff;
                Bytes::from(payload.chunk().to_owned())
            }
            1 => {
                payload.advance(4);
                let mut uncompressed = Vec::new();
                ZlibDecoder::new(payload.chunk())
                    .read_to_end(&mut uncompressed)
                    .unwrap(); //todo
                Bytes::from(uncompressed)
            }
            8 => Bytes::from(payload),
            _ => Bytes::new(),
        };
        pkt.payload = packet;
        return Ok(());
    }
}
