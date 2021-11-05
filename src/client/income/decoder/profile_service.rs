use crate::client::outcome::PbToBytes;
use crate::pb::structmsg;

#[derive(Debug, Default)]
pub struct GroupSystemMessages {
    pub self_invited: Vec<SelfInvited>,
    pub user_apply: Vec<UserApply>,
    pub user_invited: Vec<UserInvited>,
}

// 自己被邀请
#[derive(Debug, Default)]
pub struct SelfInvited {
    request_id: i64,
    invitor_uin: i64,
    invitor_nick: String,
    group_code: i64,
    group_name: String,
    checked: bool,
    actor_uin: i64,
    actor_nick: String,
}

// 用户申请进群
#[derive(Debug, Default)]
pub struct UserApply {
    request_id: i64,
    message: String,
    requester_uin: i64,
    requester_nick: String,
    group_code: i64,
    group_name: String,
    checked: bool,
    actor_uin: i64,
    actor_nick: String,
    suspicious: bool,
}

// 用户被邀请进群
#[derive(Debug, Default)]
pub struct UserInvited {
    request_id: i64,
    message: String,
    requester_uin: i64,
    requester_nick: String,
    group_code: i64,
    group_name: String,
    checked: bool,
    actor_uin: i64,
    suspicious: bool,
    action_uin: i64,
    action_uin_nick: String,
}

pub fn decode_system_msg_group_packet(payload: &[u8]) -> Option<GroupSystemMessages> {
    let rsp = structmsg::RspSystemMsgNew::from_bytes(payload);
    let mut user_apply = Vec::new();
    let mut self_invited = Vec::new();
    let mut user_invited = Vec::new();
    match rsp {
        Ok(rsp) => {
            for st in rsp.groupmsgs.iter()
                .filter(|st| st.msg.is_some())
            {
                if let Some(ref msg) = st.msg {
                    match msg.sub_type {
                        1 | 2 => {
                            match msg.group_msg_type {
                                1 => {
                                    user_apply.push(UserApply {
                                        request_id: st.msg_seq,
                                        message: msg.msg_additional.to_owned(),
                                        requester_uin: st.req_uin,
                                        requester_nick: msg.req_uin_nick.to_owned(),
                                        group_code: msg.group_code,
                                        group_name: msg.group_name.to_owned(),
                                        checked: msg.sub_type == 2,
                                        actor_uin: msg.actor_uin,
                                        actor_nick: msg.actor_uin_nick.to_owned(),
                                        suspicious: msg.warning_tips.len() > 0,
                                    })
                                }
                                2 => {
                                    self_invited.push(SelfInvited {
                                        request_id: st.msg_seq,
                                        invitor_uin: msg.action_uin,
                                        invitor_nick: msg.action_uin_nick.to_owned(),
                                        group_code: msg.group_code,
                                        group_name: msg.group_name.to_owned(),
                                        checked: msg.sub_type == 2,
                                        actor_uin: msg.actor_uin,
                                        actor_nick: msg.actor_uin_nick.to_owned(),
                                    })
                                }
                                22 => {
                                    user_invited.push(UserInvited {
                                        request_id: st.msg_seq,
                                        message: msg.msg_additional.to_owned(),
                                        requester_uin: st.req_uin,
                                        requester_nick: msg.req_uin_nick.to_owned(),
                                        group_code: msg.group_code,
                                        group_name: msg.group_name.to_owned(),
                                        checked: msg.sub_type == 2,
                                        actor_uin: msg.actor_uin,
                                        suspicious: msg.warning_tips.len() > 0,
                                        action_uin: msg.action_uin,
                                        action_uin_nick: msg.action_uin_qq_nick.to_owned(),
                                    })
                                }
                                _ => {}
                            }
                        }
                        // ?
                        3 => {}
                        // 自身状态变更(管理员/加群退群)
                        5 => {}
                        _ => {}
                    }
                }
            }
            Some(GroupSystemMessages {
                self_invited,
                user_apply,
                user_invited,
            })
        }
        Err(_) => { None }
    }
}
