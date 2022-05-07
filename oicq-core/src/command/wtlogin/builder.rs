use bytes::{BufMut, BytesMut};
use chrono::Utc;

use crate::binary::BinaryWriter;
use crate::command::wtlogin::builder::utils::*;
use crate::command::wtlogin::tlv_writer::*;
use crate::protocol::{
    oicq::{self, EncryptionMethod},
    packet::{EncryptType, Packet, PacketType},
    version::{get_version, Protocol},
};

impl super::super::super::Engine {
    // wtlogin.trans_emp
    pub fn build_qrcode_fetch_request_packet(&self) -> Packet {
        let watch = get_version(Protocol::AndroidWatch);
        let transport = &self.transport;
        let seq = self.next_seq();
        let req = self.build_oicq_request_packet(0, 0x812, &{
            let mut w = BytesMut::new();
            w.write_hex("0001110000001000000072000000");
            w.put_u32(Utc::now().timestamp() as u32);
            w.put_slice(&build_code2d_request_packet(0, 0, 0x31, &{
                let mut w = BytesMut::new();
                w.put_u16(0); // const
                w.put_u32(16); // app id
                w.put_u64(0); // const
                w.put_u8(8); // const
                w.write_bytes_short(&[]);

                w.put_u16(6);
                w.put_slice(&t16(
                    watch.sso_version,
                    16,
                    watch.app_id,
                    &transport.sig.guid,
                    watch.apk_id,
                    watch.sort_version_name,
                    watch.apk_sign,
                ));
                w.put_slice(&t1b(0, 0, 3, 4, 72, 2, 2));
                w.put_slice(&t1d(watch.misc_bitmap));
                w.put_slice(&t1f(
                    false,
                    &transport.device.os_type,
                    "7.1.2",
                    "China Mobile GSM",
                    &transport.device.apn,
                    2,
                ));
                w.put_slice(&t33(&transport.sig.guid));
                w.put_slice(&t35(8));
                w
            }));
            w
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.trans_emp".into(),
            ..Default::default()
        }
    }

    // wtlogin.trans_emp
    pub fn build_qrcode_result_query_request_packet(&self, sig: &[u8]) -> Packet {
        let seq = self.next_seq();
        let req = self.build_oicq_request_packet(0, 0x812, &{
            let mut w = BytesMut::new();
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
                w.write_bytes_short(&[]);
                w.put_u16(0);
                w
            }));
            w
        });

        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.trans_emp".into(),
            ..Default::default()
        }
    }

    // wtlogin.login
    pub fn build_qrcode_login_packet(&self, t106: &[u8], t16a: &[u8], t318: &[u8]) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x0810, &{
            let mut w = BytesMut::new();
            w.put_u16(9);
            w.put_u16(24);

            w.put_slice(&t18(16, self.uin() as u32));
            w.put_slice(&t1(self.uin() as u32, &transport.device.ip_address));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x106);
                w.write_bytes_short(t106);
                w
            });
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w.put_slice(&t100(
                transport.version.sso_version,
                transport.version.sub_app_id,
                transport.version.main_sig_map,
            ));
            w.put_slice(&t107(0));
            w.put_slice(&t142(transport.version.apk_id));
            w.put_slice(&t144(
                &transport.device.imei,
                &transport.device.gen_pb_data(),
                &transport.device.os_type,
                &transport.device.version.release,
                &transport.device.sim_info,
                &transport.device.apn,
                false,
                true,
                false,
                guid_flag(),
                &transport.device.model,
                &transport.sig.guid,
                &transport.device.brand,
                &transport.sig.tgtgt_key,
            ));

            w.put_slice(&t145(&transport.sig.guid));
            w.put_slice(&t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            ));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x16A);
                w.write_bytes_short(t16a);
                w
            });
            w.put_slice(&t154(seq));
            w.put_slice(&t141(&transport.device.sim_info, &transport.device.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec![
                "tenpay.com",
                "openmobile.qq.com",
                "docs.qq.com",
                "connect.qq.com",
                "qzone.qq.com",
                "vip.qq.com",
                "gamecenter.qq.com",
                "qun.qq.com",
                "game.qq.com",
                "qqweb.qq.com",
                "office.qq.com",
                "ti.qq.com",
                "mail.qq.com",
                "mma.qq.com",
            ]));
            w.put_slice(&t187(&transport.device.mac_address));
            w.put_slice(&t188(&transport.device.android_id));
            if !transport.device.imsi_md5.is_empty() {
                w.put_slice(&t194(transport.device.imsi_md5.as_slice()))
            }
            w.put_slice(&t191(0x00));
            if !transport.device.wifi_bssid.is_empty() && !transport.device.wifi_ssid.is_empty() {
                w.put_slice(&t202(
                    &transport.device.wifi_bssid,
                    &transport.device.wifi_ssid,
                ));
            }
            w.put_slice(&t177(
                transport.version.build_time,
                transport.version.sdk_version,
            ));
            w.put_slice(&t516());
            w.put_slice(&t521(8));
            // let v:Vec<u8> = vec![0x01, 0x00];
            // w.put_slice(&t525(&t536(&v)));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x318);
                w.write_bytes_short(t318);
                w
            });
            w
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.login".to_string(),
            uin: self.uin(),
            message: "".to_string(),
        }
    }

    // wtlogin.login
    pub fn build_device_lock_login_packet(&self) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x0810, &{
            let mut w = BytesMut::new();
            w.put_u16(20);
            w.put_u16(4);

            w.put_slice(&t8(2052));
            w.put_slice(&t104(&transport.sig.t104));
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w.put_slice(&t401(&transport.sig.g));
            w
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.login".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    // wtlogin.login
    pub fn build_captcha_packet(&self, result: String, sign: &[u8]) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x810, &{
            let mut w = BytesMut::new();
            w.put_u16(2); // sub command
            w.put_u16(4);

            w.put_slice(&t2(result, sign));
            w.put_slice(&t8(2052));
            w.put_slice(&t104(&transport.sig.t104));
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w
        });

        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.login".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    // wtlogin.login
    pub fn build_sms_request_packet(&self) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x810, &{
            let mut w = BytesMut::new();
            w.put_u16(8);
            w.put_u16(6);

            w.put_slice(&t8(2052));
            w.put_slice(&t104(&transport.sig.t104));
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w.put_slice(&t174(&transport.sig.t174));
            w.put_slice(&t17a(9));
            w.put_slice(&t197());
            w
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.login".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    // wtlogin.login
    pub fn build_sms_code_submit_packet(&self, code: &str) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x810, &{
            let mut w = BytesMut::new();
            w.put_u16(7);
            w.put_u16(7);

            w.put_slice(&t8(2052));
            w.put_slice(&t104(&transport.sig.t104));
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w.put_slice(&t174(&transport.sig.t174));
            w.put_slice(&t17c(code));
            w.put_slice(&t401(&transport.sig.g));
            w.put_slice(&t198());
            w
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.login".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    // wtlogin.login
    pub fn build_ticket_submit_packet(&self, ticket: &str) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x810, &{
            let mut w = BytesMut::new();
            w.put_u16(2);
            w.put_u16(4);

            w.put_slice(&t193(ticket));
            w.put_slice(&t8(2052));
            w.put_slice(&t104(&transport.sig.t104));
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.login".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    // wtlogin.exchange_emp
    pub fn build_request_tgtgt_no_pic_sig_packet(&self) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let codec = &self.transport.oicq_codec;
        let req = {
            let mut w = BytesMut::new();
            w.put_u16(15);
            w.put_u16(24);

            w.put_slice(&t18(16, self.uin() as u32));
            w.put_slice(&t1(self.uin() as u32, &transport.device.ip_address));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x106);
                w.write_bytes_short(&transport.sig.encrypted_a1);
                w
            });
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w.put_slice(&t100(
                transport.version.sso_version,
                2,
                transport.version.main_sig_map,
            ));
            w.put_slice(&t107(0));
            w.put_slice(&t144(
                &transport.device.android_id,
                &transport.device.gen_pb_data(),
                &transport.device.os_type,
                &transport.device.version.release,
                &transport.device.sim_info,
                &transport.device.apn,
                false,
                true,
                false,
                guid_flag(),
                &transport.device.model,
                &transport.sig.guid,
                &transport.device.brand,
                &transport.sig.tgtgt_key,
            ));
            w.put_slice(&t142(transport.version.apk_id));
            w.put_slice(&t145(&transport.sig.guid));
            w.put_slice(&t16a(&transport.sig.srm_token));
            w.put_slice(&t141(&transport.device.sim_info, &transport.device.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec![
                "tenpay.com",
                "openmobile.qq.com",
                "docs.qq.com",
                "connect.qq.com",
                "qzone.qq.com",
                "vip.qq.com",
                "gamecenter.qq.com",
                "qun.qq.com",
                "game.qq.com",
                "qqweb.qq.com",
                "office.qq.com",
                "ti.qq.com",
                "mail.qq.com",
                "mma.qq.com",
            ]));
            w.put_slice(&t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            ));
            w.put_slice(&t177(
                transport.version.build_time,
                transport.version.sdk_version,
            ));
            w.put_slice(&t400(
                &transport.sig.g,
                self.uin(),
                &transport.sig.guid,
                &transport.sig.dpwd,
                1,
                16,
                &transport.sig.rand_seed,
            ));
            w.put_slice(&t187(&transport.device.mac_address));
            w.put_slice(&t188(&transport.device.android_id));
            w.put_slice(&t194(&transport.device.imsi_md5));
            w.put_slice(&t202(
                &transport.device.wifi_bssid,
                &transport.device.wifi_ssid,
            ));
            w.put_slice(&t516());
            w.put_slice(&t521(0));
            w.put_slice(&t525(&t536(&[0x01, 0x00])));
            w.freeze()
        };
        let m = oicq::Message {
            uin: self.uin() as u32,
            command: 0x810,
            body: req,
            encryption_method: EncryptionMethod::ST,
        };
        Packet {
            packet_type: PacketType::Simple,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: codec.encode(m),
            command_name: "wtlogin.exchange_emp".into(),
            uin: self.uin(),
            message: "".to_string(),
        }
    }

    // wtlogin.exchange_emp
    pub fn build_request_change_sig_packet(&self, main_sig_map: Option<u32>) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x810, &{
            let mut w = BytesMut::new();
            w.put_u16(11);
            w.put_u16(17);

            w.put_slice(&t100(
                transport.version.sso_version,
                100,
                main_sig_map.unwrap_or(transport.version.main_sig_map),
            ));
            w.put_slice(&t10a(&transport.sig.tgt));
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w.put_slice(&t108(&transport.sig.ksid));
            let h = md5::compute(&transport.sig.d2key).to_vec();
            w.put_slice(&t144(
                &transport.device.android_id,
                &transport.device.gen_pb_data(),
                &transport.device.os_type,
                &transport.device.version.release,
                &transport.device.sim_info,
                &transport.device.apn,
                false,
                true,
                false,
                guid_flag(),
                &transport.device.model,
                &transport.sig.guid,
                &transport.device.brand,
                &h,
            ));
            w.put_slice(&t143(&transport.sig.d2));
            w.put_slice(&t142(transport.version.apk_id));
            w.put_slice(&t154(seq));
            w.put_slice(&t18(16, self.uin() as u32));
            w.put_slice(&t141(&transport.device.sim_info, &transport.device.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            ));
            w.put_slice(&t177(
                transport.version.build_time,
                transport.version.sdk_version,
            ));
            w.put_slice(&t187(&transport.device.mac_address));
            w.put_slice(&t188(&transport.device.android_id));
            w.put_slice(&t194(&transport.device.imsi_md5));
            w.put_slice(&t511(vec![
                "tenpay.com",
                "openmobile.qq.com",
                "docs.qq.com",
                "connect.qq.com",
                "qzone.qq.com",
                "vip.qq.com",
                "gamecenter.qq.com",
                "qun.qq.com",
                "game.qq.com",
                "qqweb.qq.com",
                "office.qq.com",
                "ti.qq.com",
                "mail.qq.com",
                "mma.qq.com",
            ]));
            // w.put_slice(&t202(self.device_info.wifi_bssid.as_bytes(), self.device_info.wifi_ssid.as_bytes()));
            w
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.exchange_emp".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }

    // wtlogin.login
    pub fn build_login_packet(&self, password_md5: &[u8], allow_slider: bool) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x0810, &{
            let mut w = BytesMut::new();
            w.put_u16(9);

            w.put_u16(if allow_slider { 0x17 } else { 0x16 });

            w.put_slice(&t18(16, self.uin() as u32));
            w.put_slice(&t1(self.uin() as u32, &transport.device.ip_address));
            w.put_slice(&t106(
                self.uin() as u32,
                0,
                transport.version.app_id,
                transport.version.sso_version,
                password_md5,
                true,
                &transport.sig.guid,
                &transport.sig.tgtgt_key,
                0,
            ));
            w.put_slice(&t116(
                transport.version.misc_bitmap,
                transport.version.sub_sig_map,
            ));
            w.put_slice(&t100(
                transport.version.sso_version,
                transport.version.sub_app_id,
                transport.version.main_sig_map,
            ));
            w.put_slice(&t107(0));
            w.put_slice(&t142(transport.version.apk_id));
            w.put_slice(&t144(
                &transport.device.imei,
                &transport.device.gen_pb_data(),
                &transport.device.os_type,
                &transport.device.version.release,
                &transport.device.sim_info,
                &transport.device.apn,
                false,
                true,
                false,
                guid_flag(),
                &transport.device.model,
                &transport.sig.guid,
                &transport.device.brand,
                &transport.sig.tgtgt_key,
            ));
            w.put_slice(&t145(&transport.sig.guid));
            w.put_slice(&t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            ));
            w.put_slice(&t154(seq));
            w.put_slice(&t141(&transport.device.sim_info, &transport.device.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec![
                "tenpay.com",
                "openmobile.qq.com",
                "docs.qq.com",
                "connect.qq.com",
                "qzone.qq.com",
                "vip.qq.com",
                "gamecenter.qq.com",
                "qun.qq.com",
                "game.qq.com",
                "qqweb.qq.com",
                "office.qq.com",
                "ti.qq.com",
                "mail.qq.com",
                "mma.qq.com",
            ]));

            w.put_slice(&t187(&transport.device.mac_address));
            w.put_slice(&t188(&transport.device.android_id));

            w.put_slice(&t194(&transport.device.imsi_md5));

            if allow_slider {
                w.put_slice(&t191(0x82));
            }
            w.put_slice(&t202(
                &transport.device.wifi_bssid,
                &transport.device.wifi_ssid,
            ));
            w.put_slice(&t177(
                transport.version.build_time,
                transport.version.sdk_version,
            ));
            w.put_slice(&t516());
            w.put_slice(&t521(0));
            w.put_slice(&t525(&t536(&[0x01, 0x00])));

            w.freeze()
        });
        Packet {
            packet_type: PacketType::Login,
            encrypt_type: EncryptType::EmptyKey,
            seq_id: seq as i32,
            body: req,
            command_name: "wtlogin.login".into(),
            uin: self.uin(),
            ..Default::default()
        }
    }
}

mod utils {
    use bytes::{BufMut, Bytes, BytesMut};

    use crate::command::common::PbToBytes;
    use crate::pb;
    use crate::protocol::device::Device;

    pub fn build_code2d_request_packet(seq: u32, j: u64, cmd: u16, body: &[u8]) -> Bytes {
        let mut w = BytesMut::new();
        w.put_u8(2);
        w.put_u16((43 + body.len() + 1) as u16);
        w.put_u16(cmd);
        w.put_slice(&[0; 21]);
        w.put_u8(3);
        w.put_u16(0);
        w.put_u16(50);
        w.put_u32(seq);
        w.put_u64(j);
        w.put_slice(body);
        w.put_u8(3);
        w.into()
    }

    pub trait DeviceToPb {
        fn gen_pb_data(&self) -> Bytes;
    }

    impl DeviceToPb for Device {
        fn gen_pb_data(&self) -> Bytes {
            pb::DeviceInfo {
                bootloader: self.bootloader.to_owned(),
                proc_version: self.proc_version.to_owned(),
                codename: self.version.codename.to_owned(),
                incremental: self.version.incremental.to_owned(),
                fingerprint: self.finger_print.to_owned(),
                boot_id: self.boot_id.to_owned(),
                android_id: self.android_id.to_owned(),
                base_band: self.base_band.to_owned(),
                inner_version: self.version.incremental.to_owned(),
            }
            .to_bytes()
        }
    }
}
