use crate::structs::{GroupLeave, MemberPermissionChange};
use crate::{jce, pb};

pub mod builder;
pub mod decoder;

#[derive(Debug, Default)]
pub struct ReqPush {
    pub uin: i64,
    pub msg_infos: Vec<jce::PushMessageInfo>,
}

#[derive(Debug, Clone)]
pub enum PushTransInfo {
    MemberLeave(GroupLeave),
    MemberPermissionChange(MemberPermissionChange),
    // TODO 转让
}
#[derive(Debug, Clone)]
pub struct OnlinePushTrans {
    pub msg_seq: i32,
    pub msg_uid: i64,
    pub msg_time: i32,
    pub info: PushTransInfo,
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
