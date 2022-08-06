#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyMsgBody {
    #[prost(message, optional, tag="5")]
    pub opt_msg_gray_tips: ::core::option::Option<AioGrayTipsInfo>,
    #[prost(message, optional, tag="9")]
    pub opt_msg_red_tips: ::core::option::Option<RedGrayTipsInfo>,
    #[prost(message, optional, tag="11")]
    pub opt_msg_recall: ::core::option::Option<MessageRecallReminder>,
    #[prost(message, optional, tag="26")]
    pub opt_general_gray_tip: ::core::option::Option<GeneralGrayTipInfo>,
    #[prost(message, optional, tag="33")]
    pub qq_group_digest_msg: ::core::option::Option<QqGroupDigestMsg>,
    #[prost(int32, tag="13")]
    pub service_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AioGrayTipsInfo {
    #[prost(uint32, tag="1")]
    pub show_latest: u32,
    #[prost(bytes="vec", tag="2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub remind: u32,
    #[prost(bytes="vec", tag="4")]
    pub brief: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="5")]
    pub receiver_uin: u64,
    #[prost(uint32, tag="6")]
    pub reliao_admin_opt: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralGrayTipInfo {
    #[prost(uint64, tag="1")]
    pub busi_type: u64,
    #[prost(uint64, tag="2")]
    pub busi_id: u64,
    #[prost(uint32, tag="3")]
    pub ctrl_flag: u32,
    #[prost(uint32, tag="4")]
    pub c2c_type: u32,
    #[prost(uint32, tag="5")]
    pub service_type: u32,
    #[prost(uint64, tag="6")]
    pub templ_id: u64,
    #[prost(message, repeated, tag="7")]
    pub msg_templ_param: ::prost::alloc::vec::Vec<TemplParam>,
    #[prost(string, tag="8")]
    pub content: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplParam {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRecallReminder {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(bytes="vec", tag="2")]
    pub nickname: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub recalled_msg_list: ::prost::alloc::vec::Vec<RecalledMessageMeta>,
    #[prost(bytes="vec", tag="4")]
    pub reminder_content: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub userdef: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="6")]
    pub group_type: i32,
    #[prost(int32, tag="7")]
    pub op_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecalledMessageMeta {
    #[prost(int32, tag="1")]
    pub seq: i32,
    #[prost(int32, tag="2")]
    pub time: i32,
    #[prost(int32, tag="3")]
    pub msg_random: i32,
    #[prost(int32, tag="4")]
    pub msg_type: i32,
    #[prost(int32, tag="5")]
    pub msg_flag: i32,
    #[prost(int64, tag="6")]
    pub author_uin: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedGrayTipsInfo {
    #[prost(uint32, tag="1")]
    pub show_latest: u32,
    #[prost(uint64, tag="2")]
    pub sender_uin: u64,
    #[prost(uint64, tag="3")]
    pub receiver_uin: u64,
    #[prost(string, tag="4")]
    pub sender_rich_content: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub receiver_rich_content: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub auth_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(sint32, tag="7")]
    pub msg_type: i32,
    #[prost(uint32, tag="8")]
    pub lucky_flag: u32,
    #[prost(uint32, tag="9")]
    pub hide_flag: u32,
    #[prost(uint64, tag="12")]
    pub lucky_uin: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QqGroupDigestMsg {
    #[prost(uint64, tag="1")]
    pub group_code: u64,
    #[prost(uint32, tag="2")]
    pub seq: u32,
    #[prost(uint32, tag="3")]
    pub random: u32,
    #[prost(int32, tag="4")]
    pub op_type: i32,
    #[prost(uint64, tag="5")]
    pub sender: u64,
    #[prost(uint64, tag="6")]
    pub digest_oper: u64,
    #[prost(uint32, tag="7")]
    pub op_time: u32,
    #[prost(uint32, tag="8")]
    pub lastest_msg_seq: u32,
    #[prost(bytes="vec", tag="9")]
    pub oper_nick: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub sender_nick: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="11")]
    pub ext_info: i32,
}
