use bytes::{Buf, Bytes};

use crate::client::income::decoder::online_push::GroupMessagePart;
use crate::pb::msg::Ptt;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct PrivateMessage {
    pub id: i32,
    pub internal_id: i32,
    pub self_id: i64, //?
    pub target: i64,
    pub time: i32,
    pub sender: Sender,
    pub elements: Vec<MsgElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMessage {
    pub id: i32,
    pub internal_id: i32,
    pub group_code: i64,
    pub group_name: String,
    pub sender: Sender,
    pub time: i32,
    pub elements: Vec<MsgElement>,
    pub original_obj: GroupMessagePart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sender {
    pub uin: i64,
    pub nickname: String,
    pub card_name: String,
    pub anonymous_info: AnonymousInfo,
    pub is_friend: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnonymousInfo {
    pub anonymous_id: String,
    pub anonymous_nick: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MsgElement {
    Reply {
        reply_seq: i32,
        sender: i64,
        group_id: i64,
        time: i32,
        elements: Vec<MsgElement>,
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
        guild: bool,
    },
    ShortVideo {
        name: String,
        uuid: Bytes,
        size: i32,
        thumb_size: i32,
        md5: Bytes,       // [u8;16]
        thumb_md5: Bytes, // [u8;16]
        utl: String,
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
}

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

impl Add<Vec<MsgElement>> for MsgElement {
    type Output = Vec<MsgElement>;

    fn add(self, other: Vec<MsgElement>) -> Vec<MsgElement> {
        let mut v = Vec::new();
        v.push(self);
        v.extend(other);
        v
    }
}

impl AddAssign<MsgElement> for Vec<MsgElement> {
    fn add_assign(&mut self, other: MsgElement) {
        self.push(other);
    }
}

impl GroupMessage {
    pub fn new(part: GroupMessagePart, group_name: String, sender: Sender) -> Self {
        GroupMessage {
            id: part.seq,
            internal_id: part.rand,
            group_code: part.group_code,
            group_name,
            sender,
            time: part.time,
            elements: parse_msg_elements(part.elems.clone()),
            original_obj: part,
        }
    }
}

pub fn parse_msg_elements(elems: Vec<crate::pb::msg::Elem>) -> Vec<MsgElement> {
    let mut msgs = vec![];
    for elem in elems {
        if let Some(Some(m)) = elem.src_msg.map(|m| {
            if m.orig_seqs.len() == 0 {
                None
            } else {
                Some(m)
            }
        }) {
            // 回复消息
            msgs.push(MsgElement::Reply {
                reply_seq: m.orig_seqs[0],
                time: m.time(),
                sender: m.sender_uin(),
                group_id: m.to_uin(),
                elements: parse_msg_elements(m.elems.clone()),
            });
        }

        if let Some(info) = elem.trans_elem_info {
            if info.elem_type() == 24 {
                let mut bytes = Bytes::copy_from_slice(info.elem_value());
                let i3 = bytes.len();
                if i3 > 3 {
                    if bytes.get_u8() == 1 {
                        // todo
                    }
                }
            }
        }
    }
    todo!();
    return msgs;
}
