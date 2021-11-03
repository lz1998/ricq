use std::collections::HashMap;
use std::sync::atomic::Ordering;
use bytes::{BufMut, Bytes, BytesMut};
use chrono::Utc;
use crate::binary::BinaryWriter;
use crate::crypto::EncryptSession;
use crate::client::outcome::packet::*;
use crate::client::outcome::tlv::*;
use crate::client::version::{ClientProtocol, gen_version_info};
use crate::jce::*;
use jce_struct::*;
use crate::client::outcome::PbToBytes;
use crate::pb;
use crate::pb::structmsg::{FlagInfo, ReqSystemMsgNew};

fn pack_uni_request_data(data: &[u8]) -> Bytes {
    let mut r = BytesMut::new();
    r.put_slice(&[0x0A]);
    r.put_slice(data);
    r.put_slice(&[0x0B]);
    Bytes::from(r)
}

impl crate::client::Client {
    pub async fn build_qrcode_fetch_request_packet(&self) -> (u16, Bytes) {
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
                w.put_slice(&t16(watch.sso_version, 16, watch.app_id, &self.device_info.read().await.guid, watch.apk_id.as_bytes(), watch.sort_version_name.as_bytes(), &watch.apk_sign));
                w.put_slice(&t1b(0, 0, 3, 4, 72, 2, 2));
                w.put_slice(&t1d(watch.misc_bitmap));
                w.put_slice(&t1f(false, self.device_info.read().await.os_type.as_bytes(), "7.1.2".as_bytes(), "China Mobile GSM".as_bytes(), self.device_info.read().await.apn.as_bytes(), 2));
                w.put_slice(&t33(&self.device_info.read().await.guid));
                w.put_slice(&t35(8));
                w
            }));
            w
        });
        let sso = build_sso_packet(seq, watch.app_id, self.version.sub_app_id, "wtlogin.trans_emp", &self.device_info.read().await.imei, &vec![], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(0, 2, &vec![0; 16], &sso, &vec![]);
        return (seq, packet);
    }

    pub async fn build_qrcode_result_query_request_packet(&self, sig: &[u8]) -> (u16, Bytes) {
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
        let sso = build_sso_packet(seq, watch.app_id, self.version.sub_app_id, "wtlogin.trans_emp", &self.device_info.read().await.imei, &vec![], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(0, 2, &vec![0; 16], &sso, &vec![]);
        return (seq, packet);
    }

    pub async fn build_qrcode_login_packet(&self, t106: &[u8], t16a: &[u8], t318: &[u8]) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x0810, &self.ecdh, &self.random_key, &{
            let mut w = BytesMut::new();
            w.put_u16(9);
            w.put_u16(24);

            w.put_slice(&t18(16, self.uin.load(Ordering::SeqCst) as u32));
            w.put_slice(&t1(self.uin.load(Ordering::SeqCst) as u32, &self.device_info.read().await.ip_address));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x106);
                w.write_bytes_short(t106);
                w
            });
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t100(self.version.sso_version, self.version.sub_app_id, self.version.main_sig_map));
            w.put_slice(&t107(0));
            w.put_slice(&t142(self.version.apk_id.as_bytes()));
            w.put_slice(&t144(
                self.device_info.read().await.imei.as_bytes(),
                &self.device_info.read().await.gen_pb_data(),
                self.device_info.read().await.os_type.as_bytes(),
                self.device_info.read().await.version.release.as_bytes(),
                self.device_info.read().await.sim_info.as_bytes(),
                self.device_info.read().await.apn.as_bytes(),
                false, true, false, guid_flag(),
                self.device_info.read().await.model.as_bytes(),
                &self.device_info.read().await.guid,
                self.device_info.read().await.brand.as_bytes(),
                &self.device_info.read().await.tgtgt_key,
            ));

            w.put_slice(&t145(&self.device_info.read().await.guid));
            w.put_slice(&t147(16,
                              self.version.sort_version_name.as_bytes(),
                              &self.version.apk_sign));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x16A);
                w.write_bytes_short(t16a);
                w
            });
            w.put_slice(&t154(seq));
            w.put_slice(&t141(self.device_info.read().await.sim_info.as_bytes(), self.device_info.read().await.apn.as_bytes()));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec!["tenpay.com", "openmobile.qq.com", "docs.qq.com", "connect.qq.com",
                                   "qzone.qq.com", "vip.qq.com", "gamecenter.qq.com", "qun.qq.com", "game.qq.com",
                                   "qqweb.qq.com", "office.qq.com", "ti.qq.com", "mail.qq.com", "mma.qq.com"]));
            w.put_slice(&t187(self.device_info.read().await.mac_address.as_bytes()));
            w.put_slice(&t188(self.device_info.read().await.android_id.as_bytes()));
            if self.device_info.read().await.imsi_md5.len() != 0 {
                w.put_slice(&t194(self.device_info.read().await.imsi_md5.as_slice()))
            }
            w.put_slice(&t191(0x00));
            if self.device_info.read().await.wifi_bssid.len() != 0 && self.device_info.read().await.wifi_ssid.len() != 0 {
                w.put_slice(&t202(self.device_info.read().await.wifi_bssid.as_bytes(), self.device_info.read().await.wifi_ssid.as_bytes()));
            }
            w.put_slice(&t177(self.version.build_time, self.version.sdk_version.as_str()));
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
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_device_lock_login_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x0810, &self.ecdh, &self.random_key, &{
            let mut w = Vec::new();
            w.put_u16(20);
            w.put_u16(4);

            w.put_slice(&t8(2052));
            w.put_slice(&t104(&self.cache_info.read().await.t104));
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t401(&self.cache_info.read().await.g));
            w
        });
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &vec![0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_captcha_packet(&self, result: String, sign: &[u8]) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x810, &self.ecdh, &self.random_key, &{
            let mut w = Vec::new();
            w.put_u16(2); // sub command
            w.put_u16(4);

            w.put_slice(&t2(result, sign));
            w.put_slice(&t8(2052));
            w.put_slice(&t104(&self.cache_info.read().await.t104));
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w
        });
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &vec![0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_sms_request_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x810, &self.ecdh, &self.random_key, &{
            let mut w = Vec::new();
            w.put_u16(8);
            w.put_u16(6);

            w.put_slice(&t8(2052));
            w.put_slice(&t104(&self.cache_info.read().await.t104));
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t174(&self.cache_info.read().await.t174));
            w.put_slice(&t17a(9));
            w.put_slice(&t197());
            w
        });
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &vec![0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_sms_code_submit_packet(&self, code: String) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x810, &self.ecdh, &self.random_key, &{
            let mut w = Vec::new();
            w.put_u16(7);
            w.put_u16(7);

            w.put_slice(&t8(2052));
            w.put_slice(&t104(&self.cache_info.read().await.t104));
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t174(&self.cache_info.read().await.t174));
            w.put_slice(&t17c(code));
            w.put_slice(&t401(&self.cache_info.read().await.g));
            w.put_slice(&t198());
            w
        });
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_ticket_submit_packet(&self, ticket: String) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x810, &self.ecdh, &self.random_key, &{
            let mut w = Vec::new();
            w.put_u16(2);
            w.put_u16(4);

            w.put_slice(&t193(ticket));
            w.put_slice(&t8(2052));
            w.put_slice(&t104(&self.cache_info.read().await.t104));
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w
        });
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_request_tgtgt_no_pic_sig_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x810, &EncryptSession::new(&self.cache_info.read().await.sig_info.t133), &self.cache_info.read().await.sig_info.wt_session_ticket_key, &{
            let mut w = Vec::new();
            w.put_u16(15);
            w.put_u16(24);

            w.put_slice(&t18(16, self.uin.load(Ordering::SeqCst) as u32));
            w.put_slice(&t1(self.uin.load(Ordering::SeqCst) as u32, &self.device_info.read().await.ip_address));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x106);
                w.write_bytes_short(&self.cache_info.read().await.sig_info.encrypted_a1);
                w
            });
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t100(self.version.sso_version, 2, self.version.main_sig_map));
            w.put_slice(&t107(0));
            w.put_slice(&t144(
                self.device_info.read().await.android_id.as_bytes(),
                &self.device_info.read().await.gen_pb_data(),
                self.device_info.read().await.os_type.as_bytes(),
                self.device_info.read().await.version.release.as_bytes(),
                self.device_info.read().await.sim_info.as_bytes(),
                self.device_info.read().await.apn.as_bytes(),
                false, true, false, guid_flag(),
                self.device_info.read().await.model.as_bytes(),
                &self.device_info.read().await.guid,
                self.device_info.read().await.brand.as_bytes(),
                &self.device_info.read().await.tgtgt_key,
            ));
            w.put_slice(&t142(self.version.apk_id.as_bytes()));
            w.put_slice(&t145(&self.device_info.read().await.guid));
            w.put_slice(&t16a(&self.cache_info.read().await.sig_info.srm_token));
            w.put_slice(&t141(self.device_info.read().await.sim_info.as_bytes(), self.device_info.read().await.apn.as_bytes()));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec!["tenpay.com", "openmobile.qq.com", "docs.qq.com", "connect.qq.com",
                                   "qzone.qq.com", "vip.qq.com", "gamecenter.qq.com", "qun.qq.com", "game.qq.com",
                                   "qqweb.qq.com", "office.qq.com", "ti.qq.com", "mail.qq.com", "mma.qq.com"]
            ));
            w.put_slice(&t147(16, self.version.sort_version_name.as_bytes(), &self.version.apk_sign));
            w.put_slice(&t177(self.version.build_time, &self.version.sdk_version));
            w.put_slice(&t400(&self.cache_info.read().await.g, self.uin.load(Ordering::SeqCst), &self.device_info.read().await.guid, &self.cache_info.read().await.dpwd, 1, 16, &self.random_key));
            w.put_slice(&t187(self.device_info.read().await.mac_address.as_bytes()));
            w.put_slice(&t188(self.device_info.read().await.android_id.as_bytes()));
            w.put_slice(&t194(&self.device_info.read().await.imsi_md5));
            w.put_slice(&t202(self.device_info.read().await.wifi_bssid.as_bytes(), self.device_info.read().await.wifi_ssid.as_bytes()));
            w.put_slice(&t516());
            w.put_slice(&t521(0));
            w.put_slice(&t525(&t536(&vec![0x01, 0x00])));
            w
        });
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "wtlogin.exchange_emp", 2, &self.out_going_packet_session_id.read().await, &[], &[0; 16], &req);
        (seq, packet)
    }

    pub async fn build_request_change_sig_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x810, &self.ecdh, &self.random_key, &{
            let mut w = BytesMut::new();
            w.put_u16(11);
            w.put_u16(17);

            w.put_slice(&t100(self.version.sso_version, 100, self.version.main_sig_map));
            w.put_slice(&t10a(&self.cache_info.read().await.sig_info.tgt));
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t108(&self.device_info.read().await.imei));
            let h = md5::compute(&self.cache_info.read().await.sig_info.d2key).to_vec();
            w.put_slice(&t144(
                self.device_info.read().await.android_id.as_bytes(),
                &self.device_info.read().await.gen_pb_data(),
                self.device_info.read().await.os_type.as_bytes(),
                self.device_info.read().await.version.release.as_bytes(),
                self.device_info.read().await.sim_info.as_bytes(),
                self.device_info.read().await.apn.as_bytes(),
                false, true, false, guid_flag(),
                self.device_info.read().await.model.as_bytes(),
                &self.device_info.read().await.guid,
                self.device_info.read().await.brand.as_bytes(),
                &h,
            ));
            w.put_slice(&t143(&self.cache_info.read().await.sig_info.d2));
            w.put_slice(&t142(self.version.apk_id.as_bytes()));
            w.put_slice(&t154(seq));
            w.put_slice(&t18(16, self.uin.load(Ordering::SeqCst) as u32));
            w.put_slice(&t141(self.device_info.read().await.sim_info.as_bytes(), self.device_info.read().await.apn.as_bytes()));
            w.put_slice(&t8(2052));
            w.put_slice(&t147(16, self.version.sort_version_name.as_bytes(), &self.version.apk_sign));
            w.put_slice(&t177(self.version.build_time, &self.version.sdk_version));
            w.put_slice(&t187(self.device_info.read().await.mac_address.as_bytes()));
            w.put_slice(&t188(self.device_info.read().await.android_id.as_bytes()));
            w.put_slice(&t194(&self.device_info.read().await.imsi_md5));
            w.put_slice(&t511(vec!["tenpay.com", "openmobile.qq.com", "docs.qq.com", "connect.qq.com",
                                   "qzone.qq.com", "vip.qq.com", "gamecenter.qq.com", "qun.qq.com", "game.qq.com",
                                   "qqweb.qq.com", "office.qq.com", "ti.qq.com", "mail.qq.com", "mma.qq.com"]
            ));
            // w.put_slice(&t202(self.device_info.wifi_bssid.as_bytes(), self.device_info.wifi_ssid.as_bytes()));
            w
        });
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.exchange_emp", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_client_register_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();

        let mut svc = SvcReqRegister {
            uin: self.uin.load(Ordering::SeqCst),
            bid: 1 | 2 | 4,
            conn_type: 0,
            status: 11,
            kick_pc: 0,
            kick_weak: 0,
            ios_version: self.device_info.read().await.version.sdk as i64,
            net_type: 1,
            reg_type: 0,
            guid: self.device_info.read().await.guid.to_owned(),
            is_set_status: 0,
            locale_id: 2052,
            dev_name: self.device_info.read().await.model.to_owned(),
            dev_type: self.device_info.read().await.model.to_owned(),
            os_ver: self.device_info.read().await.version.release.to_owned(),
            open_push: 1,
            large_seq: 1551,
            old_sso_ip: 0,
            new_sso_ip: 31806887127679168,
            channel_no: "".to_string(),
            cpid: 0,
            vendor_name: self.device_info.read().await.vendor_name.to_owned(),
            vendor_os_name: self.device_info.read().await.vendor_os_name.to_owned(),
            b769: Bytes::from_static(&[0x0A, 0x04, 0x08, 0x2E, 0x10, 0x00, 0x0A, 0x05, 0x08, 0x9B, 0x02, 0x10, 0x00]),
            set_mute: 0,
            // empty
            other: "".to_string(),
            online_push: 0,
            is_online: 0,
            is_show_online: 0,
            timestamp: 0,
            build_ver: "".to_string(),
            dev_param: Default::default(),
            silent_push: 0,
            last_watch_start_time: 0,
            ios_idfa: "".to_string(),
            ext_online_status: 0,
            server_buf: Default::default(),
            battery_status: 0,
        };
        let mut b = BytesMut::new();
        b.put_slice(&[0x0A]);
        b.put_slice(&svc.build());
        b.put_slice(&[0x0B]);
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("SvcReqRegister".to_string(), b.into())
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "PushService".to_string(),
            s_func_name: "SvcReqRegister".to_string(),
            s_buffer: buf.build(),
            context: Default::default(),
            status: Default::default(),
            // empty
            c_packet_type: 0,
            i_message_type: 0,
            i_request_id: 0,
            i_timeout: 0,
        };
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "StatSvc.register", &self.device_info.read().await.imei, &self.cache_info.read().await.sig_info.tgt, &self.out_going_packet_session_id.read().await, &pkt.build(), &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 1, &self.cache_info.read().await.sig_info.d2key, &sso, &self.cache_info.read().await.sig_info.d2);

        return (seq, packet);
    }

    pub async fn build_friend_group_list_request_packet(&self, friend_start_index: i16, friend_list_count: i16, group_start_index: i16, group_list_count: i16) -> (u16, Bytes) {
        let seq = self.next_seq();
        let mut d50 = BytesMut::new();
        prost::Message::encode(&pb::D50ReqBody {
            appid: 1002,
            req_music_switch: 1,
            req_mutualmark_alienation: 1,
            req_ksing_switch: 1,
            req_mutualmark_lbsshare: 1,
            // empty
            max_pkg_size: 0,
            start_time: 0,
            start_index: 0,
            req_num: 0,
            uin_list: vec![],
            req_mutualmark_score: 0,
        }, &mut d50).unwrap();

        let req = FriendListRequest {
            reqtype: 3,
            if_reflush: if friend_start_index <= 0 { 0 } else { 1 },
            uin: self.uin.load(Ordering::SeqCst),
            start_index: friend_start_index,
            friend_count: friend_list_count,
            group_id: 0,
            if_get_group_info: if group_list_count <= 0 { 0 } else { 1 },
            group_start_index: group_start_index as u8,
            group_count: group_list_count as u8,
            if_get_msf_group: 0,
            if_show_term_type: 1,
            version: 27,
            uin_list: vec![],
            app_type: 0,
            if_get_dov_id: 0,
            if_get_both_flag: 0,
            d50: Bytes::from(d50),
            d6b: Bytes::new(),
            sns_type_list: vec![13580, 13581, 13582],
        };
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("FL".to_string(), pack_uni_request_data(&req.build()))
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            c_packet_type: 0x003,
            i_request_id: 1921334514,
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetFriendListReq".to_string(),
            s_buffer: buf.build(),
            context: Default::default(),
            status: Default::default(),
            // empty
            i_message_type: 0,
            i_timeout: 0,
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "friendlist.getFriendGroupList", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        (seq, packet)
    }

    pub async fn build_system_msg_new_group_packet(&self, suspicious: bool) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = ReqSystemMsgNew {
            msg_num: 100,
            version: 1000,
            checktype: 3,
            flag: Some(FlagInfo {
                grp_msg_kick_admin: 1,
                grp_msg_hidden_grp: 1,
                grp_msg_wording_down: 1,
                grp_msg_get_official_account: 1,
                grp_msg_get_pay_in_group: 1,
                frd_msg_discuss2_many_chat: 1,
                grp_msg_not_allow_join_grp_invite_not_frd: 1,
                frd_msg_need_waiting_msg: 1,
                frd_msg_uint32_need_all_unread_msg: 1,
                grp_msg_need_auto_admin_wording: 1,
                grp_msg_get_transfer_group_msg_flag: 1,
                grp_msg_get_quit_pay_group_msg_flag: 1,
                grp_msg_support_invite_auto_join: 1,
                grp_msg_mask_invite_auto_join: 1,
                grp_msg_get_disbanded_by_admin: 1,
                grp_msg_get_c2c_invite_join_group: 1,
                //empty
                frd_msg_get_busi_card: 0,
            }),
            friend_msg_type_flag: 1,
            req_msg_type: if suspicious { 2 } else { 1 },
            // empty
            language: 0,
            latest_friend_seq: 0,
            latest_group_seq: 0,
            is_get_frd_ribbon: false,
            is_get_grp_ribbon: false,
        };
        let payload = req.to_bytes();
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "ProfileService.Pb.ReqSystemMsgNew.Group", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &payload);
        (seq, packet)
    }
}


#[cfg(test)]
mod tests {
    use bytes::BufMut;
    use chrono::Utc;
    use rand::distributions::Alphanumeric;
    use rand::{Rng, thread_rng};

    #[test]
    fn test_read() {}
}