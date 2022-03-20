use std::fmt;

use prost::Message;

pub use group_image::calculate_image_resource_id;

pub use crate::msg::elem::{
    anonymous::Anonymous,
    at::At,
    face::Face,
    flash_image::FlashImage,
    friend_image::FriendImage,
    group_image::GroupImage,
    light_app::LightApp,
    market_face::{Dice, FingerGuessing, MarketFace},
    reply::Reply,
    rich_msg::RichMsg,
    text::Text,
};
use crate::pb::msg;

mod anonymous;
mod at;
mod face;
mod flash_image;
mod friend_image;
mod group_image;
mod light_app;
mod market_face;
mod reply;
mod rich_msg;
mod text;

#[derive(Debug, Clone)]
pub enum RQElem {
    At(at::At),
    Text(text::Text),
    Face(face::Face),
    MarketFace(market_face::MarketFace),
    Dice(market_face::Dice),
    FingerGuessing(market_face::FingerGuessing),
    LightApp(light_app::LightApp),
    RichMsg(rich_msg::RichMsg),
    FriendImage(friend_image::FriendImage),
    GroupImage(group_image::GroupImage),
    FlashImage(flash_image::FlashImage),
    Other(Box<msg::elem::Elem>),
}

impl From<msg::elem::Elem> for RQElem {
    fn from(elem: msg::elem::Elem) -> Self {
        match elem.clone() {
            msg::elem::Elem::Text(e) => {
                // TODO guild at
                if !e.attr6_buf().is_empty() {
                    RQElem::At(at::At::from(e))
                } else {
                    RQElem::Text(text::Text::from(e))
                }
            }
            msg::elem::Elem::Face(e) => RQElem::Face(face::Face::from(e)),
            msg::elem::Elem::CommonElem(e) => match e.service_type() {
                // TODO image
                3 => {
                    if let Ok(flash) = msg::MsgElemInfoServtype3::decode(e.pb_elem()) {
                        if let Some(i) = flash.flash_troop_pic {
                            RQElem::FlashImage(group_image::GroupImage::from(i).flash())
                        } else if let Some(i) = flash.flash_c2c_pic {
                            RQElem::FlashImage(friend_image::FriendImage::from(i).flash())
                        } else {
                            RQElem::Other(Box::new(elem))
                        }
                    } else {
                        RQElem::Other(Box::new(elem))
                    }
                }
                33 => {
                    if let Ok(new_face) = msg::MsgElemInfoServtype33::decode(e.pb_elem()) {
                        RQElem::Face(face::Face::from(new_face))
                    } else {
                        RQElem::Other(Box::new(elem))
                    }
                }
                _ => RQElem::Other(Box::new(elem)),
            },
            msg::elem::Elem::MarketFace(e) => {
                let f = market_face::MarketFace::from(e);
                if f.name == "[骰子]" || f.name == "[随机骰子]" {
                    RQElem::Dice(market_face::Dice::from(f))
                } else if f.name == "[猜拳]" {
                    RQElem::FingerGuessing(market_face::FingerGuessing::from(f))
                } else {
                    RQElem::MarketFace(f)
                }
            }
            msg::elem::Elem::LightApp(e) => RQElem::LightApp(light_app::LightApp::from(e)),
            msg::elem::Elem::RichMsg(e) => RQElem::RichMsg(rich_msg::RichMsg::from(e)),
            msg::elem::Elem::NotOnlineImage(e) => {
                RQElem::FriendImage(friend_image::FriendImage::from(e))
            }
            msg::elem::Elem::CustomFace(e) => RQElem::GroupImage(group_image::GroupImage::from(e)),
            _ => RQElem::Other(Box::new(elem)),
        }
    }
}

impl fmt::Display for RQElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RQElem::At(e) => fmt::Display::fmt(e, f),
            RQElem::Text(e) => fmt::Display::fmt(e, f),
            RQElem::Face(e) => fmt::Display::fmt(e, f),
            RQElem::GroupImage(e) => fmt::Display::fmt(e, f),
            RQElem::FriendImage(e) => fmt::Display::fmt(e, f),
            RQElem::FlashImage(e) => fmt::Display::fmt(e, f),
            _ => write!(f, ""),
        }
    }
}

macro_rules! impl_from {
    ($key: tt, $fty: ty) => {
        impl From<$fty> for RQElem {
            fn from(e: $fty) -> Self {
                RQElem::$key(e)
            }
        }
    };
}

impl_from!(At, at::At);
impl_from!(Text, text::Text);
impl_from!(Face, face::Face);
impl_from!(MarketFace, market_face::MarketFace);
impl_from!(Dice, market_face::Dice);
impl_from!(FingerGuessing, market_face::FingerGuessing);
impl_from!(LightApp, light_app::LightApp);
impl_from!(RichMsg, rich_msg::RichMsg);
impl_from!(FriendImage, friend_image::FriendImage);
impl_from!(GroupImage, group_image::GroupImage);
impl_from!(FlashImage, flash_image::FlashImage);
impl_from!(Other, Box<msg::elem::Elem>);
