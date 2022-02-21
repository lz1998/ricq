use bytes::Bytes;
use std::fmt;

use crate::{command::common::PbToBytes, pb::msg};

#[derive(Default, Debug, Clone)]
pub struct FriendImage {
    pub image_id: String,
    pub md5: Bytes,
    pub size: i32,
    pub url: String,
    pub flash: bool,
}

impl From<FriendImage> for Vec<msg::elem::Elem> {
    fn from(i: FriendImage) -> Vec<msg::elem::Elem> {
        let image = msg::NotOnlineImage {
            file_path: Some(i.image_id.clone()),
            res_id: Some(i.image_id.clone()),
            old_pic_md5: Some(false),
            pic_md5: Some(i.md5.to_vec()),
            download_path: Some(i.image_id),
            original: Some(1),
            pb_reserve: Some(vec![0x78, 0x02]),
            ..Default::default()
        };
        if i.flash {
            let flash = msg::MsgElemInfoServtype3 {
                flash_c2c_pic: Some(image),
                ..Default::default()
            }
            .to_bytes();
            let flash_elem = msg::elem::Elem::CommonElem(msg::CommonElem {
                service_type: Some(3),
                pb_elem: Some(flash.to_vec()),
                ..Default::default()
            });
            let text_hint = msg::elem::Elem::Text(msg::Text {
                str: Some("[闪照]请使用新版手机QQ查看闪照。".to_owned()),
                ..Default::default()
            });
            vec![flash_elem, text_hint]
        } else {
            vec![msg::elem::Elem::NotOnlineImage(image)]
        }
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
            flash: false,
        }
    }
}

impl TryFrom<msg::CommonElem> for FriendImage {
    type Error = msg::CommonElem;

    fn try_from(e: msg::CommonElem) -> Result<Self, Self::Error> {
        if let Ok(flash) = msg::MsgElemInfoServtype3::from_bytes(e.pb_elem()) {
            if let Some(p) = flash.flash_c2c_pic {
                let mut friend_image: FriendImage = p.into(); //todo:url check
                friend_image.flash = true;
                return Ok(friend_image);
            }
        }
        Err(e)
    }
}

impl fmt::Display for FriendImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[FriendImage: {}]", self.url)
    }
}
