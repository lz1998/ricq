use crate::AtSubType;
use crate::{pb::msg::*, FACES_MAP};
use bytes::{Buf, BufMut};
use prost::Message;
use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
pub enum MsgElem {
    Text(TextElem),
    At(AtElem),
    Reply(ReplyElem),
    Face(FaceElem),
    LightApp(LightAppElem),
    Other(elem::Elem),
    // None,
}

impl From<Elem> for MsgElem {
    fn from(elem: Elem) -> Self {
        if elem.elem.is_none() {
            unreachable!()
        }
        let elem = elem.elem.unwrap();

        match elem {
            elem::Elem::Text(text) => match text.try_into() {
                Ok(at) => Self::At(at),
                Err(text) => Self::Text(text.into()),
            },
            elem::Elem::Face(face) => Self::Face(face.into()),
            elem::Elem::SrcMsg(src_msg) => match src_msg.try_into() {
                Ok(reply) => Self::Reply(reply),
                Err(src_msg) => Self::Other(elem::Elem::SrcMsg(src_msg)),
            },
            elem::Elem::CommonElem(common_elem) => match common_elem.try_into() {
                Ok(face) => Self::Face(face),
                Err(common_elem) => Self::Other(elem::Elem::CommonElem(common_elem)),
            },
            elem::Elem::LightApp(light_app) => match light_app.try_into() {
                Ok(light_app) => Self::LightApp(light_app),
                Err(light_app) => Self::Other(elem::Elem::LightApp(light_app)),
            },
            _ => Self::Other(elem),
        }
    }
}

impl From<MsgElem> for Vec<Elem> {
    fn from(elem: MsgElem) -> Self {
        match elem {
            MsgElem::Text(text) => text.into(),
            MsgElem::At(at) => at.into(),
            MsgElem::Reply(reply) => reply.into(),
            MsgElem::Face(face) => face.into(),
            MsgElem::LightApp(light_app) => light_app.into(),
            MsgElem::Other(elem) => vec![Elem { elem: Some(elem) }],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextElem {
    pub content: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtElem {
    pub target: i64,
    pub display: String,
    pub sub_type: AtSubType,
}

impl TryFrom<Text> for AtElem {
    type Error = Text;

    fn try_from(text: Text) -> Result<Self, Text> {
        if !text.attr6_buf().is_empty() {
            let (_, mut attr6) = text.attr6_buf().split_at(7);
            let target = attr6.get_u32();
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

#[derive(Debug, Clone, PartialEq)]
pub struct ReplyElem {
    pub reply_seq: i32,
    pub sender: i64,
    pub group_id: i64,
    pub time: i32,
    pub elements: Vec<MsgElem>,
}

impl TryFrom<SourceMsg> for ReplyElem {
    type Error = SourceMsg;

    fn try_from(msg: SourceMsg) -> Result<Self, SourceMsg> {
        if msg.orig_seqs.is_empty() {
            Err(msg)
        } else {
            Ok(Self {
                reply_seq: msg.orig_seqs[0],
                time: msg.time(),
                sender: msg.sender_uin(),
                group_id: msg.to_uin(),
                elements: msg.elems.into_iter().map(|elem| elem.into()).collect(),
            })
        }
    }
}

impl From<ReplyElem> for Vec<Elem> {
    fn from(_: ReplyElem) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaceElem {
    pub index: i32,
    pub name: String,
}

impl From<Face> for FaceElem {
    fn from(face: Face) -> Self {
        Self {
            index: face.index(),
            name: FACES_MAP
                .get(&face.index())
                .unwrap_or(&"未知表情")
                .to_string(),
        }
    }
}

impl TryFrom<CommonElem> for FaceElem {
    type Error = CommonElem;

    fn try_from(elem: CommonElem) -> Result<Self, CommonElem> {
        match MsgElemInfoServtype33::decode(elem.pb_elem()) {
            Ok(new_face) => Ok(Self {
                index: new_face.index() as i32,
                name: FACES_MAP
                    .get(&(new_face.index() as i32))
                    .unwrap_or(&"未知表情")
                    .to_string(),
            }),
            Err(_) => Err(elem),
        }
    }
}

impl From<FaceElem> for Vec<Elem> {
    fn from(face: FaceElem) -> Self {
        if face.index >= 260 {
            let text = format!("/{}", face.name).as_bytes().to_vec();
            let elem = MsgElemInfoServtype33 {
                index: Some(face.index as u32),
                text: Some(text.clone()),
                compat: Some(text),
                buf: None,
            }
            .encode_to_vec();
            vec![Elem {
                elem: Some(elem::Elem::CommonElem(CommonElem {
                    service_type: Some(33),
                    pb_elem: Some(elem),
                    business_type: Some(1),
                })),
            }]
        } else {
            vec![Elem {
                elem: Some(elem::Elem::Face(Face {
                    index: Some(face.index),
                    old: Some(((0x1445 - 4 + face.index) as u16).to_be_bytes().to_vec()),
                    buf: Some(vec![0x00, 0x01, 0x00, 0x04, 0x52, 0xCC, 0xF5, 0xD0]),
                })),
            }]
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightAppElem {
    pub content: String,
}

impl TryFrom<LightApp> for LightAppElem {
    type Error = LightApp;

    fn try_from(app: LightApp) -> Result<Self, LightApp> {
        let mut data = app.data.clone().unwrap();
        if !data.is_empty() {
            let content = if data[0] == 0 {
                data.split_off(1)
            } else {
                let mut uncompressed = Vec::new();
                flate2::read::ZlibDecoder::new(&data[1..])
                    .read_to_end(&mut uncompressed)
                    .unwrap();
                uncompressed
            };
            if !content.is_empty() && content.len() < 1024 ^ 3 {
                return Ok(Self {
                    content: String::from_utf8(content).unwrap(),
                });
            }
        }

        Err(app)
    }
}

impl From<LightAppElem> for Vec<Elem> {
    fn from(app: LightAppElem) -> Self {
        vec![Elem {
            elem: Some(elem::Elem::LightApp(LightApp {
                data: Some(zlib_encode(app.content.as_bytes())),
                ..Default::default()
            })),
        }]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceElem {
    pub id: i32,
    pub content: String,
    pub res_id: String,
    pub sub_type: String,
}

impl From<ServiceElem> for Vec<Elem> {
    fn from(s: ServiceElem) -> Self {
        let mut v = vec![];
        if s.id == 1 {
            v.push(Elem {
                elem: Some(elem::Elem::Text(Text {
                    str: Some(s.res_id.clone()),
                    ..Default::default()
                })),
            })
        }
        v.push(Elem {
            elem: Some(elem::Elem::RichMsg(RichMsg {
                template1: Some(zlib_encode(s.content.as_bytes())),
                service_id: Some(s.id),
                ..Default::default()
            })),
        });
        v
    }
}

fn zlib_encode(content: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    flate2::read::ZlibEncoder::new(content, flate2::Compression::default())
        .read_to_end(&mut buf)
        .unwrap();
    buf.insert(0, 1);
    buf
}
