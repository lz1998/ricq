#[derive(Debug)]
pub enum ClientProtocol {
    AndroidPhone,
    IPad,
    AndroidWatch,
    MacOS,
    QiDian,
}

#[derive(Default,Debug)]
pub struct VersionInfo {
    pub apk_sign: Vec<u8>,
    pub apk_id: String,
    pub sort_version_name: String,
    pub sdk_version: String,
    pub app_id: u32,
    pub sub_app_id: u32,
    pub build_time: u32,
    pub sso_version: u32,
    pub misc_bitmap: u32,
    pub sub_sig_map: u32,
    pub main_sig_map: u32,
    pub protocol: ClientProtocol,
}
impl Default for ClientProtocol{
    fn default() -> Self {
        return ClientProtocol::IPad
    }
}

pub fn gen_version_info(p: &ClientProtocol) -> VersionInfo {
    match p {
        ClientProtocol::AndroidPhone => VersionInfo {
            apk_id: String::from("com.tencent.mobileqq"),
            app_id: 537066738,
            sub_app_id: 537066738,
            sort_version_name: String::from("8.5.0"),
            build_time: 1607689988,
            apk_sign: vec![0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6, 0x8D],
            sdk_version: String::from("6.0.0.2454"),
            sso_version: 15,
            misc_bitmap: 184024956,
            sub_sig_map: 0x10400,
            main_sig_map: 34869472,
            protocol: ClientProtocol::AndroidPhone,
        },
        ClientProtocol::IPad => VersionInfo {
            apk_id: String::from("com.tencent.minihd.qq"),
            app_id: 537065739,
            sub_app_id: 537065739,
            sort_version_name: String::from("5.8.9"),
            build_time: 1595836208,
            apk_sign: vec![170, 57, 120, 244, 31, 217, 111, 249, 145, 74, 102, 158, 24, 100, 116, 199],
            sdk_version: String::from("6.0.0.2433"),
            sso_version: 12,
            misc_bitmap: 150470524,
            sub_sig_map: 66560,
            main_sig_map: 1970400,
            protocol: ClientProtocol::IPad,
        },
        ClientProtocol::AndroidWatch => VersionInfo {
            apk_id: String::from("com.tencent.qqlite"),
            app_id: 537064446,
            sub_app_id: 537064446,
            sort_version_name: String::from("2.0.5"),
            build_time: 1559564731,
            apk_sign: vec![0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6, 0x8D],
            sdk_version: String::from("6.0.0.236"),
            sso_version: 5,
            misc_bitmap: 16252796,
            sub_sig_map: 0x10400,
            main_sig_map: 34869472,
            protocol: ClientProtocol::AndroidWatch,
        },
        ClientProtocol::MacOS => VersionInfo {
            apk_id: String::from("com.tencent.minihd.qq"),
            app_id: 537064315,
            sub_app_id: 537064315,
            sort_version_name: String::from("5.8.9"),
            build_time: 1595836208,
            apk_sign: vec![170, 57, 120, 244, 31, 217, 111, 249, 145, 74, 102, 158, 24, 100, 116, 199],
            sdk_version: String::from("6.0.0.2433"),
            sso_version: 12,
            misc_bitmap: 150470524,
            sub_sig_map: 66560,
            main_sig_map: 1970400,
            protocol: ClientProtocol::MacOS,
        },
        ClientProtocol::QiDian => VersionInfo {
            apk_id: String::from("com.tencent.qidian"),
            app_id: 537061386,
            sub_app_id: 537036590,
            sort_version_name: String::from("3.8.6"),
            build_time: 1556628836,
            apk_sign: vec![160, 30, 236, 171, 133, 233, 227, 186, 43, 15, 106, 21, 140, 133, 92, 41],
            sdk_version: String::from("6.0.0.2365"),
            sso_version: 5,
            misc_bitmap: 49807228,
            sub_sig_map: 66560,
            main_sig_map: 34869472,
            protocol: ClientProtocol::QiDian,
        },
    }
}