use std::io::{Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::msg::{MessageChainBuilder, PushBuilder};
use crate::msg::{MessageElem, PushElem};
use crate::pb::msg;
use crate::{push_builder_impl, to_elem_vec_impl};

#[derive(Default, Debug, Clone)]
pub struct RichMsg {
    pub service_id: i32,
    pub template1: String,
}

impl From<msg::RichMsg> for RichMsg {
    fn from(e: msg::RichMsg) -> Self {
        let data = e.template1.unwrap_or_default();
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
                    service_id: e.service_id.unwrap_or_default(),
                    template1: String::from_utf8_lossy(&content).into_owned(),
                };
            }
        }
        Self::default()
    }
}

impl PushElem for RichMsg {
    fn push_to(elem: Self, vec: &mut Vec<MessageElem>) {
        vec.push(MessageElem::RichMsg(msg::RichMsg {
            template1: Some({
                let mut encoder = ZlibEncoder::new(vec![1], Compression::default());
                encoder.write_all(elem.template1.as_bytes()).ok();
                encoder.finish().unwrap_or_default()
            }),
            service_id: Some(elem.service_id),
            ..Default::default()
        }));
    }
}

to_elem_vec_impl!(RichMsg);
push_builder_impl!(RichMsg);
