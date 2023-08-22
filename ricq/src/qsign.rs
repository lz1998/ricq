use bytes::{BufMut, BytesMut};
use ricq_core::binary::packet_writer::WriteLV;
use ricq_core::hex::encode_hex;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct QSignClient {
    url: String,
    key: String,
    client: reqwest::Client,
    timeout: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QSignResponse<T> {
    pub code: i64,
    pub msg: String,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignData {
    pub token: String,

    pub extra: String,

    pub sign: String,

    #[serde(rename = "o3did")]
    pub o3_did: String,

    pub request_callback: Vec<RequestCallback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestCallback {
    pub cmd: String,

    pub body: String,

    pub callback_id: i64,
}

impl QSignClient {
    pub fn new(mut url: String, key: String, timeout: Duration) -> reqwest::Result<Self> {
        let client = reqwest::ClientBuilder::new().build()?;
        if url.ends_with('/') {
            url.pop();
        }
        Ok(QSignClient {
            url,
            key,
            client,
            timeout,
        })
    }

    pub async fn register(
        &self,
        uin: i64,
        qimei36: &str,
        android_id: &str,
        guid: &[u8],
    ) -> reqwest::Result<QSignResponse<String>> {
        let url = format!("{}/register", self.url);
        self.client
            .get(url)
            .query(&[
                ("uin", uin.to_string().as_str()),
                ("android_id", android_id),
                ("guid", encode_hex(guid).as_str()),
                ("qimei36", qimei36),
                ("key", self.key.as_str()),
            ])
            .timeout(self.timeout)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn custom_energy(
        &self,
        uin: i64,
        data: &str,
        salt: &[u8],
        guid: &[u8],
        android_id: &str,
    ) -> reqwest::Result<QSignResponse<String>> {
        let url = format!("{}/custom_energy", self.url);
        self.client
            .get(url)
            .query(&[
                ("uin", uin.to_string().as_str()),
                ("salt", encode_hex(salt).as_str()),
                ("data", data),
                ("android_id", android_id),
                ("guid", encode_hex(guid).as_str()),
            ])
            .timeout(self.timeout)
            .send()
            .await?
            .json()
            .await
    }

    pub fn calc_salt(uin: u64, guid: &[u8], sdk_version: &str, sub_cmd: u32) -> Vec<u8> {
        let mut buf = BytesMut::new();
        match sub_cmd {
            2 | 7 => buf.put_u64(uin),
            9 | 0xa | 0xf => buf.put_u32(0),
            _ => panic!("sub_cmd not supported"),
        }
        buf.write_short_lv(guid);
        buf.write_short_lv(sdk_version.as_bytes());
        buf.put_u32(sub_cmd);
        match sub_cmd {
            0 | 0xa | 0xf => buf.put_u32(0),
            _ => {}
        }
        buf.to_vec()
    }

    pub async fn energy(
        &self,
        uin: i64,
        data: &str,
        version: &str,
        guid: &[u8],
        android_id: &str,
    ) -> reqwest::Result<QSignResponse<String>> {
        let url = format!("{}/energy", self.url);
        self.client
            .get(url)
            .query(&[
                ("version", version),
                ("uin", uin.to_string().as_str()),
                ("data", data),
                ("guid", encode_hex(guid).as_str()),
                ("android_id", android_id),
            ])
            .timeout(self.timeout)
            .send()
            .await?
            .json()
            .await
    }

    // TODO test sign
    #[allow(clippy::too_many_arguments)]
    pub async fn sign(
        &self,
        uin: i64,
        qua: &str,
        cmd: &str,
        seq: u32,
        buffer: &[u8],
        qimei36: &str,
        android_id: &str,
        guid: &[u8],
    ) -> reqwest::Result<QSignResponse<SignData>> {
        let url = format!("{}/sign", self.url);
        self.client
            .post(url)
            .form(&[
                ("uin", uin.to_string().as_str()),
                ("qua", qua),
                ("cmd", cmd),
                ("seq", seq.to_string().as_str()),
                ("buffer", encode_hex(buffer).as_str()),
                ("qimei36", qimei36),
                ("android_id", android_id),
                ("guid", encode_hex(guid).as_str()),
            ])
            .timeout(self.timeout)
            .send()
            .await?
            .json()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::qimei::get_qimei;
    use rand::SeedableRng;
    use ricq_core::protocol::{device::Device, sig::Sig, version::ANDROID_PHONE};

    async fn get_device() -> Device {
        let mut rng = rand::prelude::StdRng::seed_from_u64(10);
        let version = ANDROID_PHONE;
        let mut device = Device::random_with_rng(&mut rng);
        let qimei = get_qimei(&mut rng, &device, &version).await.unwrap();
        device.set_qimei(qimei);
        device
    }

    #[tokio::test]
    async fn test_custom_energy() {
        let uin = 12345;
        let device = get_device().await;
        let sig = Sig::new(&device);
        let cli = QSignClient::new(
            String::from("http://localhost:8080"),
            String::from("114514"),
            Duration::from_secs(60),
        )
        .unwrap();
        let resp = cli
            .custom_energy(
                uin,
                "810_9",
                &[
                    0, 0, 0, 0, 0, 16, 64, 70, 49, 11, 87, 2, 23, 195, 124, 180, 140, 194, 140,
                    180, 113, 103, 0, 10, 54, 46, 48, 46, 48, 46, 50, 53, 52, 54, 0, 0, 0, 9, 0, 0,
                    0, 0,
                ],
                &sig.guid,
                &device.android_id,
            )
            .await;
        println!("{resp:?}");
    }
    #[tokio::test]
    async fn test_energy() {
        let uin = 12345;
        let device = get_device().await;
        let sig = Sig::new(&device);
        let cli = QSignClient::new(
            String::from("http://localhost:8080"),
            String::from("114514"),
            Duration::from_secs(60),
        )
        .unwrap();
        let resp = cli
            .energy(uin, "810_9", "6.0.0.2546", &sig.guid, &device.android_id)
            .await;
        println!("{resp:?}");
    }
}
