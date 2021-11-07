use std::collections::HashMap;
use bytes::Bytes;
use tokio::sync::RwLock;

#[derive(Default, Debug)]
pub struct LoginSigInfo {
    pub login_bitmap: u64,
    pub tgt: Bytes,
    pub tgt_key: Bytes,
    // study room manager | 0x16a
    pub srm_token: Bytes,
    pub t133: Bytes,
    pub encrypted_a1: Bytes,
    pub user_st_key: Bytes,
    pub user_st_web_sig: Bytes,
    pub s_key: Bytes,
    pub s_key_expired_time: i64,
    pub d2: Bytes,
    pub d2key: Bytes,
    pub wt_session_ticket_key: Bytes,
    // TODO 是不是可能None？
    pub device_token: Option<Bytes>,
    pub ps_key_map: HashMap<String, Bytes>,
    pub pt4token_map: HashMap<String, Bytes>,
}

pub struct QiDianAccountInfo {
    pub master_uin: i64,
    pub ext_name: String,
    pub create_time: i64,

    pub big_data_req_addrs: Vec<String>,
    pub big_data_req_session: BigDataReqSessionInfo,
}

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
    pub members: RwLock<Vec<GroupMemberInfo>>,
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

#[derive(Debug, Clone)]
pub enum GroupMemberPermission {
    Owner = 1,
    Administrator = 2,
    Member = 3,
}

impl Default for GroupMemberPermission {
    fn default() -> Self {
        Self::Member
    }
}


#[derive(Debug, Default)]
pub struct FriendInfo {
    pub uin: i64,
    pub nick: String,
    pub remark: String,
    pub face_id: i16,
}