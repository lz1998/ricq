use bytes::BufMut;
use crate::pb;

use crate::client::outcome::PbToBytes;


pub enum Msg {
    Text {
        content: String
    },
    At {
        target: i64,
        display: String,
    },
    Face {
        index: i32,
        name: String,
    },
    Image,
    GroupImage,
    FriendImage,
    Reply,
    Service,
    Forward,
    File,
    Voice,
    Video,
    LightApp,
    RedBag,
}

impl Msg {
    pub fn pack(&self) -> Vec<pb::msg::Elem> {
        match self {
            Msg::Text { content } => {
                vec![
                    pb::msg::Elem {
                        text: Some(pb::msg::Text {
                            str: Some(content.to_owned()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                ]
            }
            Msg::At { target, display } => {
                vec![
                    pb::msg::Elem {
                        text: Some(pb::msg::Text {
                            str: Some(display.to_owned()),
                            attr6_buf: Some({
                                let mut w = Vec::new();
                                w.put_u16(1);
                                w.put_u16(0);
                                w.put_u16(display.chars().count() as u16);
                                w.put_u8(if *target == 0 { 1 } else { 0 });
                                w.put_u32(*target as u32);
                                w.put_u16(0);
                                w
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    pb::msg::Elem {
                        text: Some(pb::msg::Text {
                            str: Some(" ".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]
            }
            Msg::Face { index, name } => {
                if *index >= 260 {
                    vec![
                        pb::msg::Elem {
                            common_elem: Some(pb::msg::CommonElem {
                                service_type: Some(33),
                                pb_elem: Some(pb::msg::MsgElemInfoServtype33 {
                                    index: Some(*index as u32),
                                    text: Some(("/".to_owned() + &name).as_bytes().to_vec()),
                                    compat: Some(("/".to_owned() + &name).as_bytes().to_vec()),
                                    ..Default::default()
                                }.to_bytes().to_vec()),
                                business_type: Some(1),
                            }),
                            ..Default::default()
                        }
                    ]
                } else {
                    vec![
                        pb::msg::Elem {
                            face: Some(pb::msg::Face {
                                index: Some(*index),
                                old: Some(((0x1445 - 4 + index) as i16).to_be_bytes().to_vec()),
                                buf: Some(vec![0x00, 0x01, 0x00, 0x04, 0x52, 0xCC, 0xF5, 0xD0]),
                            }),
                            ..Default::default()
                        }
                    ]
                }
            }
            _ => Vec::new()
        }
    }
}