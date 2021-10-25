use bytes::{Buf, BufMut};
use crate::hex::decode_hex;
use crate::tea::qqtea_encrypt;

pub trait BinaryWriter {
    fn write_bytes_short(&mut self, data: &[u8]);
    fn encrypt_and_write(&mut self, key: &[u8], data: &[u8]);
    fn write_hex(&mut self, h: &str);
    fn write_int_lv_packet(&mut self, offset: usize, data: &[u8]);
    fn write_string(&mut self, v: &str);
    fn write_tlv_limited_size(&mut self, data: &[u8], limit: isize);
}

impl<B> BinaryWriter for B
    where B: BufMut {
    fn write_bytes_short(&mut self, data: &[u8]) {
        self.put_u16(data.chunk().len() as u16);
        self.put_slice(data.chunk())
    }

    fn encrypt_and_write(&mut self, key: &[u8], data: &[u8]) {
        let ed = qqtea_encrypt(data, key);
        self.put_slice(&ed)
    }

    fn write_hex(&mut self, h: &str) {
        let b = decode_hex(h).unwrap();
        self.put_slice(&b);
    }

    fn write_int_lv_packet(&mut self, offset: usize, data: &[u8]) {
        self.put_u32(((data.len() + offset) as u32));
        self.put_slice(&data);
    }

    fn write_string(&mut self, v: &str) {
        let payload = v.as_bytes();
        self.put_u32((payload.len() + 4) as u32);
        self.put_slice(&payload)
    }

    fn write_tlv_limited_size(&mut self, data: &[u8], limit: isize) {
        if data.len() <= limit as usize {
            self.write_bytes_short(data);
            return;
        }
        let mut count: usize = 0;
        while count != limit as usize {
            self.put_u8(data[count]);
            count += 1;
        }
    }
}
