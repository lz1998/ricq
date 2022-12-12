use std::convert::TryFrom;

#[derive(Debug, Clone, derivative::Derivative, serde::Deserialize)]
#[derivative(Default)]
pub enum Protocol {
    #[derivative(Default)]
    IPad,
    AndroidPhone,
    AndroidWatch,
    MacOS,
    QiDian,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Version {
    pub apk_sign: &'static [u8],
    pub apk_id: &'static str,
    pub sort_version_name: &'static str,
    pub build_ver: &'static str,
    pub sdk_version: &'static str,
    pub app_id: u32,
    pub sub_app_id: u32,
    pub build_time: u32,
    pub sso_version: u32,
    pub misc_bitmap: u32,
    pub sub_sig_map: u32,
    pub main_sig_map: u32,
    pub protocol: Protocol,
}

pub const fn get_version(p: Protocol) -> Version {
    match p {
        Protocol::IPad => IPAD,
        Protocol::AndroidPhone => ANDROID_PHONE,
        Protocol::AndroidWatch => ANDROID_WATCH,
        Protocol::MacOS => MACOS,
        Protocol::QiDian => QIDIAN,
    }
}

impl From<Protocol> for Version {
    fn from(p: Protocol) -> Version {
        get_version(p)
    }
}

pub const ANDROID_PHONE: Version = Version {
    apk_id: "com.tencent.mobileqq",
    app_id: 537138832,
    sub_app_id: 537138832,
    sort_version_name: "8.9.15",
    build_ver: "8.9.15.9425",
    build_time: 1640921786,
    apk_sign: &[
        0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6,
        0x8D,
    ],
    sdk_version: "6.0.0.2494",
    sso_version: 16,
    misc_bitmap: 150470524,
    sub_sig_map: 0x10400,
    main_sig_map: 16724722,
    protocol: Protocol::AndroidPhone,
};

pub const IPAD: Version = Version {
    apk_id: "com.tencent.minihd.qq",
    app_id: 537065739,
    sub_app_id: 537065739,
    sort_version_name: "5.8.9",
    build_ver: "8.8.38.2266",
    build_time: 1595836208,
    apk_sign: &[
        0xAA, 0x39, 0x78, 0xF4, 0x1F, 0xD9, 0x6F, 0xF9, 0x91, 0x4A, 0x66, 0x9E, 0x18, 0x64, 0x74,
        0xC7,
    ],
    sdk_version: "6.0.0.2433",
    sso_version: 12,
    misc_bitmap: 150470524,
    sub_sig_map: 66560,
    main_sig_map: 1970400,
    protocol: Protocol::IPad,
};

pub const ANDROID_WATCH: Version = Version {
    apk_id: "com.tencent.qqlite",
    app_id: 537064446,
    sub_app_id: 537064446,
    sort_version_name: "2.0.5",
    build_ver: "2.0.5",
    build_time: 1559564731,
    apk_sign: &[
        0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6,
        0x8D,
    ],
    sdk_version: "6.0.0.236",
    sso_version: 5,
    misc_bitmap: 16252796,
    sub_sig_map: 0x10400,
    main_sig_map: 34869472,
    protocol: Protocol::AndroidWatch,
};

pub const MACOS: Version = Version {
    apk_id: "com.tencent.qq",              // ok
    app_id: 0x2003ca32,                    // ok
    sub_app_id: 0x2003ca32,                // ok
    sort_version_name: "6.7.9",            // ok
    build_ver: "5.8.9.3460",               // 6.7.9.xxx?
    build_time: 0,                         // ok
    apk_sign: "com.tencent.qq".as_bytes(), // ok
    sdk_version: "6.2.0.1023",             // ok
    sso_version: 7,                        // ok
    misc_bitmap: 0x7ffc,                   // ok
    sub_sig_map: 66560,                    // ?
    main_sig_map: 1970400,                 // ?
    protocol: Protocol::MacOS,
};

pub const QIDIAN: Version = Version {
    apk_id: "com.tencent.qidian",
    app_id: 537061386,
    sub_app_id: 537036590,
    sort_version_name: "3.8.6",
    build_ver: "8.8.38.2266",
    build_time: 1556628836,
    apk_sign: &[
        160, 30, 236, 171, 133, 233, 227, 186, 43, 15, 106, 21, 140, 133, 92, 41,
    ],
    sdk_version: "6.0.0.2365",
    sso_version: 5,
    misc_bitmap: 49807228,
    sub_sig_map: 66560,
    main_sig_map: 34869472,
    protocol: Protocol::QiDian,
};

impl TryFrom<&str> for Protocol {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "IPad" => Ok(Protocol::IPad),
            "AndroidPhone" => Ok(Protocol::AndroidPhone),
            "AndroidWatch" => Ok(Protocol::AndroidWatch),
            "MacOS" => Ok(Protocol::MacOS),
            "QiDian" => Ok(Protocol::QiDian),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Protocol {
    type Error = ();

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(Protocol::IPad), // default
            1 => Ok(Protocol::AndroidPhone),
            2 => Ok(Protocol::AndroidWatch),
            3 => Ok(Protocol::MacOS),
            4 => Ok(Protocol::QiDian),
            5 => Ok(Protocol::IPad),
            _ => Err(()),
        }
    }
}
