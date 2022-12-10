use std::fmt;
use std::io::{Read, Write};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

use super::fmt_extract_attr;
use crate::msg::{MessageChainBuilder, PushBuilder};
use crate::msg::{MessageElem, PushElem};
use crate::pb::msg;
use crate::{push_builder_impl, to_elem_vec_impl};

// Some of the share card message will be a LightApp with pkg id `com.tencent.structmsg`
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
        vec.push(MessageElem::LightApp(msg::LightApp {
            data: Some({
                let mut encoder = ZlibEncoder::new(vec![1], Compression::default());
                encoder.write_all(elem.content.as_bytes()).ok();
                encoder.finish().unwrap_or_default()
            }),
            ..Default::default()
        }));
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
                    content: String::from_utf8_lossy(&content).into_owned(),
                };
            }
        }
        Self::default()
    }
}

impl fmt::Display for LightApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[LightApp:")?;
        fmt_extract_attr(f, &self.content, "app", r#""app":""#, "\"")?;
        fmt_extract_attr(f, &self.content, "prompt", r#""prompt":""#, "\"")?;
        fmt_extract_attr(f, &self.content, "desc", r#""desc":""#, "\"")?;
        fmt_extract_attr(f, &self.content, "url", r#""jumpUrl":""#, "\"")?;
        fmt_extract_attr(f, &self.content, "title", r#""title":""#, "\"")?;
        fmt_extract_attr(f, &self.content, "tag", r#""tag":""#, "\"")?;
        f.write_str("]")
    }
}

to_elem_vec_impl!(LightApp);
push_builder_impl!(LightApp);
