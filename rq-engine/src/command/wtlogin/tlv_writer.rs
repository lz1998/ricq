#![allow(clippy::too_many_arguments)]

use bytes::{BufMut, Bytes, BytesMut};
use chrono::Utc;

use crate::binary::BinaryWriter;

pub fn t1(uin: u32, ip: &[u8]) -> Bytes {
    if ip.len() != 4 {
        panic!("invalid ip")
    }
    let mut buf = BytesMut::new();
    buf.put_u16(0x01);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(1);
        w.put_u32(rand::random());
        w.put_u32(uin);
        w.put_u32(Utc::now().timestamp() as u32);
        w.put_slice(ip);
        w.put_u16(0);
        w.freeze()
    });
    buf.freeze()
}

pub fn t1b(
    micro: u32,
    version: u32,
    size: u32,
    margin: u32,
    dpi: u32,
    ec_level: u32,
    hint: u32,
) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x1b);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(micro);
        w.put_u32(version);
        w.put_u32(size);
        w.put_u32(margin);
        w.put_u32(dpi);
        w.put_u32(ec_level);
        w.put_u32(hint);
        w.put_u16(0);
        w.freeze()
    });
    buf.freeze()
}

pub fn t1d(misc_bitmap: u32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x1d);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u8(1);
        w.put_u32(misc_bitmap);
        w.put_u32(0);
        w.put_u8(0);
        w.put_u32(0);
        w.freeze()
    });
    buf.freeze()
}

pub fn t1f(
    is_root: bool,
    os_name: &str,
    os_version: &str,
    sim_operator_name: &str,
    apn: &str,
    network_type: u16,
) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x1f);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u8(if is_root { 1 } else { 0 });
        w.write_bytes_short(os_name.as_bytes());
        w.write_bytes_short(os_version.as_bytes());
        w.put_u16(network_type);
        w.write_bytes_short(sim_operator_name.as_bytes());
        w.write_bytes_short(&[]);
        w.write_bytes_short(apn.as_bytes());
        w.freeze()
    });
    buf.freeze()
}

pub fn t2(result: String, sign: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x02);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(0);
        w.write_bytes_short(result.as_bytes());
        w.write_bytes_short(sign);
        w.freeze()
    });
    buf.freeze()
}

pub fn t8(local_id: u32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x8);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(0);
        w.put_u32(local_id);
        w.put_u16(0);
        w.freeze()
    });
    buf.freeze()
}

pub fn t10a(arr: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x10A);
    buf.write_bytes_short(arr);
    buf.freeze()
}

pub fn t16(
    sso_version: u32,
    app_id: u32,
    sub_app_id: u32,
    guid: &[u8],
    apk_id: &str,
    apk_version_name: &str,
    apk_sign: &[u8],
) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x16);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(sso_version);
        w.put_u32(app_id);
        w.put_u32(sub_app_id);
        w.put_slice(guid);
        w.write_bytes_short(apk_id.as_bytes());
        w.write_bytes_short(apk_version_name.as_bytes());
        w.write_bytes_short(apk_sign);
        w.freeze()
    });
    buf.freeze()
}

pub fn t16a(arr: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x16A);
    buf.write_bytes_short(arr);
    buf.freeze()
}

pub fn t16e(build_model: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x16E);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(build_model);
        w.freeze()
    });
    buf.freeze()
}

pub fn t17a(value: i32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x17a);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(value as u32);
        w.freeze()
    });
    buf.freeze()
}

pub fn t17c(code: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x17c);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.write_bytes_short(code.as_bytes());
        w.freeze()
    });
    buf.freeze()
}

pub fn t18(app_id: u32, uin: u32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x18);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(1);
        w.put_u32(1536);
        w.put_u32(app_id);
        w.put_u32(0);
        w.put_u32(uin);
        w.put_u16(0);
        w.put_u16(0);
        w.freeze()
    });
    buf.freeze()
}

pub fn t33(guid: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x33);
    buf.write_bytes_short(guid);
    buf.freeze()
}

pub fn t35(product_type: u32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x35);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(product_type);
        w.freeze()
    });
    buf.freeze()
}

pub fn t52d(dev_info: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x52d);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(dev_info);
        w.freeze()
    });
    buf.freeze()
}

pub fn t100(sso_version: u32, protocol: u32, main_sig_map: u32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x100);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(1);
        w.put_u32(sso_version);
        w.put_u32(16);
        w.put_u32(protocol);
        w.put_u32(0); // App client version
        w.put_u32(main_sig_map); // 34869472
        w.freeze()
    });
    buf.freeze()
}

pub fn t104(data: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x104);
    buf.write_bytes_short(data);
    buf.freeze()
}

pub fn t106(
    uin: u32,
    salt: u32,
    app_id: u32,
    sso_ver: u32,
    password_md5: &[u8],
    guid_available: bool,
    guid: &[u8],
    tgtgt_key: &[u8],
    wtf: u32,
) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x106);
    let body = &{
        let mut w = BytesMut::new();
        w.put_u16(4);
        w.put_u32(rand::random::<u32>());
        w.put_u32(sso_ver);
        w.put_u32(16); // appId
        w.put_u32(0); // app client version
        if uin == 0 {
            w.put_u64(salt as u64)
        } else {
            w.put_u64(uin as u64)
        }
        w.put_u32(Utc::now().timestamp() as u32);
        w.put_slice(&[0x00, 0x00, 0x00, 0x00]); // fake ip
        w.put_u8(0x01);
        w.put_slice(password_md5);
        w.put_slice(tgtgt_key);
        w.put_u32(wtf);
        w.put_u8(if guid_available { 1 } else { 0 });
        if guid.is_empty() {
            for _ in 0..4 {
                w.put_u32(rand::random::<u32>());
            }
        } else {
            w.put_slice(guid)
        }
        w.put_u32(app_id);
        w.put_u32(1); // password login
        w.write_bytes_short((uin as i64).to_string().as_bytes());
        w.put_u16(0);
        w.freeze()
    };

    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        let mut b = BytesMut::new();
        if salt != 0 {
            b.put_u32(salt);
        } else {
            b.put_u32(uin);
        }
        let mut v = BytesMut::new();
        v.put_slice(password_md5);
        v.put_slice(&[0; 4]);
        v.put_slice(&b);
        let key = md5::compute(&v).to_vec();
        w.encrypt_and_write(&key, body);
        w.freeze()
    });
    buf.freeze()
}

pub fn t107(pic_type: u16) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x107);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(pic_type);
        w.put_u8(0x00);
        w.put_u16(0);
        w.put_u8(0x01);
        w.freeze()
    });
    buf.freeze()
}

pub fn t108(ksid: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x108);
    buf.write_bytes_short(ksid);
    buf.freeze()
}

pub fn t109(android_id: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x109);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(md5::compute(android_id.as_bytes()).as_ref());
        w.freeze()
    });
    buf.freeze()
}

pub fn t116(misc_bitmap: u32, sub_sig_map: u32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x116);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u8(0x00);
        w.put_u32(misc_bitmap);
        w.put_u32(sub_sig_map);
        w.put_u8(0x01);
        w.put_u32(1600000226); // app id list
        w.freeze()
    });
    buf.freeze()
}

pub fn t124(os_type: &str, os_version: &str, sim_info: &str, apn: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x124);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.write_tlv_limited_size(os_type.as_bytes(), 16);
        w.write_tlv_limited_size(os_version.as_bytes(), 16);
        w.put_u16(2);
        w.write_tlv_limited_size(sim_info.as_bytes(), 16);
        w.write_tlv_limited_size(&[], 16);
        w.write_tlv_limited_size(apn.as_bytes(), 16);
        w.freeze()
    });
    buf.freeze()
}

pub fn t128(
    is_guid_from_file_null: bool,
    is_guid_available: bool,
    is_guid_changed: bool,
    guid_flag: u32,
    build_model: &str,
    guid: &[u8],
    build_brand: &str,
) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x128);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(0);
        w.put_u8(if is_guid_from_file_null { 1 } else { 0 });
        w.put_u8(if is_guid_available { 1 } else { 0 });
        w.put_u8(if is_guid_changed { 1 } else { 0 });
        w.put_u32(guid_flag);
        w.write_tlv_limited_size(build_model.as_bytes(), 32);
        w.write_tlv_limited_size(guid, 16);
        w.write_tlv_limited_size(build_brand.as_bytes(), 16); // app id list
        w.freeze()
    });
    buf.freeze()
}

pub fn t141(sim_info: &str, apn: &str) -> Bytes {
    let mut w = BytesMut::new();
    w.put_u16(0x141);
    w.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(1);
        w.write_bytes_short(sim_info.as_bytes());
        w.put_u16(2);
        w.write_bytes_short(apn.as_bytes());
        w.freeze()
    });
    w.freeze()
}

pub fn t142(apk_id: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x142);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(0);
        w.write_tlv_limited_size(apk_id.as_bytes(), 32);
        w.freeze()
    });
    buf.freeze()
}

pub fn t143(arr: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x143);
    buf.write_bytes_short(arr);
    buf.freeze()
}

pub fn t144(
    imei: &str,
    dev_info: &[u8],
    os_type: &str,
    os_version: &str,
    sim_info: &str,
    apn: &str,
    is_guid_from_file_null: bool,
    is_guid_available: bool,
    is_guid_changed: bool,
    guid_flag: u32,
    build_model: &str,
    guid: &[u8],
    build_brand: &str,
    tgtgt_key: &[u8],
) -> Bytes {
    let mut w = BytesMut::new();
    w.put_u16(0x144);
    w.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.encrypt_and_write(tgtgt_key, &{
            let mut w = BytesMut::new();
            w.put_u16(5);
            w.put_slice(&t109(imei));
            w.put_slice(&t52d(dev_info));
            w.put_slice(&t124(os_type, os_version, sim_info, apn));
            w.put_slice(&t128(
                is_guid_from_file_null,
                is_guid_available,
                is_guid_changed,
                guid_flag,
                build_model,
                guid,
                build_brand,
            ));
            w.put_slice(&t16e(build_model.as_bytes()));
            w.freeze()
        });
        w.freeze()
    });
    w.freeze()
}

pub fn t145(guid: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x145);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(guid);
        w.freeze()
    });
    buf.freeze()
}

pub fn t147(app_id: u32, apk_version_name: &str, apk_signature_md5: &[u8]) -> Bytes {
    let mut w = BytesMut::new();
    w.put_u16(0x147);
    w.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(app_id);
        w.write_tlv_limited_size(apk_version_name.as_bytes(), 32);
        w.write_tlv_limited_size(apk_signature_md5, 32);
        w.freeze()
    });
    w.freeze()
}

pub fn t154(seq: u16) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x154);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(seq as u32);
        w.freeze()
    });
    buf.freeze()
}

pub fn t166(image_type: u8) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x166);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u8(image_type);
        w.freeze()
    });
    buf.freeze()
}

pub fn t174(data: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x174);
    buf.write_bytes_short(data);
    buf.freeze()
}

pub fn t177(build_time: u32, sdk_version: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x177);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u8(0x01);
        w.put_u32(build_time);
        w.write_bytes_short(sdk_version.as_bytes());
        w.freeze()
    });
    buf.freeze()
}

pub fn t187(mac_address: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x187);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(md5::compute(mac_address.as_bytes()).as_ref());
        w.freeze()
    });
    buf.freeze()
}

pub fn t188(android_id: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x188);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(md5::compute(android_id.as_bytes()).as_ref());
        w.freeze()
    });
    buf.freeze()
}

pub fn t191(k: u8) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x191);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u8(k);
        w.freeze()
    });
    buf.freeze()
}

pub fn t193(ticket: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x193);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(ticket.as_bytes());
        w.freeze()
    });
    buf.freeze()
}

pub fn t194(imsi_md5: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x194);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(imsi_md5);
        w.freeze()
    });
    buf.freeze()
}

pub fn t197() -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x197);
    let v: [u8; 1] = [0];
    buf.write_bytes_short(&v);
    buf.freeze()
}

pub fn t198() -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x198);
    let v: [u8; 1] = [0];
    buf.write_bytes_short(&v);
    buf.freeze()
}

pub fn t202(wifi_bssid: &str, wifi_ssid: &str) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x202);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.write_tlv_limited_size(wifi_bssid.as_bytes(), 16);
        w.write_tlv_limited_size(wifi_ssid.as_bytes(), 32);
        w.freeze()
    });
    buf.freeze()
}

pub fn t400(
    g: &[u8],
    uin: i64,
    guid: &[u8],
    dpwd: &[u8],
    j2: i64,
    j3: i64,
    rand_seed: &[u8],
) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x400);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.encrypt_and_write(g, &{
            let mut ww = BytesMut::new();
            ww.put_u16(1);
            ww.put_u64(uin as u64);
            ww.put_slice(guid);
            ww.put_slice(dpwd);
            ww.put_u32(j2 as u32);
            ww.put_u32(j3 as u32);
            ww.put_u32(Utc::now().timestamp_millis() as u32);
            ww.put_slice(rand_seed);
            ww.freeze()
        });
        w.freeze()
    });
    buf.freeze()
}

pub fn t401(d: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x401);
    buf.write_bytes_short(d);
    buf.freeze()
}

pub fn t511(domains: Vec<&str>) -> Bytes {
    let mut arr2 = Vec::new();
    for d in domains {
        if !d.is_empty() {
            arr2.push(d)
        }
    }
    let mut buf = BytesMut::new();
    buf.put_u16(0x511);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(arr2.len() as u16);
        for d in arr2 {
            let index_of = match d.find('(') {
                None => -1,
                Some(i) => i as isize,
            };
            let index_of2 = match d.find(')') {
                None => -1,
                Some(i) => i as isize,
            };
            if index_of != 0 || index_of2 <= 0 {
                w.put_u8(0x01);
                w.write_bytes_short(d.as_bytes())
            } else {
                let mut b: u8;
                let z: bool;
                if let Ok(i) = d[(index_of + 1) as usize..index_of2 as usize].parse::<i32>() {
                    let z2 = (1048576 & i) > 0;
                    if (i & 134217728) > 0 {
                        z = true
                    } else {
                        z = false
                    }
                    if z2 {
                        b = 1
                    } else {
                        b = 0
                    }
                    if z {
                        b |= 2
                    }
                    w.put_u8(b);
                    w.write_bytes_short(d[(index_of2 + 1) as usize..].as_bytes());
                }
            }
        }
        w.freeze()
    });
    buf.freeze()
}

pub fn t516() -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x516);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(0);
        w.freeze()
    });
    buf.freeze()
}

pub fn t521(i: u32) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x521);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u32(i);
        w.put_u16(0);
        w.freeze()
    });
    buf.freeze()
}

pub fn t525(t536: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x525);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_u16(1);
        w.put_slice(t536);
        w.freeze()
    });
    buf.freeze()
}

pub fn t536(login_extra_data: &[u8]) -> Bytes {
    let mut buf = BytesMut::new();
    buf.put_u16(0x536);
    buf.write_bytes_short(&{
        let mut w = BytesMut::new();
        w.put_slice(login_extra_data);
        w.freeze()
    });
    buf.freeze()
}

pub fn guid_flag() -> u32 {
    let mut flag: u32 = 0;
    flag |= 1 << 24 & 0xFF000000;
    flag |= 0 << 8 & 0xFF00;
    flag
}

#[cfg(test)]
mod tests {
    use lazy_static::*;

    use crate::command::wtlogin::tlv_writer::*;

    lazy_static! {
        static ref GUID: [u8; 16] =
            [142, 27, 163, 177, 172, 31, 181, 137, 118, 115, 8, 126, 24, 49, 54, 169];
        static ref TGTGT_KEY: [u8; 16] =
            [199, 12, 183, 107, 3, 28, 81, 148, 116, 20, 229, 112, 0, 64, 152, 255];
        static ref UIN: u32 = 349195854;
        static ref OS_NAME: String = "android".to_string();
        static ref OS_VERSION: String = "7.1.2".to_string();
        static ref SIM_INFO: String = "T-Mobile".to_string();
        static ref IMEI: String = "468356291846738".to_string();
        static ref IMEI_MD5: &'static [u8] = "9792b1bba1867318bf782af418306ef8".as_bytes();
        static ref WIFI_BSSID: String = "00:50:56:C0:00:08".to_string();
        static ref WIFI_SSID: String = "<unknown ssid>".to_string();
        static ref APN: String = "wifi".to_string();
        static ref APK_SIGN: [u8; 16] = [
            0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E,
            0xB6, 0x8D
        ];
        static ref APK_ID: String = "com.tencent.mobileqq".to_string();
        static ref APP_ID: u32 = 537066738;
        static ref SUB_APP_ID: u32 = 537066738;
        static ref SSO_VERSION: u32 = 15;
        static ref SDK_VERSION: &'static str = "6.0.0.2454";
        static ref MISC_BITMAP: u32 = 184024956;
        static ref SUB_SIG_MAP: u32 = 0x10400;
        static ref MAIN_SIG_MAP: u32 = 34869472;
        static ref MAC_ADDRESS: String = "00:50:56:C0:00:08".to_string();
        static ref IS_ROOT: bool = false;
        static ref ANDROID_ID: String = "QKQ1.191117.002".to_string();
        static ref APK_VERSION_NAME: String = "2.0.5".to_string();
        static ref DEV_INFO: &'static [u8] =
            "dev_info_dev_info_dev_info_dev_info_dev_info_".as_bytes();
        static ref BUILD_MODEL: String = "mirai".to_string();
        static ref BUILD_BRAND: String = "mamoe".to_string();
        static ref OS_TYPE: String = "android".to_string();
    }

    #[test]
    fn test_param() {
        println!("{:?}", *GUID);
        println!("{:?}", "test param");
    }

    #[test]
    fn test_t1() {
        let result = t1(*UIN, &[192, 168, 1, 1]);
        println!("{:?}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t1b() {
        let result = t1b(0, 0, 3, 4, 72, 2, 2);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t1d() {
        let result = t1d(*MISC_BITMAP);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t1f() {
        let result = t1f(*IS_ROOT, &OS_NAME, &OS_VERSION, "China Mobile GSM", &APN, 2);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t2() {
        let result = t2("result".to_string(), "sign".as_ref());
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t8() {
        let result = t8(123456);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t10a() {
        let result = t10a(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t16() {
        let result = t16(
            *SSO_VERSION,
            *APP_ID,
            *SUB_APP_ID,
            &*GUID,
            &APK_ID,
            &APK_VERSION_NAME,
            &*APK_SIGN,
        );
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t16a() {
        let result = t16a(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t16e() {
        let result = t16e(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t17a() {
        let result = t17a(*UIN as i32);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t17c() {
        let result = t17c(&IMEI);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t18() {
        let result = t18(*APP_ID, *UIN);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t33() {
        let result = t33(&*GUID);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t35() {
        let product_type = 8;
        let result = t35(product_type);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t52d() {
        let result = t52d(*DEV_INFO);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t100() {
        let result = t100(*SSO_VERSION, 2, *MAIN_SIG_MAP);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t104() {
        let result = t104(&*GUID);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t106() {
        let result = t106(
            *UIN,
            0,
            *APP_ID,
            *SSO_VERSION,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            true,
            &*GUID,
            &*TGTGT_KEY,
            0,
        );
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t107() {
        let result = t107(3);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t108() {
        let result = t108(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t109() {
        let result = t109(&ANDROID_ID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t116() {
        let result = t116(*MAIN_SIG_MAP, *SUB_SIG_MAP);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t124() {
        let result = t124(&OS_TYPE, &OS_VERSION, &SIM_INFO, &APN);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t128() {
        let result = t128(false, true, false, 16, &BUILD_MODEL, &*GUID, &BUILD_BRAND);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t141() {
        let result = t141(&SIM_INFO, &APN);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t142() {
        let result = t142(&APK_ID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t143() {
        let result = t143(&[1, 2, 3]);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t144() {
        let result = t144(
            &IMEI,
            *DEV_INFO,
            &OS_TYPE,
            &OS_VERSION,
            &SIM_INFO,
            &APN,
            false,
            true,
            false,
            16,
            &BUILD_MODEL,
            &*GUID,
            &BUILD_BRAND,
            &*TGTGT_KEY,
        );
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t145() {
        let result = t145(&*GUID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t147() {
        let result = t147(16, &APK_VERSION_NAME, &*APK_SIGN);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t154() {
        let seq = (0x3635 + 1) & 0x7FFF;
        println!("{}", seq);
        let result = t154(seq);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t166() {
        let result = t166(1);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t174() {
        let result = t174(&*GUID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t177() {
        let result = t177(*MISC_BITMAP, *SDK_VERSION);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t187() {
        let result = t187(&MAC_ADDRESS);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t188() {
        let result = t188(&ANDROID_ID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t191() {
        let result = t191(127_u8);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t193() {
        let result = t193("some ticket");
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t194() {
        let result = t194(*IMEI_MD5);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t197() {
        let result = t197();
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t198() {
        let result = t198();
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t202() {
        let result = t202(&WIFI_BSSID, &WIFI_SSID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t400() {
        let result = t400(&*GUID, *UIN as i64, &*GUID, &*GUID, 2, 2, &*GUID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t401() {
        let result = t401(&*GUID);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t511() {
        let result = t511(vec![
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
        ]);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t516() {
        let result = t516();
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t521() {
        let result = t521(6);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t525() {
        let result = t525(&t536(&*GUID));
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_tlv() {
        let result = guid_flag();
        println!("{:?}", result);
    }
}
