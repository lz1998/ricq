use crate::pb::msg::*;
use crate::AtSubType;
use bytes::{Buf, BufMut};
use prost::Message;

pub enum MsgElem {
    Text(TextElem),
    At(AtElem),
    None,
}

// impl From<Elem> for MsgElem {
//     fn from(elem: Elem) -> Self {
//         if let Some(text) = elem.text {
//             return match text.try_into() {
//                 Ok(at) => Self::At(at),
//                 Err(text) => Self::Text(text.into()),
//             };
//         }
//         Self::None
//     }
// }

impl From<MsgElem> for Vec<Elem> {
    fn from(elem: MsgElem) -> Self {
        match elem {
            MsgElem::Text(text) => text.into(),
            MsgElem::At(at) => at.into(),
            MsgElem::None => Default::default(),
        }
    }
}

pub struct TextElem {
    pub content: String,
}

pub struct AtElem {
    pub target: i64,
    pub display: String,
    pub sub_type: AtSubType,
}

impl From<Text> for TextElem {
    fn from(text: Text) -> Self {
        Self {
            content: text.str().to_owned(),
        }
    }
}

impl From<TextElem> for Vec<Elem> {
    fn from(text: TextElem) -> Vec<Elem> {
        vec![Elem {
            elem: Some(elem::Elem::Text(Text {
                str: Some(text.content),
                ..Default::default()
            })),
        }]
    }
}

impl TryFrom<Text> for AtElem {
    type Error = Text;

    fn try_from(text: Text) -> Result<Self, Text> {
        if !text.attr6_buf().is_empty() {
            let (_, mut attr6) = text.attr6_buf().split_at(7);
            let target = attr6.get_i32();
            return Ok(Self {
                target: target as i64,
                display: text.str().to_owned(),
                sub_type: AtSubType::AtGroupMember,
            });
        } else if !text.pb_reserve().is_empty() {
            if let Ok(resv) = crate::pb::msg::TextResvAttr::decode(text.pb_reserve()) {
                if resv.at_type() == 2 {
                    return Ok(Self {
                        target: resv.at_member_tinyid() as i64,
                        display: text.str().to_owned(),
                        sub_type: AtSubType::AtGuildMember,
                    });
                } else if resv.at_type() == 4 {
                    return Ok(Self {
                        target: resv.at_channel_info.unwrap_or_default().channel_id() as i64,
                        display: text.str().to_owned(),
                        sub_type: AtSubType::AtGuildChannel,
                    });
                }
            }
        }

        Err(text)
    }
}

impl From<AtElem> for Vec<Elem> {
    fn from(at: AtElem) -> Vec<Elem> {
        let mut r = vec![];
        match at.sub_type {
            super::AtSubType::AtGroupMember => {
                r.push(Elem {
                    elem: Some(elem::Elem::Text(Text {
                        str: Some(at.display.to_owned()),
                        attr6_buf: Some({
                            let mut w = Vec::new();
                            w.put_u16(1);
                            w.put_u16(0);
                            w.put_u16(at.display.chars().count() as u16);
                            w.put_u8(if at.target == 0 { 1 } else { 0 });
                            w.put_u32(at.target as u32);
                            w.put_u16(0);
                            w
                        }),
                        ..Default::default()
                    })),
                });
            }
            super::AtSubType::AtGuildMember => unimplemented!(),
            super::AtSubType::AtGuildChannel => unimplemented!(),
        }
        r.push(Elem {
            elem: Some(elem::Elem::Text(Text {
                str: Some(" ".to_string()),
                ..Default::default()
            })),
        });
        r
    }
}
