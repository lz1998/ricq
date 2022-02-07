use bytes::Bytes;
use std::fmt;

use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct FriendImage {
    pub image_id: String,
    pub md5: Bytes,
    pub size: i32,
    pub url: String,
}

impl From<msg::NotOnlineImage> for FriendImage {
    fn from(e: msg::NotOnlineImage) -> Self {
        let url = if let Some(orig_url) = &e.orig_url {
            format!("https://c2cpicdw.qpic.cn{}", orig_url)
        } else {
            let download_path = if let Some(path) = &e.download_path {
                path
            } else {
                e.res_id()
            };
            format!(
                "https://c2cpicdw.qpic.cn/offpic_new/0{}{}/0?term=3",
                if !download_path.starts_with('/') {
                    "/"
                } else {
                    ""
                },
                download_path
            )
        };
        Self {
            image_id: e.file_path().to_owned(),
            size: e.file_len(),
            url,
            md5: Bytes::copy_from_slice(e.pic_md5()),
        }
    }
}

impl fmt::Display for FriendImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[FriendImage: {}]", self.url)
    }
}
