use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use crate::device::{DeviceInfo, Version};
use crate::encrypt::EncryptECDH;
use crate::version::{ClientProtocol, gen_version_info, VersionInfo};

pub struct Client {
    seq_id: Arc<AtomicU16>,
    pub ecdh: EncryptECDH,
    //随机16位
    pub random_key: Vec<u8>,
    pub version: VersionInfo,
    pub device_info: DeviceInfo,
    pub out_going_packet_session_id: Vec<u8>,
    pub ksid: Vec<u8>,
}

impl Client {
    pub fn new() -> Client {
        Client {
            seq_id: Arc::new(AtomicU16::new(0x3635)),
            ecdh: EncryptECDH::new(),
            random_key: vec![],
            version: gen_version_info(&ClientProtocol::IPad),
            device_info: DeviceInfo {
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
                version: Version {
                    incremental: "".to_string(),
                    release: "".to_string(),
                    codename: "".to_string(),
                    sdk: 0,
                },
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
            },
            out_going_packet_session_id: vec![0x02, 0xB0, 0x5B, 0x8B],
            ksid: vec![],
        }
    }
    pub fn next_seq(&mut self) -> u16 {
        self.seq_id.fetch_add(1, Ordering::Relaxed)
    }
}