use crate::pb::msg::Ptt;
use bytes::Bytes;

mod face;
mod from;
mod into;
pub use face::FACES_MAP;

use crate::client::engine::decoder::online_push::GroupMessagePart;

#[derive(Debug, Clone, PartialEq)]
pub struct PrivateMessageEvent {
    pub id: i32,
    pub internal_id: i32,
    pub self_id: i64, //?
    pub target: i64,
    pub time: i32,
    pub sender: Sender,
    pub elements: Vec<MsgElem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMessageEvent {
    pub id: i32,
    pub internal_id: i32,
    pub group_code: i64,
    pub group_name: String,
    pub sender: Sender,
    pub time: i32,
    pub elements: Vec<MsgElem>,
    pub original_obj: GroupMessagePart,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMessageReceiptEvent {
    pub rand: i32,
    pub seq: i32,
    pub msg_event: GroupMessageEvent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sender {
    pub uin: i64,
    pub nickname: String,
    pub card_name: String,
    pub anonymous_info: AnonymousInfo,
    pub is_friend: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnonymousInfo {
    pub anonymous_id: String,
    pub anonymous_nick: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MsgElem {
    Reply {
        reply_seq: i32,
        sender: i64,
        group_id: i64,
        time: i32,
        elements: Vec<MsgElem>,
    },
    GroupFile {
        name: String,
        size: i64,
        path: String,
        busid: i32,
    },
    Text {
        content: String,
    },
    Voice {
        name: String,
        md5: [u8; 16],
        size: i32,
        url: String,
        data: Bytes, // sending
    },
    GroupVoice {
        data: Bytes,
        ptt: Ptt,
    },
    PrivateVoice {
        date: Bytes,
        ptt: Ptt,
    },
    Face {
        index: i32,
        name: String,
    },
    At {
        target: i64,
        display: String,
        sub_type: AtSubType,
    },
    ShortVideo {
        name: String,
        uuid: Bytes,
        size: i32,
        thumb_size: i32,
        md5: Bytes,       // [u8;16]
        thumb_md5: Bytes, // [u8;16]
        url: String,
        guild: bool,
    },
    Service {
        id: i32,
        content: String,
        res_id: String,
        sub_type: String,
    },
    LightApp {
        content: String,
    },
    RedBag {
        red_bag_type: RedBagType,
        title: String,
    },
    Music {
        music_type: MusicType,
        title: String,
        brief: String,
        summary: String,
        url: String,
        picture_url: String,
        music_url: String,
    },
    AnimatedSticker {
        id: i32,
        name: String,
    },

    GroupImage {
        image_id: String,
        file_id: i64,
        image_type: i32,
        image_biz_type: ImageBizType,
        size: i32,
        width: i32,
        height: i32,
        md5: Bytes,
        url: String,

        effect_id: i32,
        flash: bool,
    },
    FriendImage {
        image_id: String,
        md5: Bytes,
        size: i32,
        url: String,

        flash: bool,
    },
    GuildImage {
        file_id: i64,
        file_path: String,
        image_type: i32,
        size: i32,
        width: i32,
        height: i32,
        download_index: String,
        md5: Bytes,
        url: String,
    },

    MarketFace {
        name: String,
        face_id: Bytes,
        tab_id: i32,
        item_type: i32,
        sub_type: i32,
        media_type: i32,
        encrypt_key: Bytes,
        magic_value: String,
    },
    Dice {
        name: String,
        face_id: Bytes,
        tab_id: i32,
        item_type: i32,
        sub_type: i32,
        media_type: i32,
        encrypt_key: Bytes,
        magic_value: String,
        value: i32,
    },
    FingerGuess {
        name: String,
        face_id: Bytes,
        tab_id: i32,
        item_type: i32,
        sub_type: i32,
        media_type: i32,
        encrypt_key: Bytes,
        magic_value: String,
        value: i32,
        finger_guess_name: String,
    },
    None,
}

const FINGER_GUESS_NAME_SET: [&str; 3] = ["石头", "剪刀", "布"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusicType {
    QQMusic,    // QQ音乐
    CloudMusic, // 网易云音乐
    MiguMusic,  // 咪咕音乐
    KugouMusic, // 酷狗音乐
    KuwoMusic,  // 酷我音乐
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedBagType {
    RedBagSimple,
    RedBagLucky,
    RedBagSimpleTheme,
    RedBagLuckyTheme,
    RedBagWord,
    RedBagSimpleSpecify,
    RedBagLuckySpecify,
    RedBagSimpleSpecifyOver3,
    RedBagLuckySpecifyOver3,
    RedBagVoice,
    RedBagLook, // ?
    RedBagVoiceC2C,
    RedBagH5,
    RedBagKSong,
    RedBagEmoji,
    RedBagDraw,
    RedBagH5Common,
    RedBagWordChain,
    RedBagKeyword,        // ?
    RedBagDrawMultiModel, // ??
}

impl From<i32> for RedBagType {
    fn from(i: i32) -> Self {
        match i {
            2 => RedBagType::RedBagSimple,
            3 => RedBagType::RedBagLucky,
            4 => RedBagType::RedBagSimpleTheme,
            5 => RedBagType::RedBagLuckyTheme,
            6 => RedBagType::RedBagWord,
            7 => RedBagType::RedBagSimpleSpecify,
            8 => RedBagType::RedBagLuckySpecify,
            11 => RedBagType::RedBagSimpleSpecifyOver3,
            12 => RedBagType::RedBagLuckySpecifyOver3,
            13 => RedBagType::RedBagVoice,
            14 => RedBagType::RedBagLook,
            15 => RedBagType::RedBagVoiceC2C,
            17 => RedBagType::RedBagH5,
            18 => RedBagType::RedBagKSong,
            19 => RedBagType::RedBagEmoji,
            22 => RedBagType::RedBagDraw,
            20 => RedBagType::RedBagH5Common,
            24 => RedBagType::RedBagWordChain,
            25 => RedBagType::RedBagKeyword,
            26 => RedBagType::RedBagDrawMultiModel,
            _ => RedBagType::RedBagSimple,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AtSubType {
    AtGroupMember,
    AtGuildChannel,
    AtGuildMember,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageBizType {
    UnknownBizType,
    CustomFaceImage,
    HotImage,
    DouImage, // 斗图
    ZhiTuImage,
    StickerImage,
    SelfieImage,
    StickerAdImage,
    RelatedEmoImage,
    HotSearchImage,
}

impl From<u32> for ImageBizType {
    fn from(v: u32) -> Self {
        match v {
            0 => ImageBizType::UnknownBizType,
            1 => ImageBizType::CustomFaceImage,
            2 => ImageBizType::HotImage,
            3 => ImageBizType::DouImage,
            4 => ImageBizType::ZhiTuImage,
            7 => ImageBizType::StickerImage,
            8 => ImageBizType::SelfieImage,
            9 => ImageBizType::StickerAdImage,
            10 => ImageBizType::RelatedEmoImage,
            13 => ImageBizType::HotSearchImage,
            _ => ImageBizType::UnknownBizType,
        }
    }
}

impl Into<u32> for ImageBizType {
    fn into(self) -> u32 {
        match self {
            ImageBizType::UnknownBizType => 0,
            ImageBizType::CustomFaceImage => 1,
            ImageBizType::HotImage => 2,
            ImageBizType::DouImage => 3,
            ImageBizType::ZhiTuImage => 4,
            ImageBizType::StickerImage => 7,
            ImageBizType::SelfieImage => 8,
            ImageBizType::StickerAdImage => 9,
            ImageBizType::RelatedEmoImage => 10,
            ImageBizType::HotSearchImage => 13,
        }
    }
}

pub(crate) fn parse_elems(elems: Vec<crate::pb::msg::Elem>) -> Vec<MsgElem> {
    let mut msg_elems = vec![];
    elems.into_iter().map(|e| e.into()).for_each(|e| {
        if e != MsgElem::None {
            msg_elems.push(e);
        }
    });
    msg_elems
}

pub(crate) fn into_elems(msg_elems: Vec<MsgElem>) -> Vec<crate::pb::msg::Elem> {
    let mut elems = vec![];
    for e in msg_elems {
        let e: Vec<crate::pb::msg::Elem> = e.into();
        elems.extend(e.into_iter());
    }
    elems
}

pub(crate) fn at(target: i64, display: String, sub_type: AtSubType) -> MsgElem {
    MsgElem::At {
        target,
        display,
        sub_type,
    }
}

pub(crate) fn face(face_id: i32) -> MsgElem {
    MsgElem::Face {
        index: face_id,
        name: face::FACES_MAP
            .get(&face_id)
            .unwrap_or(&"未知表情")
            .to_string(),
    }
}
