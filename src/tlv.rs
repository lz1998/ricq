use bytes::{Buf, BufMut, Bytes, BytesMut};
use chrono::Utc;
use crate::writer::BinaryWriter;


pub fn t1(uin: u32, ip: &Vec<u8>) -> Vec<u8> {
    if ip.len() != 4 {
        panic!("invalid ip")
    }
    let mut buf = Vec::new();
    buf.put_u16(0x01);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(1);
        w.put_u32(rand::random());
        w.put_u32(uin);
        w.put_u32(Utc::now().timestamp_millis() as u32);
        w.put_slice(&ip);
        w.put_u16(0);
        w
    });
    return buf;
}

pub fn t1b(micro: u32, version: u32, size: u32, margin: u32, dpi: u32, ecLevel: u32, hint: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x1b);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u32(micro);
        w.put_u32(version);
        w.put_u32(size);
        w.put_u32(margin);
        w.put_u32(dpi);
        w.put_u32(ecLevel);
        w.put_u32(hint);
        w.put_u16(0);
        w
    });
    return buf;
}

pub fn t1d(misc_bitmap: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x1d);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u8(1);
        w.put_u32(misc_bitmap);
        w.put_u32(0);
        w.put_u8(0);
        w.put_u32(0);
        w
    });
    return buf;
}

pub fn t1f(is_root: bool, os_name: &[u8], os_version: &[u8], sim_operator_name: &[u8], apn: &[u8], network_type: u16) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x1f);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u8(if is_root { 1 } else { 0 });
        w.write_bytes_short(os_name);
        w.write_bytes_short(os_version);
        w.put_u16(network_type);
        w.write_bytes_short(sim_operator_name);
        w.write_bytes_short(&vec![]);
        w.write_bytes_short(apn);
        w
    });
    return buf;
}

pub fn t2(result: &str, sign: &Vec<u8>) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x02);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(0);
        w.write_string(result);
        w.write_bytes_short(sign);
        w
    });
    return buf;
}

pub fn t8(local_id: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x8);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(0);
        w.put_u32(local_id);
        w.put_u16(0);
        w
    });
    return buf;
}

pub fn t10a(arr: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x10A);
    buf.write_bytes_short(arr);
    return buf;
}

pub fn t16(sso_version: u32, app_id: u32, sub_app_id: u32, guid: &[u8], apk_id: &[u8], apk_version_name: &[u8], apk_sign: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x16);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u32(sso_version);
        w.put_u32(app_id);
        w.put_u32(sub_app_id);
        w.put_slice(guid);
        w.write_bytes_short(apk_id);
        w.write_bytes_short(apk_version_name);
        w.write_bytes_short(apk_sign);
        w
    });
    return buf;
}

pub fn t16a(arr: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x16A);
    buf.write_bytes_short(arr);
    return buf;
}

pub fn t16e(build_model: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x16A);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_slice(&build_model);
        w
    });
    return buf;
}

pub fn t17a(value: i32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x17a);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u32(value as u32);
        w
    });
    return buf;
}

pub fn t17c(code: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x17c);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.write_bytes_short(&code.as_bytes().to_vec());
        w
    });
    return buf;
}

pub fn t18(app_id: u32, uin: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x18);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(1);
        w.put_u32(1536);
        w.put_u32(app_id);
        w.put_u32(0);
        w.put_u32(uin);
        w.put_u16(0);
        w.put_u16(0);
        w
    });
    return buf;
}

pub fn t33(guid: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x33);
    buf.write_bytes_short(guid);
    return buf;
}

pub fn t35(product_type: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x35);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u32(product_type);
        w
    });
    return buf;
}

pub fn t52d(dev_info: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x52d);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_slice(&dev_info);
        w
    });
    return buf;
}

pub fn t100(sso_version: u32, protocol: u32, main_sig_map: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x100);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(1);
        w.put_u32(sso_version);
        w.put_u32(16);
        w.put_u32(protocol);
        w.put_u32(0);             // App client version
        w.put_u32(main_sig_map);  // 34869472
        w
    });
    return buf;
}

pub fn t104(data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x104);
    buf.write_bytes_short(&data);
    buf
}

pub fn t106(uin: u32, salt: u32, app_id: u32, sso_ver: u32, password_md5: [u8; 16], guid_available: bool, guid: &[u8], tgtgt_key: &[u8], wtf: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x106);
    buf
}

pub fn t107(pic_type: u16) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x107);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(pic_type);
        w.push(0x00);
        w.put_u16(0);
        w.push(0x01);
        w
    });
    buf
}

pub fn t108(imei: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x108);
    buf.write_bytes_short(&imei.as_bytes().to_vec());
    buf
}

pub fn t109(android_id: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x109);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        let mut v: Vec<u8> = Vec::new();
        for i in android_id {
            v.push(*i)
        }
        w.put_slice(md5::compute(&v).as_ref());
        w
    });
    buf
}

pub fn t116(misc_bitmap: u32, sub_sig_map: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x116);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.push(0x00);
        w.put_u32(misc_bitmap);
        w.put_u32(sub_sig_map);
        w.push(0x01);
        w.put_u32(1600000226);  // app id list
        w
    });
    buf
}

pub fn t124(os_type: &[u8], os_version: &[u8], sim_info: &[u8], apn: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x124);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.write_tlv_limited_size(os_type, 16);
        w.write_tlv_limited_size(os_version, 16);
        w.put_u16(2);
        w.write_tlv_limited_size(sim_info, 16);
        let b: Vec<u8> = Vec::new();
        w.write_tlv_limited_size(&b, 16);
        w.write_tlv_limited_size(apn, 16);
        w
    });
    buf
}

pub fn t128(is_guid_from_file_null: bool, is_guid_available: bool, is_guid_changed: bool, guid_flag: u32, build_model: &[u8], guid: &[u8], build_brand: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x128);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(0);
        w.put_u8(if is_guid_from_file_null { 1 } else { 0 });
        w.put_u8(if is_guid_available { 1 } else { 0 });
        w.put_u8(if is_guid_changed { 1 } else { 0 });
        w.put_u32(guid_flag);
        w.write_tlv_limited_size(build_model, 32);
        w.write_tlv_limited_size(guid, 16);
        w.write_tlv_limited_size(build_brand, 16);  // app id list
        w
    });
    buf
}

pub fn t141(sim_info: &[u8], apn: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x141);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(1);
        w.write_bytes_short(sim_info);
        w.put_u16(2);
        w.write_bytes_short(apn);
        w
    });
    buf
}

pub fn t142(apk_id: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x142);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(0);
        w.write_tlv_limited_size(apk_id, 32);
        w
    });
    buf
}

pub fn t143(arr: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x143);
    buf.write_bytes_short(arr);
    buf
}

pub fn t144(imei: &[u8], dev_info: &[u8], os_type: &[u8], os_version: &[u8], sim_info: &[u8], apn: &[u8],
            is_guid_from_file_null: bool, is_guid_available: bool, is_guid_changed: bool,
            guid_flag: u32,
            build_model: &[u8], guid: &[u8], build_brand: &[u8], tgtgt_key: &[u8]
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x144);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.encrypt_and_write(tgtgt_key, &{
            let mut ww = Vec::new();
            ww.put_u16(5);
            ww.put_slice(t109(imei).as_slice());
            ww.put_slice(t52d(dev_info).as_slice());
            ww.put_slice(t124(os_type, os_version, sim_info, apn).as_slice());
            ww.put_slice(t128(is_guid_from_file_null, is_guid_available, is_guid_changed, guid_flag, build_model, guid, build_brand).as_slice());
            ww.put_slice(t16e(build_model).as_slice());
            ww
        });
        w
    });
    buf
}


pub fn t145(guid: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x145);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_slice(&guid);
        w
    });
    buf
}

pub fn t147(app_id: u32, apk_version_name: &[u8], apk_signature_md5: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x147);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u32(app_id);
        w.write_tlv_limited_size(apk_version_name, 32);
        w.write_tlv_limited_size(apk_signature_md5, 32);
        w
    });
    buf
}

pub fn t154(seq: u16) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x154);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u32(seq as u32);
        w
    });
    buf
}

pub fn t166(image_type: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x166);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u8(image_type);
        w
    });
    buf
}

pub fn t174(data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x174);
    buf.write_bytes_short(&data);
    buf
}

pub fn t177(build_time: u32, sdk_version: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x177);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u8(0x01);
        w.put_u32(build_time);
        w.put_slice(sdk_version.as_bytes());
        w
    });
    buf
}

pub fn t187(mac_address: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x187);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_slice(md5::compute(&mac_address).as_ref());
        w
    });
    buf
}

pub fn t188(android_id: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x188);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        let mut v: Vec<u8> = Vec::new();
        for i in android_id {
            v.push(*i)
        }
        w.put_slice(md5::compute(&v).as_ref());
        w
    });
    buf
}

pub fn t191(k: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x191);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u8(k);
        w
    });
    buf
}

pub fn t193(ticket: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x193);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_slice(ticket.as_bytes());
        w
    });
    buf
}

pub fn t194(imsi_md5: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x194);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_slice(imsi_md5);
        w
    });
    buf
}

pub fn t197() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x197);
    let v: [u8; 1] = [0];
    buf.write_bytes_short(&v);
    buf
}

pub fn t198() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x198);
    let v: [u8; 1] = [0];
    buf.write_bytes_short(&v);
    buf
}

pub fn t202(wifi_bssid: &[u8], wifi_ssid: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x202);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.write_tlv_limited_size(wifi_bssid, 16);
        w.write_tlv_limited_size(wifi_ssid, 32);
        w
    });
    buf
}

pub fn t400(g: &[u8], uin: i64, guid: &[u8], dpwd: &[u8], j2: i64, j3: i64, rand_seed: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x400);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.encrypt_and_write(&g, &{
            let mut ww = Vec::new();
            ww.put_u16(1);
            ww.put_u64(uin as u64);
            ww.put_slice(&guid);
            ww.put_slice(&dpwd);
            ww.put_u32(j2 as u32);
            ww.put_u32(j3 as u32);
            ww.put_u32(Utc::now().timestamp_millis() as u32);
            ww
        });
        w
    });
    buf
}

pub fn t401(d: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x401);
    buf.write_bytes_short(&d);
    buf
}

// TODO
pub fn t511(domains: Vec<&str>) -> Vec<u8> {
    let mut arr2 = Vec::new();
    for d in domains {
        if d != "" {
            arr2.push(d)
        }
    }
    let mut buf = Vec::new();
    buf.put_u16(0x511);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(arr2.len() as u16);
        for d in arr2 {

        }
        w
    });
    buf
}

pub fn t516() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x516);
    let v: [u8; 1] = [0];
    buf.write_bytes_short(&v);
    buf
}

pub fn t521(i: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x521);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u32(i);
        w.put_u16(0);
        w
    });
    buf
}

pub fn t525(t536: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x525);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_u16(1);
        w.put_slice(&t536);
        w
    });
    buf
}

pub fn t536(login_extra_data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.put_u16(0x536);
    buf.write_bytes_short(&{
        let mut w = Vec::new();
        w.put_slice(&login_extra_data);
        w
    });
    buf
}

pub fn guid_flag() -> u32 {
    let mut flag: u32 = 0;
    flag |= 1 << 24 & 0xFF000000;
    flag |= 0 << 8 & 0xFF00;
    flag
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::tlv::{t1, t104, t109, t124, t128, t144, t16, t16e, t17c, t187, t191, t193, t194, t197, t1b, t1d, t1f, t33, t35, t52d};

    #[test]
    fn test_t1() {
        let result = t1(875543533, &vec![192, 168, 1, 1]);
        println!("{}", result.len());
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
        let result = t1d(16252796);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t1f() {
        let result = t1f(false, "android".as_bytes(), "7.1.2".as_bytes(), "China Mobile GSM".as_bytes(), "wifi".as_bytes(), 2);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t16() {
        let sso_version = 5;
        let app_id = 16;
        let sub_app_id = 537064446;
        let guid = vec![1, 2, 3]; // 这是一个md5
        let apk_id = "com.tencent.qqlite".as_bytes();
        let apk_version_name = "2.0.5".as_bytes();
        let apk_sign = vec![0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6, 0x8D];
        let result = t16(5, 16, 537064446, &guid, apk_id, apk_version_name, &apk_sign);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t16e() {
        let build_model = "".as_bytes();
        let result = t16e(&build_model);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t17c() {
        let result = t17c("123");
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t33() {
        let guid = vec![1, 2, 3]; // 这是一个md5
        let result = t33(&guid);
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
        let dev_info = vec![1, 2, 3];
        let result = t52d(&dev_info);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t104() {
        let guid = vec![1, 2, 3]; // 这是一个md5
        let result = t104(&guid);
        println!("{}", result.len());
        println!("{:?}", result)
    }

    #[test]
    fn test_t109() {
        let guid = vec![1, 2, 3];
        let result = t109(&guid);
        println!("{}", result.len());
        println!("{:?}", result);
        println!("{:?}", md5::compute(&guid).as_ref())
    }

    #[test]
    fn test_t124() {
        let guid = vec![1, 2, 3];
        let result = t124(&guid,&guid,&guid,&guid);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t128() {
        let guid = vec![1, 2, 3, 4, 5, 6];
        let result = t128(false, false, false,16,&guid, &guid,&guid);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    // TODO
    fn test_t144_break() {
        let id = "QKQ1.191117.002".as_bytes();
        let num_id = "860954519384918".as_bytes();
        let guid = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let os_type = "android".as_bytes();
        let os_version = "7.1.2".as_bytes();
        let apn = "wifi".as_bytes();
        let sim_info = "sim info".as_bytes();
        let build_model = "umi".as_bytes();
        let build_brand = "Xiaomi".as_bytes();

        let result1 = t109(id);
        let result2 = t52d(num_id);
        let result3 = t124(os_type, os_version, sim_info, apn);
        let result4 = t128(false, true, false,
                           16, build_model, &guid, build_brand);
        let result5 = t16e(build_model);
        println!("{}", result1.len());
        println!("{:?}", result1);
        println!("{}", result2.len());
        println!("{:?}", result2);
        println!("{}", result3.len());
        println!("{:?}", result3);
        println!("{}", result4.len());
        println!("{:?}", result4);
        println!("{}", result5.len());
        println!("{:?}", result5);
    }

    #[test]
    fn test_t144() {
        let guid = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let result = t144("860954519384918".as_bytes(), &guid, "android".as_bytes(),
                          "7.1.2".as_bytes(), &guid, "wifi".as_bytes(),
                          false, true, false,
                          16,"umi".as_bytes(), &guid, "Xiaomi".as_bytes(), &guid);
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t187() {
        let result = t187("08:00:20:0A:8C:6D");
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t191() {
        let result = t191(127 as u8);
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
        let mut imsi_md5 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut imsi_md5 = "2a168c450df2e42b0c088f7b891650b5";
        let result = t194(imsi_md5.as_bytes());
        println!("{}", result.len());
        println!("{:?}", result);
    }

    #[test]
    fn test_t197() {
        let result = t197();
        println!("{}", result.len());
        println!("{:?}", result);
    }
}