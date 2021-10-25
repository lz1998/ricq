use bytes::BufMut;
use crate::binary_writer::{BinaryWriter};
use crate::encrypt::IEncryptMethod;
use crate::version::{ClientProtocol, gen_version_info};

pub fn build_oicq_request_packet<E>(uin: u32, command_id: u16, encrypt: &E, key: &[u8], body: &[u8]) -> Vec<u8>
    where E: IEncryptMethod
{
    let body = encrypt.do_encrypt(body, key);
    {
        let mut p = Vec::new();
        p.put_u8(0x02);
        p.put_u16(27 + 2 + (body.len() as u16));
        p.put_u16(8001);
        p.put_u16(command_id);
        p.put_u16(1);
        p.put_u32(uin);
        p.put_u8(3);
        p.put_u8(encrypt.id());
        p.put_u8(0);
        p.put_u32(2);
        p.put_u32(0);
        p.put_u32(0);
        p.put_slice(&body);
        p.put_u8(0x03);
        p
    }
}

pub fn build_sso_packet(seq: u16, app_id: u32, sub_app_id: u32, command_name: &str, imei: &str, ext_data: &[u8], out_packet_session_id: &[u8], body: &[u8], ksid: &[u8]) -> Vec<u8> {
    {
        let mut p = Vec::new();
        p.write_int_lv_packet(4, &{
            let mut writer = Vec::new();
            writer.put_u32(seq as u32);
            writer.put_u32(app_id);
            writer.put_u32(sub_app_id);
            writer.put_slice(&vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00]);
            if ext_data.len() == 0 || ext_data.len() == 4 {
                writer.put_u32(0x04)
            } else {
                writer.put_u32((ext_data.len() + 4) as u32);
                writer.put_slice(&ext_data);
            }
            writer.write_string(command_name);
            writer.write_int_lv_packet(4, &{
                let mut w = Vec::new();
                w.put_slice(out_packet_session_id);
                w
            });
            writer.write_string(imei);
            writer.put_u32(0x04);
            {
                writer.put_u16((ksid.len() + 2) as u16);
                writer.put_slice(ksid);
            }
            writer.put_u32(0x04);
            writer
        });

        p.write_int_lv_packet(4, &{
            let mut writer = Vec::new();
            writer.put_slice(body);
            writer
        });
        p
    }
}

pub fn build_code2d_request_packet(seq: u32, j: u64, cmd: u16, body: &[u8]) -> Vec<u8> {
    let mut w = Vec::new();
    w.put_u8(2);
    w.put_u16((43 + body.len() + 1) as u16);
    w.put_u16(cmd);
    w.put_slice(&vec![0; 21]);
    w.put_u8(3);
    w.put_u16(0);
    w.put_u16(50);
    w.put_u32(seq);
    w.put_u64(j);
    w.put_slice(body);
    w.put_u8(3);
    w
}

pub fn build_login_packet(uin: u32, body_type: u8, key: &[u8], body: &[u8], extra_data: &[u8]) -> Vec<u8> {
    let mut w = Vec::new();

    w.write_int_lv_packet(4, &{
        let mut w = Vec::new();
        w.put_u32(0x00_00_00_0A);
        w.put_u8(body_type);
        w.write_int_lv_packet(4, &{
            let mut w = Vec::new();
            w.put_slice(extra_data);
            w
        });
        w.put_u8(0x00);
        w.write_string(&uin.to_string());
        if key.len() == 0 {
            w.put_slice(body);
        } else {
            w.encrypt_and_write(key, body);
        }
        w
    });
    w
}

pub fn build_uni_packet(uin: i64, seq: u16, command_name: &str, encrypt_type: u8, session_id: &[u8], extra_data: &[u8], key: &[u8], body: &[u8]) -> Vec<u8> {
    let mut w2 = Vec::new();
    {
        w2.put_u32(0x0B);
        w2.put_u8(encrypt_type);
        w2.put_u32(seq as u32);
        w2.put_u8(0);
        w2.write_string(&uin.to_string());

        let mut w3 = Vec::new();
        w3.write_uni_packet(command_name, session_id, extra_data, body);
        w2.encrypt_and_write(key, &w3);
    }
    let mut w = Vec::new();
    w.put_u32((w2.len() + 4) as u32);
    w.put_slice(&w2);
    w
}

// pub fn build_qrcode_fetch_request_packet(seq: u16) -> Vec<u8> {
//     let watch = gen_version_info(&ClientProtocol::AndroidWatch);
// }