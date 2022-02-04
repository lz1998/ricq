use crate::pb;

pub mod builder;
pub mod decoder;
pub mod music;

// 群 @全体 剩余次数
#[derive(Default, Debug)]
pub struct GroupAtAllRemainInfo {
    pub can_at_all: bool,
    pub remain_at_all_count_for_group: u32,
    pub remain_at_all_count_for_uin: u32,
}

pub struct OcrResponse {
    pub texts: Vec<pb::oidb::TextDetection>,
    pub language: String,
}
