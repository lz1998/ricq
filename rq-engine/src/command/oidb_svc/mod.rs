pub mod builder;
pub mod decoder;

// 群 @全体 剩余次数
#[derive(Default, Debug)]
pub struct GroupAtAllRemainInfo {
    pub can_at_all: bool,
    pub remain_at_all_count_for_group: u32,
    pub remain_at_all_count_for_uin: u32,
}
