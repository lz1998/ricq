use std::fmt;

use crate::msg::elem::flash_image::FlashImage;
use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct FriendImage {
    pub image_id: String,
    pub md5: Vec<u8>,
    pub size: i32,
    pub url: String,
}

impl FriendImage {
    pub fn flash(self) -> FlashImage {
        FlashImage::from(self)
    }

    pub fn build_url(s: &str) -> String {
        format!(
            "https://c2cpicdw.qpic.cn/offpic_new/0{}{}/0?term=3",
            if !s.starts_with("/") { "/" } else { "" },
            s
        )
    }
}

impl From<FriendImage> for msg::NotOnlineImage {
    fn from(e: FriendImage) -> Self {
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

impl From<FriendImage> for Vec<msg::elem::Elem> {
    fn from(e: FriendImage) -> Vec<msg::elem::Elem> {
        vec![msg::elem::Elem::NotOnlineImage(e.into())]
    }
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
            Self::build_url(download_path)
        };
        Self {
            image_id: e.file_path().to_owned(),
            size: e.file_len(),
            url,
            md5: e.pic_md5().to_vec(),
        }
    }
}

impl fmt::Display for FriendImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[FriendImage: {}]", self.url)
    }
}
