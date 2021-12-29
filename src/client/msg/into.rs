use crate::pb::msg::{CommonElem, Elem, Face, MsgElemInfoServtype33, RichMsg, Text};
use bytes::BufMut;
use flate2::bufread::ZlibEncoder;
use prost::Message;
use std::io::Read;

impl Into<Vec<Elem>> for super::MsgElem {
    fn into(self) -> Vec<Elem> {
        match self {
            Self::Text { content } => vec![Elem {
                text: Some(crate::pb::msg::Text {
                    str: Some(content.to_owned()),
                    ..Default::default()
                }),
                ..Default::default()
            }],

            Self::Face { index, name } => {
                if index >= 260 {
                    let text = format!("/{}", name).as_bytes().to_vec();
                    let elem = MsgElemInfoServtype33 {
                        index: Some(index as u32),
                        text: Some(text.clone()),
                        compat: Some(text),
                        buf: None,
                    };
                    let mut b = Vec::new();
                    elem.encode(&mut b).unwrap();
                    vec![Elem {
                        common_elem: Some(CommonElem {
                            service_type: Some(33),
                            pb_elem: Some(b),
                            business_type: Some(1),
                        }),
                        ..Default::default()
                    }]
                } else {
                    vec![Elem {
                        face: Some(Face {
                            index: Some(index),
                            old: Some(((0x1445 - 4 + index) as u16).to_be_bytes().to_vec()),
                            buf: Some(vec![0x00, 0x01, 0x00, 0x04, 0x52, 0xCC, 0xF5, 0xD0]),
                        }),
                        ..Default::default()
                    }]
                }
            }

            Self::At {
                target,
                display,
                sub_type,
            } => {
                let mut r = vec![];
                match sub_type {
                    super::AtSubType::AtGroupMember => {
                        r.push(Elem {
                            text: Some(crate::pb::msg::Text {
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
                            }),
                            ..Default::default()
                        });
                    }
                    super::AtSubType::AtGuildMember => unimplemented!(),
                    super::AtSubType::AtGuildChannel => unimplemented!(),
                }
                r.push(Elem {
                    text: Some(crate::pb::msg::Text {
                        str: Some(" ".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                });
                r
            }

            Self::Service {
                id,
                content,
                res_id,
                sub_type,
            } => {
                if id == 1 {
                    vec![Elem {
                        text: Some(Text {
                            str: Some(res_id),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }]
                } else {
                    todo!()
                    // vec![Elem {
                    //     rich_msg: Some(RichMsg {
                    //         template1: { ZlibEncoder::new(content.as_bytes(), Compa).read_to_end() },
                    //         service_id: Some(id),
                    //         ..Default::default()
                    //     }),
                    //     ..Default::default()
                    // }]
                }
            }

            _ => todo!(),
        }
    }
}
