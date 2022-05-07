use bytes::{Buf, BufMut, Bytes, BytesMut};
use rand::Rng;

use crate::binary::BinaryWriter;
use crate::crypto::{qqtea_decrypt, EncryptECDH};
use crate::{RQError, RQResult};

#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
pub enum EncryptionMethod {
    #[derivative(Default)]
    ECDH,
    ST,
}

#[derive(Default)]
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

impl Default for Codec {
    fn default() -> Self {
        Self {
            ecdh: Default::default(),
            random_key: Bytes::from(rand::thread_rng().gen::<[u8; 16]>().to_vec()),
            wt_session_ticket_key: Default::default(),
        }
    }
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

        let len = w.len();
        w[1..3].as_mut().put_u16(len as u16);
        w.freeze()
    }

    pub fn decode<B>(&self, mut reader: B) -> RQResult<Message>
    where
        B: Buf,
    {
        let flag = reader.get_u8();
        if flag != 2 {
            return Err(RQError::UnknownFlag(flag));
        }
        let mut m = Message::default();
        reader.get_u16(); // len
        reader.get_u16(); // version
        m.command = reader.get_u16();
        reader.get_u16(); // 1
        m.uin = reader.get_i32() as u32;
        reader.get_u8();
        let encrypt_type = reader.get_u8();
        reader.get_u8();
        match encrypt_type {
            0 => {
                let len = reader.remaining() - 1;
                let d = reader.copy_to_bytes(len);
                m.body = Bytes::from(qqtea_decrypt(&d, &self.ecdh.initial_share_key));
            }
            3 => {
                let len = reader.remaining() - 1;
                let d = reader.copy_to_bytes(len);
                m.body = Bytes::from(qqtea_decrypt(&d, &self.wt_session_ticket_key));
            }
            _ => return Err(RQError::UnknownEncryptType),
        }
        Ok(m)
    }
}
