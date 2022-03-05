use std::collections::HashMap;

use bytes::{Buf, Bytes};

pub trait BinaryReader {
    fn read_string(&mut self) -> String;
    fn read_string_short(&mut self) -> String;
    fn read_bytes_short(&mut self) -> Bytes;
    fn read_tlv_map(&mut self, tag_size: usize) -> HashMap<u16, Bytes>;
    fn read_string_limit(&mut self, limit: usize) -> String;
}

impl<B> BinaryReader for B
where
    B: Buf,
{
    fn read_string(&mut self) -> String {
        let len = self.get_i32() as usize - 4;
        String::from_utf8_lossy(&self.copy_to_bytes(len)).to_string()
    }

    fn read_string_short(&mut self) -> String {
        let len = self.get_u16() as usize;
        String::from_utf8_lossy(&self.copy_to_bytes(len)).to_string()
    }

    fn read_bytes_short(&mut self) -> Bytes {
        let len = self.get_u16() as usize;
        self.copy_to_bytes(len)
    }

    fn read_tlv_map(&mut self, tag_size: usize) -> HashMap<u16, Bytes> {
        let mut m = HashMap::new();
        loop {
            if self.remaining() < tag_size {
                return m;
            }
            let mut k = 0;
            if tag_size == 1 {
                k = self.get_u8() as u16;
            } else if tag_size == 2 {
                k = self.get_u16();
            } else if tag_size == 4 {
                k = self.get_i32() as u16;
            }
            if k == 255 {
                return m;
            }
            if self.remaining() < 2 {
                return m;
            }
            let len = self.get_u16() as usize;
            if self.remaining() < len {
                return m;
            }
            m.insert(k, self.copy_to_bytes(len));
        }
    }

    fn read_string_limit(&mut self, limit: usize) -> String {
        String::from_utf8_lossy(&self.copy_to_bytes(limit)).to_string()
    }
}
