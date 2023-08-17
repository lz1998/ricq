use std::convert::TryFrom;
// oicq/wlogin_sdk/request/WtloginHelper.java SigType
pub const WLOGIN_A2: u32 = 64;
pub const WLOGIN_A5: u32 = 2;
pub const WLOGIN_AQSIG: u32 = 2097152;
pub const WLOGIN_D2: u32 = 262144;
pub const WLOGIN_DA2: u32 = 33554432;
pub const WLOGIN_LHSIG: u32 = 4194304;
pub const WLOGIN_LSKEY: u32 = 512;
pub const WLOGIN_OPENKEY: u32 = 16384;
pub const WLOGIN_PAYTOKEN: u32 = 8388608;
pub const WLOGIN_PF: u32 = 16777216;
pub const WLOGIN_PSKEY: u32 = 1048576;
pub const WLOGIN_PT4_TOKEN: u32 = 134217728;
pub const WLOGIN_QRPUSH: u32 = 67108864;
pub const WLOGIN_RESERVED: u32 = 16;
pub const WLOGIN_SID: u32 = 524288;
pub const WLOGIN_SIG64: u32 = 8192;
pub const WLOGIN_SKEY: u32 = 4096;
pub const WLOGIN_ST: u32 = 128;
pub const WLOGIN_STWEB: u32 = 32;
pub const WLOGIN_TOKEN: u32 = 32768;
pub const WLOGIN_VKEY: u32 = 131072;

#[derive(Debug, Clone, derivative::Derivative, serde::Deserialize)]
#[derivative(Default)]
pub enum Protocol {
    #[derivative(Default)]
    IPad,
    AndroidPhone,
    AndroidWatch,
    AndroidPad,
    MacOS,
    QiDian,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Version {
    pub apk_sign: &'static [u8],
    pub apk_id: &'static str,
    pub app_key: &'static str,
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
    pub qua: &'static str,
    pub protocol: Protocol,
}

pub const fn get_version(p: Protocol) -> Version {
    match p {
        Protocol::IPad => IPAD,
        Protocol::AndroidPhone => ANDROID_PHONE,
        Protocol::AndroidWatch => ANDROID_WATCH,
        Protocol::AndroidPad => ANDROID_PAD,
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
    app_id: 537164840,
    sub_app_id: 537164840,
    app_key: "0S200MNJT807V3GE",
    sort_version_name: "8.9.63.11390",
    build_ver: "8.9.63.11390",
    build_time: 1685069178,
    apk_sign: &[
        0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6,
        0x8D,
    ],
    sdk_version: "6.0.0.2546",
    sso_version: 20,
    misc_bitmap: 150470524,
    sub_sig_map: 0x10400,
    // 16724722
    main_sig_map: WLOGIN_A5
        | WLOGIN_RESERVED
        | WLOGIN_STWEB
        | WLOGIN_A2
        | WLOGIN_ST
        | WLOGIN_LSKEY
        | WLOGIN_SKEY
        | WLOGIN_SIG64
        | 1 << 16
        | WLOGIN_VKEY
        | WLOGIN_D2
        | WLOGIN_SID
        | WLOGIN_PSKEY
        | WLOGIN_AQSIG
        | WLOGIN_LHSIG
        | WLOGIN_PAYTOKEN,
    qua: "V1_AND_SQ_8.9.63_4194_YYB_D",
    protocol: Protocol::AndroidPhone,
};

pub const APAD: Version = Version {
    apk_id: "com.tencent.mobileqq",
    app_id: 537164888,
    sub_app_id: 537164888,
    sort_version_name: "8.9.63.11390",
    build_ver: "8.9.33.614",
    build_time: 1685069178,
    apk_sign: &[
        0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6,
        0x8D,
    ],
    sdk_version: "6.0.0.2546",
    sso_version: 20,
    misc_bitmap: 150470524,
    sub_sig_map: 0x10400,
    // 16724722
    main_sig_map: WLOGIN_A5
        | WLOGIN_RESERVED
        | WLOGIN_STWEB
        | WLOGIN_A2
        | WLOGIN_ST
        | WLOGIN_LSKEY
        | WLOGIN_SKEY
        | WLOGIN_SIG64
        | 1 << 16
        | WLOGIN_VKEY
        | WLOGIN_D2
        | WLOGIN_SID
        | WLOGIN_PSKEY
        | WLOGIN_AQSIG
        | WLOGIN_LHSIG
        | WLOGIN_PAYTOKEN,
    protocol: Protocol::AndroidPad,
    app_key: "0S200MNJT807V3GE",
    qua: "V1_AND_SQ_8.9.63_4194_YYB_D",
};

pub const IPAD: Version = Version {
    apk_id: "com.tencent.minihd.qq",
    app_id: 537151363,
    sub_app_id: 537151363,
    sort_version_name: "8.9.33.614",
    build_ver: "8.9.33.614",
    build_time: 1595836208,
    apk_sign: &[
        170, 57, 120, 244, 31, 217, 111, 249, 145, 74, 102, 158, 24, 100, 116, 199,
    ],
    sdk_version: "6.0.0.2433",
    sso_version: 19,
    misc_bitmap: 150470524,
    sub_sig_map: 66560,
    // 1970400
    main_sig_map: WLOGIN_STWEB
        | WLOGIN_A2
        | WLOGIN_ST
        | WLOGIN_SKEY
        | WLOGIN_VKEY
        | WLOGIN_D2
        | WLOGIN_SID
        | WLOGIN_PSKEY,
    protocol: Protocol::IPad,
    app_key: "",
    qua: "",
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
    qua: "",
    app_key: "",
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
    app_key: "",
    qua: "",
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
    app_key: "",
    qua: "",
};

pub const ANDROID_PAD: Version = Version {
    apk_id: "com.tencent.mobileqq",
    app_id: 537154261,
    sub_app_id: 537154261,
    sort_version_name: "8.9.38.10545",
    build_ver: "8.8.38.2266",
    build_time: 1556628836,
    apk_sign: &[
        0xa6, 0xb7, 0x45, 0xbf, 0x24, 0xa2, 0xc2, 0x77, 0x52, 0x77, 0x16, 0xf6, 0xf3, 0x6e, 0xb6,
        0x8d,
    ],
    sdk_version: "6.0.0.2535",
    sso_version: 19,
    misc_bitmap: 150470524,
    sub_sig_map: 66560,
    main_sig_map: 16724722,
    protocol: Protocol::AndroidPad,
    app_key: "",
    qua: "",
};

impl TryFrom<&str> for Protocol {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "IPad" => Ok(Protocol::IPad),
            "AndroidPhone" | "APhone" => Ok(Protocol::AndroidPhone),
            "AndroidWatch" | "AWatch" => Ok(Protocol::AndroidWatch),
            "AndroidPad" | "APad" => Ok(Protocol::AndroidPad),
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
            6 => Ok(Protocol::AndroidPad),
            _ => Err(()),
        }
    }
}
