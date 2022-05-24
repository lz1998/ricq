use std::collections::HashMap;

use bytes::Bytes;

use crate::structs::*;

pub mod builder;
pub mod decoder;

#[derive(Debug, Default)]
pub struct FriendListResponse {
    /// 好友列表
    pub friend_list: Vec<FriendInfo>,
    /// 好友分组
    pub friend_group_list: HashMap<u8, FriendGroupInfo>,
    /// 好友数量
    pub total_count: i16,
    /// 在线好友数量
    pub online_friend_count: i16,
}

#[derive(Debug)]
pub struct GroupListResponse {
    pub groups: Vec<GroupInfo>,
    pub vec_cookie: Bytes,
}

#[derive(Debug)]
pub struct GroupMemberListResponse {
    pub next_uin: i64,
    pub list: Vec<GroupMemberInfo>,
}
