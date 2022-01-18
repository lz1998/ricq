use bytes::Bytes;
use crate::msg::MsgElem;
use crate::command::online_push::GroupMessagePart;

use crate::jce;

#[derive(Default, Debug)]
pub struct AccountInfo {
    pub nickname: String,
    pub age: u8,
    pub gender: u8,
}

#[derive(Default, Debug)]
pub struct AddressInfo {
    pub srv_sso_addrs: Vec<String>,
    pub other_srv_addrs: Vec<String>,
    pub file_storage_info: jce::FileStoragePushFSSvcList,
}

#[derive(Debug, Default)]
pub struct OtherClientInfo {
    pub app_id: i64,
    pub instance_id: i32,
    pub sub_platform: String,
    pub device_kind: String,
}

pub struct QiDianAccountInfo {
    pub master_uin: i64,
    pub ext_name: String,
    pub create_time: i64,

    pub big_data_req_addrs: Vec<String>,
    pub big_data_req_session: BigDataReqSessionInfo,
}

#[derive(Debug, Default)]
pub struct BigDataReqSessionInfo {
    pub sig_session: Bytes,
    pub session_key: Bytes,
}

#[derive(Debug, Default)]
pub struct GroupInfo {
    pub uin: i64,
    pub code: i64,
    pub name: String,
    pub memo: String,
    pub owner_uin: i64,
    pub group_create_time: u32,
    pub group_level: u32,
    pub member_count: u16,
    pub max_member_count: u16,
    // 最后一条信息的SEQ,只有通过 GetGroupInfo 函数获取的 GroupInfo 才会有
    pub last_msg_seq: i64,
    // lock: RWMutex todo
}

#[derive(Debug, Default, Clone)]
pub struct GroupMemberInfo {
    pub group_code: i64,
    pub uin: i64,
    pub gender: u8,
    pub nickname: String,
    pub card_name: String,
    pub level: u16,
    pub join_time: i64,
    pub last_speak_time: i64,
    pub special_title: String,
    pub special_title_expire_time: i64,
    pub shut_up_timestamp: i64,
    pub permission: GroupMemberPermission,
}

#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(Default)]
pub enum GroupMemberPermission {
    Owner = 1,
    Administrator = 2,
    #[derivative(Default)]
    Member = 3,
}

#[derive(Debug, Default)]
pub struct FriendInfo {
    pub uin: i64,
    pub nick: String,
    pub remark: String,
    pub face_id: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrivateMessageEvent {
    pub id: i32,
    pub internal_id: i32,
    pub self_id: i64, //?
    pub target: i64,
    pub time: i32,
    pub sender: Sender,
    pub elements: Vec<MsgElem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMessageEvent {
    pub id: i32,
    pub internal_id: i32,
    pub group_code: i64,
    pub group_name: String,
    pub sender: Sender,
    pub time: i32,
    pub elements: Vec<MsgElem>,
    pub original_obj: GroupMessagePart,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMessageReceiptEvent {
    pub rand: i32,
    pub seq: i32,
    pub msg_event: GroupMessageEvent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sender {
    pub uin: i64,
    pub nickname: String,
    pub card_name: String,
    pub anonymous_info: AnonymousInfo,
    pub is_friend: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnonymousInfo {
    pub anonymous_id: String,
    pub anonymous_nick: String,
}
