use std::{fmt, mem};

use crate::msg::{MessageChainBuilder, MessageElem, PushBuilder, PushElem};
use crate::pb::msg;
use crate::to_elem_vec_impl;

#[derive(Default, Debug, Clone)]
pub struct Text {
    pub content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

to_elem_vec_impl!(Text);

impl From<msg::Text> for Text {
    fn from(e: msg::Text) -> Self {
        Self {
            content: e.str.unwrap_or_default(),
        }
    }
}

impl PushElem for Text {
    fn push_to(elem: Self, vec: &mut Vec<MessageElem>) {
        vec.push(MessageElem::Text(msg::Text {
            str: Some(elem.content),
            ..Default::default()
        }));
    }
}

impl PushBuilder for Text {
    fn push_builder(elem: Self, builder: &mut MessageChainBuilder) {
        builder.buf_string.push_str(&elem.content);
    }
}

pub fn flush_builder(builder: &mut MessageChainBuilder) {
    if !builder.buf_string.is_empty() {
        let s = mem::take(&mut builder.buf_string);
        builder.elems.push(
            MessageElem::Text(msg::Text {
                str: Some(s),
                ..Default::default()
            })
        );
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
