use crate::client::outcome::PbToBytes;
use crate::hex::encode_hex;
use crate::pb;
use bytes::{BufMut, Bytes};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

const DEVICE_FILE_PATH: &str = "device.json";

//手机设备信息
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DeviceInfo {
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
    pub version: Version,
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
    // generate
    #[serde(skip)]
    pub guid: Bytes,
    // generate
    #[serde(skip)]
    pub tgtgt_key: Bytes,
}

impl DeviceInfo {
    pub fn random() -> DeviceInfo {
        let mut rng = rand::thread_rng();
        DeviceInfo {
            display: "GMC.".to_string() + &rng.gen_range(100000..999999).to_string() + ".001",
            product: "iarim".to_string(),
            device: "sagit".to_string(),
            board: "eomam".to_string(),
            model: "MI 6".to_string(),
            finger_print: "xiaomi/iarim/sagit:10/eomam.200122.001/".to_string()
                + &rng.gen_range(1000000..9999999).to_string()
                + &":user/release-keys".to_string(),
            boot_id: random_uuid(),
            proc_version: "Linux 5.4.0-54-generic-".to_string()
                + &random_string(8)
                + &" (android-build@google.com)".to_string(),
            imei: random_imei(),
            brand: "Xiaomi".to_string(),
            bootloader: "U-boot".to_string(),
            base_band: "".to_string(),
            version: Version::default(),
            sim_info: "T-Mobile".to_string(),
            os_type: "android".to_string(),
            mac_address: "00:50:56:C0:00:08".to_string(),
            ip_address: vec![10, 0, 1, 3],
            wifi_bssid: "00:50:56:C0:00:08".to_string(),
            wifi_ssid: "<unknown ssid>".to_string(),
            imsi_md5: md5::compute(rand::thread_rng().gen::<[u8; 16]>()).to_vec(),
            android_id: encode_hex(&rand::thread_rng().gen::<[u8; 8]>()),
            apn: "wifi".to_string(),
            vendor_name: "MIUI".to_string(),
            vendor_os_name: "gmc".to_string(),
            guid: Bytes::new(),      // md5(android_id + mac_address)
            tgtgt_key: Bytes::new(), // random bytes
        }
    }

    pub fn gen_guid(&mut self) {
        self.guid =
            Bytes::from(md5::compute(self.android_id.to_owned() + &self.mac_address).to_vec());
    }

    pub fn gen_tgtgt_key(&mut self) {
        // TODO 这里可能可以用随机bytes代替
        let mut r = rand::thread_rng().gen::<[u8; 16]>().to_vec();
        r.put_slice(&self.guid);
        self.tgtgt_key = Bytes::from(md5::compute(&r).to_vec());
    }

    pub fn gen_pb_data(&self) -> Bytes {
        pb::DeviceInfo {
            bootloader: self.bootloader.to_owned(),
            proc_version: self.proc_version.to_owned(),
            codename: self.version.codename.to_owned(),
            incremental: self.version.incremental.to_owned(),
            fingerprint: self.finger_print.to_owned(),
            boot_id: self.boot_id.to_owned(),
            android_id: self.android_id.to_owned(),
            base_band: self.base_band.to_owned(),
            inner_version: self.version.incremental.to_owned(),
        }
        .to_bytes()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn load_or_new() -> Self {
        Self::load_from_file().unwrap_or_else(|_| {
            let info = Self::random();
            info.save_to_file().unwrap();
            info
        })
    }

    pub fn load_from_file() -> Result<Self, std::io::Error> {
        use std::io::Read;
        let mut file = std::fs::File::open(DEVICE_FILE_PATH)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(Self::from_json(&s)?)
    }

    pub fn save_to_file(&self) -> Result<(), std::io::Error> {
        use std::io::Write;
        let mut file = std::fs::File::create(DEVICE_FILE_PATH)?;
        file.write_all(self.to_json().as_bytes())?;
        Ok(())
    }
}

//系统版本
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub incremental: String,
    pub release: String,
    pub codename: String,
    pub sdk: u32,
}

impl Default for Version {
    fn default() -> Self {
        Version {
            incremental: "5891938".to_string(),
            release: "10".to_string(),
            codename: "REL".to_string(),
            sdk: 29,
        }
    }
}

pub fn random_string(len: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
}

pub fn random_uuid() -> String {
    let r = md5::compute(&rand::thread_rng().gen::<[u8; 16]>()).to_vec();
    format!(
        "{}-{}-{}-{}-{}",
        encode_hex(&r[0..4]),
        encode_hex(&r[4..6]),
        encode_hex(&r[6..8]),
        encode_hex(&r[8..10]),
        encode_hex(&r[10..16])
    )
}

pub fn random_imei() -> String {
    let mut sum = 0;
    let mut str = String::new();
    let mut rng = rand::thread_rng();
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

#[cfg(test)]
mod tests {
    use crate::client::device::{random_imei, random_string, random_uuid};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_random() {
        let random_bytes = Vec::from(rand::thread_rng().gen::<[u8; 5]>());
        println!("{:?}", random_bytes);
        let _rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        println!("{}", random_string(5));
        println!("{}", random_uuid());
        println!("{}", random_imei());
    }
}
