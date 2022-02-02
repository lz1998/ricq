use std::fmt;

use prost::Message;

use crate::pb::msg;

pub mod at;
pub mod face;
pub mod market_face;
pub mod reply;
pub mod text;

#[derive(Debug, Clone)]
pub enum RQElem {
    At(at::At),
    Text(text::Text),
    Face(face::Face),
    Reply(reply::Reply),
    MarketFace(market_face::MarketFace),
    Dice(market_face::Dice),
    FingerGuessing(market_face::FingerGuessing),
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
                33 => {
                    if let Ok(new_face) = msg::MsgElemInfoServtype33::decode(e.pb_elem()) {
                        RQElem::Face(face::Face::from(new_face))
                    } else {
                        RQElem::Other(Box::new(elem))
                    }
                }
                _ => RQElem::Other(Box::new(elem)),
            },
            msg::elem::Elem::SrcMsg(e) => RQElem::Reply(reply::Reply::from(e)),
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
            RQElem::Reply(e) => fmt::Display::fmt(e, f),
            _ => write!(f, ""),
        }
    }
}
