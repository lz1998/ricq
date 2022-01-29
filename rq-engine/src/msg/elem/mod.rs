use prost::Message;

use crate::pb::msg;

pub mod at;
pub mod face;
pub mod reply;
pub mod text;

#[derive(Debug, Clone)]
pub enum RQElem {
    At(at::At),
    Text(text::Text),
    Face(face::Face),
    Reply(reply::Reply),
    Other(msg::elem::Elem),
    Unknown,
}

impl From<msg::Elem> for RQElem {
    fn from(e: msg::Elem) -> Self {
        if e.elem.is_none() {
            return RQElem::Unknown;
        }
        let elem = e.elem.unwrap();
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
                33 => {
                    if let Ok(new_face) = msg::MsgElemInfoServtype33::decode(e.pb_elem()) {
                        RQElem::Face(face::Face::from(new_face))
                    } else {
                        RQElem::Other(elem)
                    }
                }
                _ => RQElem::Other(elem),
            },
            _ => RQElem::Other(elem),
        }
    }
}
