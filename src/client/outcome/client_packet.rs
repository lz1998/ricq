use std::sync::atomic::Ordering;
use bytes::{BufMut, Bytes, BytesMut};
use chrono::Utc;
use crate::binary::BinaryWriter;
use crate::crypto::EncryptSession;
use crate::client::outcome::packet::{build_code2d_request_packet, build_login_packet, build_oicq_request_packet, build_sso_packet, build_uni_packet};
use crate::client::outcome::tlv::*;
use crate::client::version::{ClientProtocol, gen_version_info};


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
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x0810, &self.ecdh, &self.random_key, &{
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst) as u32, 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_device_lock_login_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x0810, &self.ecdh, &self.random_key, &{
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst) as u32, 2, &vec![0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_captcha_packet(&self, result: String, sign: &[u8]) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x810, &self.ecdh, &self.random_key, &{
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst) as u32, 2, &vec![0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_sms_request_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x810, &self.ecdh, &self.random_key, &{
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst) as u32, 2, &vec![0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_sms_code_submit_packet(&self, code: String) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x810, &self.ecdh, &self.random_key, &{
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst) as u32, 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_ticket_submit_packet(&self, ticket: String) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x810, &self.ecdh, &self.random_key, &{
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst) as u32, 2, &[0; 16], &sso, &[]);
        (seq, packet)
    }

    pub async fn build_request_tgtgt_no_pic_sig_packet(&self) -> (u16, Bytes) {
        let seq = self.next_seq();
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x810, &EncryptSession::new(&self.cache_info.read().await.sig_info.t133), &self.cache_info.read().await.sig_info.wt_session_ticket_key, &{
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
        let req = build_oicq_request_packet(self.uin.load(Ordering::SeqCst) as u32, 0x810, &self.ecdh, &self.random_key, &{
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
        let packet = build_login_packet(self.uin.load(Ordering::SeqCst) as u32, 2, &[0; 16], &sso, &[]);
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