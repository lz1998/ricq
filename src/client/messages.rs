use crate::client::income::decoder::online_push::GroupMessagePart;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct PrivateMessage {
    pub id: i32,
    pub internal_id: i32,
    pub self_id: i64, //?
    pub target: i64,
    pub time: i32,
    pub sender: Sender,
    pub elements: Vec<MsgElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMessage {
    pub id: i32,
    pub internal_id: i32,
    pub group_code: i64,
    pub group_name: String,
    pub sender: Sender,
    pub time: i32,
    pub elements: Vec<MsgElement>,
    pub original_obj: GroupMessagePart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sender {
    pub uin: i64,
    pub nickname: String,
    pub card_name: String,
    pub anonymous_info: AnonymousInfo,
    pub is_friend: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnonymousInfo {
    pub anonymous_id: String,
    pub anonymous_nick: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgElement {
    // todo copilot write this...
    Text(String),
    Image(String),
    Audio(String),
    Video(String),
    File(String),
    Face(String),
    Location(String),
    RichText(String),
    Unknown(String),
}

impl Add<Vec<MsgElement>> for MsgElement {
    type Output = Vec<MsgElement>;

    fn add(self, other: Vec<MsgElement>) -> Vec<MsgElement> {
        let mut v = Vec::new();
        v.push(self);
        v.extend(other);
        v
    }
}

impl AddAssign<MsgElement> for Vec<MsgElement> {
    fn add_assign(&mut self, other: MsgElement) {
        self.push(other);
    }
}

impl GroupMessage {
    pub fn new(part: GroupMessagePart, group_name: String, sender: Sender) -> Self {
        GroupMessage {
            id: part.seq,
            internal_id: part.rand,
            group_code: part.group_code,
            group_name,
            sender,
            time: part.time,
            elements: parse_msg_elements(part.elems.clone()),
            original_obj: part,
        }
    }
}

pub fn parse_msg_elements(input: Vec<crate::pb::msg::Elem>) -> Vec<MsgElement> {
    todo!()
}
