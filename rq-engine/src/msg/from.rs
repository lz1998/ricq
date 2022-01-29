use std::io::Read;

use bytes::{Buf, Bytes};
use flate2::bufread::ZlibDecoder;
use prost::Message;

use crate::binary::BinaryReader;
use crate::pb::msg::*;
use crate::structs::AnonymousInfo;

use super::AtSubType;
use super::{ImageBizType, MsgElem};

impl From<SourceMsg> for MsgElem {
    fn from(src_msg: SourceMsg) -> Self {
        return if !src_msg.orig_seqs.is_empty() {
            Self::Reply {
                reply_seq: src_msg.orig_seqs[0],
                time: src_msg.time(),
                sender: src_msg.sender_uin(),
                group_id: src_msg.to_uin(),
                elements: super::parse_elems(src_msg.elems),
            }
        } else {
            Self::Other(elem::Elem::SrcMsg(src_msg))
        };
    }
}
impl From<TransElem> for MsgElem {
    fn from(info: TransElem) -> Self {
        if info.elem_type() != 24 {
            return Self::Other(elem::Elem::TransElemInfo(info));
        }
        let mut bytes = Bytes::copy_from_slice(info.elem_value());
        let i3 = bytes.len();
        if i3 <= 3 || bytes.get_u8() != 1 {
            return Self::Other(elem::Elem::TransElemInfo(info));
        }
        let pb = bytes.read_bytes_short();
        if let Ok(obj_msg) = ObjMsg::decode(pb) {
            if obj_msg.msg_content_info.is_empty() {
                return Self::Other(elem::Elem::TransElemInfo(info));
            }
            if let Some(info) = obj_msg.msg_content_info[0].msg_file.clone() {
                return Self::GroupFile {
                    name: info.file_name,
                    size: info.file_size,
                    path: String::from_utf8(info.file_path).unwrap(), // should be ok
                    busid: info.bus_id,
                };
            } else {
                Self::Other(elem::Elem::TransElemInfo(info))
            }
        } else {
            Self::Other(elem::Elem::TransElemInfo(info))
        }
    }
}
impl From<LightApp> for MsgElem {
    fn from(light_app: LightApp) -> Self {
        let data = light_app.clone().data.unwrap();
        if data.len() > 1 {
            let content = if data[0] == 0 {
                data[1..].to_vec()
            } else {
                let mut uncompressed = Vec::new();
                ZlibDecoder::new(&data[1..])
                    .read_to_end(&mut uncompressed)
                    .unwrap();
                uncompressed
            };
            if !content.is_empty() && content.len() < 1024 ^ 3 {
                return Self::LightApp {
                    content: String::from_utf8(content).unwrap(),
                };
            }
        }
        return Self::Other(elem::Elem::LightApp(light_app));
    }
}
impl From<Text> for MsgElem {
    fn from(text: Text) -> Self {
        if !text.attr6_buf().is_empty() {
            let (_, mut attr6) = text.attr6_buf().split_at(7);
            let target = attr6.get_u32();
            return super::at(
                target as i64,
                text.str().to_owned(),
                AtSubType::AtGroupMember,
            );
        } else if !text.pb_reserve().is_empty() {
            if let Ok(resv) = crate::pb::msg::TextResvAttr::decode(text.pb_reserve()) {
                if resv.at_type() == 2 {
                    return super::at(
                        resv.at_member_tinyid() as i64,
                        text.str().to_owned(),
                        AtSubType::AtGuildMember,
                    );
                } else if resv.at_type() == 4 {
                    return super::at(
                        resv.at_channel_info.unwrap_or_default().channel_id() as i64,
                        text.str().to_owned(),
                        AtSubType::AtGuildChannel,
                    );
                }
            }
        } else {
            return Self::Text {
                content: text.str().to_owned(),
            };
        }
        return Self::Other(elem::Elem::Text(text));
    }
}
impl From<RichMsg> for MsgElem {
    fn from(rich_msg: RichMsg) -> Self {
        let content = if rich_msg.template1()[0] == 0 {
            String::from_utf8(rich_msg.template1()[1..].to_vec()).unwrap()
        } else if rich_msg.template1()[0] == 1 {
            let mut uncompressed = Vec::new();
            ZlibDecoder::new(&rich_msg.template1()[1..])
                .read_to_end(&mut uncompressed)
                .unwrap();
            String::from_utf8(uncompressed).unwrap()
        } else {
            String::default()
        };

        if !content.is_empty() {
            // TODO ...
            return Self::Other(elem::Elem::RichMsg(rich_msg));
        }
        Self::Other(elem::Elem::RichMsg(rich_msg))
    }
}
impl From<CustomFace> for MsgElem {
    fn from(custom_face: CustomFace) -> Self {
        if custom_face.md5().is_empty() {
            return Self::Other(elem::Elem::CustomFace(custom_face));
        }
        let url = if let Some(orig_url) = &custom_face.orig_url {
            format!("https://gchat.qpic.cn{}", orig_url)
        } else {
            format!(
                "https://gchat.qpic.cn/gchatpic_new/0/0-0-{}{}",
                calculate_image_resource_id(&custom_face.md5()[1..37], true),
                "/0?term=2"
            )
        };
        // guild image todo
        return Self::GroupImage {
            file_id: custom_face.file_id() as i64,
            image_id: custom_face.file_path().to_owned(),
            size: custom_face.size(),
            width: custom_face.width(),
            height: custom_face.height(),
            url,
            image_biz_type: if custom_face.pb_reserve().is_empty() {
                ImageBizType::UnknownBizType
            } else if let Ok(resv) = crate::pb::msg::ResvAttr::decode(custom_face.pb_reserve()) {
                ImageBizType::from(resv.image_biz_type())
            } else {
                ImageBizType::UnknownBizType
            },
            md5: Bytes::copy_from_slice(custom_face.md5()),
            image_type: 0, // unchecked
            effect_id: 0,  // unchecked
            flash: false,
        };
    }
}
impl From<QqWalletMsg> for MsgElem {
    fn from(qq_wallet_msg: QqWalletMsg) -> Self {
        let aio_body = qq_wallet_msg.clone().aio_body.unwrap();
        if aio_body.msg_type() <= 1000 && aio_body.red_type.is_some() {
            return Self::RedBag {
                red_bag_type: super::RedBagType::from(aio_body.msg_type()),
                title: aio_body.receiver.unwrap().title().to_owned(),
            };
        }
        return Self::Other(elem::Elem::QqWalletMsg(qq_wallet_msg));
    }
}
impl From<NotOnlineImage> for MsgElem {
    fn from(not_online_image: NotOnlineImage) -> Self {
        let url = if let Some(orig_url) = &not_online_image.orig_url {
            format!("https://c2cpicdw.qpic.cn{}", orig_url)
        } else {
            let download_path = if let Some(path) = &not_online_image.download_path {
                path
            } else {
                not_online_image.res_id()
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
        Self::FriendImage {
            image_id: not_online_image.file_path().to_owned(),
            size: not_online_image.file_len(),
            url,
            md5: Bytes::copy_from_slice(not_online_image.pic_md5()),
            flash: false,
        }
    }
}
impl From<CommonElem> for MsgElem {
    fn from(common_elem: CommonElem) -> Self {
        match common_elem.service_type() {
            3 => {
                if let Ok(flash) =
                    crate::pb::msg::MsgElemInfoServtype3::decode(common_elem.pb_elem())
                {
                    if let Some(pic) = flash.flash_troop_pic {
                        return Self::GroupImage {
                            file_id: pic.file_id() as i64,
                            image_id: pic.file_path().to_owned(),
                            size: pic.size(),
                            width: pic.width(),
                            height: pic.height(),
                            md5: Bytes::copy_from_slice(pic.md5()),
                            flash: true,
                            url: String::default(),
                            image_type: 0,
                            image_biz_type: ImageBizType::UnknownBizType,
                            effect_id: 0,
                        };
                    }
                    if let Some(pic) = flash.flash_c2c_pic {
                        return Self::FriendImage {
                            image_id: pic.file_path().to_owned(),
                            size: pic.file_len(),
                            md5: Bytes::copy_from_slice(pic.pic_md5()),
                            flash: true,
                            url: String::default(),
                        };
                    }
                }
            }
            33 => {
                if let Ok(new_face) =
                    crate::pb::msg::MsgElemInfoServtype33::decode(common_elem.pb_elem())
                {
                    return super::face(new_face.index() as i32);
                }
            }
            37 => {
                if let Ok(animated_sticker) =
                    crate::pb::msg::MsgElemInfoServtype37::decode(common_elem.pb_elem())
                {
                    return Self::AnimatedSticker {
                        id: animated_sticker.qsid() as i32,
                        name: {
                            let mut s =
                                String::from_utf8(animated_sticker.text().to_vec()).unwrap();
                            if s.starts_with('/') {
                                let _ = s.split_off(1);
                            }
                            s
                        },
                    };
                }
            }
            _ => {}
        }
        return Self::Other(elem::Elem::CommonElem(common_elem));
    }
}
impl From<Face> for MsgElem {
    fn from(face: Face) -> Self {
        super::face(face.index())
    }
}
impl From<VideoFile> for MsgElem {
    fn from(video_file: VideoFile) -> Self {
        Self::ShortVideo {
            name: String::from_utf8(video_file.file_name().to_vec()).unwrap(),
            uuid: Bytes::copy_from_slice(video_file.file_uuid()),
            size: video_file.file_size(),
            thumb_size: video_file.thumb_file_size(),
            md5: Bytes::copy_from_slice(video_file.file_md5()),
            thumb_md5: Bytes::copy_from_slice(video_file.thumb_file_md5()),
            url: String::default(),
            guild: false,
        }
    }
}
impl From<MarketFace> for MsgElem {
    fn from(market_face: MarketFace) -> Self {
        let name = String::from_utf8(market_face.face_name().to_vec()).unwrap();
        let magic_value = String::from_utf8(market_face.mobileparam().to_vec()).unwrap();
        if &name == "[骰子]" || &name == "[随机骰子]" {
            return Self::Dice {
                name,
                face_id: Bytes::copy_from_slice(market_face.face_id()),
                tab_id: market_face.tab_id() as i32,
                item_type: market_face.item_type() as i32,
                sub_type: market_face.sub_type() as i32,
                media_type: market_face.media_type() as i32,
                encrypt_key: Bytes::copy_from_slice(market_face.key()),
                value: parse_magic_value(&magic_value),
                magic_value,
            };
        } else if &name == "[猜拳]" {
            let value = parse_magic_value(&magic_value);
            return Self::FingerGuess {
                name,
                face_id: Bytes::copy_from_slice(market_face.face_id()),
                tab_id: market_face.tab_id() as i32,
                item_type: market_face.item_type() as i32,
                sub_type: market_face.sub_type() as i32,
                media_type: market_face.media_type() as i32,
                encrypt_key: Bytes::copy_from_slice(market_face.key()),
                value,
                magic_value,
                finger_guess_name: super::FINGER_GUESS_NAME_SET[value as usize].to_owned(),
            };
        } else {
            return Self::MarketFace {
                name,
                face_id: Bytes::copy_from_slice(market_face.face_id()),
                tab_id: market_face.tab_id() as i32,
                item_type: market_face.item_type() as i32,
                sub_type: market_face.sub_type() as i32,
                media_type: market_face.media_type() as i32,
                encrypt_key: Bytes::copy_from_slice(market_face.key()),
                magic_value,
            };
        }
    }
}

impl From<Elem> for MsgElem {
    fn from(e: Elem) -> Self {
        if e.elem.is_none() {
            return Self::None;
        }
        let e = e.elem.unwrap();
        match e {
            elem::Elem::SrcMsg(src_msg) => MsgElem::from(src_msg),
            elem::Elem::TransElemInfo(info) => MsgElem::from(info),
            elem::Elem::LightApp(light_app) => MsgElem::from(light_app),
            elem::Elem::VideoFile(video_file) => MsgElem::from(video_file),
            elem::Elem::Text(text) => MsgElem::from(text),
            elem::Elem::RichMsg(rich_msg) => MsgElem::from(rich_msg),
            elem::Elem::CustomFace(custom_face) => MsgElem::from(custom_face),
            elem::Elem::MarketFace(market_face) => MsgElem::from(market_face),
            elem::Elem::NotOnlineImage(not_online_image) => MsgElem::from(not_online_image),
            elem::Elem::QqWalletMsg(qq_wallet_msg) => MsgElem::from(qq_wallet_msg),
            elem::Elem::Face(face) => MsgElem::from(face),
            elem::Elem::CommonElem(common_elem) => MsgElem::from(common_elem),
            _ => return Self::Other(e),
        }
    }
}

impl From<AnonymousGroupMessage> for AnonymousInfo {
    fn from(msg: AnonymousGroupMessage) -> Self {
        Self {
            anonymous_id: base64::encode(&msg.anon_id()),
            anonymous_nick: String::from_utf8_lossy(msg.anon_nick()).to_string(),
        }
    }
}

fn parse_magic_value(magic_value: &str) -> i32 {
    let mut value = magic_value.split('=');
    value.next();
    value.next().unwrap().parse::<i32>().unwrap()
}

fn to_uuid(md5: &str) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        &md5[0..8],
        &md5[8..12],
        &md5[12..16],
        &md5[16..20],
        &md5[20..32],
    )
}

fn calculate_image_resource_id(md5: &[u8], no_dash: bool) -> String {
    let mut r = "{".to_owned();
    let md5 = crate::hex::encode_hex(md5).to_uppercase();
    if no_dash {
        r.push_str(&md5);
    } else {
        r.push_str(&to_uuid(&md5));
    }
    r.push_str("}.png");
    r
}
