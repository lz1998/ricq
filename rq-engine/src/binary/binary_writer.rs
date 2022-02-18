use crate::crypto::qqtea_encrypt;
use crate::hex::decode_hex;
use bytes::{Buf, BufMut, BytesMut};

pub trait BinaryWriter {
    fn write_bytes_short(&mut self, data: &[u8]);
    fn encrypt_and_write(&mut self, key: &[u8], data: &[u8]);
    fn write_hex(&mut self, h: &str);
    fn write_int_lv_packet(&mut self, offset: usize, data: &[u8]);
    fn write_string(&mut self, v: &str);
    fn write_uni_packet(
        &mut self,
        command_name: &str,
        session_id: &[u8],
        extra_data: &[u8],
        body: &[u8],
    );
    fn write_tlv_limited_size(&mut self, data: &[u8], limit: isize);
}

impl<B> BinaryWriter for B
where
    B: BufMut,
{
    fn write_bytes_short(&mut self, data: &[u8]) {
        self.put_u16(data.len() as u16);
        self.put_slice(data.chunk())
    }

    fn encrypt_and_write(&mut self, key: &[u8], data: &[u8]) {
        let ed = qqtea_encrypt(data, key);
        self.put_slice(&ed)
    }

    fn write_hex(&mut self, h: &str) {
        let b = decode_hex(h).expect("write_hex failed");
        self.put_slice(&b);
    }

    fn write_int_lv_packet(&mut self, offset: usize, data: &[u8]) {
        self.put_u32((data.len() + offset) as u32);
        self.put_slice(data);
    }

    fn write_string(&mut self, v: &str) {
        let payload = v.as_bytes();
        self.put_u32((payload.len() + 4) as u32);
        self.put_slice(payload)
    }

    fn write_uni_packet(
        &mut self,
        command_name: &str,
        session_id: &[u8],
        extra_data: &[u8],
        body: &[u8],
    ) {
        let mut w1 = BytesMut::new();
        {
            w1.write_string(command_name);
            w1.put_u32(8);
            w1.put_slice(session_id);
            if extra_data.is_empty() {
                w1.put_u32(0x04)
            } else {
                w1.put_u32((extra_data.len() + 4) as u32);
                w1.put_slice(extra_data);
            }
        }

        self.put_u32((w1.len() + 4) as u32);
        self.put_slice(&w1);
        self.put_u32((body.len() + 4) as u32);
        self.put_slice(body);
    }

    fn write_tlv_limited_size(&mut self, data: &[u8], limit: isize) {
        if data.len() <= limit as usize {
            self.write_bytes_short(data);
            return;
        }
        self.write_bytes_short(&data[..(limit as usize)])
    }
}
