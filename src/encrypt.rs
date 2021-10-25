use openssl::bn::{BigNum, BigNumContext};
use openssl::ec::{EcGroup, EcPoint, EcKey, PointConversionForm};
use openssl::nid::Nid;
use crate::hex::decode_hex;

pub trait IEncryptMethod {
    fn id(&self) -> u8;
    fn do_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8>;
}

#[derive(Debug)]
pub struct EncryptECDH {
    initial_share_key: Vec<u8>,
    public_key: Vec<u8>,
    public_key_ver: u16,
}

impl EncryptECDH {
    pub fn new() -> EncryptECDH {
        let mut ecdh = EncryptECDH {
            initial_share_key: vec![],
            public_key: vec![],
            public_key_ver: 1,
        };
        ecdh.generate_key("04EBCA94D733E399B2DB96EACDD3F69A8BB0F74224E2B44E3357812211D2E62EFBC91BB553098E25E33A799ADC7F76FEB208DA7C6522CDB0719A305180CC54A82E");
        ecdh
    }
    pub fn generate_key(&mut self, s_pub_key: &str) {
        let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap();
        let mut ctx = BigNumContext::new().unwrap();

        let server_public_key = decode_hex(s_pub_key).unwrap();
        let server_public_key = EcPoint::from_bytes(&group, &server_public_key, &mut ctx).unwrap();

        let client_key = EcKey::generate(&group).unwrap();
        let client_public_key = client_key.public_key().to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut ctx).unwrap();


        let mut shared_key = EcPoint::new(&group).unwrap();
        shared_key.mul(&group, &server_public_key, client_key.private_key(), &mut ctx).unwrap();
        let mut x = BigNum::new().unwrap();
        let mut y = BigNum::new().unwrap();
        shared_key.affine_coordinates(&group, &mut x, &mut y, &mut ctx).unwrap();

        self.initial_share_key = md5::compute(&x.to_vec()[0..16]).to_vec();
        self.public_key = client_public_key;
        self.public_key_ver = 1;
    }
}

impl IEncryptMethod for EncryptECDH {
    fn id(&self) -> u8 {
        return 0x87;
    }

    fn do_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        todo!()
    }
}

pub struct EncryptSession {
    t133: Vec<u8>,
}

impl IEncryptMethod for EncryptSession {
    fn id(&self) -> u8 {
        return 69;
    }

    fn do_encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Mul;
    use openssl::bn::{BigNum, BigNumContext};
    use openssl::ec::{EcGroup, EcPoint, EcKey, PointConversionForm};
    use openssl::nid::Nid;
    use crate::encrypt::{EncryptECDH, IEncryptMethod};
    use crate::hex::decode_hex;
    use md5;
    // use p256::{EncodedPoint, PublicKey, AffinePoint};
    // use p256::elliptic_curve::sec1::FromEncodedPoint;


    #[test]
    fn test_ecdh_generate_key() {
        let mut e = EncryptECDH::new();
        e.generate_key("04EBCA94D733E399B2DB96EACDD3F69A8BB0F74224E2B44E3357812211D2E62EFBC91BB553098E25E33A799ADC7F76FEB208DA7C6522CDB0719A305180CC54A82E");
        println!("{:?}", e.initial_share_key);
        println!("{:?}", e.public_key);
    }
}