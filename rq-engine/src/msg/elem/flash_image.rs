use std::fmt;

use crate::command::common::PbToBytes;
use crate::msg::elem::{FriendImage, GroupImage};
use crate::pb::msg;

#[derive(Debug, Clone)]
pub enum FlashImage {
    FriendImage(FriendImage),
    GroupImage(GroupImage),
}

impl FlashImage {
    pub fn url(&self) -> String {
        match self {
            FlashImage::FriendImage(i) => i.url(),
            FlashImage::GroupImage(i) => i.url(),
        }
    }
}

impl From<FlashImage> for Vec<msg::elem::Elem> {
    fn from(e: FlashImage) -> Self {
        let flash = {
            match e {
                FlashImage::FriendImage(image) => msg::MsgElemInfoServtype3 {
                    flash_c2c_pic: Some(image.into()),
                    ..Default::default()
                },
                FlashImage::GroupImage(image) => msg::MsgElemInfoServtype3 {
                    flash_troop_pic: Some(image.into()),
                    ..Default::default()
                },
            }
        }
        .to_bytes();
        vec![
            msg::elem::Elem::CommonElem(msg::CommonElem {
                service_type: Some(3),
                pb_elem: Some(flash.to_vec()),
                ..Default::default()
            }),
            msg::elem::Elem::Text(msg::Text {
                str: Some("[闪照]请使用新版手机QQ查看闪照。".to_owned()),
                ..Default::default()
            }),
        ]
    }
}

impl From<FriendImage> for FlashImage {
    fn from(e: FriendImage) -> Self {
        Self::FriendImage(e)
    }
}
impl From<GroupImage> for FlashImage {
    fn from(e: GroupImage) -> Self {
        Self::GroupImage(e)
    }
}

impl fmt::Display for FlashImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlashImage::FriendImage(i) => {
                write!(f, "[FlashImage(friend): {}]", i.url())
            }
            FlashImage::GroupImage(i) => {
                write!(f, "[FlashImage(group): {}]", i.url())
            }
        }
    }
}
