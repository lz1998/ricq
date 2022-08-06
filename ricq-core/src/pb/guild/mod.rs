/// see sub_37628C
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0xf5bRsp {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="4")]
    pub bots: ::prost::alloc::vec::Vec<GuildMemberInfo>,
    #[prost(message, repeated, tag="5")]
    pub members: ::prost::alloc::vec::Vec<GuildMemberInfo>,
    #[prost(uint32, optional, tag="10")]
    pub next_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub finished: ::core::option::Option<u32>,
    #[prost(string, optional, tag="24")]
    pub next_query_param: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="25")]
    pub member_with_roles: ::prost::alloc::vec::Vec<GuildGroupMembersInfo>,
    #[prost(uint64, optional, tag="26")]
    pub next_role_id_index: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0xf88Rsp {
    #[prost(message, optional, tag="1")]
    pub profile: ::core::option::Option<GuildUserProfile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0xfc9Rsp {
    #[prost(message, optional, tag="1")]
    pub profile: ::core::option::Option<GuildUserProfile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0xf57Rsp {
    #[prost(message, optional, tag="1")]
    pub rsp: ::core::option::Option<GuildMetaRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0xf55Rsp {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<GuildChannelInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0xf5dRsp {
    #[prost(message, optional, tag="1")]
    pub rsp: ::core::option::Option<ChannelListRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0x1017Rsp {
    #[prost(message, optional, tag="1")]
    pub p1: ::core::option::Option<P10x1017>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct P10x1017 {
    #[prost(uint64, optional, tag="1")]
    pub tiny_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="3")]
    pub roles: ::prost::alloc::vec::Vec<GuildUserRole>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0x1019Rsp {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    /// 3: ?
    /// 4: 
    #[prost(message, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<GuildRole>,
}
//
//message ChannelOidb0x100dReq { // 修改身份组
//optional uint64 guildId = 1;
//repeated uint64 roleId = 2; 
//repeated int32 unkonwn = 3; // 3: ? 三个1
//repeated ModifyGuildRole role = 4;
//}

//
//message ChannelOidb0x1016Req { // 新建身份组
//optional uint64 guildId = 1;
//repeated int32 unknown = 2; // 2: ? 三个1
//optional ModifyGuildRole role = 3;
//repeated uint64 initialUsers = 4;
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOidb0x1016Rsp {
    #[prost(uint64, optional, tag="2")]
    pub role_id: ::core::option::Option<u64>,
}
//
//message ChannelOidb0x101aReq { // 修改身份组
//optional uint64 guildId = 1;
//repeated SetGuildRole setRoles = 2;
//repeated SetGuildRole removeRoles = 3;
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMetaRsp {
    #[prost(uint64, optional, tag="3")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="4")]
    pub meta: ::core::option::Option<GuildMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelListRsp {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    /// 5: Category infos
    #[prost(message, repeated, tag="2")]
    pub channels: ::prost::alloc::vec::Vec<GuildChannelInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildGroupMembersInfo {
    #[prost(uint64, optional, tag="1")]
    pub role_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub members: ::prost::alloc::vec::Vec<GuildMemberInfo>,
    #[prost(string, optional, tag="3")]
    pub role_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="4")]
    pub color: ::core::option::Option<u32>,
}
/// see sub_374334
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMemberInfo {
    #[prost(string, optional, tag="2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    /// uncertainty
    #[prost(int64, optional, tag="4")]
    pub last_speak_time: ::core::option::Option<i64>,
    /// uncertainty
    #[prost(int32, optional, tag="5")]
    pub role: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag="8")]
    pub tiny_id: ::core::option::Option<u64>,
}
/// 频道系统用户资料
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildUserProfile {
    #[prost(uint64, optional, tag="2")]
    pub tiny_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="3")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub avatar_url: ::core::option::Option<::prost::alloc::string::String>,
    /// 15: avatar url info
    ///
    /// uncertainty
    #[prost(int64, optional, tag="16")]
    pub join_time: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildRole {
    #[prost(uint64, optional, tag="1")]
    pub role_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub argb_color: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="4")]
    pub independent: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub num: ::core::option::Option<i32>,
    /// 是否拥有 存疑
    #[prost(int32, optional, tag="6")]
    pub owned: ::core::option::Option<i32>,
    /// 权限不足或不显示
    #[prost(int32, optional, tag="7")]
    pub disabled: ::core::option::Option<i32>,
    /// 9: ?
    #[prost(int32, optional, tag="8")]
    pub max_num: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildUserRole {
    #[prost(uint64, optional, tag="1")]
    pub role_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub argb_color: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="4")]
    pub independent: ::core::option::Option<i32>,
}
//
//message SetGuildRole {
//optional uint64 roleId = 1;
//optional uint64 targetId = 2;
//}

//
//message ModifyGuildRole {
//optional string roleName = 1;
//optional uint32 color = 2;
//optional int32 independent = 3; // 身份组单独显示
//}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMeta {
    #[prost(uint64, optional, tag="2")]
    pub guild_code: ::core::option::Option<u64>,
    #[prost(int64, optional, tag="4")]
    pub create_time: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="5")]
    pub max_member_count: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="6")]
    pub member_count: ::core::option::Option<i64>,
    #[prost(string, optional, tag="8")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="11")]
    pub robot_max_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="12")]
    pub admin_max_num: ::core::option::Option<i32>,
    #[prost(string, optional, tag="13")]
    pub profile: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="14")]
    pub avatar_seq: ::core::option::Option<i64>,
    #[prost(uint64, optional, tag="18")]
    pub owner_id: ::core::option::Option<u64>,
    #[prost(int64, optional, tag="19")]
    pub cover_seq: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="20")]
    pub client_id: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildChannelInfo {
    #[prost(uint64, optional, tag="1")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub channel_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="3")]
    pub creator_uin: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub create_time: ::core::option::Option<i64>,
    #[prost(uint64, optional, tag="5")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(int32, optional, tag="6")]
    pub final_notify_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub channel_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub talk_permission: ::core::option::Option<i32>,
    /// 11 - 14 : MsgInfo
    #[prost(uint64, optional, tag="15")]
    pub creator_tiny_id: ::core::option::Option<u64>,
    /// 16: Member info ?
    #[prost(int32, optional, tag="22")]
    pub visible_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="28")]
    pub top_msg: ::core::option::Option<GuildChannelTopMsgInfo>,
    #[prost(int32, optional, tag="31")]
    pub current_slow_mode_key: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="32")]
    pub slow_mode_infos: ::prost::alloc::vec::Vec<GuildChannelSlowModeInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildChannelSlowModeInfo {
    #[prost(int32, optional, tag="1")]
    pub slow_mode_key: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub speak_frequency: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub slow_mode_circle: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub slow_mode_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildChannelTopMsgInfo {
    #[prost(uint64, optional, tag="1")]
    pub top_msg_seq: ::core::option::Option<u64>,
    #[prost(int64, optional, tag="2")]
    pub top_msg_time: ::core::option::Option<i64>,
    #[prost(uint64, optional, tag="3")]
    pub top_msg_operator_tiny_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelContentHead {
    #[prost(uint64, optional, tag="1")]
    pub r#type: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub sub_type: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub random: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub cnt_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub time: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="7")]
    pub meta: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMessageMember {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub tinyid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub source_guild_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub source_guild_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub nick_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="6")]
    pub member_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="7")]
    pub notify_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelEvent {
    #[prost(uint64, optional, tag="1")]
    pub r#type: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub version: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub op_info: ::core::option::Option<ChannelMsgOpInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelExtInfo {
    #[prost(bytes="vec", optional, tag="1")]
    pub from_nick: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub guild_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub channel_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="4")]
    pub visibility: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub notify_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub offline_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub name_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub member_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="9")]
    pub timestamp: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="10")]
    pub event_version: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="11")]
    pub events: ::prost::alloc::vec::Vec<ChannelEvent>,
    #[prost(message, optional, tag="12")]
    pub from_role_info: ::core::option::Option<ChannelRole>,
    #[prost(message, optional, tag="13")]
    pub freq_limit_info: ::core::option::Option<ChannelFreqLimitInfo>,
    #[prost(message, repeated, tag="14")]
    pub direct_message_member: ::prost::alloc::vec::Vec<DirectMessageMember>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFreqLimitInfo {
    #[prost(uint32, optional, tag="1")]
    pub is_limited: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub left_count: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="3")]
    pub limit_timestamp: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInfo {
    #[prost(uint64, optional, tag="1")]
    pub id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub color: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub hoist: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLoginSig {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub appid: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMeta {
    #[prost(uint64, optional, tag="1")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(message, optional, tag="2")]
    pub login_sig: ::core::option::Option<ChannelLoginSig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgContent {
    #[prost(message, optional, tag="1")]
    pub head: ::core::option::Option<ChannelMsgHead>,
    #[prost(message, optional, tag="2")]
    pub ctrl_head: ::core::option::Option<ChannelMsgCtrlHead>,
    #[prost(message, optional, tag="3")]
    pub body: ::core::option::Option<super::msg::MessageBody>,
    #[prost(message, optional, tag="4")]
    pub ext_info: ::core::option::Option<ChannelExtInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgCtrlHead {
    #[prost(bytes="vec", repeated, tag="1")]
    pub include_uin: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// repeated uint64 excludeUin = 2; // bytes?
    /// repeated uint64 featureid = 3;
    #[prost(uint32, optional, tag="4")]
    pub offline_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub visibility: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="6")]
    pub ctrl_flag: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="7")]
    pub events: ::prost::alloc::vec::Vec<ChannelEvent>,
    #[prost(uint64, optional, tag="8")]
    pub level: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="9")]
    pub personal_levels: ::prost::alloc::vec::Vec<PersonalLevel>,
    #[prost(uint64, optional, tag="10")]
    pub guild_sync_seq: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="11")]
    pub member_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub private_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgHead {
    #[prost(message, optional, tag="1")]
    pub routing_head: ::core::option::Option<ChannelRoutingHead>,
    #[prost(message, optional, tag="2")]
    pub content_head: ::core::option::Option<ChannelContentHead>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgMeta {
    #[prost(uint64, optional, tag="1")]
    pub at_all_seq: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgOpInfo {
    #[prost(uint64, optional, tag="1")]
    pub operator_tinyid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub operator_role: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub reason: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub timestamp: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub at_type: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalLevel {
    #[prost(uint64, optional, tag="1")]
    pub to_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub level: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRole {
    #[prost(uint64, optional, tag="1")]
    pub id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelRoutingHead {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub from_tinyid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub guild_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub from_appid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="7")]
    pub direct_message_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Df62ReqBody {
    #[prost(message, optional, tag="1")]
    pub msg: ::core::option::Option<ChannelMsgContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Df62RspBody {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub errmsg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub send_time: ::core::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub head: ::core::option::Option<ChannelMsgHead>,
    #[prost(uint32, optional, tag="5")]
    pub err_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub trans_svr_info: ::core::option::Option<TransSvrInfo>,
    #[prost(message, optional, tag="7")]
    pub freq_limit_info: ::core::option::Option<ChannelFreqLimitInfo>,
    #[prost(message, optional, tag="8")]
    pub body: ::core::option::Option<super::msg::MessageBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransSvrInfo {
    #[prost(uint32, optional, tag="1")]
    pub sub_type: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="2")]
    pub ret_code: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub err_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub trans_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetMsgRspCountReq {
    #[prost(message, repeated, tag="1")]
    pub guild_msg_list: ::prost::alloc::vec::Vec<GuildMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetMsgRspCountRsp {
    #[prost(message, repeated, tag="1")]
    pub guild_msg_info_list: ::prost::alloc::vec::Vec<GuildMsgInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SvrChannelMsg {
    #[prost(uint64, optional, tag="1")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub id: ::prost::alloc::vec::Vec<MsgId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgInfo {
    #[prost(uint64, optional, tag="1")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub resp_data: ::prost::alloc::vec::Vec<MsgRespData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmojiReaction {
    #[prost(string, optional, tag="1")]
    pub emoji_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="2")]
    pub emoji_type: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub cnt: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="4")]
    pub is_clicked: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="10001")]
    pub is_default_emoji: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMsg {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub channel_msg_list: ::prost::alloc::vec::Vec<SvrChannelMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMsgInfo {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub channel_msg_info_list: ::prost::alloc::vec::Vec<ChannelMsgInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCnt {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<MsgId>,
    #[prost(message, repeated, tag="2")]
    pub emoji_reaction: ::prost::alloc::vec::Vec<EmojiReaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgId {
    #[prost(uint64, optional, tag="1")]
    pub version: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub seq: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRespData {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<MsgId>,
    #[prost(bytes="vec", optional, tag="2")]
    pub cnt: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUserInfo {
    #[prost(message, optional, tag="1")]
    pub client_identity: ::core::option::Option<ClientIdentity>,
    #[prost(uint32, optional, tag="2")]
    pub member_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub permission: ::core::option::Option<ChannelUserPermission>,
    #[prost(message, repeated, tag="4")]
    pub role_groups: ::prost::alloc::vec::Vec<BaseRoleGroupInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUserPermission {
    #[prost(bool, optional, tag="1")]
    pub allow_read_feed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="2")]
    pub allow_write_feed: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientIdentity {
    #[prost(uint32, optional, tag="1")]
    pub client_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseGuildInfo {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub join_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseRoleGroupInfo {
    #[prost(uint64, optional, tag="1")]
    pub role_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub color: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StChannelInfo {
    #[prost(message, optional, tag="1")]
    pub sign: ::core::option::Option<StChannelSign>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub icon_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StChannelSign {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StEmotionReactionInfo {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub emoji_reaction_list: ::prost::alloc::vec::Vec<EmojiReaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StCommonExt {
    #[prost(message, repeated, tag="1")]
    pub map_info: ::prost::alloc::vec::Vec<CommonEntry>,
    #[prost(string, optional, tag="2")]
    pub attach_info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub map_bytes_info: ::prost::alloc::vec::Vec<BytesEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesEntry {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonEntry {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentMetaData {
    #[prost(message, optional, tag="1")]
    pub count: ::core::option::Option<RichTextContentCount>,
    #[prost(int64, optional, tag="2")]
    pub content_id: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMetaData {
    #[prost(message, optional, tag="1")]
    pub content: ::core::option::Option<ContentMetaData>,
    #[prost(uint64, optional, tag="2")]
    pub last_modified_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedRedTouchTransInfo {
    #[prost(string, optional, tag="1")]
    pub feed_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="3")]
    pub create_ts: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="4")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub page_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub red_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub insert_page_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoticeOperation {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub schema: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RichTextContentCount {
    #[prost(uint64, optional, tag="1")]
    pub text_word: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub at: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub url: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub emoji: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub image: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub video: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StAnimation {
    #[prost(uint32, optional, tag="1")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub height: ::core::option::Option<u32>,
    #[prost(string, optional, tag="3")]
    pub animation_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="4")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StBusiReportInfo {
    #[prost(message, optional, tag="1")]
    pub recom_report: ::core::option::Option<StRecomReportInfo>,
    #[prost(string, optional, tag="2")]
    pub trace_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StChannelShareInfo {
    #[prost(string, optional, tag="1")]
    pub feed_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub poster_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub feed_publish_at: ::core::option::Option<u64>,
    #[prost(message, optional, tag="4")]
    pub channel_sign: ::core::option::Option<StChannelSign>,
    #[prost(uint64, optional, tag="5")]
    pub update_duration_ms: ::core::option::Option<u64>,
    #[prost(message, optional, tag="6")]
    pub sign: ::core::option::Option<StChannelShareSign>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StChannelShareSign {
    #[prost(uint64, optional, tag="1")]
    pub create_at: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StCircleRankItem {
    #[prost(int32, optional, tag="1")]
    pub rank_no: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub circle_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="3")]
    pub fuel_value: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub feed_num: ::core::option::Option<i64>,
    #[prost(string, optional, tag="5")]
    pub circle_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StClientInfo {
    #[prost(string, optional, tag="1")]
    pub feedclientkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub client_map: ::prost::alloc::vec::Vec<CommonEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StComment {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub post_user: ::core::option::Option<StUser>,
    #[prost(uint64, optional, tag="3")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(string, optional, tag="4")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="5")]
    pub reply_count: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="6")]
    pub vec_reply: ::prost::alloc::vec::Vec<StReply>,
    #[prost(bytes="vec", optional, tag="7")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="8")]
    pub like_info: ::core::option::Option<StLike>,
    #[prost(uint32, optional, tag="9")]
    pub type_flag: ::core::option::Option<u32>,
    #[prost(string, repeated, tag="10")]
    pub at_uin_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="11")]
    pub type_flag2: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="12")]
    pub create_time_ns: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="13")]
    pub store_ext_info: ::prost::alloc::vec::Vec<CommonEntry>,
    #[prost(string, optional, tag="14")]
    pub third_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="15")]
    pub source_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="16")]
    pub rich_contents: ::core::option::Option<StRichText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDebugInfo {
    #[prost(message, repeated, tag="1")]
    pub debug_map: ::prost::alloc::vec::Vec<CommonEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDittoFeed {
    #[prost(uint32, optional, tag="1")]
    pub ditto_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub ditto_pattern_id: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub ditto_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub ditto_data_new: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StExifInfo {
    #[prost(message, repeated, tag="1")]
    pub kvs: ::prost::alloc::vec::Vec<CommonEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StExternalMedalWallInfo {
    #[prost(bool, optional, tag="1")]
    pub need_red_point: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="2")]
    pub medal_infos: ::prost::alloc::vec::Vec<StMedalInfo>,
    #[prost(string, optional, tag="3")]
    pub medal_wall_jump_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="4")]
    pub need_show_entrance: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StFeed {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub title: ::core::option::Option<StRichText>,
    #[prost(message, optional, tag="3")]
    pub subtitle: ::core::option::Option<StRichText>,
    #[prost(message, optional, tag="4")]
    pub poster: ::core::option::Option<StUser>,
    #[prost(message, repeated, tag="5")]
    pub videos: ::prost::alloc::vec::Vec<StVideo>,
    #[prost(message, optional, tag="6")]
    pub contents: ::core::option::Option<StRichText>,
    #[prost(uint64, optional, tag="7")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(message, optional, tag="8")]
    pub emotion_reaction: ::core::option::Option<StEmotionReactionInfo>,
    #[prost(uint32, optional, tag="9")]
    pub comment_count: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="10")]
    pub vec_comment: ::prost::alloc::vec::Vec<StComment>,
    #[prost(message, optional, tag="11")]
    pub share: ::core::option::Option<StShare>,
    #[prost(message, optional, tag="12")]
    pub visitor_info: ::core::option::Option<StVisitor>,
    #[prost(message, repeated, tag="13")]
    pub images: ::prost::alloc::vec::Vec<StImage>,
    #[prost(message, optional, tag="14")]
    pub poi_info: ::core::option::Option<StPoiInfoV2>,
    #[prost(message, repeated, tag="15")]
    pub tag_infos: ::prost::alloc::vec::Vec<StTagInfo>,
    #[prost(bytes="vec", optional, tag="16")]
    pub busi_report: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, packed="false", tag="17")]
    pub op_mask: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag="18")]
    pub opinfo: ::core::option::Option<StOpinfo>,
    #[prost(message, repeated, tag="19")]
    pub ext_info: ::prost::alloc::vec::Vec<CommonEntry>,
    #[prost(string, optional, tag="20")]
    pub pattern_info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="21")]
    pub channel_info: ::core::option::Option<StChannelInfo>,
    #[prost(uint64, optional, tag="22")]
    pub create_time_ns: ::core::option::Option<u64>,
    #[prost(message, optional, tag="23")]
    pub summary: ::core::option::Option<StFeedSummary>,
    #[prost(message, optional, tag="24")]
    pub recom_info: ::core::option::Option<StRecomInfo>,
    #[prost(message, optional, tag="25")]
    pub meta: ::core::option::Option<FeedMetaData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StFeedAbstract {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub poster: ::core::option::Option<StUser>,
    #[prost(message, optional, tag="4")]
    pub pic: ::core::option::Option<StImage>,
    #[prost(uint32, optional, tag="5")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="6")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(message, optional, tag="7")]
    pub video: ::core::option::Option<StVideo>,
    #[prost(uint32, optional, tag="8")]
    pub fuel_num: ::core::option::Option<u32>,
    #[prost(string, optional, tag="9")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="10")]
    pub images: ::prost::alloc::vec::Vec<StImage>,
    #[prost(message, optional, tag="11")]
    pub count_info: ::core::option::Option<StFeedCount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StFeedCount {
    #[prost(int64, optional, tag="1")]
    pub liked: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub push: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub comment: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub visitor: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StFeedSummary {
    #[prost(uint32, optional, tag="1")]
    pub layout_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StFollowRecomInfo {
    #[prost(string, optional, tag="1")]
    pub follow_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub follow_users: ::prost::alloc::vec::Vec<StFollowUser>,
    #[prost(string, optional, tag="6")]
    pub comm_friend_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub comm_group_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StFollowUser {
    #[prost(uint64, optional, tag="1")]
    pub uid: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub nick: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGpsv2 {
    #[prost(int64, optional, tag="1")]
    pub lat: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub lon: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub e_type: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub alt: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGuidePublishBubble {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub background_image: ::core::option::Option<StImage>,
    #[prost(string, optional, tag="3")]
    pub jump_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StIconInfo {
    #[prost(string, optional, tag="1")]
    pub icon_url40: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub icon_url100: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub icon_url140: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub icon_url640: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub icon_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StImage {
    #[prost(uint32, optional, tag="1")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub height: ::core::option::Option<u32>,
    #[prost(string, optional, tag="3")]
    pub pic_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub vec_image_url: ::prost::alloc::vec::Vec<StImageUrl>,
    #[prost(string, optional, tag="5")]
    pub pic_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="6")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="7")]
    pub image_md5: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub layer_pic_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub pattern_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="10")]
    pub display_index: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StImageUrl {
    #[prost(uint32, optional, tag="1")]
    pub level_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub height: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="5")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StLightInteractInfo {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<StUser>,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<StRelationInfo>,
    #[prost(uint32, optional, tag="3")]
    pub count: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StLike {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub status: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="4")]
    pub vec_user: ::prost::alloc::vec::Vec<StUser>,
    #[prost(bytes="vec", optional, tag="5")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="6")]
    pub post_user: ::core::option::Option<StUser>,
    #[prost(uint32, optional, tag="7")]
    pub has_liked_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub owner_status: ::core::option::Option<u32>,
    #[prost(string, optional, tag="9")]
    pub jump_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StLiteBanner {
    #[prost(message, optional, tag="1")]
    pub icon: ::core::option::Option<StImage>,
    #[prost(string, optional, tag="2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub jump_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub activity_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub json_style: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="6")]
    pub ext_info: ::prost::alloc::vec::Vec<CommonEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StMaterialDataNew {
    #[prost(string, optional, tag="1")]
    pub material_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub material_list: ::prost::alloc::vec::Vec<StSingleMaterial>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StMedalInfo {
    #[prost(int32, optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub medal_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub medal_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="4")]
    pub rank: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="5")]
    pub is_high_light: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub is_new: ::core::option::Option<bool>,
    #[prost(string, optional, tag="7")]
    pub jump_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub icon_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub background_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub describe: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="11")]
    pub report_value: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StNotice {
    #[prost(message, optional, tag="1")]
    pub psv_feed: ::core::option::Option<StFeed>,
    #[prost(message, optional, tag="2")]
    pub origine_feed: ::core::option::Option<StFeed>,
    #[prost(message, optional, tag="3")]
    pub patton_info: ::core::option::Option<StNoticePattonInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StNoticePattonInfo {
    #[prost(uint32, optional, tag="1")]
    pub patton_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub plain_txt: ::core::option::Option<StPlainTxtInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StNoticeTxtInfo {
    #[prost(message, optional, tag="1")]
    pub content: ::core::option::Option<StRichText>,
    #[prost(message, optional, tag="2")]
    pub content_of_reference: ::core::option::Option<StRichText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StOpinfo {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub create_time: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StPlainTxtInfo {
    #[prost(message, optional, tag="1")]
    pub txt_info: ::core::option::Option<StNoticeTxtInfo>,
    #[prost(message, optional, tag="2")]
    pub operation: ::core::option::Option<NoticeOperation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StPoiInfoV2 {
    #[prost(string, optional, tag="1")]
    pub poi_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub poi_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub district_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag="7")]
    pub gps: ::core::option::Option<StGpsv2>,
    #[prost(int32, optional, tag="8")]
    pub distance: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub hot_value: ::core::option::Option<i32>,
    #[prost(string, optional, tag="10")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub country: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub province: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="14")]
    pub poi_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="15")]
    pub poi_order_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="16")]
    pub default_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub district: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub dian_ping_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub distance_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub display_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StPrePullCacheFeed {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub poster: ::core::option::Option<StUser>,
    #[prost(uint64, optional, tag="3")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="4")]
    pub busi_tranparent: ::prost::alloc::vec::Vec<BytesEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StProxyInfo {
    #[prost(int32, optional, tag="1")]
    pub cmd_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub sub_cmd_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub app_protocol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="4")]
    pub req_body: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRankingItem {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<StUser>,
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<StRelationInfo>,
    #[prost(int64, optional, tag="3")]
    pub score: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="4")]
    pub grade: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="5")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="6")]
    pub rank_no: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub in_topic_list: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRecomForward {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub poster: ::core::option::Option<StUser>,
    #[prost(uint64, optional, tag="5")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="6")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRecomInfo {
    #[prost(string, optional, tag="1")]
    pub recom_reason: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="2")]
    pub recom_attach_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="3")]
    pub recom_trace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="4")]
    pub client_seal_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="5")]
    pub icon_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub recom_reason_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRecomReportInfo {
    #[prost(message, repeated, tag="1")]
    pub recom_infos: ::prost::alloc::vec::Vec<StSingleRecomReportInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRelationInfo {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub relation: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="4")]
    pub relation_state: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub score: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="6")]
    pub is_block: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="7")]
    pub is_blocked: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="8")]
    pub is_friend: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="9")]
    pub is_uncare: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag="10")]
    pub im_bit_map: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StReply {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub post_user: ::core::option::Option<StUser>,
    #[prost(uint64, optional, tag="3")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(string, optional, tag="4")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub target_user: ::core::option::Option<StUser>,
    #[prost(bytes="vec", optional, tag="6")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="7")]
    pub like_info: ::core::option::Option<StLike>,
    #[prost(uint32, optional, tag="8")]
    pub type_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub modifyflag: ::core::option::Option<u32>,
    #[prost(string, repeated, tag="10")]
    pub at_uin_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="11")]
    pub type_flag2: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="12")]
    pub create_time_ns: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="13")]
    pub store_ext_info: ::prost::alloc::vec::Vec<CommonEntry>,
    #[prost(string, optional, tag="14")]
    pub third_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub target_reply_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="16")]
    pub source_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="17")]
    pub rich_contents: ::core::option::Option<StRichText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StReportInfo {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="2")]
    pub busi_report: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRichText {
    #[prost(message, repeated, tag="1")]
    pub contents: ::prost::alloc::vec::Vec<StRichTextContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRichTextAtContent {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub guild_info: ::core::option::Option<GuildChannelBaseGuildInfo>,
    #[prost(message, optional, tag="3")]
    pub role_group_id: ::core::option::Option<GuildChannelBaseRoleGroupInfo>,
    #[prost(message, optional, tag="4")]
    pub user: ::core::option::Option<StUser>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildChannelBaseGuildInfo {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub join_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildChannelBaseRoleGroupInfo {
    #[prost(uint64, optional, tag="1")]
    pub role_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub color: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRichTextChannelContent {
    #[prost(message, optional, tag="1")]
    pub channel_info: ::core::option::Option<StChannelInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRichTextContent {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub pattern_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub text_content: ::core::option::Option<StRichTextTextContent>,
    #[prost(message, optional, tag="4")]
    pub at_content: ::core::option::Option<StRichTextAtContent>,
    #[prost(message, optional, tag="5")]
    pub url_content: ::core::option::Option<StRichTextUrlContent>,
    #[prost(message, optional, tag="6")]
    pub emoji_content: ::core::option::Option<StRichTextEmojiContent>,
    #[prost(message, optional, tag="7")]
    pub channel_content: ::core::option::Option<StRichTextChannelContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRichTextEmojiContent {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRichTextTextContent {
    #[prost(string, optional, tag="1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StRichTextUrlContent {
    #[prost(string, optional, tag="1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub display_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StSameTopicGuideInfo {
    #[prost(uint32, optional, tag="1")]
    pub is_same_topic_guide: ::core::option::Option<u32>,
    #[prost(int64, optional, tag="2")]
    pub stay_show_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag="3")]
    pub hash_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub words: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub jump_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub report_ext: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StShare {
    #[prost(string, optional, tag="1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="4")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub author: ::core::option::Option<StUser>,
    #[prost(message, optional, tag="6")]
    pub poster: ::core::option::Option<StUser>,
    #[prost(message, repeated, tag="7")]
    pub videos: ::prost::alloc::vec::Vec<StVideo>,
    #[prost(string, optional, tag="8")]
    pub shorturl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub share_card_info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="10")]
    pub share_qzone_info: ::core::option::Option<StShareQzoneInfo>,
    #[prost(message, repeated, tag="11")]
    pub images: ::prost::alloc::vec::Vec<StImage>,
    #[prost(uint32, optional, tag="12")]
    pub publish_total_user: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub shared_count: ::core::option::Option<u32>,
    #[prost(message, optional, tag="14")]
    pub channel_share_info: ::core::option::Option<StChannelShareInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StShareQzoneInfo {
    #[prost(message, repeated, tag="1")]
    pub entrys: ::prost::alloc::vec::Vec<CommonEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StSingleMaterial {
    #[prost(string, optional, tag="1")]
    pub material_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StSingleRecomReportInfo {
    #[prost(string, optional, tag="1")]
    pub report_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="2")]
    pub report_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StTagInfo {
    #[prost(string, optional, tag="1")]
    pub tag_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub tag_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub tag_dec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub user_list: ::prost::alloc::vec::Vec<StUser>,
    #[prost(message, repeated, tag="5")]
    pub feed_list: ::prost::alloc::vec::Vec<StFeedAbstract>,
    #[prost(uint32, optional, tag="6")]
    pub tag_total_user: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub tag_total_feed: ::core::option::Option<u32>,
    #[prost(string, optional, tag="8")]
    pub tag_wording: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="9")]
    pub tag_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub follow_state: ::core::option::Option<u32>,
    #[prost(message, optional, tag="11")]
    pub share_info: ::core::option::Option<StShare>,
    #[prost(uint32, optional, tag="12")]
    pub is_top: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub is_selected: ::core::option::Option<u32>,
    #[prost(int64, optional, tag="14")]
    pub user_view_history: ::core::option::Option<i64>,
    #[prost(message, optional, tag="15")]
    pub medal: ::core::option::Option<StTagMedalInfo>,
    #[prost(uint32, optional, tag="16")]
    pub status: ::core::option::Option<u32>,
    #[prost(message, optional, tag="17")]
    pub opt_info: ::core::option::Option<StTagOperateInfo>,
    #[prost(uint32, optional, tag="18")]
    pub tag_base_status: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="19")]
    pub is_recommend: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="20")]
    pub tag_view_history: ::core::option::Option<i64>,
    #[prost(string, optional, tag="21")]
    pub operate_icon_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="99")]
    pub tag_report: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="100")]
    pub tag_icon_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StTagMedalInfo {
    #[prost(string, optional, tag="1")]
    pub tag_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub tag_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub rank: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StTagOperateInfo {
    #[prost(string, optional, tag="1")]
    pub create_user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub cover_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub background_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub banner_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub banner_skip_link: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="7")]
    pub activity_start_time: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="8")]
    pub activity_end_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag="9")]
    pub recommend_reason: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="10")]
    pub is_white: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="11")]
    pub be_white_start_time: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="12")]
    pub be_white_end_time: ::core::option::Option<i64>,
    #[prost(string, optional, tag="13")]
    pub publish_schema: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StUnifiedTag {
    #[prost(string, optional, tag="1")]
    pub unified_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub unified_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StUser {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub nick: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub icon: ::core::option::Option<StIconInfo>,
    #[prost(string, optional, tag="4")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="5")]
    pub follow_state: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub sex: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="8")]
    pub birthday: ::core::option::Option<u64>,
    #[prost(string, optional, tag="9")]
    pub school: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub location: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="12")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="13")]
    pub frd_state: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="14")]
    pub relation_state: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub black_state: ::core::option::Option<u32>,
    #[prost(message, optional, tag="16")]
    pub medal: ::core::option::Option<StTagMedalInfo>,
    #[prost(int32, optional, tag="17")]
    pub constellation: ::core::option::Option<i32>,
    #[prost(string, optional, tag="18")]
    pub jump_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub location_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub third_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub company: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub certification_desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="23")]
    pub desc_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="24")]
    pub channel_user_info: ::core::option::Option<GuildChannelBaseChannelUserInfo>,
    #[prost(string, optional, tag="25")]
    pub login_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildChannelBaseChannelUserInfo {
    #[prost(message, optional, tag="1")]
    pub client_identity: ::core::option::Option<ClientIdentity>,
    #[prost(uint32, optional, tag="2")]
    pub member_type: ::core::option::Option<u32>,
    /// optional ChannelUserPermission permission = 3;
    #[prost(message, repeated, tag="4")]
    pub role_groups: ::prost::alloc::vec::Vec<GuildChannelBaseRoleGroupInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StUserGroupInfo {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub user_list: ::prost::alloc::vec::Vec<StUser>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StUserRecomInfo {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<StUser>,
    #[prost(message, repeated, tag="2")]
    pub feed_list: ::prost::alloc::vec::Vec<StFeedAbstract>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StVideo {
    #[prost(string, optional, tag="1")]
    pub file_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub file_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub duration: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub height: ::core::option::Option<u32>,
    #[prost(string, optional, tag="6")]
    pub play_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="7")]
    pub trans_status: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub video_prior: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub video_rate: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="10")]
    pub vec_video_url: ::prost::alloc::vec::Vec<StVideoUrl>,
    #[prost(bytes="vec", optional, tag="11")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="12")]
    pub approval_status: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub video_source: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="14")]
    pub media_quality_rank: ::core::option::Option<u32>,
    #[prost(float, optional, tag="15")]
    pub media_quality_score: ::core::option::Option<f32>,
    #[prost(string, optional, tag="16")]
    pub video_md5: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="17")]
    pub is_quic: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="18")]
    pub orientation: ::core::option::Option<u32>,
    #[prost(message, optional, tag="19")]
    pub cover: ::core::option::Option<StImage>,
    #[prost(string, optional, tag="20")]
    pub pattern_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="21")]
    pub display_index: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StVideoUrl {
    #[prost(uint32, optional, tag="1")]
    pub level_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub play_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub video_prior: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub video_rate: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub trans_status: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="7")]
    pub has_watermark: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StVisitor {
    #[prost(uint32, optional, tag="1")]
    pub view_count: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub busi_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub recom_count: ::core::option::Option<u32>,
    #[prost(string, optional, tag="4")]
    pub view_desc: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StWearingMedal {
    #[prost(message, repeated, tag="1")]
    pub medal_infos: ::prost::alloc::vec::Vec<StWearingMedalInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StWearingMedalInfo {
    #[prost(int32, optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub medal_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub medal_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppChannelMsg {
    #[prost(string, optional, tag="1")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub expire_time_ms: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub schema_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="5")]
    pub schema: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryChannelInfo {
    #[prost(uint32, optional, tag="1")]
    pub channel_index: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CategoryInfo {
    #[prost(uint32, optional, tag="1")]
    pub category_index: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="2")]
    pub channel_info: ::prost::alloc::vec::Vec<CategoryChannelInfo>,
    #[prost(bytes="vec", optional, tag="3")]
    pub category_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="4")]
    pub category_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChanInfoFilter {
    #[prost(uint32, optional, tag="2")]
    pub channel_name: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub creator_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub create_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub guild_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub msg_notify_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub speak_permission: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub last_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub last_cnt_msg_seq: ::core::option::Option<u32>,
    #[prost(message, optional, tag="14")]
    pub voice_channel_info_filter: ::core::option::Option<VoiceChannelInfoFilter>,
    #[prost(message, optional, tag="15")]
    pub live_channel_info_filter: ::core::option::Option<LiveChannelInfoFilter>,
    #[prost(uint32, optional, tag="16")]
    pub banned_speak: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeChanInfo {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub chan_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="4")]
    pub info_seq: ::core::option::Option<MsgSeq>,
    #[prost(uint32, optional, tag="5")]
    pub update_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub chan_info_filter: ::core::option::Option<ChanInfoFilter>,
    #[prost(message, optional, tag="7")]
    pub chan_info: ::core::option::Option<ServChannelInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeGuildInfo {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub info_seq: ::core::option::Option<MsgSeq>,
    #[prost(message, optional, tag="4")]
    pub face_seq: ::core::option::Option<MsgSeq>,
    #[prost(uint32, optional, tag="5")]
    pub update_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub guild_info_filter: ::core::option::Option<GuildInfoFilter>,
    #[prost(message, optional, tag="7")]
    pub guild_info: ::core::option::Option<GuildInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelId {
    #[prost(uint64, optional, tag="1")]
    pub chan_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServChannelInfo {
    #[prost(uint64, optional, tag="1")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub channel_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="3")]
    pub creator_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="6")]
    pub msg_notify_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub speak_permission: ::core::option::Option<u32>,
    #[prost(message, optional, tag="11")]
    pub last_msg_seq: ::core::option::Option<MsgSeq>,
    #[prost(message, optional, tag="12")]
    pub last_cnt_msg_seq: ::core::option::Option<MsgSeq>,
    #[prost(message, optional, tag="14")]
    pub voice_channel_info: ::core::option::Option<VoiceChannelInfo>,
    #[prost(message, optional, tag="15")]
    pub live_channel_info: ::core::option::Option<LiveChannelInfo>,
    #[prost(uint32, optional, tag="16")]
    pub banned_speak: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommGrayTips {
    #[prost(uint64, optional, tag="1")]
    pub busi_type: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub busi_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub ctrl_flag: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub templ_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="5")]
    pub templ_param: ::prost::alloc::vec::Vec<comm_gray_tips::TemplParam>,
    #[prost(bytes="vec", optional, tag="6")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="10")]
    pub tips_seq_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="100")]
    pub pb_reserv: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `CommGrayTips`.
pub mod comm_gray_tips {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TemplParam {
        #[prost(bytes="vec", optional, tag="1")]
        pub name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes="vec", optional, tag="2")]
        pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChan {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="4")]
    pub create_id: ::prost::alloc::vec::Vec<ChannelId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuild {
    #[prost(uint64, optional, tag="1")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub guild_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyChan {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="4")]
    pub delete_id: ::prost::alloc::vec::Vec<ChannelId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyGuild {
    #[prost(uint64, optional, tag="1")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub guild_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBody {
    #[prost(message, optional, tag="1")]
    pub read_notify: ::core::option::Option<ReadNotify>,
    #[prost(message, optional, tag="2")]
    pub comm_gray_tips: ::core::option::Option<CommGrayTips>,
    #[prost(message, optional, tag="3")]
    pub create_guild: ::core::option::Option<CreateGuild>,
    #[prost(message, optional, tag="4")]
    pub destroy_guild: ::core::option::Option<DestroyGuild>,
    #[prost(message, optional, tag="5")]
    pub join_guild: ::core::option::Option<JoinGuild>,
    #[prost(message, optional, tag="6")]
    pub kick_off_guild: ::core::option::Option<KickOffGuild>,
    #[prost(message, optional, tag="7")]
    pub quit_guild: ::core::option::Option<QuitGuild>,
    #[prost(message, optional, tag="8")]
    pub change_guild_info: ::core::option::Option<ChangeGuildInfo>,
    #[prost(message, optional, tag="9")]
    pub create_chan: ::core::option::Option<CreateChan>,
    #[prost(message, optional, tag="10")]
    pub destroy_chan: ::core::option::Option<DestroyChan>,
    #[prost(message, optional, tag="11")]
    pub change_chan_info: ::core::option::Option<ChangeChanInfo>,
    #[prost(message, optional, tag="12")]
    pub set_admin: ::core::option::Option<SetAdmin>,
    #[prost(message, optional, tag="13")]
    pub set_msg_recv_type: ::core::option::Option<SetMsgRecvType>,
    #[prost(message, optional, tag="14")]
    pub update_msg: ::core::option::Option<UpdateMsg>,
    #[prost(message, optional, tag="17")]
    pub set_top: ::core::option::Option<SetTop>,
    #[prost(message, optional, tag="18")]
    pub switch_channel: ::core::option::Option<SwitchVoiceChannel>,
    #[prost(message, optional, tag="21")]
    pub update_category: ::core::option::Option<UpdateCategory>,
    #[prost(message, optional, tag="22")]
    pub update_voice_block_list: ::core::option::Option<UpdateVoiceBlockList>,
    #[prost(message, optional, tag="23")]
    pub set_mute: ::core::option::Option<SetMute>,
    #[prost(message, optional, tag="24")]
    pub live_status_change_room: ::core::option::Option<LiveRoomStatusChangeMsg>,
    #[prost(message, optional, tag="25")]
    pub switch_live_room: ::core::option::Option<SwitchLiveRoom>,
    #[prost(message, repeated, tag="39")]
    pub events: ::prost::alloc::vec::Vec<MsgEvent>,
    #[prost(message, optional, tag="40")]
    pub scheduler: ::core::option::Option<SchedulerMsg>,
    #[prost(message, optional, tag="41")]
    pub app_channel: ::core::option::Option<AppChannelMsg>,
    #[prost(message, optional, tag="44")]
    pub feed_event: ::core::option::Option<FeedEvent>,
    #[prost(message, optional, tag="46")]
    pub weak_msg_app_channel: ::core::option::Option<AppChannelMsg>,
    #[prost(message, optional, tag="48")]
    pub read_feed_notify: ::core::option::Option<ReadFeedNotify>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedEvent {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="3")]
    pub feed_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub msg_summary: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="5")]
    pub event_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFeedNotify {
    #[prost(uint64, optional, tag="2")]
    pub report_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupProStatus {
    #[prost(uint32, optional, tag="1")]
    pub is_enable: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub is_banned: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub is_frozen: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildInfo {
    #[prost(uint64, optional, tag="2")]
    pub guild_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub owner_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="5")]
    pub member_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub member_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub guild_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub guild_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, repeated, packed="false", tag="9")]
    pub robot_list: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, packed="false", tag="10")]
    pub admin_list: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, optional, tag="11")]
    pub robot_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub admin_max_num: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="13")]
    pub profile: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="14")]
    pub face_seq: ::core::option::Option<u64>,
    #[prost(message, optional, tag="15")]
    pub guild_status: ::core::option::Option<GroupProStatus>,
    #[prost(uint32, optional, tag="16")]
    pub channel_num: ::core::option::Option<u32>,
    #[prost(message, optional, tag="5002")]
    pub member_change_seq: ::core::option::Option<MsgSeq>,
    #[prost(message, optional, tag="5003")]
    pub guild_info_change_seq: ::core::option::Option<MsgSeq>,
    #[prost(message, optional, tag="5004")]
    pub channel_change_seq: ::core::option::Option<MsgSeq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildInfoFilter {
    #[prost(uint32, optional, tag="2")]
    pub guild_code: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub owner_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub create_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub member_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub member_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub guild_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub guild_name: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub robot_list: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub admin_list: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub robot_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub admin_max_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub profile: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="14")]
    pub face_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub guild_status: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub channel_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5002")]
    pub member_change_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5003")]
    pub guild_info_change_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5004")]
    pub channel_change_seq: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinGuild {
    #[prost(uint64, optional, tag="3")]
    pub member_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub member_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="5")]
    pub member_tinyid: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KickOffGuild {
    #[prost(uint64, optional, tag="3")]
    pub member_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub set_black: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="5")]
    pub member_tinyid: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveChannelInfo {
    #[prost(uint64, optional, tag="1")]
    pub room_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub anchor_uin: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveChannelInfoFilter {
    #[prost(uint32, optional, tag="1")]
    pub is_need_room_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub is_need_anchor_uin: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub is_need_name: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveRoomStatusChangeMsg {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub room_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub anchor_tinyid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="5")]
    pub action: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEvent {
    #[prost(uint64, optional, tag="1")]
    pub seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub event_type: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub event_version: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSeq {
    #[prost(uint64, optional, tag="1")]
    pub seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuitGuild {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadNotify {
    #[prost(uint64, optional, tag="1")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub read_msg_seq: ::core::option::Option<MsgSeq>,
    #[prost(message, optional, tag="4")]
    pub read_cnt_msg_seq: ::core::option::Option<MsgSeq>,
    #[prost(bytes="vec", optional, tag="5")]
    pub read_msg_meta: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulerMsg {
    #[prost(bytes="vec", optional, tag="1")]
    pub creator_head_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="2")]
    pub wording: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub expire_time_ms: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAdmin {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub chan_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub admin_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub admin_tinyid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="6")]
    pub operate_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMsgRecvType {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub chan_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub operator_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub msg_notify_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMute {
    #[prost(uint32, optional, tag="1")]
    pub action: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub tiny_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTop {
    #[prost(uint32, optional, tag="1")]
    pub action: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchDetail {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub platform: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchLiveRoom {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    /// optional uint64 roomId = 3;
    /// optional uint64 tinyid = 4;
    #[prost(message, optional, tag="3")]
    pub user_info: ::core::option::Option<SwitchLiveRoomUserInfo>,
    /// JOIN = 1 QUIT = 2
    #[prost(uint32, optional, tag="4")]
    pub action: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchLiveRoomUserInfo {
    #[prost(uint64, optional, tag="1")]
    pub tiny_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchVoiceChannel {
    #[prost(uint64, optional, tag="1")]
    pub member_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag="2")]
    pub enter_detail: ::core::option::Option<SwitchDetail>,
    #[prost(message, optional, tag="3")]
    pub leave_detail: ::core::option::Option<SwitchDetail>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCategory {
    #[prost(message, repeated, tag="1")]
    pub category_info: ::prost::alloc::vec::Vec<CategoryInfo>,
    #[prost(message, optional, tag="2")]
    pub no_classify_category_info: ::core::option::Option<CategoryInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMsg {
    #[prost(uint64, optional, tag="1")]
    pub msg_seq: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="2")]
    pub orig_msg_uncountable: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag="3")]
    pub event_type: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub event_version: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub operator_tinyid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub operator_role: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="7")]
    pub reason: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="8")]
    pub timestamp: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVoiceBlockList {
    #[prost(uint32, optional, tag="1")]
    pub action: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub object_tinyid: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceChannelInfo {
    #[prost(uint32, optional, tag="1")]
    pub member_max_num: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceChannelInfoFilter {
    #[prost(uint32, optional, tag="1")]
    pub member_max_num: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsg {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub result: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub rsp_begin_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub rsp_end_seq: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="6")]
    pub msgs: ::prost::alloc::vec::Vec<ChannelMsgContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgReq {
    #[prost(message, optional, tag="1")]
    pub channel_param: ::core::option::Option<ChannelParam>,
    #[prost(uint32, optional, tag="2")]
    pub with_version_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub direct_message_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMsgRsp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="3")]
    pub channel_msg: ::core::option::Option<ChannelMsg>,
    #[prost(uint32, optional, tag="4")]
    pub with_version_flag: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="5")]
    pub get_msg_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelNode {
    #[prost(uint64, optional, tag="1")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub cnt_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub member_read_msg_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub member_read_cnt_seq: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="7")]
    pub notify_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub channel_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="9")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub meta: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub read_msg_meta: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="12")]
    pub event_time: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelParam {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub begin_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub end_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub time: ::core::option::Option<u64>,
    #[prost(uint64, repeated, packed="false", tag="6")]
    pub version: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag="7")]
    pub seqs: ::prost::alloc::vec::Vec<MsgCond>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectMessageSource {
    #[prost(uint64, optional, tag="1")]
    pub tiny_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="3")]
    pub guild_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub member_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub nick_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirstViewMsg {
    #[prost(uint32, optional, tag="1")]
    pub push_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub seq: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="3")]
    pub guild_nodes: ::prost::alloc::vec::Vec<GuildNode>,
    #[prost(message, repeated, tag="4")]
    pub channel_msgs: ::prost::alloc::vec::Vec<ChannelMsg>,
    #[prost(uint64, optional, tag="5")]
    pub get_msg_time: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="6")]
    pub direct_message_guild_nodes: ::prost::alloc::vec::Vec<GuildNode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirstViewReq {
    #[prost(uint64, optional, tag="1")]
    pub last_msg_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub udc_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub direct_message_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FirstViewRsp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub udc_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub guild_count: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="6")]
    pub self_tinyid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="7")]
    pub direct_message_switch: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub direct_message_guild_count: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildNode {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub guild_code: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="3")]
    pub channel_nodes: ::prost::alloc::vec::Vec<ChannelNode>,
    #[prost(bytes="vec", optional, tag="4")]
    pub guild_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="5")]
    pub peer_source: ::core::option::Option<DirectMessageSource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCond {
    #[prost(uint64, optional, tag="1")]
    pub seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub event_version: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiChannelMsg {
    #[prost(uint32, optional, tag="1")]
    pub push_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub seq: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="3")]
    pub channel_msgs: ::prost::alloc::vec::Vec<ChannelMsg>,
    #[prost(uint64, optional, tag="4")]
    pub get_msg_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiChannelMsgReq {
    #[prost(message, repeated, tag="1")]
    pub channel_params: ::prost::alloc::vec::Vec<ChannelParam>,
    #[prost(uint32, optional, tag="2")]
    pub seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub direct_message_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiChannelMsgRsp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub seq: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqBody {
    #[prost(message, optional, tag="1")]
    pub channel_param: ::core::option::Option<ChannelParam>,
    #[prost(uint32, optional, tag="2")]
    pub direct_message_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspBody {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="3")]
    pub channel_msg: ::core::option::Option<ChannelMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StAlterFeedReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_req_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="4")]
    pub m_bitmap: ::core::option::Option<u64>,
    #[prost(int32, optional, tag="5")]
    pub from: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub src: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="7")]
    pub alter_feed_ext_info: ::prost::alloc::vec::Vec<CommonEntry>,
    #[prost(string, optional, tag="8")]
    pub json_feed: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="9")]
    pub client_content: ::core::option::Option<StClientContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StAlterFeedRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_rsp_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StClientContent {
    #[prost(message, repeated, tag="1")]
    pub client_image_contents: ::prost::alloc::vec::Vec<StClientImageContent>,
    #[prost(message, repeated, tag="2")]
    pub client_video_contents: ::prost::alloc::vec::Vec<StClientVideoContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StClientImageContent {
    #[prost(string, optional, tag="1")]
    pub task_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub pic_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StClientVideoContent {
    #[prost(string, optional, tag="1")]
    pub task_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub video_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub video_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub cover_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDelFeedReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(int32, optional, tag="3")]
    pub from: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub src: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDelFeedRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoCommentReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(uint32, optional, tag="2")]
    pub comment_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub comment: ::core::option::Option<StComment>,
    #[prost(message, optional, tag="4")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(int32, optional, tag="5")]
    pub from: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub busi_req_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="7")]
    pub src: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoCommentRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub comment: ::core::option::Option<StComment>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_rsp_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoLikeReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(uint32, optional, tag="2")]
    pub like_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub like: ::core::option::Option<StLike>,
    #[prost(message, optional, tag="4")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(bytes="vec", optional, tag="5")]
    pub busi_req_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="6")]
    pub comment: ::core::option::Option<StComment>,
    #[prost(message, optional, tag="7")]
    pub reply: ::core::option::Option<StReply>,
    #[prost(int32, optional, tag="8")]
    pub from: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub src: ::core::option::Option<i32>,
    #[prost(message, optional, tag="10")]
    pub emotion_reaction: ::core::option::Option<StEmotionReactionInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoLikeRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub like: ::core::option::Option<StLike>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_rsp_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="4")]
    pub emotion_reaction: ::core::option::Option<StEmotionReactionInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoReplyReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(uint32, optional, tag="2")]
    pub reply_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub reply: ::core::option::Option<StReply>,
    #[prost(message, optional, tag="4")]
    pub comment: ::core::option::Option<StComment>,
    #[prost(message, optional, tag="5")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(int32, optional, tag="6")]
    pub from: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub busi_req_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="8")]
    pub src: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoReplyRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub reply: ::core::option::Option<StReply>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_rsp_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoSecurityReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(message, optional, tag="3")]
    pub comment: ::core::option::Option<StComment>,
    #[prost(message, optional, tag="4")]
    pub reply: ::core::option::Option<StReply>,
    #[prost(message, optional, tag="5")]
    pub poster: ::core::option::Option<StUser>,
    #[prost(int32, optional, tag="6")]
    pub sec_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StDoSecurityRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StModifyFeedReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(uint64, optional, tag="3")]
    pub m_bitmap: ::core::option::Option<u64>,
    #[prost(int32, optional, tag="4")]
    pub from: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub src: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="6")]
    pub modify_feed_ext_info: ::prost::alloc::vec::Vec<CommonEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StModifyFeedRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_rsp_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StPublishFeedReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_req_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="4")]
    pub from: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub src: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="6")]
    pub store_feed_ext_info: ::prost::alloc::vec::Vec<CommonEntry>,
    #[prost(string, optional, tag="7")]
    pub json_feed: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub client_content: ::core::option::Option<StClientContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StPublishFeedRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(bytes="vec", optional, tag="3")]
    pub busi_rsp_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FocusInfo {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub channel_id_list: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOnlinePush {
    #[prost(message, repeated, tag="1")]
    pub msgs: ::prost::alloc::vec::Vec<ChannelMsgContent>,
    #[prost(uint32, optional, tag="2")]
    pub general_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub need_resp: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub server_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="5")]
    pub compress_flag: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub compress_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="7")]
    pub focus_info: ::core::option::Option<FocusInfo>,
    #[prost(uint32, optional, tag="8")]
    pub huge_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPushResp {
    #[prost(bytes="vec", optional, tag="1")]
    pub server_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PressMsg {
    #[prost(message, repeated, tag="1")]
    pub msgs: ::prost::alloc::vec::Vec<ChannelMsgContent>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBuf {
    #[prost(uint32, optional, tag="1")]
    pub svr_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub svr_port: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub echo_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNoticesReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(uint32, optional, tag="2")]
    pub page_num: ::core::option::Option<u32>,
    #[prost(string, optional, tag="3")]
    pub attach_info: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNoticesRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, repeated, tag="2")]
    pub notices: ::prost::alloc::vec::Vec<StNotice>,
    #[prost(uint32, optional, tag="3")]
    pub total_num: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="4")]
    pub is_finish: ::core::option::Option<bool>,
    #[prost(string, optional, tag="5")]
    pub attach_info: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedInsertCommentInfo {
    #[prost(string, optional, tag="1")]
    pub comment_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshToast {
    #[prost(string, optional, tag="1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetChannelFeedsReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(uint32, optional, tag="2")]
    pub count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub from: ::core::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub channel_sign: ::core::option::Option<StChannelSign>,
    #[prost(string, optional, tag="5")]
    pub feed_attch_info: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetChannelFeedsRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, repeated, tag="2")]
    pub vec_feed: ::prost::alloc::vec::Vec<StFeed>,
    #[prost(uint32, optional, tag="3")]
    pub is_finish: ::core::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub user: ::core::option::Option<StUser>,
    #[prost(string, optional, tag="5")]
    pub feed_attch_info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub refresh_toast: ::core::option::Option<RefreshToast>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetChannelShareFeedReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(uint32, optional, tag="2")]
    pub from: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub channel_share_info: ::core::option::Option<StChannelShareInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetChannelShareFeedRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetFeedCommentsReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(string, optional, tag="2")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub feed_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="4")]
    pub list_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub from: ::core::option::Option<u32>,
    #[prost(string, optional, tag="6")]
    pub attch_info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub entry_schema: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetFeedCommentsRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, repeated, tag="2")]
    pub vec_comment: ::prost::alloc::vec::Vec<StComment>,
    #[prost(uint32, optional, tag="3")]
    pub total_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub is_finish: ::core::option::Option<u32>,
    #[prost(string, optional, tag="5")]
    pub attch_info: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetFeedDetailReq {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(uint32, optional, tag="2")]
    pub from: ::core::option::Option<u32>,
    #[prost(string, optional, tag="3")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub feed_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="5")]
    pub create_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="6")]
    pub detail_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="7")]
    pub channel_sign: ::core::option::Option<StChannelSign>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StGetFeedDetailRsp {
    #[prost(message, optional, tag="1")]
    pub ext_info: ::core::option::Option<StCommonExt>,
    #[prost(message, optional, tag="2")]
    pub feed: ::core::option::Option<StFeed>,
    #[prost(message, optional, tag="3")]
    pub login_user: ::core::option::Option<StUser>,
}
