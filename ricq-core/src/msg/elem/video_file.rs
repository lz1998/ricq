use std::fmt;

use crate::{push_builder_impl, to_elem_vec_impl};
use crate::hex::encode_hex;
use crate::msg::{MessageElem, PushElem};
use crate::msg::{MessageChainBuilder, PushBuilder};
use crate::pb::msg;

#[derive(Default, Debug, Clone)]
pub struct VideoFile {
    pub name: String,
    pub uuid: Vec<u8>,
    pub size: i32,
    pub thumb_size: i32,
    pub md5: Vec<u8>,
    pub thumb_md5: Vec<u8>,
}

impl From<msg::VideoFile> for VideoFile {
    fn from(e: msg::VideoFile) -> Self {
        Self {
            name: e.file_name.unwrap_or_default(),
            uuid: e.file_uuid.unwrap_or_default(),
            size: e.file_size.unwrap_or_default(),
            thumb_size: e.thumb_file_size.unwrap_or_default(),
            md5: e.file_md5.unwrap_or_default(),
            thumb_md5: e.thumb_file_md5.unwrap_or_default(),
        }
    }
}

impl PushElem for VideoFile {
    fn push_to(elem: Self, vec: &mut Vec<MessageElem>) {
        vec.push(
            MessageElem::Text(msg::Text {
                str: Some("你的QQ暂不支持查看视频短片，请期待后续版本。".into()),
                ..Default::default()
            })
        );
        vec.push(
            MessageElem::VideoFile(msg::VideoFile {
                file_uuid: Some(elem.uuid),
                file_name: Some(format!("{}.mp4", encode_hex(&elem.md5))),
                file_md5: Some(elem.md5),
                file_format: Some(3),
                file_time: Some(10),
                file_size: Some(elem.size),
                thumb_width: Some(1280),
                thumb_height: Some(720),
                thumb_file_md5: Some(elem.thumb_md5),
                thumb_file_size: Some(elem.thumb_size),
                busi_type: Some(0), // guild 4601
                from_chat_type: Some(-1),
                to_chat_type: Some(-1),
                bool_support_progressive: Some(true),
                file_width: Some(1280), // guild 0
                file_height: Some(720), // guild 0
                sub_busi_type: None,    // guild 4601
                video_attr: None,       // guild 0
                ..Default::default()
            })
        );
    }
}

impl fmt::Display for VideoFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[VideoFile: {}]", self.name)
    }
}

to_elem_vec_impl!(VideoFile);
push_builder_impl!(VideoFile);