use std::collections::HashMap;

use crate::pb;

pub mod builder;
pub mod decoder;

// 群 @全体 剩余次数
#[derive(Default, Debug)]
pub struct GroupAtAllRemainInfo {
    pub can_at_all: bool,
    pub remain_at_all_count_for_group: u32,
    pub remain_at_all_count_for_uin: u32,
}

pub struct OcrResponse {
    pub texts: Vec<pb::oidb::TextDetection>,
    pub language: String,
}

// 编辑个人资料
#[derive(Default, Debug)]
pub struct ProfileDetailUpdate(pub HashMap<u16, Vec<u8>>);

impl ProfileDetailUpdate {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, value: String) {
        self.0.insert(20002, value.into_bytes());
    }
    pub fn email(&mut self, value: String) {
        self.0.insert(20011, value.into_bytes());
    }
    pub fn personal_note(&mut self, value: String) {
        self.0.insert(20019, value.into_bytes());
    }
    pub fn company(&mut self, value: String) {
        self.0.insert(24008, value.into_bytes());
    }
    pub fn college(&mut self, value: String) {
        self.0.insert(20021, value.into_bytes());
    }
}

pub enum ShareTarget {
    Friend(i64),
    Group(i64),
    Guild { guild_id: u64, channel_id: u64 },
}

impl ShareTarget {
    pub fn send_type(&self) -> u32 {
        match self {
            ShareTarget::Friend { .. } => 0,
            ShareTarget::Group { .. } => 1,
            ShareTarget::Guild { .. } => 3,
        }
    }
}

pub struct MusicShare {
    pub title: String,
    pub brief: String,
    pub summary: String,
    pub url: String,
    pub picture_url: String,
    pub music_url: String,
}

pub struct MusicVersion {
    pub app_id: u64,
    pub app_type: u32,
    pub platform: u32,
    pub sdk_version: &'static str,
    pub package_name: &'static str,
    pub signature: &'static str,
}

impl MusicVersion {
    pub const QQ: MusicVersion = MusicVersion {
        app_id: 100497308,
        app_type: 1,
        platform: 1,
        sdk_version: "0.0.0",
        package_name: "com.tencent.qqmusic",
        signature: "cbd27cd7c861227d013a25b2d10f0799",
    };

    pub const NETEASE: MusicVersion = MusicVersion {
        app_id: 100495085,
        app_type: 1,
        platform: 1,
        sdk_version: "0.0.0",
        package_name: "com.netease.cloudmusic",
        signature: "da6b069da1e2982db3e386233f68d76d",
    };

    pub const MIGU: MusicVersion = MusicVersion {
        app_id: 1101053067,
        app_type: 1,
        platform: 1,
        sdk_version: "0.0.0",
        package_name: "cmccwm.mobilemusic",
        signature: "6cdc72a439cef99a3418d2a78aa28c73",
    };

    pub const KUGOU: MusicVersion = MusicVersion {
        app_id: 205141,
        app_type: 1,
        platform: 1,
        sdk_version: "0.0.0",
        package_name: "com.kugou.android",
        signature: "fe4a24d80fcf253a00676a808f62c2c6",
    };

    pub const KUWO: MusicVersion = MusicVersion {
        app_id: 100243533,
        app_type: 1,
        platform: 1,
        sdk_version: "0.0.0",
        package_name: "cn.kuwo.player",
        signature: "bf9ff4ffb4c558a34ee3fd52c223ebf5",
    };
}

pub struct LinkShare {
    pub title: String,
    pub summary: Option<String>,
    /// 从消息列表中看到的文字,默认为 "[分享]" + title
    pub brief: Option<String>,
    /// 预览图网址, 默认为 QQ 浏览器图标,似乎对域名有限制
    pub picture_url: Option<String>,
    pub url: String,
}
