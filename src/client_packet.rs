use std::io::Read;
use byteorder::{ReadBytesExt, WriteBytesExt};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::Utc;
use crate::binary_reader::BinaryReader;
use crate::client::Client;
use crate::packet::{build_code2d_request_packet, build_login_packet, build_oicq_request_packet, build_sso_packet};
use crate::tlv::{t16, t1b, t1d, t1f, t33, t35};
use crate::version::{ClientProtocol, gen_version_info};
use crate::binary_writer::BinaryWriter;
use crate::tea::qqtea_decrypt;
use flate2::read::ZlibDecoder;

#[derive(Default, Debug)]
pub struct IncomingPacket {
    pub seq_id: u16,
    pub flag1: i32,
    pub flag2: u8,
    pub flag3: u8,
    pub uin_string: String,
    pub command_name: String,
    pub session_id: Vec<u8>,
    pub payload: Vec<u8>,
}

pub trait ClientPacket {
    fn build_qrcode_fetch_request_packet(&mut self) -> (u16, Vec<u8>);
    fn build_qrcode_result_query_request_packet(&mut self, sig: &[u8]) -> (u16, Vec<u8>);
    fn parse_incoming_packet(&mut self, payload: &mut [u8]) -> Option<IncomingPacket>;
    fn parse_sso_frame(&mut self, pkt: &mut IncomingPacket) -> bool;
}

impl ClientPacket for Client {
    fn build_qrcode_fetch_request_packet(&mut self) -> (u16, Vec<u8>) {
        let watch = gen_version_info(&ClientProtocol::AndroidWatch);
        let seq = self.next_seq();
        let req = build_oicq_request_packet(0, 0x812, &self.ecdh, &self.random_key, &{
            let mut w = Vec::new();
            w.write_hex("0001110000001000000072000000");
            w.put_u32(Utc::now().timestamp() as u32);
            w.put_slice(&build_code2d_request_packet(0, 0, 0x31, &{
                let mut w = Vec::new();
                w.put_u16(0); // const
                w.put_u32(16); // app id
                w.put_u64(0); // const
                w.put_u8(8);  // const
                w.write_bytes_short(&vec![]);

                w.put_u16(6);
                w.put_slice(&t16(watch.sso_version, 16, watch.app_id, &self.device_info.guid, watch.apk_id.as_bytes(), watch.sort_version_name.as_bytes(), &watch.apk_sign));
                w.put_slice(&t1b(0, 0, 3, 4, 72, 2, 2));
                w.put_slice(&t1d(watch.misc_bitmap));
                w.put_slice(&t1f(false, self.device_info.os_type.as_bytes(), "7.1.2".as_bytes(), "China Mobile GSM".as_bytes(), self.device_info.apn.as_bytes(), 2));
                w.put_slice(&t33(&self.device_info.guid));
                w.put_slice(&t35(8));
                w
            }));
            w
        });
        let sso = build_sso_packet(seq, watch.app_id, self.version.sub_app_id, "wtlogin.trans_emp", &self.device_info.imei, &vec![], &self.out_going_packet_session_id, &req, &self.ksid);
        let packet = build_login_packet(0, 2, &vec![0; 16], &sso, &vec![]);
        return (seq, packet);
    }

    fn build_qrcode_result_query_request_packet(&mut self, sig: &[u8]) -> (u16, Vec<u8>) {
        let seq = self.next_seq();
        let watch = gen_version_info(&ClientProtocol::AndroidWatch);
        let req = build_oicq_request_packet(0, 0x812, &self.ecdh, &self.random_key, &{
            let mut w = Vec::new();
            w.write_hex("0000620000001000000072000000"); // trans header
            w.put_u32(Utc::now().timestamp() as u32);
            w.put_slice(&build_code2d_request_packet(1, 0, 0x12, &{
                let mut w = Vec::new();
                w.put_u16(5);
                w.put_u8(1);
                w.put_u32(8);
                w.put_u32(16);
                w.write_bytes_short(sig);
                w.put_u64(0);
                w.put_u8(8);
                w.write_bytes_short(&vec![]);
                w.put_u16(0);
                w
            }));
            w
        });
        let sso = build_sso_packet(seq, watch.app_id, self.version.sub_app_id, "wtlogin.trans_emp", &self.device_info.imei, &vec![], &self.out_going_packet_session_id, &req, &self.ksid);
        let packet = build_login_packet(0, 2, &vec![0; 16], &sso, &vec![]);
        return (seq, packet);
    }

    fn parse_incoming_packet(&mut self, payload: &mut [u8]) -> Option<IncomingPacket> {
        if payload.len() < 6 {
            return None;
        }
        let mut pkt = IncomingPacket::default();
        let mut payload = Bytes::from(payload.to_owned());
        pkt.flag1 = payload.get_i32();
        pkt.flag2 = payload.get_u8();
        pkt.flag3 = payload.get_u8();
        pkt.uin_string = payload.read_string();
        pkt.payload = match pkt.flag2 {
            0 => payload.chunk().to_vec(),
            1 => qqtea_decrypt(payload.chunk(), &self.sig_info.d2key),
            2 => qqtea_decrypt(payload.chunk(), &[0; 16]),
            _ => { vec![] }
        };
        if pkt.payload.len() == 0 {
            return None;
        }
        if pkt.flag1 != 0x0A && pkt.flag1 != 0x0B {
            return None;
        }
        if !self.parse_sso_frame(&mut pkt) {
            return None;
        }
        Some(pkt)
    }

    fn parse_sso_frame(&mut self, pkt: &mut IncomingPacket) -> bool {
        let mut payload = Bytes::from(pkt.payload.clone());
        let len = payload.get_i32() as usize - 4;
        if payload.remaining() < len {
            return false;
        }
        pkt.seq_id = payload.get_i32() as u16;
        let ret_code = payload.get_i32();
        if ret_code != 0 {
            if ret_code == -10008 {
                return false;//ErrSessionExpired
            }
            return false;//unsuccessful
        }

        // extra data
        let len = payload.get_i32() as usize - 4;
        payload.advance(len);

        pkt.command_name = payload.read_string();

        let len = payload.get_i32() as usize - 4;
        pkt.session_id = payload.copy_to_bytes(len).to_vec();
        if pkt.command_name == "Heartbeat.Alive" {
            return true;
        }
        let compressed_flag = payload.get_i32();
        let packet = match compressed_flag {
            0 => {
                let _ = (payload.get_i32() as u64) & 0xffffffff;
                payload.chunk().to_vec()
            }
            1 => {
                payload.advance(4);
                let mut uncompressed = Vec::new();
                ZlibDecoder::new(payload.chunk()).read_to_end(&mut uncompressed);
                uncompressed
            }
            8 => {
                payload.to_vec()
            }
            _ => { vec![] }
        };
        pkt.payload = packet;
        true
    }
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;
    use chrono::Utc;
    use rand::distributions::Alphanumeric;
    use rand::{Rng, thread_rng};
    use crate::device::{random_imei, random_string, random_uuid};
    use crate::tlv::{t1, t16, t1b, t1d, t1f, t33, t35};

    #[test]
    fn test_read() {}
}