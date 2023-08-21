use rand::{CryptoRng, RngCore};
use ricq_core::protocol::device::Device;
use ricq_core::protocol::qimei::{Qimei, QimeiRequest, QimeiResponse};
use ricq_core::protocol::version::Version;
use ricq_core::{RQError, RQResult};

pub async fn get_qimei<RNG: RngCore + CryptoRng>(
    rng: &mut RNG,
    device: &Device,
    version: &Version,
) -> RQResult<Qimei> {
    let crypt_key = "0123456789abcdef".as_bytes();
    let req = QimeiRequest::new(rng, device, version, crypt_key).unwrap();
    let resp: QimeiResponse = reqwest::Client::new()
        .post("https://snowflake.qq.com/ola/android")
        .json(&req)
        .send()
        .await
        .map_err(|e| RQError::Other(e.to_string()))?
        .json()
        .await
        .map_err(|e| RQError::Other(e.to_string()))?;
    resp.to_payload(crypt_key).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ricq_core::protocol::version::ANDROID_PHONE;

    #[tokio::test]
    async fn test_qimei() {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(1239123240);
        let device = Device::random_with_rng(&mut rng);
        let version = ANDROID_PHONE;
        let resp = get_qimei(&mut rng, &device, &version).await.unwrap();
        println!("{resp:?}")
    }
}
