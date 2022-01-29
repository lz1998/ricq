use super::ImageBizType;
use crate::pb::msg::*;
use crate::{
    pb::msg::{
        AnimationImageShow, CommonElem, CustomFace, Elem, Face, LightApp, MsgElemInfoServtype3,
        MsgElemInfoServtype33, MsgElemInfoServtype37, ResvAttr, RichMsg, Text,
    },
    MsgElem,
};
use bytes::BufMut;
use flate2::{bufread::ZlibEncoder, Compression};
use prost::Message;
use std::{io::Read, vec};

impl From<MsgElem> for Vec<Elem> {
    fn from(msg_elem: MsgElem) -> Vec<Elem> {
        match msg_elem {
            MsgElem::Text { content } => vec![Elem {
                elem: Some(elem::Elem::Text(Text {
                    str: Some(content),
                    ..Default::default()
                })),
            }],

            MsgElem::Face { index, name } => {
                if index >= 260 {
                    let text = format!("/{}", name).as_bytes().to_vec();
                    let elem = MsgElemInfoServtype33 {
                        index: Some(index as u32),
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
                            index: Some(index),
                            old: Some(((0x1445 - 4 + index) as u16).to_be_bytes().to_vec()),
                            buf: Some(vec![0x00, 0x01, 0x00, 0x04, 0x52, 0xCC, 0xF5, 0xD0]),
                        })),
                    }]
                }
            }

            MsgElem::At {
                target,
                display,
                sub_type,
            } => {
                let mut r = vec![];
                match sub_type {
                    super::AtSubType::AtGroupMember => {
                        r.push(Elem {
                            elem: Some(elem::Elem::Text(crate::pb::msg::Text {
                                str: Some(display.to_owned()),
                                attr6_buf: Some({
                                    let mut w = Vec::new();
                                    w.put_u16(1);
                                    w.put_u16(0);
                                    w.put_u16(display.chars().count() as u16);
                                    w.put_u8(if target == 0 { 1 } else { 0 });
                                    w.put_u32(target as u32);
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
                    elem: Some(elem::Elem::Text(crate::pb::msg::Text {
                        str: Some(" ".to_string()),
                        ..Default::default()
                    })),
                });
                r
            }

            MsgElem::Service {
                id,
                content,
                res_id,
                ..
            } => {
                if id == 1 {
                    vec![Elem {
                        elem: Some(elem::Elem::Text(Text {
                            str: Some(res_id),
                            ..Default::default()
                        })),
                    }]
                } else {
                    vec![Elem {
                        elem: Some(elem::Elem::RichMsg(RichMsg {
                            template1: Some(zlib_encode(content.as_bytes())),
                            service_id: Some(id),
                            ..Default::default()
                        })),
                    }]
                }
            }

            MsgElem::LightApp { content } => {
                vec![Elem {
                    elem: Some(elem::Elem::LightApp(LightApp {
                        data: Some(zlib_encode(content.as_bytes())),
                        ..Default::default()
                    })),
                }]
            }

            MsgElem::AnimatedSticker { id, mut name } => {
                if name.is_empty() {
                    name = super::face::FACES_MAP.get(&id).unwrap().to_string();
                }
                name = ["/".to_string(), name].concat();
                let business = if id == 114 { 2 } else { 1 };
                let pb_elem = MsgElemInfoServtype37 {
                    packid: Some("1".as_bytes().to_vec()),
                    stickerid: Some(
                        super::face::STICKER_MAP
                            .get(&id)
                            .unwrap()
                            .as_bytes()
                            .to_vec(),
                    ),
                    qsid: Some(id as u32),
                    sourcetype: Some(1),
                    stickertype: Some(business),
                    resultid: None,
                    text: Some(name.as_bytes().to_vec()),
                    surpriseid: None,
                    randomtype: Some(1),
                }
                .encode_to_vec();
                let pb_reverse = Elem {
                    elem: Some(elem::Elem::Text(Text {
                        str: Some(format!("[{}]请使用最新版手机QQ体验新功能", name)),
                        ..Default::default()
                    })),
                }
                .encode_to_vec();
                vec![
                    Elem {
                        elem: Some(elem::Elem::CommonElem(CommonElem {
                            service_type: Some(37),
                            pb_elem: Some(pb_elem),
                            business_type: Some(business as i32),
                        })),
                    },
                    Elem {
                        elem: Some(elem::Elem::Text(Text {
                            str: Some(name),
                            pb_reserve: Some(pb_reverse),
                            ..Default::default()
                        })),
                    },
                ]
            }

            MsgElem::GroupImage {
                mut width,
                mut height,
                file_id,
                image_id,
                image_type,
                size,
                md5,
                flash,
                effect_id,
                image_biz_type,
                url: _,
            } => {
                if width == 0 {
                    width = 720
                }
                if height == 0 {
                    height = 480
                }

                let mut face = CustomFace {
                    file_type: Some(66),
                    useful: Some(1),
                    biz_type: Some(5),
                    width: Some(width),
                    height: Some(height),
                    file_id: Some(file_id as i32),
                    file_path: Some(image_id),
                    image_type: Some(image_type),
                    size: Some(size),
                    md5: Some(md5.to_vec()),
                    flag: Some(vec![0x00; 4]),
                    ..Default::default()
                };
                if flash {
                    let flash = MsgElemInfoServtype3 {
                        flash_troop_pic: Some(face),
                        ..Default::default()
                    }
                    .encode_to_vec();
                    vec![
                        Elem {
                            elem: Some(elem::Elem::CommonElem(CommonElem {
                                service_type: Some(3),
                                pb_elem: Some(flash),
                                ..Default::default()
                            })),
                        },
                        Elem {
                            elem: Some(elem::Elem::Text(Text {
                                str: Some("[闪照]请使用新版手机QQ查看闪照。".to_string()),
                                ..Default::default()
                            })),
                        },
                    ]
                } else {
                    let mut res = ResvAttr::default();
                    if effect_id != 0 {
                        res.image_show = Some(AnimationImageShow {
                            effect_id: Some(effect_id),
                            animation_param: Some("{}".as_bytes().to_vec()),
                        });
                        face.flag = Some(vec![0x11, 0x00, 0x00, 0x00]);
                    }
                    if image_biz_type != ImageBizType::UnknownBizType {
                        res.image_biz_type = Some(image_biz_type.into());
                    }
                    face.pb_reserve = Some(res.encode_to_vec());
                    vec![Elem {
                        elem: Some(elem::Elem::CustomFace(face)),
                    }]
                }
            }

            _ => vec![], // TODO
        }
    }
}

fn zlib_encode(content: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    ZlibEncoder::new(content, Compression::default())
        .read_to_end(&mut buf)
        .unwrap();
    buf.insert(0, 1);
    buf
}
