use crate::protocol::protobuf::{ChannelMsg, GuildNode};
use ricq_core::common::RQAddr;

#[derive(Clone, Debug, Default)]
pub struct FirstViewResponse {
    pub guild_count: u32,
    pub self_tinyid: u64,
    pub direct_message_switch: u32,
    pub direct_message_guild_count: u32,
}

#[derive(Clone, Debug, Default)]
pub struct FirstViewMessage {
    pub push_flag: u32,
    pub guild_nodes: Vec<GuildNode>,
    pub channel_msgs: Vec<ChannelMsg>,
    pub get_msg_time: u64,
    pub direct_message_guild_nodes: Vec<GuildNode>,
}

#[derive(Clone, Debug, Default)]
pub struct FirstView {
    pub response: FirstViewResponse,
    pub message: FirstViewMessage,
}

#[derive(Clone, Debug, Default)]
pub struct GuildUserProfile {
    pub tiny_id: u64,
    pub nickname: String,
    pub avatar_url: String,
    pub join_time: i64,
}

#[derive(Clone, Debug, Default)]
pub struct GuildSelfProfile {
    pub tiny_id: u64,
    pub nickname: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, Default)]
pub struct GuildImage {
    pub file_id: u64,
    pub file_name: String,
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub image_type: i32,
    pub download_index: Vec<u8>,
    pub signature: Vec<u8>,
    pub md5: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum GuildImageStoreResp {
    Exist {
        file_id: u64,
        addrs: Vec<RQAddr>,
        download_index: Vec<u8>,
    },
    NotExist {
        file_id: u64,
        upload_key: Vec<u8>,
        upload_addrs: Vec<RQAddr>,
        download_index: Vec<u8>,
    },
}

pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/", "guild.rs"));
}
