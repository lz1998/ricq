use std::io::{Read, Write};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

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

impl From<LightApp> for Vec<msg::elem::Elem> {
    fn from(e: LightApp) -> Self {
        vec![msg::elem::Elem::LightApp(msg::LightApp {
            data: Some({
                let mut encoder = ZlibEncoder::new(vec![1], Compression::default());
                encoder.write_all(e.content.as_bytes()).ok();
                encoder.finish().unwrap_or_default()
            }),
            ..Default::default()
        })]
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
                    content: String::from_utf8(content).unwrap(),
                };
            }
        }
        Self::default()
    }
}
