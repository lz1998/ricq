use std::borrow::Borrow;
use std::io::Read;
use byteorder::{ReadBytesExt, WriteBytesExt};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::Utc;
use crate::binary_reader::BinaryReader;
use crate::client::Client;
use crate::packet::{build_code2d_request_packet, build_login_packet, build_oicq_request_packet, build_sso_packet};
use crate::tlv::{guid_flag, t1, t100, t107, t116, t141, t142, t144, t147, t154, t16, t177, t18, t187, t188, t191, t194, t1b, t1d, t1f, t202, t33, t35, t511, t516, t521, t525, t536, t8};
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

impl IncomingPacket {
    fn decrypt_payload(&mut self, ecdh_share_key: &[u8], random: &[u8], session_key: &[u8]) {
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
            self.payload = qqtea_decrypt(&data, ecdh_share_key);
        }
        if encrypt_type == 3 {
            let len = payload.remaining() - 1;
            let data = payload.copy_to_bytes(len);
            self.payload = qqtea_decrypt(&data, session_key);
        }
        //return error
    }
}


pub trait ClientPacket {
    fn build_qrcode_fetch_request_packet(&mut self) -> (u16, Vec<u8>);
    fn build_qrcode_result_query_request_packet(&mut self, sig: &[u8]) -> (u16, Vec<u8>);
    fn parse_incoming_packet(&mut self, payload: &mut [u8]) -> Option<IncomingPacket>;
    fn parse_sso_frame(&mut self, pkt: &mut IncomingPacket) -> bool;
    fn build_qrcode_login_packet(&mut self, t106: &[u8], t16a: &[u8], t318: &[u8]) -> (u16, Vec<u8>);
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
        if pkt.flag2 == 2 {
            pkt.decrypt_payload(&self.ecdh.initial_share_key, &self.random_key, &self.sig_info.wt_session_ticket_key)
        }
        Some(pkt)
    }

    fn parse_sso_frame(&mut self, pkt: &mut IncomingPacket) -> bool {
        let mut payload = Bytes::from(pkt.payload.to_owned());
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
        println!("compress_flag: {}", compressed_flag);
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

    fn build_qrcode_login_packet(&mut self, t106: &[u8], t16a: &[u8], t318: &[u8]) -> (u16, Vec<u8>) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin as u32, 0x0810, &self.ecdh, &self.random_key, &{
            let mut w: Vec<u8> = Vec::new();
            w.put_u16(9);
            w.put_u16(24);

            w.put_slice(&t18(16, self.uin as u32));
            w.put_slice(&t1(self.uin as u32, &self.device_info.ip_address));
            w.put_slice(&{
                let mut ww = Vec::new();
                ww.put_u16(0x106);
                ww.write_bytes_short(t106);
                ww
            });
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t100(self.version.sso_version, self.version.sub_app_id, self.version.main_sig_map));
            w.put_slice(&t107(0));
            w.put_slice(&t142(self.version.apk_id.as_bytes()));
            let device_info = crate::pb::DeviceInfo {
                bootloader: "".to_string(),
                proc_version: "".to_string(),
                codename: "".to_string(),
                incremental: "".to_string(),
                fingerprint: "".to_string(),
                boot_id: "".to_string(),
                android_id: "".to_string(),
                base_band: "".to_string(),
                inner_version: "".to_string()
            };
            let mut buf = Vec::new();
            prost::Message::encode(&device_info, &mut buf).unwrap();
            w.put_slice(&t144(
                self.device_info.imei.as_bytes(),
                &buf,
                self.device_info.os_type.as_bytes(),
                self.device_info.version.release.as_bytes(),
                self.device_info.sim_info.as_bytes(),
                self.device_info.apn.as_bytes(),
                false, true, false, guid_flag(),
                self.device_info.model.as_bytes(),
                &self.device_info.guid,
                self.device_info.brand.as_bytes(),
                &self.device_info.tgtgt_key,
            ));

            w.put_slice(&t154(seq));
            w.put_slice(&t147(16,
                              self.version.sort_version_name.as_bytes(),
                              &self.version.apk_sign));
            w.put_slice(&{
                let mut ww = Vec::new();
                ww.put_u16(0x16A);
                ww.write_bytes_short(t16a);
                ww
            });
            w.put_slice(&t154(seq));
            w.put_slice(&t141(self.device_info.sim_info.as_bytes(), self.device_info.apn.as_bytes()));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec!["tenpay.com", "openmobile.qq.com", "docs.qq.com", "connect.qq.com",
                                   "qzone.qq.com", "vip.qq.com", "gamecenter.qq.com", "qun.qq.com", "game.qq.com",
                                   "qqweb.qq.com", "office.qq.com", "ti.qq.com", "mail.qq.com", "mma.qq.com"]));
            w.put_slice(&t187(self.device_info.mac_address.as_bytes()));
            w.put_slice(&t188(self.device_info.android_id.as_bytes()));
            if self.device_info.imsi_md5.len() != 0 {
                w.put_slice(&t194(self.device_info.imsi_md5.as_slice()))
            }
            w.put_slice(&t191(0x00));
            if self.device_info.wifi_bssid.len() != 0 && self.device_info.wifi_ssid.len() != 0 {
                w.put_slice(&t202(self.device_info.wifi_bssid.as_bytes(), self.device_info.wifi_ssid.as_bytes()));
            }
            w.put_slice(&t177(self.version.build_time, self.version.sdk_version.as_str()));
            w.put_slice(&t516());
            w.put_slice(&t521(8));
            // let v:Vec<u8> = vec![0x01, 0x00];
            // w.put_slice(&t525(&t536(&v)));
            w.put_slice(&{
                let mut ww = Vec::new();
                ww.put_u16(0x318);
                ww.write_bytes_short(t318);
                ww
            });
            w
        });
        let sso: Vec<u8> = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", self.device_info.imei.as_str(), &[], self.out_going_packet_session_id.as_slice(), &req, self.ksid.as_slice());
        let packet: Vec<u8> = build_login_packet(self.uin as u32, 2, &vec![0; 16], &sso, &[]);
        (seq, packet)
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