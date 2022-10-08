use crate::protocol::protobuf::{ChannelMsg, GuildNode};
use bytes::Bytes;
use dynamic_protobuf::{dynamic_message, DynamicMessage};
use ricq_core::common::RQAddr;
use ricq_core::msg::{MessageChainBuilder, MessageElem};

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
    pub md5: Vec<u8>,
    pub server_ip: u32,
    pub server_port: u16,
}

impl ricq_core::msg::PushElem for GuildImage {
    fn push_to(img: Self, vec: &mut Vec<MessageElem>) {
        vec.push(MessageElem::CustomFace(ricq_core::pb::msg::CustomFace {
            file_path: Some(img.file_name),
            file_id: Some(img.file_id as _),
            server_ip: Some(img.server_ip),
            server_port: Some(img.server_port as _),
            file_type: Some(66),
            signature: None,
            useful: Some(1),
            md5: Some(img.md5),
            biz_type: Some(0),
            image_type: Some(img.image_type),
            width: Some(img.width),
            height: Some(img.height),
            source: Some(200),
            size: Some(img.size),
            origin: Some(1),
            thumb_width: Some((img.width * 10 / 3) as _),
            thumb_height: Some((img.height * 10 / 3) as _),
            show_len: Some(0),
            download_len: Some(0),
            pb_reserve: {
                let m = dynamic_message! {
                    1 => 0u32,
                    2 => 0u32,
                    6 => DynamicMessage::new(),
                    10 => 0u32,
                    15 => 1u32, // or 8?
                    20 => Bytes::from(img.download_index)
                }
                .encode_to_vec();

                Some(m)
            },
            ..Default::default()
        }));
    }
}

impl ricq_core::msg::PushBuilder for GuildImage {
    fn push_builder(elem: Self, builder: &mut MessageChainBuilder) {
        ricq_core::msg::PushElem::push_to(elem, &mut builder.elems);
    }
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

#[allow(clippy::all)]
pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/", "guild.rs"));
}
