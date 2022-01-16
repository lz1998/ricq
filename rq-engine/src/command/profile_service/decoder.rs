use bytes::Bytes;

use crate::command::common::PbToBytes;
use crate::command::profile_service::*;
use crate::RQResult;
use crate::{pb, RQError};

impl super::super::super::Engine {
    // ProfileService.Pb.ReqSystemMsgNew.Group
    pub fn decode_system_msg_group_packet(&self, payload: Bytes) -> RQResult<GroupSystemMessages> {
        let rsp = pb::structmsg::RspSystemMsgNew::from_bytes(&payload);
        let mut user_apply = Vec::new();
        let mut self_invited = Vec::new();
        let mut user_invited = Vec::new();
        match rsp {
            Ok(rsp) => {
                for st in rsp.groupmsgs.iter().filter(|st| st.msg.is_some()) {
                    if let Some(ref msg) = st.msg {
                        match msg.sub_type {
                            1 | 2 => match msg.group_msg_type {
                                1 => user_apply.push(UserApply {
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
                                }),
                                2 => self_invited.push(SelfInvited {
                                    request_id: st.msg_seq,
                                    invitor_uin: msg.action_uin,
                                    invitor_nick: msg.action_uin_nick.to_owned(),
                                    group_code: msg.group_code,
                                    group_name: msg.group_name.to_owned(),
                                    checked: msg.sub_type == 2,
                                    actor_uin: msg.actor_uin,
                                    actor_nick: msg.actor_uin_nick.to_owned(),
                                }),
                                22 => user_invited.push(UserInvited {
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
                                }),
                                _ => {}
                            },
                            // ?
                            3 => {}
                            // 自身状态变更(管理员/加群退群)
                            5 => {}
                            _ => {}
                        }
                    }
                }
                Ok(GroupSystemMessages {
                    self_invited,
                    user_apply,
                    user_invited,
                })
            }
            Err(_) => Err(RQError::Decode(
                "failed to decode RspSystemMsgNew".to_string(),
            )),
        }
    }
}
