mod image_info;

use tokio::sync::RwLock;

pub use crate::engine::structs::*;
pub use image_info::*;

// TODO 大群会占用大量内存，可以考虑提供 trait，用磁盘存储
#[derive(Default, Debug)]
pub struct Group {
    pub info: GroupInfo,
    pub members: RwLock<Vec<GroupMemberInfo>>,
}
