use std::io::{Read, Write};

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};

use crate::{push_builder_impl, to_elem_vec_impl};
use crate::msg::{MessageElem, PushElem};
use crate::msg::{MessageChainBuilder, PushBuilder};
use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct LightApp {
    pub content: String,
}

impl LightApp {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl PushElem for LightApp {
    fn push_to(elem: Self, vec: &mut Vec<MessageElem>) {
        vec.push(
            MessageElem::LightApp(msg::LightApp {
                data: Some({
                    let mut encoder = ZlibEncoder::new(vec![1], Compression::default());
                    encoder.write_all(elem.content.as_bytes()).ok();
                    encoder.finish().unwrap_or_default()
                }),
                ..Default::default()
            })
        );
    }
}

impl From<msg::LightApp> for LightApp {
    fn from(e: msg::LightApp) -> Self {
        let data = e.data.unwrap_or_default();
        if data.len() > 1 {
            let content = if data[0] == 0 {
                data[1..].to_vec()
            } else {
                let mut uncompressed = Vec::new();
                ZlibDecoder::new(&data[1..])
                    .read_to_end(&mut uncompressed)
                    .ok();
                uncompressed
            };
            if !content.is_empty() && content.len() < 1024 ^ 3 {
                return Self {
                    content: String::from_utf8_lossy(&content).to_string(),
                };
            }
        }
        Self::default()
    }
}

to_elem_vec_impl!(LightApp);
push_builder_impl!(LightApp);