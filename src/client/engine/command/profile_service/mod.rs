pub mod builder;
pub mod decoder;

#[derive(Debug, Default)]
pub struct GroupSystemMessages {
    pub self_invited: Vec<SelfInvited>,
    pub user_apply: Vec<UserApply>,
    pub user_invited: Vec<UserInvited>,
}

// 自己被邀请
#[derive(Debug, Default)]
pub struct SelfInvited {
    pub request_id: i64,
    pub invitor_uin: i64,
    pub invitor_nick: String,
    pub group_code: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor_uin: i64,
    pub actor_nick: String,
}

// 用户申请进群
#[derive(Debug, Default)]
pub struct UserApply {
    pub request_id: i64,
    pub message: String,
    pub requester_uin: i64,
    pub requester_nick: String,
    pub group_code: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor_uin: i64,
    pub actor_nick: String,
    pub suspicious: bool,
}

// 用户被邀请进群
#[derive(Debug, Default)]
pub struct UserInvited {
    pub request_id: i64,
    pub message: String,
    pub requester_uin: i64,
    pub requester_nick: String,
    pub group_code: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor_uin: i64,
    pub suspicious: bool,
    pub action_uin: i64,
    pub action_uin_nick: String,
}
