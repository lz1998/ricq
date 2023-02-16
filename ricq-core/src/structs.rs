use bytes::Bytes;
use std::time::Duration;

pub use crate::command::multi_msg::{ForwardMessage, ForwardNode, MessageNode};
pub use crate::command::oidb_svc::{
    LinkShare, MusicShare, MusicVersion, ProfileDetailUpdate, ShareTarget,
};
pub use crate::command::stat_svc::{CustomOnlineStatus, ExtOnlineStatus, OnlineStatus, Status};
use crate::msg::MessageChain;
use crate::{jce, pb};

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
    // 全群禁言时间
    pub shut_up_timestamp: i64,
    // 自己被禁言时间
    pub my_shut_up_timestamp: i64,
    // 最后一条信息的SEQ,只有通过 GetGroupInfo 函数获取的 GroupInfo 才会有
    pub last_msg_seq: i64,
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

/// 好友信息
#[derive(Debug, Default, Clone)]
pub struct FriendInfo {
    pub uin: i64,
    pub nick: String,
    pub remark: String,
    pub face_id: i16,
    pub group_id: u8,
}

/// 好友分组信息
#[derive(Debug, Default, Clone)]
pub struct FriendGroupInfo {
    pub group_id: u8,
    pub group_name: String,
    pub friend_count: i32,
    pub online_friend_count: i32,
    pub seq_id: u8,
}

#[derive(Debug, Default, Clone)]
pub struct SummaryCardInfo {
    pub uin: i64,
    pub sex: u8,
    pub age: u8,
    pub nickname: String,
    pub level: i32,
    pub city: String,
    pub sign: String,
    pub mobile: String,
    pub login_days: i64,
    /// 用于点赞
    pub cookie: Bytes,
}

#[derive(Debug, Clone, Default)]
pub struct FriendMessage {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub target: i64,
    pub time: i32,
    pub from_uin: i64,
    pub from_nick: String,
    pub elements: MessageChain,
}

#[derive(Debug, Clone, Default)]
pub struct GroupMessage {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub group_code: i64,
    pub group_name: String,
    pub group_card: String,
    pub from_uin: i64,
    pub time: i32,
    pub elements: MessageChain,
}

#[derive(Debug, Clone, Default)]
pub struct GroupTempMessage {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub from_uin: i64,
    pub from_nick: String,
    pub time: i32,
    pub elements: MessageChain,
    pub group_code: i64,
}

#[derive(Debug, Clone, Default)]
pub struct NewMember {
    pub group_code: i64,
    pub member_uin: i64,
}

#[derive(Debug, Clone, Default)]
pub struct GroupMute {
    pub group_code: i64,
    pub operator_uin: i64,
    pub target_uin: i64,
    pub duration: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct FriendMessageRecall {
    pub msg_seq: i32,
    pub friend_uin: i64,
    pub time: i64,
}

#[derive(Debug, Clone, Default)]
pub struct GroupMessageRecall {
    pub msg_seq: i32,
    pub group_code: i64,
    pub operator_uin: i64,
    pub author_uin: i64,
    pub time: i32,
}

#[derive(Debug, Clone, Default)]
pub struct GroupLeave {
    pub group_code: i64,
    pub member_uin: i64,
    pub operator_uin: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct FriendPoke {
    pub sender: i64,
    pub receiver: i64,
}

#[derive(Debug, Clone, Default)]
pub struct GroupPoke {
    pub group_code: i64,
    pub sender: i64,
    pub receiver: i64,
}

#[derive(Debug, Clone, Default)]
pub struct GroupNameUpdate {
    pub group_code: i64,
    pub operator_uin: i64,
    pub group_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct DeleteFriend {
    pub uin: i64,
}

#[derive(Debug, Clone, Default)]
pub struct MemberPermissionChange {
    pub group_code: i64,
    pub member_uin: i64,
    pub new_permission: GroupMemberPermission,
}

#[derive(Debug, Clone, Default)]
pub struct GroupDisband {
    pub group_code: i64,
    pub operator_uin: i64,
}

// 用于撤回
#[derive(Debug, Clone, Default)]
pub struct MessageReceipt {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub time: i64,
}

#[derive(Debug, Clone, Default)]
pub struct GroupAudio(pub pb::msg::Ptt);

#[derive(Debug, Clone, Default)]
pub struct GroupAudioMessage {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub group_code: i64,
    pub group_name: String,
    pub group_card: String,
    pub from_uin: i64,
    pub time: i32,
    pub audio: GroupAudio,
}

#[derive(Debug, Clone, Default)]
pub struct FriendAudio(pub pb::msg::Ptt);

#[derive(Debug, Clone, Default)]
pub struct FriendAudioMessage {
    pub seqs: Vec<i32>,
    pub rands: Vec<i32>,
    pub target: i64,
    pub time: i32,
    pub from_uin: i64,
    pub from_nick: String,
    pub audio: FriendAudio,
}
// 群文件总数
#[derive(Debug, Clone, Default)]
pub struct GroupFileCount {
    pub is_full: bool,
    pub all_file_count: u32,
    pub limit_count: u32,
    pub file_too_many: bool,
}

// 群文件列表
#[derive(Debug, Clone, Default)]
pub struct GroupFileList {
    pub all_file_count: u32,
    pub is_end: bool,
    pub items: Vec<GroupFileItem>,
    pub role: u32,
    pub next_index: u32,
}
// 群文件列表
#[derive(Debug, Clone, Default)]
pub struct GroupFileItem {
    pub r#type: u32,
    pub folder_info: GroupFolderInfo,
    pub file_info: GroupFileInfo,
}

// 群文件夹
#[derive(Debug, Clone, Default)]
pub struct GroupFolderInfo {
    pub folder_id: String,
    pub parent_folder_id: String,
    pub folder_name: String,
    pub create_time: u32,
    pub modify_time: u32,
    pub create_uin: u64,
    pub creator_name: String,
    pub total_file_count: u32,
}
// 群文件
#[derive(Debug, Clone, Default)]
pub struct GroupFileInfo {
    pub file_id: String,
    pub file_name: String,
    pub file_size: u64,
    pub bus_id: u32,
    pub uploaded_size: u64,
    pub upload_time: u32,
    pub dead_time: u32,
    pub modify_time: u32,
    pub download_times: u32,
    pub sha: String,
    pub sha3: Bytes,
    pub md5: Bytes,
    pub local_path: String,
    pub uploader_name: String,
    pub uploader_uin: u64,
    pub parent_folder_id: String,
}
