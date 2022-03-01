use bytes::Bytes;
use rand::distributions::DistString;
use rand::{distributions::Alphanumeric, Rng, RngCore};
use serde::{Deserialize, Serialize};

use crate::hex::encode_hex;

//系统版本
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OSVersion {
    pub incremental: String,
    pub release: String,
    pub codename: String,
    pub sdk: u32,
}

impl Default for OSVersion {
    fn default() -> Self {
        OSVersion {
            incremental: "5891938".into(),
            release: "10".into(),
            codename: "REL".into(),
            sdk: 29,
        }
    }
}

//手机设备信息
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub display: String,
    pub product: String,
    pub device: String,
    pub board: String,
    pub model: String,
    pub finger_print: String,
    pub boot_id: String,
    pub proc_version: String,
    pub imei: String,
    pub brand: String,
    pub bootloader: String,
    pub base_band: String,
    pub version: OSVersion,
    pub sim_info: String,
    pub os_type: String,
    pub mac_address: String,
    pub ip_address: Vec<u8>,
    pub wifi_bssid: String,
    pub wifi_ssid: String,
    pub imsi_md5: Vec<u8>,
    pub android_id: String,
    pub apn: String,
    pub vendor_name: String,
    pub vendor_os_name: String,
}

impl Device {
    pub fn random() -> Self {
        Self::random_with_rng(&mut rand::thread_rng())
    }

    pub fn random_with_rng<RNG: RngCore>(rng: &mut RNG) -> Self {
        Self {
            display: format!("GMC.{}.001", rng.gen_range(100000..999999)),
            product: "iarim".into(),
            device: "sagit".into(),
            board: "eomam".into(),
            model: "MI 6".into(),
            finger_print: format!(
                "xiaomi/iarim/sagit:10/eomam.200122.001/{}:user/release-keys",
                rng.gen_range(1000000..9999999)
            ),
            boot_id: random_uuid(rng),
            proc_version: format!(
                "Linux 5.4.0-54-generic-{} (android-build@google.com)",
                Alphanumeric.sample_string(rng, 8)
            ),
            imei: random_imei(rng),
            brand: "Xiaomi".into(),
            bootloader: "U-boot".into(),
            base_band: "".into(),
            version: OSVersion::default(),
            sim_info: "T-Mobile".into(),
            os_type: "android".into(),
            mac_address: "00:50:56:C0:00:08".into(),
            ip_address: vec![10, 0, 1, 3],
            wifi_bssid: "00:50:56:C0:00:08".into(),
            wifi_ssid: "<unknown ssid>".into(),
            imsi_md5: md5::compute(rng.gen::<[u8; 16]>()).to_vec(),
            android_id: encode_hex(&rng.gen::<[u8; 8]>()),
            apn: "wifi".into(),
            vendor_name: "MIUI".into(),
            vendor_os_name: "gmc".into(),
        }
    }

    pub fn ksid(&self) -> Bytes {
        Bytes::from(
            format!("|{}|A8.2.7.27f6ea96", self.imei)
                .as_bytes()
                .to_vec(),
        )
    }
}

pub fn random_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

pub fn random_uuid<RNG: RngCore>(rng: &mut RNG) -> String {
    let r = md5::compute(&rng.gen::<[u8; 16]>()).to_vec();
    format!(
        "{}-{}-{}-{}-{}",
        encode_hex(&r[0..4]),
        encode_hex(&r[4..6]),
        encode_hex(&r[6..8]),
        encode_hex(&r[8..10]),
        encode_hex(&r[10..16])
    )
}

pub fn random_imei<RNG: RngCore>(rng: &mut RNG) -> String {
    let mut sum = 0;
    let mut str = String::new();
    for i in 0..14 {
        let mut to_add = rng.gen_range(0..10);
        if (i + 2) % 2 == 0 {
            to_add *= 2;
            if to_add >= 10 {
                to_add = (to_add % 10) + 1
            }
        }
        sum += to_add;
        str.push_str(&to_add.to_string());
    }
    let ctrl_digit = (sum * 9) % 10;
    str.push_str(&ctrl_digit.to_string());
    str
}
