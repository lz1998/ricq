use std::fmt;

use bytes::{Buf, BufMut};

use crate::msg::{MessageChainBuilder, PushBuilder};
use crate::msg::{MessageElem, PushElem};
use crate::pb::msg;
use crate::{push_builder_impl, to_elem_vec_impl};

#[derive(Default, Debug, Clone)]
pub struct At {
    pub target: i64,
    pub display: String,
}

impl At {
    pub fn new(target: i64) -> Self {
        Self {
            target,
            display: format!("@{target}"),
        }
    }
}

impl PushElem for At {
    fn push_to(elem: Self, vec: &mut Vec<MessageElem>) {
        vec.push(MessageElem::Text(msg::Text {
            attr6_buf: Some({
                let mut w = Vec::new();
                w.put_u16(1);
                w.put_u16(0);
                w.put_u16(elem.display.chars().count() as u16);
                w.put_u8(if elem.target == 0 { 1 } else { 0 });
                w.put_u32(elem.target as u32);
                w.put_u16(0);
                w
            }),
            str: Some(elem.display),
            ..Default::default()
        }));
    }
}

impl From<msg::Text> for At {
    fn from(e: msg::Text) -> Self {
        let (_, mut attr6) = e.attr6_buf().split_at(7);
        let target = attr6.get_u32() as i64;
        Self {
            target,
            display: e.str.unwrap_or_default(),
        }
    }
}

impl fmt::Display for At {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.display)
    }
}

to_elem_vec_impl!(At);
push_builder_impl!(At);
