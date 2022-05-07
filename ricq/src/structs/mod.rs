mod image_info;
mod video_info;

use tokio::sync::RwLock;

pub use image_info::*;
pub use ricq_core::structs::*;
pub use video_info::*;

// TODO 大群会占用大量内存，可以考虑提供 trait，用磁盘存储
#[derive(Default, Debug)]
pub struct Group {
    pub info: GroupInfo,
    pub members: RwLock<Vec<GroupMemberInfo>>,
}
