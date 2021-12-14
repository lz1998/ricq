use bytes::{BufMut, Bytes};
// use openssl::bn::{BigNum, BigNumContext};
// use openssl::ec::{EcGroup, EcPoint, EcKey, PointConversionForm};
// use openssl::nid::Nid;
use super::qqtea_encrypt;
use crate::binary::BinaryWriter;
use crate::hex::decode_hex;
use p256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};

pub trait IEncryptMethod {
    fn id(&self) -> u8;
    fn do_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8>;
}

#[derive(Debug)]
pub struct EncryptECDH {
    pub initial_share_key: Bytes,
    pub public_key: Bytes,
    public_key_ver: u16,
}

impl Default for EncryptECDH {
    fn default() -> Self {
        let mut ecdh = EncryptECDH {
            initial_share_key: Bytes::new(),
            public_key: Bytes::new(),
            public_key_ver: 1,
        };
        ecdh.generate_key("04EBCA94D733E399B2DB96EACDD3F69A8BB0F74224E2B44E3357812211D2E62EFBC91BB553098E25E33A799ADC7F76FEB208DA7C6522CDB0719A305180CC54A82E");
        ecdh
    }
}

impl EncryptECDH {
    pub fn generate_key(&mut self, s_pub_key: &str) {
        let s_pub_key = decode_hex(s_pub_key).unwrap(); // decode pub key
        let secret = EphemeralSecret::random(rand::thread_rng()); // gen private key
        let pub_key = PublicKey::from_sec1_bytes(&s_pub_key).unwrap(); // gen public key

        let share = secret.diffie_hellman(&pub_key); // count public share
        let share_x = &share.as_bytes()[0..16];
        self.initial_share_key = Bytes::copy_from_slice(&md5::compute(share_x).0);

        let self_public_key = secret.public_key();
        let point = EncodedPoint::from(self_public_key);
        self.public_key = Bytes::copy_from_slice(point.as_bytes());
    }
}

impl IEncryptMethod for EncryptECDH {
    fn id(&self) -> u8 {
        return 0x87;
    }

    fn do_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        let mut w = Vec::new();
        w.put_u8(0x02);
        w.put_u8(0x01);
        w.put_slice(key);
        w.put_u16(0x01_31);
        w.put_u16(self.public_key_ver);
        w.put_u16(self.public_key.len() as u16);
        w.put_slice(&self.public_key);
        w.encrypt_and_write(&self.initial_share_key, data);
        w
    }
}

pub struct EncryptSession {
    t133: Vec<u8>,
}

impl EncryptSession {
    pub fn new(t133: &[u8]) -> EncryptSession {
        EncryptSession {
            t133: t133.to_vec(),
        }
    }
}

impl IEncryptMethod for EncryptSession {
    fn id(&self) -> u8 {
        return 69;
    }

    fn do_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        let encrypt = qqtea_encrypt(data, key);
        let mut w = Vec::new();
        w.put_u16(self.t133.len() as u16);
        w.put_slice(&self.t133);
        w.put_slice(&encrypt);
        w
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::EncryptECDH;

    #[test]
    fn test_ecdh_generate_key() {
        let mut e = EncryptECDH::default();
        e.generate_key("04EBCA94D733E399B2DB96EACDD3F69A8BB0F74224E2B44E3357812211D2E62EFBC91BB553098E25E33A799ADC7F76FEB208DA7C6522CDB0719A305180CC54A82E");
        println!("{:?}", e.initial_share_key);
        println!("{:?}", e.public_key);
    }
}
