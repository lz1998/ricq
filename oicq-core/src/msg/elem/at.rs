use bytes::{Buf, BufMut};
use std::fmt;

use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct At {
    pub target: i64,
    pub display: String,
}

impl At {
    pub fn new(target: i64) -> Self {
        Self {
            target,
            display: format!("@{}", target),
        }
    }
}

impl From<At> for Vec<msg::elem::Elem> {
    fn from(e: At) -> Self {
        vec![msg::elem::Elem::Text(msg::Text {
            attr6_buf: Some({
                let mut w = Vec::new();
                w.put_u16(1);
                w.put_u16(0);
                w.put_u16(e.display.chars().count() as u16);
                w.put_u8(if e.target == 0 { 1 } else { 0 });
                w.put_u32(e.target as u32);
                w.put_u16(0);
                w
            }),
            str: Some(e.display),
            ..Default::default()
        })]
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
