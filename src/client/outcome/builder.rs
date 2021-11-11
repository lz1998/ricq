use std::collections::HashMap;
use std::sync::atomic::Ordering;
use bytes::{BufMut, Bytes, BytesMut};
use chrono::Utc;
use rand::seq;
use crate::binary::BinaryWriter;
use crate::crypto::EncryptSession;
use crate::client::outcome::packet::*;
use crate::client::outcome::tlv::*;
use crate::client::version::{ClientProtocol, gen_version_info};
use crate::jce::*;
use jce_struct::*;
use crate::client::outcome::PbToBytes;
use crate::pb;
use crate::pb::{GroupMemberReqBody, msg};
use crate::pb::msg::{GetMessageRequest, PbC2cReadedReportReq, PbGroupReadedReportReq, PbMsgReadedReportReq, UinPairReadInfo};
use crate::pb::oidb::{D769ConfigSeq, D769RspBody, D88dGroupExInfoOnly, D88dGroupHeadPortrait, D88dGroupInfo, D88dReqBody, OidbssoPkg, ReqGroupInfo};
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
                w.put_slice(&t16(watch.sso_version, 16, watch.app_id, &self.device_info.read().await.guid, &watch.apk_id, &watch.sort_version_name, &watch.apk_sign));
                w.put_slice(&t1b(0, 0, 3, 4, 72, 2, 2));
                w.put_slice(&t1d(watch.misc_bitmap));
                w.put_slice(&t1f(false, &self.device_info.read().await.os_type, "7.1.2", "China Mobile GSM", &self.device_info.read().await.apn, 2));
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
            w.put_slice(&t142(&self.version.apk_id));
            w.put_slice(&t144(
                &self.device_info.read().await.imei,
                &self.device_info.read().await.gen_pb_data(),
                &self.device_info.read().await.os_type,
                &self.device_info.read().await.version.release,
                &self.device_info.read().await.sim_info,
                &self.device_info.read().await.apn,
                false, true, false, guid_flag(),
                &self.device_info.read().await.model,
                &self.device_info.read().await.guid,
                &self.device_info.read().await.brand,
                &self.device_info.read().await.tgtgt_key,
            ));

            w.put_slice(&t145(&self.device_info.read().await.guid));
            w.put_slice(&t147(16,
                              &self.version.sort_version_name,
                              &self.version.apk_sign));
            w.put_slice(&{
                let mut w = Vec::new();
                w.put_u16(0x16A);
                w.write_bytes_short(t16a);
                w
            });
            w.put_slice(&t154(seq));
            w.put_slice(&t141(&self.device_info.read().await.sim_info, &self.device_info.read().await.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec!["tenpay.com", "openmobile.qq.com", "docs.qq.com", "connect.qq.com",
                                   "qzone.qq.com", "vip.qq.com", "gamecenter.qq.com", "qun.qq.com", "game.qq.com",
                                   "qqweb.qq.com", "office.qq.com", "ti.qq.com", "mail.qq.com", "mma.qq.com"]));
            w.put_slice(&t187(&self.device_info.read().await.mac_address));
            w.put_slice(&t188(&self.device_info.read().await.android_id));
            if self.device_info.read().await.imsi_md5.len() != 0 {
                w.put_slice(&t194(self.device_info.read().await.imsi_md5.as_slice()))
            }
            w.put_slice(&t191(0x00));
            if self.device_info.read().await.wifi_bssid.len() != 0 && self.device_info.read().await.wifi_ssid.len() != 0 {
                w.put_slice(&t202(&self.device_info.read().await.wifi_bssid, &self.device_info.read().await.wifi_ssid));
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_sms_code_submit_packet(&self, code: &str) -> (u16, Bytes) {
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

    pub async fn build_ticket_submit_packet(&self, ticket: &str) -> (u16, Bytes) {
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
                &self.device_info.read().await.android_id,
                &self.device_info.read().await.gen_pb_data(),
                &self.device_info.read().await.os_type,
                &self.device_info.read().await.version.release,
                &self.device_info.read().await.sim_info,
                &self.device_info.read().await.apn,
                false, true, false, guid_flag(),
                &self.device_info.read().await.model,
                &self.device_info.read().await.guid,
                &self.device_info.read().await.brand,
                &self.device_info.read().await.tgtgt_key,
            ));
            w.put_slice(&t142(&self.version.apk_id));
            w.put_slice(&t145(&self.device_info.read().await.guid));
            w.put_slice(&t16a(&self.cache_info.read().await.sig_info.srm_token));
            w.put_slice(&t141(&self.device_info.read().await.sim_info, &self.device_info.read().await.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec!["tenpay.com", "openmobile.qq.com", "docs.qq.com", "connect.qq.com",
                                   "qzone.qq.com", "vip.qq.com", "gamecenter.qq.com", "qun.qq.com", "game.qq.com",
                                   "qqweb.qq.com", "office.qq.com", "ti.qq.com", "mail.qq.com", "mma.qq.com"]
            ));
            w.put_slice(&t147(16, &self.version.sort_version_name, &self.version.apk_sign));
            w.put_slice(&t177(self.version.build_time, &self.version.sdk_version));
            w.put_slice(&t400(&self.cache_info.read().await.g, self.uin.load(Ordering::SeqCst), &self.device_info.read().await.guid, &self.cache_info.read().await.dpwd, 1, 16, &self.random_key));
            w.put_slice(&t187(&self.device_info.read().await.mac_address));
            w.put_slice(&t188(&self.device_info.read().await.android_id));
            w.put_slice(&t194(&self.device_info.read().await.imsi_md5));
            w.put_slice(&t202(&self.device_info.read().await.wifi_bssid, &self.device_info.read().await.wifi_ssid));
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
                &self.device_info.read().await.android_id,
                &self.device_info.read().await.gen_pb_data(),
                &self.device_info.read().await.os_type,
                &self.device_info.read().await.version.release,
                &self.device_info.read().await.sim_info,
                &self.device_info.read().await.apn,
                false, true, false, guid_flag(),
                &self.device_info.read().await.model,
                &self.device_info.read().await.guid,
                &self.device_info.read().await.brand,
                &h,
            ));
            w.put_slice(&t143(&self.cache_info.read().await.sig_info.d2));
            w.put_slice(&t142(&self.version.apk_id));
            w.put_slice(&t154(seq));
            w.put_slice(&t18(16, self.uin.load(Ordering::SeqCst) as u32));
            w.put_slice(&t141(&self.device_info.read().await.sim_info, &self.device_info.read().await.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t147(16, &self.version.sort_version_name, &self.version.apk_sign));
            w.put_slice(&t177(self.version.build_time, &self.version.sdk_version));
            w.put_slice(&t187(&self.device_info.read().await.mac_address));
            w.put_slice(&t188(&self.device_info.read().await.android_id));
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

        let svc = SvcReqRegister {
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
            ..Default::default()
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
            ..Default::default()
        };
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "StatSvc.register", &self.device_info.read().await.imei, &self.cache_info.read().await.sig_info.tgt, &self.out_going_packet_session_id.read().await, &pkt.build(), &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 1, &self.cache_info.read().await.sig_info.d2key, &sso, &self.cache_info.read().await.sig_info.d2);

        return (seq, packet);
    }

    // TODO 还没测试
    pub async fn build_heartbeat_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "Heartbeat.Alive", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 0, &[], &sso, &[]);
        (seq, packet)
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
            ..Default::default()
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
            ..Default::default()
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
                ..Default::default()
            }),
            friend_msg_type_flag: 1,
            req_msg_type: if suspicious { 2 } else { 1 },
            ..Default::default()
        };
        let payload = req.to_bytes();
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "ProfileService.Pb.ReqSystemMsgNew.Group", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &payload);
        (seq, packet)
    }

    pub async fn build_login_packet(&self, allow_slider: bool) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst), 0x0810, &self.ecdh, &self.random_key, &{
            let mut w = BytesMut::new();
            w.put_u16(9);

            w.put_u16(if allow_slider { 0x17 } else { 0x16 });

            w.put_slice(&t18(16, self.uin.load(Ordering::SeqCst) as u32));
            w.put_slice(&t1(self.uin.load(Ordering::SeqCst) as u32, &self.device_info.read().await.ip_address));
            w.put_slice(&t106(self.uin.load(Ordering::SeqCst) as u32, 0, self.version.app_id, self.version.sso_version, &self.password_md5, true, &self.device_info.read().await.guid, &self.device_info.read().await.tgtgt_key, 0));
            w.put_slice(&t116(self.version.misc_bitmap, self.version.sub_sig_map));
            w.put_slice(&t100(self.version.sso_version, self.version.sub_app_id, self.version.main_sig_map));
            w.put_slice(&t107(0));
            w.put_slice(&t142(&self.version.apk_id));
            w.put_slice(&t144(
                &self.device_info.read().await.imei,
                &self.device_info.read().await.gen_pb_data(),
                &self.device_info.read().await.os_type,
                &self.device_info.read().await.version.release,
                &self.device_info.read().await.sim_info,
                &self.device_info.read().await.apn,
                false, true, false, guid_flag(),
                &self.device_info.read().await.model,
                &self.device_info.read().await.guid,
                &self.device_info.read().await.brand,
                &self.device_info.read().await.tgtgt_key,
            ));
            w.put_slice(&t145(&self.device_info.read().await.guid));
            w.put_slice(&t147(16, &self.version.sort_version_name, &self.version.apk_sign));
            w.put_slice(&t154(seq));
            w.put_slice(&t141(&self.device_info.read().await.sim_info, &self.device_info.read().await.apn));
            w.put_slice(&t8(2052));
            w.put_slice(&t511(vec![
                "tenpay.com", "openmobile.qq.com", "docs.qq.com", "connect.qq.com",
                "qzone.qq.com", "vip.qq.com", "gamecenter.qq.com", "qun.qq.com", "game.qq.com",
                "qqweb.qq.com", "office.qq.com", "ti.qq.com", "mail.qq.com", "mma.qq.com",
            ]));

            w.put_slice(&t187(&self.device_info.read().await.mac_address));
            w.put_slice(&t188(&self.device_info.read().await.android_id));

            if self.device_info.read().await.imsi_md5.len() != 0 {
                w.put_slice(&t194(&self.device_info.read().await.imsi_md5))
            }

            if allow_slider {
                w.put_slice(&t191(0x82));
            }
            if self.device_info.read().await.wifi_bssid.len() != 0 && self.device_info.read().await.wifi_ssid.len() != 0 {
                w.put_slice(&t202(&self.device_info.read().await.wifi_bssid, &self.device_info.read().await.wifi_ssid));
            }
            w.put_slice(&t177(self.version.build_time, &self.version.sdk_version));
            w.put_slice(&t516());
            w.put_slice(&t521(0));
            w.put_slice(&t525(&t536(&[0x01, 0x00])));

            w.freeze()
        });
        let sso = build_sso_packet(seq, self.version.app_id, self.version.sub_app_id, "wtlogin.login", &self.device_info.read().await.imei, &[], &self.out_going_packet_session_id.read().await, &req, &self.cache_info.read().await.ksid);
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst), 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_group_list_request_packet(&self, vec_cookie: &[u8]) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = TroopListRequest {
            uin: self.uin.load(Ordering::SeqCst),
            get_msf_msg_flag: 1,
            cookies: Bytes::from(vec_cookie.to_vec()),
            group_info: vec![],
            group_flag_ext: 1,
            version: 7,
            company_id: 0,
            version_num: 1,
            get_long_group_name: 1,
        };
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("GetTroopListReqV2Simplify".to_string(), pack_uni_request_data(&req.build()))
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            c_packet_type: 0x00,
            i_message_type: 0,
            i_request_id: self.next_packet_seq(),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetTroopListReqV2Simplify".to_string(),
            s_buffer: buf.build(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "friendlist.GetTroopListReqV2", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        (seq, packet)
    }

    pub async fn build_group_sending_packet(&self, group_code: i64, r: i32, pkg_num: i32, pkg_index: i32, pkg_div: i32, forward: bool, elems: Vec<msg::Elem>) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = msg::SendMessageRequest {
            routing_head: Some(msg::RoutingHead {
                c2c: None,
                grp: Some(msg::Grp { group_code: Some(group_code) }),
                grp_tmp: None,
                wpa_tmp: None,
            }),
            content_head: Some(msg::ContentHead {
                pkg_num: Some(pkg_num),
                pkg_index: Some(pkg_index),
                div_seq: Some(pkg_div),
                auto_reply: None,
            }),
            msg_body: Some(msg::MessageBody {
                rich_text: Some(msg::RichText {
                    elems,
                    attr: None,
                    not_online_file: None,
                    ptt: None,
                }),
                msg_content: None,
                msg_encrypt_content: None,
            }),
            msg_seq: Some(self.next_group_seq()),
            msg_rand: Some(r),
            sync_cookie: Some(Vec::new()),
            msg_via: Some(1),
            msg_ctrl: if forward { Some(msg::MsgCtrl { msg_flag: Some(4) }) } else { None },
            data_statist: None,
            multi_send_seq: None,
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "MessageSvc.PbSendMsg", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &req.to_bytes());
        (seq, packet)
    }

    pub async fn build_group_member_info_request_packet(&self, group_code: i64, uin: i64) -> (u16, Bytes) {
        let seq = self.next_seq();
        let payload = GroupMemberReqBody {
            group_code,
            uin,
            new_client: true,
            client_type: 1,
            rich_card_name_ver: 1,
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "group_member_card.get_group_member_card_info", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &payload.to_bytes());
        (seq, packet)
    }

    pub async fn build_group_member_list_request_packet(&self, group_uin: i64, group_code: i64, next_uin: i64) -> (u16, Bytes) {
        let seq = self.next_seq();
        let payload = TroopMemberListRequest {
            uin: self.uin.load(Ordering::SeqCst),
            group_code,
            next_uin,
            group_uin,
            version: 2,
            ..Default::default()
        };
        let mut b = BytesMut::new();
        b.put_slice(&[0x0A]);
        b.put_slice(&payload.build());
        b.put_slice(&[0x0B]);
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("GTML".to_string(), b.into())
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            i_request_id: self.next_packet_seq(),
            s_servant_name: "mqq.IMService.FriendListServiceServantObj".to_string(),
            s_func_name: "GetTroopMemberListReq".to_string(),
            s_buffer: buf.build(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "friendlist.GetTroopMemberListReq", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        return (seq, packet);
    }

    pub async fn build_delete_online_push_packet(&self, uin: i64, svrip: i32, push_token: Bytes, seq: u16, del_msg: Vec<PushMessageInfo>) -> (u16, Bytes) {
        let mut req = SvcRespPushMsg {
            uin,
            svrip,
            push_token,
            ..Default::default()
        };
        for m in del_msg {
            req.del_infos.push(DelMsgInfo {
                from_uin: m.from_uin,
                msg_time: m.msg_time,
                msg_seq: m.msg_seq,
                msg_cookies: m.msg_cookies,
                ..Default::default()
            })
        }
        let b = pack_uni_request_data(&req.build());
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("resp".to_string(), b.into())
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            i_request_id: seq as i32,
            s_servant_name: "OnlinePush".to_string(),
            s_func_name: "SvcRespPushMsg".to_string(),
            s_buffer: buf.build(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "OnlinePush.RespPush", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        (seq, packet)
    }

    pub async fn build_conf_push_resp_packet(&self, t: i32, pkt_seq: i64, jce_buf: Bytes) -> (u16, Bytes) {
        let seq = self.next_seq();
        let mut req = JceMut::new();
        req.put_i32(t, 1);
        req.put_i64(pkt_seq, 2);
        req.put_bytes(jce_buf, 3);

        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("PushResp".to_string(), pack_uni_request_data(&req.freeze()))
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "QQService.ConfigPushSvc.MainServant".to_string(),
            s_func_name: "PushResp".to_string(),
            s_buffer: buf.build(),
            context: Default::default(),
            status: Default::default(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "ConfigPushSvc.PushResp", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        (seq, packet)
    }

    pub async fn build_get_offline_msg_request_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let reg_req = SvcReqRegisterNew {
            request_optional: 0x101C2 | 32,
            c2c_msg: SvcReqGetMsgV2 {
                uin: self.uin.load(Ordering::SeqCst),
                date_time: match self.last_message_time.load(Ordering::SeqCst) {
                    0 => { 1 }
                    _ => { self.last_message_time.load(Ordering::SeqCst) as i32 }
                },
                recive_pic: 1,
                ability: 15,
                channel: 4,
                inst: 1,
                channel_ex: 1,
                sync_cookie: Bytes::from(self.cache_info.read().await.sync_cookie.to_vec()),
                sync_flag: 0,
                ramble_flag: 0,
                general_abi: 1,
                pub_account_cookie: Bytes::from(self.cache_info.read().await.pub_account_cookie.to_vec()),
            },
            group_msg: SvcReqPullGroupMsgSeq {
                verify_type: 0,
                filter: 1,
                ..Default::default()
            },
            end_seq: Utc::now().timestamp(),
            ..Default::default()
        };
        let flag = 0; // flag := msg.SyncFlag_START
        let msg_req = GetMessageRequest {
            sync_flag: Option::from(flag),
            sync_cookie: Option::from(self.cache_info.read().await.sync_cookie.to_vec()),
            ramble_flag: Option::from(0),
            context_flag: Option::from(1),
            online_sync_flag: Option::from(0),
            latest_ramble_number: Option::from(20),
            other_ramble_number: Option::from(3),
            ..Default::default()
        }.to_bytes();
        let mut buf = BytesMut::new();
        buf.put_slice(&[0, 0, 0, 0]);
        buf.put_slice(&msg_req);
        let buf = buf.freeze();
        let mut req = JceMut::new();
        req.put_bytes(buf, 0);
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("req_PbOffMsg".to_string(), req.freeze()),
                ("req_OffMsg".to_string(), pack_uni_request_data(&reg_req.build()))
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "RegPrxySvc".to_string(),
            s_buffer: buf.build(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "RegPrxySvc.getOffMsg", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        (seq, packet)
    }

    pub async fn build_sync_msg_request_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let oidb_req = D769RspBody {
            config_list: vec![
                D769ConfigSeq {
                    r#type: Some(46),
                    version: Some(0),
                },
                D769ConfigSeq {
                    r#type: Some(283),
                    version: Some(0),
                },
            ],
            ..Default::default()
        }.to_bytes();
        let reg_req = SvcReqRegisterNew {
            request_optional: 128 | 64 | 256 | 2 | 8192 | 16384 | 65536,
            dis_group_msg_filter: 1,
            c2c_msg: SvcReqGetMsgV2 {
                uin: self.uin.load(Ordering::SeqCst),
                date_time: match self.last_message_time.load(Ordering::SeqCst) {
                    0 => { 1 }
                    _ => { self.last_message_time.load(Ordering::SeqCst) as i32 }
                },
                recive_pic: 1,
                ability: 15,
                channel: 4,
                inst: 1,
                channel_ex: 1,
                sync_cookie: Bytes::from(self.cache_info.read().await.sync_cookie.to_vec()),
                sync_flag: 0, // START
                ramble_flag: 0,
                general_abi: 1,
                pub_account_cookie: Bytes::from(self.cache_info.read().await.pub_account_cookie.to_vec()),
            },
            group_mask: 2,
            end_seq: rand::random::<u32>() as i64,
            _0769_body: oidb_req,
            ..Default::default()
        };
        let flag = 0; // flag := msg.SyncFlag_START
        let mut msg_req = GetMessageRequest {
            sync_flag: Some(flag),
            sync_cookie: Some(self.cache_info.read().await.sync_cookie.to_vec()),
            ramble_flag: Some(0),
            context_flag: Some(1),
            online_sync_flag: Some(0),
            latest_ramble_number: Some(20),
            other_ramble_number: Some(3),
            msg_req_type: Some(1),
            ..Default::default()
        };
        let off_msg = msg_req.to_bytes();
        msg_req.msg_req_type = Some(2);
        msg_req.sync_cookie = None;
        msg_req.pubaccount_cookie = Some(self.cache_info.read().await.pub_account_cookie.to_vec());
        let pub_msg = msg_req.to_bytes();
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("req_PbOffMsg".to_string(), {
                    let mut w = JceMut::new();
                    w.put_bytes({
                                    let mut b = BytesMut::new();
                                    b.put_slice(&[0; 4]);
                                    b.put_slice(&off_msg);
                                    b.freeze()
                                }, 0);
                    w.freeze()
                }),
                ("req_PbPubMsg".to_string(), {
                    let mut w = JceMut::new();
                    w.put_bytes({
                                    let mut b = BytesMut::new();
                                    b.put_slice(&[0; 4]);
                                    b.put_slice(&pub_msg);
                                    b.freeze()
                                }, 0);
                    w.freeze()
                }),
                ("req_OffMsg".to_string(), pack_uni_request_data(&reg_req.build())),
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "RegPrxySvc".to_string(),
            s_buffer: buf.build(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "RegPrxySvc.infoSync", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        (seq, packet)
    }

    pub async fn build_group_msg_read_packet(&self, group_code: i64, msg_seq: i64) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = PbMsgReadedReportReq {
            grp_read_report: vec![PbGroupReadedReportReq { group_code: Some(group_code as u64), last_read_seq: Some(msg_seq as u64) }],
            ..Default::default()
        }.to_bytes();
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "PbMessageSvc.PbMsgReadedReport", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &req.build());
        (seq, packet)
    }

    pub async fn build_private_msg_read_packet(&self, uin: i64, time: i64) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = PbMsgReadedReportReq {
            c2_c_read_report: Option::from(PbC2cReadedReportReq {
                pair_info: vec![UinPairReadInfo {
                    peer_uin: Option::from(uin as u64),
                    last_read_time: Option::from(time as u32),
                    ..Default::default()
                }],
                sync_cookie: Some(self.cache_info.read().await.sync_cookie.to_vec()),
                ..Default::default()
            }),
            ..Default::default()
        }.to_bytes();
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "PbMessageSvc.PbMsgReadedReport", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &req.build());
        (seq, packet)
    }

    pub async fn build_device_list_request_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = SvcReqGetDevLoginInfo {
            guid: self.device_info.read().await.guid.to_owned(),
            login_type: 1,
            app_name: "com.tencent.mobileqq".to_string(),
            require_max: 20,
            get_dev_list_type: 20,
            ..Default::default()
        };
        let buf = RequestDataVersion3 {
            map: HashMap::from([
                ("SvcReqGetDevLoginInfo".to_string(), pack_uni_request_data(&req.build()))
            ])
        };
        let pkt = RequestPacket {
            i_version: 3,
            s_servant_name: "StatSvc".to_string(),
            s_func_name: "SvcReqGetDevLoginInfo".to_string(),
            s_buffer: buf.build(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "StatSvc.GetDevLoginInfo", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &pkt.build());
        (seq, packet)
    }

    pub async fn build_group_info_request_packet(&self, group_code: i64) -> (u16, Bytes) {
        let seq = self.next_seq();
        let body = D88dReqBody {
            app_id: Some(self.version.app_id),
            req_group_info: vec![
                ReqGroupInfo {
                    group_code: Some(group_code as u64),
                    stgroupinfo: Some(D88dGroupInfo {
                        group_owner: Some(0),
                        group_uin: Some(0),
                        group_create_time: Some(0),
                        group_flag: Some(0),
                        group_member_max_num: Some(0),
                        group_member_num: Some(0),
                        group_option: Some(0),
                        group_level: Some(0),
                        group_face: Some(0),
                        group_name: Some(vec![]),
                        group_memo: Some(vec![]),
                        group_finger_memo: Some(vec![]),
                        group_last_msg_time: Some(0),
                        group_cur_msg_seq: Some(0),
                        group_question: Some(vec![]),
                        group_answer: Some(vec![]),
                        group_grade: Some(0),
                        active_member_num: Some(0),
                        head_portrait_seq: Some(0),
                        msg_head_portrait: Some(D88dGroupHeadPortrait::default()),
                        st_group_ex_info: Some(D88dGroupExInfoOnly::default()),
                        group_sec_level: Some(0),
                        cmduin_privilege: Some(0),
                        no_finger_open_flag: Some(0),
                        no_code_finger_open_flag: Some(0),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            pc_client_version: Some(0),
        };
        let payload = OidbssoPkg {
            command: 2189,
            bodybuffer: body.to_bytes().to_vec(),
            ..Default::default()
        };
        let packet = build_uni_packet(self.uin.load(Ordering::SeqCst), seq, "OidbSvc.0x88d_0", 1, &self.out_going_packet_session_id.read().await, &[], &self.cache_info.read().await.sig_info.d2key, &payload.to_bytes());
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