use crate::hex::encode_hex;
use crate::protocol::device::Device;
use crate::protocol::version::Version;
use crate::{RQError, RQResult};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::Engine;
use rand::distributions::Slice;
use rand::{CryptoRng, Rng, RngCore};
use rsa::traits::PaddingScheme;
use serde::{Deserialize, Serialize};
use x509_cert::spki::DecodePublicKey;

const RSA_PUB_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDEIxgwoutfwoJxcGQeedgP7FG9
qaIuS0qzfR8gWkrkTZKM2iWHn2ajQpBRZjMSoSf6+KJGvar2ORhBfpDXyVtZCKpq
LQ+FLkpncClKVIrBwv6PHyUvuCb0rIarmgDnzkfQAqVufEtR64iazGDKatvJ9y6B
9NMbHddGSAUmRTCrHQIDAQAB
-----END PUBLIC KEY-----"#;

const SECRET: &str = "ZdJqM15EeO2zWc08";

pub fn aes_decrypt(text: &[u8], key: &[u8]) -> RQResult<Vec<u8>> {
    cbc::Decryptor::<aes::Aes128>::new_from_slices(key, key)?
        .decrypt_padded_vec_mut::<Pkcs7>(text)
        .map_err(Into::into)
}

pub fn aes_encrypt(text: &[u8], key: &[u8]) -> RQResult<Vec<u8>> {
    Ok(cbc::Encryptor::<aes::Aes128>::new_from_slices(key, key)?
        .encrypt_padded_vec_mut::<Pkcs7>(text))
}

pub fn rsa_pub_key() -> RQResult<rsa::RsaPublicKey> {
    rsa::RsaPublicKey::from_public_key_pem(RSA_PUB_KEY).map_err(Into::into)
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceReserved<'a> {
    pub bod: &'a str,
    pub brd: &'a str,
    pub clone: &'a str,
    pub containe: &'a str,
    pub dv: &'a str,
    pub first_level: &'a str,
    pub harmony: &'a str,
    pub host: &'a str,
    pub kelong: &'a str,
    pub kernel: &'a str,
    pub manufact: &'a str,
    pub multi_user: &'a str,
    pub name: &'a str,
    pub oo: &'a str,
    pub oz: &'a str,
    pub uptimes: String,
}

impl<'a> DeviceReserved<'a> {
    pub fn from_device<RNG: RngCore>(rng: &mut RNG, device: &'a Device) -> DeviceReserved<'a> {
        let now = chrono::Local::now();
        let offset = chrono::Duration::seconds(rng.gen_range(0..14400));
        let uptimes = now - offset;
        DeviceReserved {
            bod: &device.board,
            brd: &device.brand,
            clone: "0",
            containe: "",
            dv: &device.device,
            first_level: "",
            harmony: "0",
            host: "se.infra",
            kelong: "0",
            kernel: &device.proc_version,
            manufact: &device.brand,
            multi_user: "0",
            name: &device.model,
            oo: "Xecjt+9S1+f8Pz2VLSxgpw==",
            oz: "UhYmelwouA+V2nPWbOvLTgN2/m8jwGB+yUB5v9tysQg=",
            uptimes: uptimes.format("%F %T").to_string(),
        }
    }
}

pub fn rand_beacon_id<RNG: RngCore>(rng: &mut RNG) -> String {
    let mut beacon_id = String::with_capacity(1024);
    let month = chrono::Local::now().format("%Y-%m-01").to_string();
    let rand1 = rng.gen_range(100000..999999).to_string();
    let rand2 = rng.gen_range(100000000..999999999).to_string();
    for i in 1..=40 {
        match i {
            1 | 2 | 13 | 14 | 17 | 18 | 21 | 22 | 25 | 26 | 29 | 30 | 33 | 34 | 37 | 38 => {
                beacon_id.push('k');
                beacon_id.push_str(i.to_string().as_str());
                beacon_id.push(':');
                beacon_id.push_str(month.as_str());
                beacon_id.push_str(rand1.as_str());
                beacon_id.push('.');
                beacon_id.push_str(rand2.as_str());
            }
            3 => {
                beacon_id.push_str("k3:0000000000000000");
            }
            4 => {
                beacon_id.push_str("k4:");
                beacon_id.push_str(
                    rng.sample_iter(Slice::new("123456789abcdef".as_bytes()).expect("empty slice"))
                        .take(16)
                        .map(|n| *n as char)
                        .collect::<String>()
                        .as_str(),
                );
            }
            _ => {
                beacon_id.push('k');
                beacon_id.push_str(i.to_string().as_str());
                beacon_id.push(':');
                beacon_id.push_str(rng.gen_range(0..10000).to_string().as_str());
            }
        }
        beacon_id.push(';');
    }
    beacon_id
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QimeiRequestPayload<'a> {
    android_id: &'a str,
    app_key: &'a str,
    app_version: &'a str,
    audit: &'a str,
    beacon_id_src: String,
    brand: &'a str,
    channel_id: &'a str,
    cid: &'a str,
    device_type: &'a str,
    imei: &'a str,
    imsi: &'a str,
    mac: &'a str,
    model: &'a str,
    network_type: &'a str,
    oaid: &'a str,
    os_version: String,
    package_id: &'a str,
    platform_id: i64,
    qimei: &'a str,
    qimei36: &'a str,
    reserved: String,
    sdk_name: &'a str,
    sdk_version: &'a str,
    user_id: &'a str,
}

impl<'a> QimeiRequestPayload<'a> {
    pub fn new<RNG: RngCore>(
        rng: &mut RNG,
        device: &'a Device,
        version: &Version,
    ) -> QimeiRequestPayload<'a> {
        let device_reserved = DeviceReserved::from_device(rng, device);
        let beacon_id = rand_beacon_id(rng);
        QimeiRequestPayload {
            android_id: &device.android_id,
            app_key: version.app_key,
            app_version: version.sort_version_name,
            audit: "",
            beacon_id_src: beacon_id,
            brand: &device.brand,
            channel_id: "2017",
            cid: "",
            device_type: "",
            imei: &device.imei,
            imsi: "",
            mac: "",
            model: &device.model,
            network_type: "unknown",
            oaid: "",
            os_version: format!(
                "Android {},level {}",
                device.version.release, device.version.sdk
            ),
            package_id: version.apk_id,
            platform_id: 1,
            qimei: "",
            qimei36: "",
            reserved: serde_json::to_string(&device_reserved).expect("json"),
            sdk_name: "",
            sdk_version: "1.2.13.6",
            user_id: "{}",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct QimeiRequest {
    extra: String,
    key: String,
    nonce: String,
    params: String,
    sign: String,
    time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct QimeiResponse {
    code: i64,
    data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Qimei {
    pub q16: String,
    pub q36: String,
}

impl QimeiRequest {
    pub fn new<RNG: RngCore + CryptoRng>(
        rng: &mut RNG,
        device: &Device,
        version: &Version,
        crypt_key: &[u8],
    ) -> RQResult<QimeiRequest> {
        let payload = serde_json::to_string(&QimeiRequestPayload::new(rng, device, version))?;
        let ts = chrono::Local::now().timestamp() * 1000;
        let nonce = rng
            .sample_iter(Slice::new("abcdef1234567890".as_bytes()).unwrap())
            .take(16)
            .map(|n| *n as char)
            .collect::<String>();
        let pub_key = rsa_pub_key()?;
        let encrypted_aes_key = rsa::Pkcs1v15Encrypt.encrypt(rng, &pub_key, crypt_key)?;
        let encrypted_payload = aes_encrypt(payload.as_bytes(), crypt_key)?;
        let key = base64::engine::general_purpose::STANDARD.encode(encrypted_aes_key);
        let params = base64::engine::general_purpose::STANDARD.encode(encrypted_payload);
        Ok(QimeiRequest {
            extra: "".to_string(),
            sign: encode_hex(
                &md5::compute(format!("{}{}{}{}{}", key, params, ts, nonce, SECRET)).to_vec(),
            ),
            key,
            nonce,
            params,
            time: ts,
        })
    }
}

impl QimeiResponse {
    pub fn to_payload(self, crypt_key: &[u8]) -> RQResult<Qimei> {
        if self.code != 0 {
            return Err(RQError::QimeiError(self.code));
        }
        let encrypted_response = base64::engine::general_purpose::STANDARD.decode(self.data)?;
        let decrypted_response = aes_decrypt(&encrypted_response, crypt_key)?;
        serde_json::from_slice(&decrypted_response).map_err(Into::into)
    }
}
