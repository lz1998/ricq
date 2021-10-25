use byteorder::WriteBytesExt;
use bytes::BufMut;
use chrono::Utc;
use crate::client::Client;
use crate::packet::{build_code2d_request_packet, build_login_packet, build_oicq_request_packet, build_sso_packet};
use crate::tlv::{t16, t1b, t1d, t1f, t33, t35};
use crate::version::{ClientProtocol, gen_version_info};
use crate::binary_writer::BinaryWriter;

trait ClientPacket {
    fn build_qrcode_fetch_request_packet(&mut self) -> (u16, Vec<u8>);
    fn build_qrcode_result_query_request_packet(&mut self, sig: &[u8]) -> (u16, Vec<u8>);
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
}
