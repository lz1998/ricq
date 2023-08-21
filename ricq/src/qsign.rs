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
pub struct RegisterResponse<T> {
    pub code: i64,
    pub msg: String,
    pub data: T,
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
    ) -> reqwest::Result<RegisterResponse<String>> {
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
    ) -> reqwest::Result<RegisterResponse<String>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::qimei::get_qimei;
    use rand::SeedableRng;
    use ricq_core::protocol::device::Device;
    use ricq_core::protocol::sig::Sig;
    use ricq_core::protocol::version::ANDROID_PHONE;

    #[tokio::test]
    async fn test_custom_energy() {
        let mut rng = rand::prelude::StdRng::seed_from_u64(10);
        let version = ANDROID_PHONE;
        let mut device = Device::random_with_rng(&mut rng);
        let qimei = get_qimei(&mut rng, &device, &version).await.unwrap();
        device.set_qimei(qimei);
        let uin = 12345;
        let sig = Sig::new(&device);
        let cli = QSignClient::new(
            String::from("http://localhost:8080"),
            String::from("114514"),
            Duration::from_secs(60),
        )
        .unwrap();
        let resp = cli
            .register(
                uin,
                &device.qimei.as_ref().unwrap().q36,
                &device.android_id,
                &sig.guid,
            )
            .await;
        println!("{resp:?}");
    }
}
