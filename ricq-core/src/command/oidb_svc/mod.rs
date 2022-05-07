use std::collections::HashMap;

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

// 编辑个人资料
#[derive(Default, Debug)]
pub struct ProfileDetailUpdate(pub HashMap<u16, Vec<u8>>);

impl ProfileDetailUpdate {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, value: String) {
        self.0.insert(20002, value.into_bytes());
    }
    pub fn email(&mut self, value: String) {
        self.0.insert(20011, value.into_bytes());
    }
    pub fn personal_note(&mut self, value: String) {
        self.0.insert(20019, value.into_bytes());
    }
    pub fn company(&mut self, value: String) {
        self.0.insert(24008, value.into_bytes());
    }
    pub fn college(&mut self, value: String) {
        self.0.insert(20021, value.into_bytes());
    }
}
