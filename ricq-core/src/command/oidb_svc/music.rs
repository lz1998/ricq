pub struct MusicShare {
    pub title: String,
    pub brief: String,
    pub summary: String,
    pub url: String,
    pub picture_url: String,
    pub music_url: String,
}

pub enum MusicType {
    QQ,
    Cloud,
    Migu,
    Kugou,
    Kuwo,
}

impl MusicType {
    pub fn version(&self) -> &'static MusicVersion {
        match self {
            MusicType::QQ => QQ_MUSIC,
            MusicType::Cloud => CLOUD_MUSIC,
            MusicType::Migu => MIGU_MUSIC,
            MusicType::Kugou => KUGOU_MUSIC,
            MusicType::Kuwo => KUWO_MUSIC,
        }
    }
}

pub struct MusicVersion {
    pub app_id: u64,
    pub app_type: u32,
    pub platform: u32,
    pub sdk_version: &'static str,
    pub package_name: &'static str,
    pub signature: &'static str,
}

pub static QQ_MUSIC: &MusicVersion = &MusicVersion {
    app_id: 100497308,
    app_type: 1,
    platform: 1,
    sdk_version: "0.0.0",
    package_name: "com.tencent.qqmusic",
    signature: "cbd27cd7c861227d013a25b2d10f0799",
};

pub static CLOUD_MUSIC: &MusicVersion = &MusicVersion {
    app_id: 100495085,
    app_type: 1,
    platform: 1,
    sdk_version: "0.0.0",
    package_name: "com.netease.cloudmusic",
    signature: "da6b069da1e2982db3e386233f68d76d",
};

pub static MIGU_MUSIC: &MusicVersion = &MusicVersion {
    app_id: 1101053067,
    app_type: 1,
    platform: 1,
    sdk_version: "0.0.0",
    package_name: "cmccwm.mobilemusic",
    signature: "6cdc72a439cef99a3418d2a78aa28c73",
};

pub static KUGOU_MUSIC: &MusicVersion = &MusicVersion {
    app_id: 205141,
    app_type: 1,
    platform: 1,
    sdk_version: "0.0.0",
    package_name: "com.kugou.android",
    signature: "fe4a24d80fcf253a00676a808f62c2c6",
};

pub static KUWO_MUSIC: &MusicVersion = &MusicVersion {
    app_id: 100243533,
    app_type: 1,
    platform: 1,
    sdk_version: "0.0.0",
    package_name: "cn.kuwo.player",
    signature: "bf9ff4ffb4c558a34ee3fd52c223ebf5",
};
