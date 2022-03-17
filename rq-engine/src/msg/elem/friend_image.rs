use std::fmt;

use serde::{Deserialize, Serialize};

use crate::msg::elem::flash_image::FlashImage;
use crate::pb::msg;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PrivateImage {
    pub image_id: String,
    pub md5: Vec<u8>,
    pub size: i32,
    pub orig_url: Option<String>,
    pub res_id: Option<String>,
    pub download_path: Option<String>,
}

impl PrivateImage {
    pub fn flash(self) -> FlashImage {
        FlashImage::from(self)
    }

    pub fn url(&self) -> String {
        if let Some(orig_url) = &self.orig_url {
            format!("https://c2cpicdw.qpic.cn{}", orig_url)
        } else {
            let download_path = if let Some(path) = &self.download_path {
                path.clone()
            } else {
                self.res_id.clone().unwrap_or_default()
            };
            format!(
                "https://c2cpicdw.qpic.cn/offpic_new/0/{}/0?term=3",
                download_path
            )
        }
    }
}

impl From<PrivateImage> for msg::NotOnlineImage {
    fn from(e: PrivateImage) -> Self {
        msg::NotOnlineImage {
            file_path: Some(e.image_id.clone()),
            res_id: Some(e.image_id.clone()),
            old_pic_md5: Some(false),
            pic_md5: Some(e.md5),
            download_path: Some(e.image_id),
            original: Some(1),
            pb_reserve: Some(vec![0x78, 0x02]),
            ..Default::default()
        }
    }
}

impl From<PrivateImage> for Vec<msg::elem::Elem> {
    fn from(e: PrivateImage) -> Vec<msg::elem::Elem> {
        vec![msg::elem::Elem::NotOnlineImage(e.into())]
    }
}

impl From<msg::NotOnlineImage> for PrivateImage {
    fn from(e: msg::NotOnlineImage) -> Self {
        Self {
            image_id: e.file_path.unwrap_or_default(),
            size: e.file_len.unwrap_or_default(),
            md5: e.pic_md5.unwrap_or_default(),
            orig_url: e.orig_url,
            res_id: e.res_id,
            download_path: e.download_path,
        }
    }
}

impl fmt::Display for PrivateImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[FriendImage: {}]", self.url())
    }
}
