use bytes::{BufMut, Bytes, BytesMut};

use crate::binary::BinaryWriter;
use crate::crypto::EncryptECDH;
use crate::RQResult;

pub enum EncryptionMethod {
    ECDH,
    ST,
}

pub struct Message {
    pub uin: u32,
    pub command: u16,
    pub body: Bytes,
    pub encryption_method: EncryptionMethod,
}

pub struct Codec {
    pub ecdh: EncryptECDH,
    pub random_key: Bytes,
    pub wt_session_ticket_key: Bytes,
}
impl Codec {
    pub fn encode(&self, m: Message) -> Bytes {
        let mut w = BytesMut::new();
        w.put_u8(0x02);
        w.put_u16(0); // TODO w.len()
        w.put_u16(8001);
        w.put_u16(m.command);
        w.put_u16(1);
        w.put_u32(m.uin);
        w.put_u8(0x03);
        match m.encryption_method {
            EncryptionMethod::ECDH => w.put_u8(0x87),
            EncryptionMethod::ST => w.put_u8(0x45),
        }
        w.put_u8(0);
        w.put_u32(2);
        w.put_u32(0);
        w.put_u32(0);
        match m.encryption_method {
            EncryptionMethod::ECDH => {
                w.put_u8(0x02);
                w.put_u8(0x01);
                w.put_slice(&self.random_key);
                w.put_u16(0x01_31);
                w.put_u16(self.ecdh.public_key_ver);
                w.put_u16(self.ecdh.public_key.len() as u16);
                w.put_slice(&self.ecdh.public_key);
                w.encrypt_and_write(&self.ecdh.initial_share_key, &m.body);
            }
            EncryptionMethod::ST => {
                w.put_u8(0x01);
                w.put_u8(0x03);
                w.put_slice(&self.random_key);
                w.put_u16(0x0102);
                w.put_u16(0x0000);
                w.encrypt_and_write(&self.random_key, &m.body);
            }
        }
        w.put_u8(0x03);

        // TODO 不知道有没有更好的写法
        let len = w.len() as u16;
        w.as_mut()[1..3].copy_from_slice(&len.to_be_bytes());

        w.freeze()
    }

    pub fn decode(&self, b: Bytes) -> RQResult<Message> {
        todo!()
    }
}
