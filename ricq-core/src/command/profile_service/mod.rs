pub mod builder;
pub mod decoder;

#[derive(Debug, Default, Clone)]
pub struct GroupSystemMessages {
    pub self_invited: Vec<SelfInvited>,
    pub join_group_requests: Vec<JoinGroupRequest>,
}

// 自己被邀请
#[derive(Debug, Default, Clone)]
pub struct SelfInvited {
    pub msg_seq: i64,
    pub msg_time: i64,
    pub invitor_uin: i64,
    pub invitor_nick: String,
    pub group_code: i64,
    pub group_name: String,
    pub actor_uin: i64,
    pub actor_nick: String,
}

// 用户申请进群
#[derive(Debug, Default, Clone)]
pub struct JoinGroupRequest {
    pub msg_seq: i64,
    pub msg_time: i64,
    pub message: String,
    pub req_uin: i64,
    pub req_nick: String,
    pub group_code: i64,
    pub group_name: String,
    pub actor_uin: i64,
    pub suspicious: bool,
    pub invitor_uin: Option<i64>,
    pub invitor_nick: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct FriendSystemMessages {
    pub requests: Vec<NewFriendRequest>,
}

#[derive(Debug, Default, Clone)]
pub struct NewFriendRequest {
    pub msg_seq: i64,
    pub message: String,
    pub req_uin: i64,
    pub req_nick: String,
}
