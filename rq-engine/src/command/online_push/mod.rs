use crate::structs::GroupMemberPermission;
use crate::{jce, pb};

pub mod builder;
pub mod decoder;

#[derive(Debug, Default)]
pub struct ReqPush {
    pub uin: i64,
    pub msg_infos: Vec<jce::PushMessageInfo>,
}

#[derive(Debug, Default)]
pub struct GroupMuteEvent {}

#[derive(Debug, Default)]
pub struct GroupMessageRecalledEvent {}

#[derive(Debug, Default)]
pub struct GroupRedBagLuckyKingNotifyEvent {}

#[derive(Debug, Default)]
pub struct GroupDigestEvent {}

pub enum OnlinePushTrans {
    MemberLeave {
        msg_uid: i64,
        // 和group_code不一样
        group_uin: i64,
        member_uin: i64,
    },
    MemberKicked {
        msg_uid: i64,
        // 和group_code不一样
        group_uin: i64,
        member_uin: i64,
        operator_uin: i64,
    },
    MemberPermissionChanged {
        msg_uid: i64,
        // 和group_code不一样
        group_uin: i64,
        member_uin: i64,
        new_permission: GroupMemberPermission,
    },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GroupMessagePart {
    pub seq: i32,
    pub rand: i32,
    pub group_code: i64,
    pub from_uin: i64,
    pub elems: Vec<pb::msg::Elem>,
    pub time: i32,
    // 语音消息
    pub ptt: Option<pb::msg::Ptt>,

    // 整个message有多少个part，大于elem.len()时，应等待下一个片段到达后合并
    pub pkg_num: i32,
    // 分片的第几段
    pub pkg_index: i32,
    // 分片id，相同id的应该合并，且根据pkg_index排序
    pub div_seq: i32,
}

#[derive(Debug, Default)]
pub struct Sub0x27Event {
    pub group_name_updated_events: Vec<GroupNameUpdatedEvent>,
    pub del_friend_events: Vec<i64>,
}

#[derive(Debug, Default)]
pub struct GroupNameUpdatedEvent {
    pub group_code: i64,
    pub new_name: String,
    pub operator_uin: i64,
}

// 需要同步群成员
pub struct GroupMemberNeedSync {
    pub group_code: i64,
}
