//手机设备信息
pub struct DeviceInfo {
    pub display: String,
    pub product: String,
    pub device: String,
    pub board: String,
    pub model: String,
    pub finger_print: String,
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
    pub guid: Vec<u8>,
}

impl DeviceInfo {
    pub fn random() -> DeviceInfo {
        DeviceInfo {
            display: "".to_string(),
            product: "".to_string(),
            device: "".to_string(),
            board: "".to_string(),
            model: "".to_string(),
            finger_print: "".to_string(),
            imei: "".to_string(),
            brand: "".to_string(),
            bootloader: "".to_string(),
            base_band: "".to_string(),
            version: Version::new(),
            sim_info: "".to_string(),
            os_type: "".to_string(),
            mac_address: "".to_string(),
            ip_address: vec![],
            wifi_bssid: "".to_string(),
            wifi_ssid: "".to_string(),
            imsi_md5: vec![],
            android_id: "".to_string(),
            apn: "".to_string(),
            vendor_name: "".to_string(),
            vendor_os_name: "".to_string(),
            guid: vec![],
        }
    }
}

//系统版本
pub struct Version {
    pub incremental: String,
    pub release: String,
    pub codename: String,
    pub sdk: u32,
}

impl Version {
    pub fn new() -> Version {
        Version {
            incremental: "5891938".to_string(),
            release: "10".to_string(),
            codename: "REL".to_string(),
            sdk: 29,
        }
    }
}