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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::tlv::{t1, t16, t1b, t1d, t1f, t33, t35};

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
}