#![allow(clippy::too_many_arguments)]

use bytes::{BufMut, BytesMut};
use std::time::UNIX_EPOCH;

use crate::binary::packet_writer::{PacketWriter, WriteLV};
use crate::binary::BinaryWriter;

pub fn tlv<'a, B: BufMut + WriteLV, W: PacketWriter<B> + 'a>(
    tag: u16,
    body_writer: W,
) -> impl PacketWriter<B> + 'a {
    move |buf: &mut B| {
        buf.put_u16(tag);
        buf.write_short_lv(body_writer);
    }
}

pub fn t1<B: BufMut + WriteLV>(uin: u32, ip: &[u8]) -> impl PacketWriter<B> + '_ {
    if ip.len() != 4 {
        panic!("invalid ip")
    }
    tlv(0x01, move |w: &mut B| {
        w.put_u16(1);
        w.put_u32(rand::random());
        w.put_u32(uin);
        w.put_u32(UNIX_EPOCH.elapsed().unwrap().as_secs() as u32);
        w.put_slice(ip);
        w.put_u16(0);
    })
}

pub fn t1b<B: BufMut + WriteLV>(
    micro: u32,
    version: u32,
    size: u32,
    margin: u32,
    dpi: u32,
    ec_level: u32,
    hint: u32,
) -> impl PacketWriter<B> {
    tlv(0x1b, move |w: &mut B| {
        w.put_u32(micro);
        w.put_u32(version);
        w.put_u32(size);
        w.put_u32(margin);
        w.put_u32(dpi);
        w.put_u32(ec_level);
        w.put_u32(hint);
        w.put_u16(0);
    })
}

pub fn t1d<B: BufMut + WriteLV>(misc_bitmap: u32) -> impl PacketWriter<B> {
    tlv(0x1d, move |w: &mut B| {
        w.put_u8(1);
        w.put_u32(misc_bitmap);
        w.put_u32(0);
        w.put_u8(0);
        w.put_u32(0);
    })
}

pub fn t1f<'a, B: BufMut + WriteLV>(
    is_root: bool,
    os_name: &'a str,
    os_version: &'a str,
    sim_operator_name: &'a str,
    apn: &'a str,
    network_type: u16,
) -> impl PacketWriter<B> + 'a {
    tlv(0x1f, move |w: &mut B| {
        w.put_u8(if is_root { 1 } else { 0 });
        w.write_bytes_short(os_name.as_bytes());
        w.write_bytes_short(os_version.as_bytes());
        w.put_u16(network_type);
        w.write_bytes_short(sim_operator_name.as_bytes());
        w.write_bytes_short(&[]);
        w.write_bytes_short(apn.as_bytes());
    })
}

pub fn t2<B: BufMut + WriteLV>(result: String, sign: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x02, move |w: &mut B| {
        w.put_u16(0);
        w.write_bytes_short(result.as_bytes());
        w.write_bytes_short(sign);
    })
}

pub fn t8<B: BufMut + WriteLV>(local_id: u32) -> impl PacketWriter<B> {
    tlv(0x08, move |w: &mut B| {
        w.put_u16(0);
        w.put_u32(local_id);
        w.put_u16(0);
    })
}

pub fn t10a<B: BufMut + WriteLV>(arr: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x10A, arr)
}

pub fn t16<'a, B: BufMut + WriteLV>(
    sso_version: u32,
    app_id: u32,
    sub_app_id: u32,
    guid: &'a [u8],
    apk_id: &'a str,
    apk_version_name: &'a str,
    apk_sign: &'a [u8],
) -> impl PacketWriter<B> + 'a {
    tlv(0x16, move |w: &mut B| {
        w.put_u32(sso_version);
        w.put_u32(app_id);
        w.put_u32(sub_app_id);
        w.put_slice(guid);
        w.write_bytes_short(apk_id.as_bytes());
        w.write_bytes_short(apk_version_name.as_bytes());
        w.write_bytes_short(apk_sign);
    })
}

pub fn t16a<B: BufMut + WriteLV>(arr: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x16A, arr)
}

pub fn t16e<B: BufMut + WriteLV>(build_model: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x16E, build_model)
}

pub fn t17a<B: BufMut + WriteLV>(value: i32) -> impl PacketWriter<B> {
    tlv(0x17a, move |buf: &mut B| buf.put_u32(value as u32))
}

pub fn t17c<B: BufMut + WriteLV>(code: &str) -> impl PacketWriter<B> + '_ {
    tlv(0x17c, move |w: &mut B| {
        w.write_short_lv(code.as_bytes());
    })
}

pub fn t18<B: BufMut + WriteLV>(app_id: u32, uin: u32) -> impl PacketWriter<B> {
    tlv(0x18, move |w: &mut B| {
        w.put_u16(1);
        w.put_u32(1536);
        w.put_u32(app_id);
        w.put_u32(0);
        w.put_u32(uin);
        w.put_u16(0);
        w.put_u16(0);
    })
}

pub fn t33<B: BufMut + WriteLV>(guid: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x33, guid)
}

pub fn t35<B: BufMut + WriteLV>(product_type: u32) -> impl PacketWriter<B> {
    tlv(0x35, move |buf: &mut B| buf.put_u32(product_type))
}

pub fn t52d<B: BufMut + WriteLV>(dev_info: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x52d, dev_info)
}

pub fn t100<B: BufMut + WriteLV>(
    sso_version: u32,
    protocol: u32,
    main_sig_map: u32,
) -> impl PacketWriter<B> {
    tlv(0x100, move |w: &mut B| {
        w.put_u16(1);
        w.put_u32(sso_version);
        w.put_u32(16);
        w.put_u32(protocol);
        w.put_u32(0); // App client version
        w.put_u32(main_sig_map); // 34869472
    })
}

pub fn t104<B: BufMut + WriteLV>(data: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x104, data)
}

pub fn t106<'a, B: BufMut + WriteLV>(
    uin: u32,
    salt: u32,
    app_id: u32,
    sso_ver: u32,
    password_md5: &'a [u8],
    guid_available: bool,
    guid: &'a [u8],
    tgtgt_key: &'a [u8],
    wtf: u32,
) -> impl PacketWriter<B> + 'a {
    tlv(0x106, move |w: &mut B| {
        let key = md5::compute(&{
            let mut v = BytesMut::new();
            v.put_slice(password_md5);
            v.put_slice(&[0; 4]);
            v.put_u32(if salt != 0 { salt } else { uin });
            v
        })
        .to_vec();
        w.encrypt_and_write(&key, &{
            let mut w = BytesMut::new();
            w.put_u16(4);
            w.put_u32(rand::random::<u32>());
            w.put_u32(sso_ver);
            w.put_u32(16); // appId
            w.put_u32(0); // app client version
            w.put_u64(if uin == 0 { salt as u64 } else { uin as u64 });
            w.put_u32(UNIX_EPOCH.elapsed().unwrap().as_secs() as u32);
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
            w.write_short_lv((uin as i64).to_string().as_bytes());
            w.put_u16(0);
            w.freeze()
        });
    })
}

pub fn t107<B: BufMut + WriteLV>(pic_type: u16) -> impl PacketWriter<B> {
    tlv(0x107, move |w: &mut B| {
        w.put_u16(pic_type);
        w.put_u8(0x00);
        w.put_u16(0);
        w.put_u8(0x01);
    })
}

pub fn t108<B: BufMut + WriteLV>(ksid: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x108, ksid)
}

pub fn t109<B: BufMut + WriteLV>(android_id: &str) -> impl PacketWriter<B> + '_ {
    tlv(0x109, move |buf: &mut B| {
        buf.put_slice(md5::compute(android_id.as_bytes()).as_ref());
    })
}

pub fn t116<B: BufMut + WriteLV>(misc_bitmap: u32, sub_sig_map: u32) -> impl PacketWriter<B> {
    tlv(0x116, move |w: &mut B| {
        w.put_u8(0x00);
        w.put_u32(misc_bitmap);
        w.put_u32(sub_sig_map);
        w.put_u8(0x01);
        w.put_u32(1600000226); // app id list
    })
}

pub fn t124<'a, B: BufMut + WriteLV>(
    os_type: &'a str,
    os_version: &'a str,
    sim_info: &'a str,
    apn: &'a str,
) -> impl PacketWriter<B> + 'a {
    tlv(0x124, move |w: &mut B| {
        w.write_tlv_limited_size(os_type.as_bytes(), 16);
        w.write_tlv_limited_size(os_version.as_bytes(), 16);
        w.put_u16(2);
        w.write_tlv_limited_size(sim_info.as_bytes(), 16);
        w.write_tlv_limited_size(&[], 16);
        w.write_tlv_limited_size(apn.as_bytes(), 16);
    })
}

pub fn t128<'a, B: BufMut + WriteLV>(
    is_guid_from_file_null: bool,
    is_guid_available: bool,
    is_guid_changed: bool,
    guid_flag: u32,
    build_model: &'a str,
    guid: &'a [u8],
    build_brand: &'a str,
) -> impl PacketWriter<B> + 'a {
    tlv(0x128, move |w: &mut B| {
        w.put_u16(0);
        w.put_u8(if is_guid_from_file_null { 1 } else { 0 });
        w.put_u8(if is_guid_available { 1 } else { 0 });
        w.put_u8(if is_guid_changed { 1 } else { 0 });
        w.put_u32(guid_flag);
        w.write_tlv_limited_size(build_model.as_bytes(), 32);
        w.write_tlv_limited_size(guid, 16);
        w.write_tlv_limited_size(build_brand.as_bytes(), 16); // app id list
    })
}

pub fn t141<'a, B: BufMut + WriteLV>(sim_info: &'a str, apn: &'a str) -> impl PacketWriter<B> + 'a {
    tlv(0x141, move |w: &mut B| {
        w.put_u16(1);
        w.write_bytes_short(sim_info.as_bytes());
        w.put_u16(2);
        w.write_bytes_short(apn.as_bytes());
    })
}

pub fn t142<B: BufMut + WriteLV>(apk_id: &str) -> impl PacketWriter<B> + '_ {
    tlv(0x142, move |w: &mut B| {
        w.put_u16(0);
        w.write_tlv_limited_size(apk_id.as_bytes(), 32);
    })
}

pub fn t143<B: BufMut + WriteLV>(arr: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x143, arr)
}

pub fn t144<'a, B: BufMut + WriteLV>(
    imei: &'a str,
    dev_info: &'a [u8],
    os_type: &'a str,
    os_version: &'a str,
    sim_info: &'a str,
    apn: &'a str,
    is_guid_from_file_null: bool,
    is_guid_available: bool,
    is_guid_changed: bool,
    guid_flag: u32,
    build_model: &'a str,
    guid: &'a [u8],
    build_brand: &'a str,
    tgtgt_key: &'a [u8],
) -> impl PacketWriter<B> + 'a {
    tlv(0x144, move |w: &mut B| {
        w.encrypt_and_write(tgtgt_key, &{
            let mut w = Vec::new();
            w.put_u16(5);
            t109(imei).write(&mut w);
            t52d(dev_info).write(&mut w);
            t124(os_type, os_version, sim_info, apn).write(&mut w);
            t128(
                is_guid_from_file_null,
                is_guid_available,
                is_guid_changed,
                guid_flag,
                build_model,
                guid,
                build_brand,
            )
            .write(&mut w);
            t16e(build_model.as_bytes()).write(&mut w);
            w
        });
    })
}

pub fn t145<B: BufMut + WriteLV>(guid: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x145, guid)
}

pub fn t147<'a, B: BufMut + WriteLV>(
    app_id: u32,
    apk_version_name: &'a str,
    apk_signature_md5: &'a [u8],
) -> impl PacketWriter<B> + 'a {
    tlv(0x147, move |w: &mut B| {
        w.put_u32(app_id);
        w.write_tlv_limited_size(apk_version_name.as_bytes(), 32);
        w.write_tlv_limited_size(apk_signature_md5, 32);
    })
}

pub fn t154<B: BufMut + WriteLV>(seq: u16) -> impl PacketWriter<B> {
    tlv(0x154, move |buf: &mut B| buf.put_u32(seq as u32))
}

pub fn t166<B: BufMut + WriteLV>(image_type: u8) -> impl PacketWriter<B> {
    tlv(0x166, move |buf: &mut B| buf.put_u8(image_type))
}

pub fn t174<B: BufMut + WriteLV>(data: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x174, data)
}

pub fn t177<B: BufMut + WriteLV>(build_time: u32, sdk_version: &str) -> impl PacketWriter<B> + '_ {
    tlv(0x177, move |w: &mut B| {
        w.put_u8(0x01);
        w.put_u32(build_time);
        w.write_short_lv(sdk_version.as_bytes());
    })
}

pub fn t187<B: BufMut + WriteLV>(mac_address: &str) -> impl PacketWriter<B> + '_ {
    tlv(0x187, move |buf: &mut B| {
        buf.put_slice(md5::compute(mac_address.as_bytes()).as_ref())
    })
}

pub fn t188<B: BufMut + WriteLV>(android_id: &str) -> impl PacketWriter<B> + '_ {
    tlv(0x188, move |buf: &mut B| {
        buf.put_slice(md5::compute(android_id.as_bytes()).as_ref())
    })
}

pub fn t191<B: BufMut + WriteLV>(k: u8) -> impl PacketWriter<B> {
    tlv(0x191, move |buf: &mut B| buf.put_u8(k))
}

pub fn t193<B: BufMut + WriteLV>(ticket: &str) -> impl PacketWriter<B> + '_ {
    tlv(0x193, ticket.as_bytes())
}

pub fn t194<B: BufMut + WriteLV>(imsi_md5: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x194, imsi_md5)
}

pub fn t197<B: BufMut + WriteLV>() -> impl PacketWriter<B> {
    tlv(0x197, [0u8].as_slice())
}

pub fn t198<B: BufMut + WriteLV>() -> impl PacketWriter<B> {
    tlv(0x198, [0u8].as_slice())
}

pub fn t202<'a, B: BufMut + WriteLV>(
    wifi_bssid: &'a str,
    wifi_ssid: &'a str,
) -> impl PacketWriter<B> + 'a {
    tlv(0x202, move |w: &mut B| {
        w.write_tlv_limited_size(wifi_bssid.as_bytes(), 16);
        w.write_tlv_limited_size(wifi_ssid.as_bytes(), 32);
    })
}

pub fn t318<B: BufMut + WriteLV>(tgt_qr: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x318, tgt_qr)
}

pub fn t400<'a, B: BufMut + WriteLV>(
    g: &'a [u8],
    uin: i64,
    guid: &'a [u8],
    dpwd: &'a [u8],
    j2: i64,
    j3: i64,
    rand_seed: &'a [u8],
) -> impl PacketWriter<B> + 'a {
    tlv(0x400, move |w: &mut B| {
        w.encrypt_and_write(g, &{
            let mut ww = Vec::new();
            ww.put_u16(1);
            ww.put_u64(uin as u64);
            ww.put_slice(guid);
            ww.put_slice(dpwd);
            ww.put_u32(j2 as u32);
            ww.put_u32(j3 as u32);
            ww.put_u32(UNIX_EPOCH.elapsed().unwrap().as_millis() as u32);
            ww.put_slice(rand_seed);
            ww
        });
    })
}

pub fn t401<B: BufMut + WriteLV>(d: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x401, d)
}

pub fn t511<B: BufMut + WriteLV>(domains: Vec<&str>) -> impl PacketWriter<B> + '_ {
    tlv(0x511, move |w: &mut B| {
        let mut arr2 = Vec::new();
        for d in domains {
            if !d.is_empty() {
                arr2.push(d)
            }
        }
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
                w.write_short_lv(d.as_bytes())
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
                    w.write_short_lv(d[(index_of2 + 1) as usize..].as_bytes());
                }
            }
        }
    })
}

pub fn t516<B: BufMut + WriteLV>() -> impl PacketWriter<B> {
    tlv(0x516, |buf: &mut B| buf.put_u32(0))
}

pub fn t521<B: BufMut + WriteLV>(i: u32) -> impl PacketWriter<B> {
    tlv(0x521, move |w: &mut B| {
        w.put_u32(i);
        w.put_u16(0);
    })
}

pub fn t525<'a, B: BufMut + WriteLV, T: PacketWriter<B> + 'a>(
    t536: T,
) -> impl PacketWriter<B> + 'a {
    tlv(0x525, move |w: &mut B| {
        w.put_u16(1);
        t536.write(w);
    })
}

pub fn t536<B: BufMut + WriteLV>(login_extra_data: &[u8]) -> impl PacketWriter<B> + '_ {
    tlv(0x526, login_extra_data)
}

pub fn guid_flag() -> u32 {
    let mut flag: u32 = 0;
    flag |= 1 << 24 & 0xFF000000;
    flag |= 0; // flag |= 0 << 8 & 0xFF00;
    flag
}

#[cfg(test)]
mod tests {
    use crate::command::wtlogin::tlv_writer::*;

    const GUID: [u8; 16] = [
        142, 27, 163, 177, 172, 31, 181, 137, 118, 115, 8, 126, 24, 49, 54, 169,
    ];
    const TGTGT_KEY: [u8; 16] = [
        199, 12, 183, 107, 3, 28, 81, 148, 116, 20, 229, 112, 0, 64, 152, 255,
    ];
    const UIN: u32 = 349195854;
    const OS_NAME: &str = "android";
    const OS_VERSION: &str = "7.1.2";
    const SIM_INFO: &str = "T-Mobile";
    const IMEI: &str = "468356291846738";
    const IMEI_MD5: &[u8] = "9792b1bba1867318bf782af418306ef8".as_bytes();
    const WIFI_BSSID: &str = "00:50:56:C0:00:08";
    const WIFI_SSID: &str = "<unknown ssid>";
    const APN: &str = "wifi";
    const APK_SIGN: [u8; 16] = [
        0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6,
        0x8D,
    ];
    const APK_ID: &str = "com.tencent.mobileqq";
    const APP_ID: u32 = 537066738;
    const SUB_APP_ID: u32 = 537066738;
    const SSO_VERSION: u32 = 15;
    const SDK_VERSION: &str = "6.0.0.2454";
    const MISC_BITMAP: u32 = 184024956;
    const SUB_SIG_MAP: u32 = 0x10400;
    const MAIN_SIG_MAP: u32 = 34869472;
    const MAC_ADDRESS: &str = "00:50:56:C0:00:08";
    const IS_ROOT: bool = false;
    const ANDROID_ID: &str = "QKQ1.191117.002";
    const APK_VERSION_NAME: &str = "2.0.5";
    const DEV_INFO: &[u8] = "dev_info_dev_info_dev_info_dev_info_dev_info_".as_bytes();
    const BUILD_MODEL: &str = "mirai";
    const BUILD_BRAND: &str = "mamoe";
    const OS_TYPE: &str = "android";

    #[test]
    fn test_param() {
        println!("{GUID:?}");
        println!("{:?}", "test param");
    }

    #[test]
    fn test_t1() {
        let result = t1(UIN, &[192, 168, 1, 1]);
        println!("{:?}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t1b() {
        let result = t1b(0, 0, 3, 4, 72, 2, 2);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t1d() {
        let result = t1d(MISC_BITMAP);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t1f() {
        let result = t1f(IS_ROOT, OS_NAME, OS_VERSION, "China Mobile GSM", APN, 2);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t2() {
        let result = t2("result".to_string(), "sign".as_ref());
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t8() {
        let result = t8(123456);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t10a() {
        let result = t10a(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t16() {
        let result = t16(
            SSO_VERSION,
            APP_ID,
            SUB_APP_ID,
            &GUID,
            APK_ID,
            APK_VERSION_NAME,
            &APK_SIGN,
        );
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t16a() {
        let result = t16a(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t16e() {
        let result = t16e(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t17a() {
        let result = t17a(UIN as i32);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t17c() {
        let result = t17c(IMEI);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t18() {
        let result = t18(APP_ID, UIN);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t33() {
        let result = t33(&GUID);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t35() {
        let product_type = 8;
        let result = t35(product_type);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t52d() {
        let result = t52d(DEV_INFO);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t100() {
        let result = t100(SSO_VERSION, 2, MAIN_SIG_MAP);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t104() {
        let result = t104(&GUID);
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t106() {
        let result = t106(
            UIN,
            0,
            APP_ID,
            SSO_VERSION,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            true,
            &GUID,
            &TGTGT_KEY,
            0,
        );
        println!("{}", result.len());
        println!("{result:?}")
    }

    #[test]
    fn test_t107() {
        let result = t107(3);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t108() {
        let result = t108(IMEI.as_bytes());
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t109() {
        let result = t109(ANDROID_ID);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t116() {
        let result = t116(MAIN_SIG_MAP, SUB_SIG_MAP);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t124() {
        let result = t124(OS_TYPE, OS_VERSION, SIM_INFO, APN);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t128() {
        let result = t128(false, true, false, 16, BUILD_MODEL, &GUID, BUILD_BRAND);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t141() {
        let result = t141(SIM_INFO, APN);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t142() {
        let result = t142(APK_ID);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t143() {
        let result = t143(&[1, 2, 3]);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t144() {
        let result = t144(
            IMEI,
            DEV_INFO,
            OS_TYPE,
            OS_VERSION,
            SIM_INFO,
            APN,
            false,
            true,
            false,
            16,
            BUILD_MODEL,
            &GUID,
            BUILD_BRAND,
            &TGTGT_KEY,
        );
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t145() {
        let result = t145(&GUID);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t147() {
        let result = t147(16, APK_VERSION_NAME, &APK_SIGN);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t154() {
        let seq = (0x3635 + 1) & 0x7FFF;
        println!("{seq}");
        let result = t154(seq);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t166() {
        let result = t166(1);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t174() {
        let result = t174(&GUID);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t177() {
        let result = t177(MISC_BITMAP, SDK_VERSION);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t187() {
        let result = t187(MAC_ADDRESS);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t188() {
        let result = t188(ANDROID_ID);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t191() {
        let result = t191(127_u8);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t193() {
        let result = t193("some ticket");
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t194() {
        let result = t194(IMEI_MD5);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t197() {
        let result = t197();
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t198() {
        let result = t198();
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t202() {
        let result = t202(WIFI_BSSID, WIFI_SSID);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t400() {
        let result = t400(&GUID, UIN as i64, &GUID, &GUID, 2, 2, &GUID);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t401() {
        let result = t401(&GUID);
        println!("{}", result.len());
        println!("{result:?}");
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
        println!("{result:?}");
    }

    #[test]
    fn test_t516() {
        let result = t516();
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t521() {
        let result = t521(6);
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_t525() {
        let result = t525(&t536(&GUID));
        println!("{}", result.len());
        println!("{result:?}");
    }

    #[test]
    fn test_tlv() {
        let result = guid_flag();
        println!("{result:?}");
    }
}
