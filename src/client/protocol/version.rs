#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
pub enum Protocol {
    #[derivative(Default)]
    IPad,
    AndroidPhone,
    AndroidWatch,
    MacOS,
    QiDian,
}

#[derive(Default, Debug)]
pub struct Version {
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
    pub protocol: Protocol,
}
