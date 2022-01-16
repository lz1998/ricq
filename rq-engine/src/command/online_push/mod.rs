use crate::structs::{FriendInfo, GroupMemberPermission};
use crate::{jce, pb};

pub mod builder;
pub mod decoder;

#[derive(Debug, Default)]
pub struct ReqPush {
    pub resp: ReqPushResp,
    pub push_infos: Vec<PushInfo>,
}

#[derive(Debug, Default)]
pub struct ReqPushResp {
    pub uin: i64,
    pub msg_infos: Vec<jce::PushMessageInfo>,
}

#[derive(Debug, Default)]
pub struct PushInfo {
    pub msg_seq: i16,
    pub msg_time: i64,
    pub msg_uid: i64,
    pub push_msg: PushMsg,
}

#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
pub enum PushMsg {
    #[derivative(Default)]
    Unknown,
    T0x2dc {
        group_mute_events: Vec<GroupMuteEvent>,
        group_recalled_events: Vec<GroupMessageRecalledEvent>,
        group_red_bag_lucky_king_events: Vec<GroupRedBagLuckyKingNotifyEvent>,
        group_digest_events: Vec<GroupDigestEvent>,
    },
    T0x210 {},
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

pub struct FriendMessageRecalledEvent {
    pub friend_uin: i64,
    pub message_id: i32,
    pub time: i64,
}

pub struct NewFriendEvent {
    pub friend: FriendInfo,
}

#[derive(Debug, Default)]
pub struct GroupLeaveEvent {
    pub group_code: i64,
    pub operator: i64,
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

pub struct FriendPokeNotifyEvent {
    pub sender: i64,
    pub receiver: i64,
}

// 需要同步群成员
pub struct GroupMemberNeedSync {
    pub group_code: i64,
}
