use std::fmt;

use serde::{Deserialize, Serialize};

use crate::msg::elem::flash_image::FlashImage;
use crate::pb::msg;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FriendImage {
    pub res_id: String,
    pub file_path: String,
    pub md5: Vec<u8>,
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub image_type: i32,
    pub orig_url: String,
    pub download_path: String,
}

impl FriendImage {
    pub fn flash(self) -> FlashImage {
        FlashImage::from(self)
    }

    pub fn url(&self) -> String {
        if !self.orig_url.is_empty() {
            return format!("https://c2cpicdw.qpic.cn{}", self.orig_url);
        }
        format!(
            "https://c2cpicdw.qpic.cn/offpic_new/0/{}/0?term=3",
            if !self.download_path.is_empty() {
                self.download_path.clone()
            } else {
                self.res_id.clone()
            }
        )
    }
}

impl From<FriendImage> for msg::NotOnlineImage {
    fn from(e: FriendImage) -> Self {
        msg::NotOnlineImage {
            file_path: Some(e.file_path),
            res_id: Some(e.res_id),
            old_pic_md5: Some(false),
            pic_md5: Some(e.md5),
            download_path: Some(e.download_path),
            original: Some(1), // 是否原图，如果是0需要写 24，25
            file_len: Some(e.size),
            img_type: Some(e.image_type),
            pic_width: Some(e.width),
            pic_height: Some(e.height),
            biz_type: Some(0),
            show_len: Some(0),
            download_len: Some(0),
            pb_reserve: Some(vec![0x78, 0x02]),
            ..Default::default()
        }
    }
}

impl From<FriendImage> for Vec<msg::elem::Elem> {
    fn from(e: FriendImage) -> Vec<msg::elem::Elem> {
        vec![msg::elem::Elem::NotOnlineImage(e.into())]
    }
}

impl From<msg::NotOnlineImage> for FriendImage {
    fn from(e: msg::NotOnlineImage) -> Self {
        Self {
            file_path: e.file_path.unwrap_or_default(),
            size: e.file_len.unwrap_or_default(),
            width: e.pic_width.unwrap_or_default(),
            height: e.pic_height.unwrap_or_default(),
            md5: e.pic_md5.unwrap_or_default(),
            orig_url: e.orig_url.unwrap_or_default(),
            res_id: e.res_id.unwrap_or_default(),
            download_path: e.download_path.unwrap_or_default(),
            image_type: e.img_type.unwrap_or_default(),
        }
    }
}

impl fmt::Display for FriendImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[FriendImage: {}]", self.url())
    }
}
