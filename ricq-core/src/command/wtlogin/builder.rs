use bytes::{BufMut, BytesMut};

use crate::binary::packet_writer::{PacketWriter, WriteLV};
use crate::binary::BinaryWriter;
use crate::command::wtlogin::builder::utils::*;
use crate::command::wtlogin::tlv_writer::*;
use crate::protocol::version::Protocol;
use crate::protocol::{
    oicq::{self, EncryptionMethod},
    packet::{EncryptType, Packet, PacketType},
};

impl super::super::super::Engine {
    // wtlogin.trans_emp
    pub fn build_qrcode_fetch_request_packet(&self) -> Packet {
        let transport = &self.transport;
        let seq = self.next_seq();
        let req = self.build_oicq_request_packet(0, 0x812, &{
            let mut w = BytesMut::new();
            let req_body = build_code2d_request_packet(0, 0, 0x31, &{
                let mut w = BytesMut::new();
                w.put_u16(0); // const
                w.put_u32(16); // app id
                w.put_u64(0); // const long user
                w.put_u8(8); // const
                w.write_short_lv([].as_slice());

                w.put_u16(6);
                t16(
                    transport.version.sso_version,
                    16, // app id ?
                    transport.version.sub_app_id,
                    &transport.sig.guid,
                    transport.version.apk_id,
                    transport.version.sort_version_name,
                    transport.version.apk_sign,
                )
                    .write(&mut w);
                t1b(0, 0, 3, 4, 72, 2, 2).write(&mut w);
                t1d(transport.version.misc_bitmap).write(&mut w);
                if matches!(transport.version.protocol, Protocol::MacOS) {
                    t1f(
                        false,
                        "Mac OSX",
                        "10",
                        "mac carrier",
                        &transport.device.apn,
                        2, // wifi
                    )
                        .write(&mut w);
                } else {
                    t1f(
                        false,
                        &transport.device.os_type,
                        "7.1.2",
                        &transport.device.sim_info,
                        &transport.device.apn,
                        2, // wifi
                    )
                        .write(&mut w);
                }
                t33(&transport.sig.guid).write(&mut w);
                if matches!(transport.version.protocol, Protocol::MacOS) {
                    t35(5).write(&mut w);
                } else {
                    t35(8).write(&mut w);
                }
                w
            });
            w.put_u8(0);
            w.put_u16(req_body.len() as u16);
            w.put_u32(transport.version.app_id);
            w.put_u32(114); // const role
            w.write_hex("000000");
            w.put_slice(&req_body);
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
            let req_body = build_code2d_request_packet(1, 0, 0x12, &{
                let mut w = Vec::new();
                w.put_u16(5);
                w.put_u8(1);
                w.put_u32(8); // 0x68 ?
                w.put_u32(16);
                w.write_short_lv(sig);
                w.put_u64(0);
                w.put_u8(8);
                w.write_short_lv([].as_slice());
                w.put_u16(0);
                w
            });
            w.put_u8(0);
            w.put_u16(req_body.len() as u16);
            w.put_u32(self.transport.version.app_id);
            w.put_u32(114); // const role
            w.write_hex("000000");
            w.put_slice(&req_body);
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
    pub fn build_qrcode_login_packet(&self, tmp_pwd: &[u8], tmp_no_pic_sig: &[u8], tgt_qr: &[u8]) -> Packet {
        let seq = self.next_seq();
        let transport = &self.transport;
        let req = self.build_oicq_request_packet(self.uin(), 0x0810, &{
            let mut w = BytesMut::new();
            w.put_u16(9);
            w.put_u16(24);

            t18(16, self.uin() as u32).write(&mut w);
            t1(self.uin() as u32, &transport.device.ip_address).write(&mut w);
            fn t106<B:BufMut+WriteLV>(tmp_pwd: &[u8]) -> impl PacketWriter<B> + '_ {
                move |buf: &mut B| {
                    buf.put_u16(0x106);
                    buf.write_short_lv(tmp_pwd);
                }
            }
            t106(tmp_pwd).write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
            t100(
                transport.version.sso_version,
                transport.version.sub_app_id,
                transport.version.main_sig_map,
            )
                .write(&mut w);
            t107(0).write(&mut w);
            t142(transport.version.apk_id).write(&mut w);
            t144(
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
            )
                .write(&mut w);

            t145(&transport.sig.guid).write(&mut w);
            t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            )
                .write(&mut w);
            t16a(tmp_no_pic_sig).write(&mut w);
            t154(seq).write(&mut w);
            t141(&transport.device.sim_info, &transport.device.apn).write(&mut w);
            t8(2052).write(&mut w);
            t511(vec![
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
            ])
                .write(&mut w);
            t187(&transport.device.mac_address).write(&mut w);
            t188(&transport.device.android_id).write(&mut w);
            if !transport.device.imsi_md5.is_empty() {
                t194(transport.device.imsi_md5.as_slice()).write(&mut w);
            }
            t191(0x00).write(&mut w);
            if !transport.device.wifi_bssid.is_empty() && !transport.device.wifi_ssid.is_empty() {
                t202(&transport.device.wifi_bssid, &transport.device.wifi_ssid).write(&mut w);
            }
            t177(transport.version.build_time, transport.version.sdk_version).write(&mut w);
            t516().write(&mut w);
            t521(8).write(&mut w);
            // let v:Vec<u8> = vec![0x01, 0x00];
            // w.put_slice(&t525(&t536(&v)));
            t318(tgt_qr).write(&mut w);
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

            t8(2052).write(&mut w);
            t104(&*transport.sig.t104).write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
            t401(&transport.sig.g).write(&mut w);
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
            t2(result, sign).write(&mut w);
            t8(2052).write(&mut w);
            t104(&transport.sig.t104).write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
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

            t8(2052).write(&mut w);
            t104(&transport.sig.t104).write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
            t174(&transport.sig.t174).write(&mut w);
            t17a(9).write(&mut w);
            t197().write(&mut w);
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

            t8(2052).write(&mut w);
            t104(&transport.sig.t104).write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
            t174(&transport.sig.t174).write(&mut w);
            t17c(code).write(&mut w);
            t401(&transport.sig.g).write(&mut w);
            t198().write(&mut w);
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

            t193(ticket).write(&mut w);
            t8(2052).write(&mut w);
            t104(&transport.sig.t104).write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
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

            t18(16, self.uin() as u32).write(&mut w);
            t1(self.uin() as u32, &transport.device.ip_address).write(&mut w);
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x106);
                w.write_short_lv(&*transport.sig.encrypted_a1);
                w
            });
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
            t100(
                transport.version.sso_version,
                2,
                transport.version.main_sig_map,
            )
                .write(&mut w);
            t107(0).write(&mut w);
            t144(
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
            )
                .write(&mut w);
            t142(transport.version.apk_id).write(&mut w);
            t145(&transport.sig.guid).write(&mut w);
            t16a(&transport.sig.srm_token).write(&mut w);
            t141(&transport.device.sim_info, &transport.device.apn).write(&mut w);
            t8(2052).write(&mut w);
            t511(vec![
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
            ])
                .write(&mut w);
            t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            )
                .write(&mut w);
            t177(transport.version.build_time, transport.version.sdk_version).write(&mut w);
            t400(
                &transport.sig.g,
                self.uin(),
                &transport.sig.guid,
                &transport.sig.dpwd,
                1,
                16,
                &transport.sig.rand_seed,
            )
                .write(&mut w);
            t187(&transport.device.mac_address).write(&mut w);
            t188(&transport.device.android_id).write(&mut w);
            t194(&transport.device.imsi_md5).write(&mut w);
            t202(&transport.device.wifi_bssid, &transport.device.wifi_ssid).write(&mut w);
            t516().write(&mut w);
            t521(0).write(&mut w);
            t525(t536(&[0x01, 0x00])).write(&mut w);
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
            t100(
                transport.version.sso_version,
                100,
                main_sig_map.unwrap_or(transport.version.main_sig_map),
            )
                .write(&mut w);
            t10a(&transport.sig.tgt).write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
            t108(&transport.sig.ksid).write(&mut w);
            let h = md5::compute(&transport.sig.d2key).to_vec();
            t144(
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
            )
                .write(&mut w);
            t143(&transport.sig.d2).write(&mut w);
            t142(transport.version.apk_id).write(&mut w);
            t154(seq).write(&mut w);
            t18(16, self.uin() as u32).write(&mut w);
            t141(&transport.device.sim_info, &transport.device.apn).write(&mut w);
            t8(2052).write(&mut w);
            t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            )
                .write(&mut w);
            t177(transport.version.build_time, transport.version.sdk_version).write(&mut w);
            t187(&transport.device.mac_address).write(&mut w);
            t188(&transport.device.android_id).write(&mut w);
            t194(&transport.device.imsi_md5).write(&mut w);
            t511(vec![
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
            ])
                .write(&mut w);
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

            t18(16, self.uin() as u32).write(&mut w);
            t1(self.uin() as u32, &transport.device.ip_address).write(&mut w);
            t106(
                self.uin() as u32,
                0,
                transport.version.app_id,
                transport.version.sso_version,
                password_md5,
                true,
                &transport.sig.guid,
                &transport.sig.tgtgt_key,
                0,
            )
                .write(&mut w);
            t116(transport.version.misc_bitmap, transport.version.sub_sig_map).write(&mut w);
            t100(
                transport.version.sso_version,
                transport.version.sub_app_id,
                transport.version.main_sig_map,
            )
                .write(&mut w);
            t107(0).write(&mut w);
            t142(transport.version.apk_id).write(&mut w);
            t144(
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
            )
                .write(&mut w);
            t145(&transport.sig.guid).write(&mut w);
            t147(
                16,
                transport.version.sort_version_name,
                transport.version.apk_sign,
            )
                .write(&mut w);
            t154(seq).write(&mut w);
            t141(&transport.device.sim_info, &transport.device.apn).write(&mut w);
            t8(2052).write(&mut w);
            t511(vec![
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
            ])
                .write(&mut w);

            t187(&transport.device.mac_address).write(&mut w);
            t188(&transport.device.android_id).write(&mut w);

            t194(&transport.device.imsi_md5).write(&mut w);

            if allow_slider {
                t191(0x82).write(&mut w);
            }
            t202(&transport.device.wifi_bssid, &transport.device.wifi_ssid).write(&mut w);
            t177(transport.version.build_time, transport.version.sdk_version).write(&mut w);
            t516().write(&mut w);
            t521(0).write(&mut w);
            t525(t536(&[0x01, 0x00])).write(&mut w);

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
    use std::time::UNIX_EPOCH;

    use crate::command::common::PbToBytes;
    use crate::pb;
    use crate::protocol::device::Device;

    pub fn build_code2d_request_packet(seq: u32, j: u64, cmd: u16, body: &[u8]) -> Bytes {
        let mut w = BytesMut::new();
        w.put_u32(UNIX_EPOCH.elapsed().unwrap().as_secs() as u32);
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
