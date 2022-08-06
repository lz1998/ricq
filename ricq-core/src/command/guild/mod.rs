pub mod builder;
pub mod decoder;
mod dynamic_msg;

#[derive(Clone, Debug, Default)]
pub struct FirstViewResponse {
    pub guild_count: u32,
    pub self_tinyid: u64,
    pub direct_message_switch: u32,
    pub direct_message_guild_count: u32,
}

#[derive(Clone, Debug, Default)]
pub struct GuildUserProfile {
    pub tiny_id: u64,
    pub nickname: String,
    pub avatar_url: String,
    pub join_time: Option<i64>,
}
