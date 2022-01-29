use bytes::{Buf, BufMut};

use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct At {
    pub target: i64,
    pub display: String,
}

impl Into<msg::Elem> for At {
    fn into(self) -> msg::Elem {
        msg::Elem {
            elem: Some(msg::elem::Elem::Text(msg::Text {
                attr6_buf: Some({
                    let mut w = Vec::new();
                    w.put_u16(1);
                    w.put_u16(0);
                    w.put_u16(self.display.chars().count() as u16);
                    w.put_u8(if self.target == 0 { 1 } else { 0 });
                    w.put_u32(self.target as u32);
                    w.put_u16(0);
                    w
                }),
                str: Some(self.display),
                ..Default::default()
            })),
        }
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
