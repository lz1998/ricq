#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqBody {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub start_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub identify_flag: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed="false", tag="4")]
    pub uin_list: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag="5")]
    pub memberlist_opt: ::core::option::Option<Memberlist>,
    #[prost(uint32, optional, tag="6")]
    pub member_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub filter_method: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub online_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspBody {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub start_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub identify_flag: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="4")]
    pub memberlist: ::prost::alloc::vec::Vec<Memberlist>,
    #[prost(bytes="vec", optional, tag="5")]
    pub errorinfo: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memberlist {
    #[prost(uint64, optional, tag="1")]
    pub member_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub uin_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub uin_flagex: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub uin_mobile_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub uin_arch_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub join_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub old_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub new_msg_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub last_speak_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub point: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub shutup_timestap: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub flagex2: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="14")]
    pub special_title: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="15")]
    pub special_title_expire_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub active_day: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="17")]
    pub uin_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="18")]
    pub privilege: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="19")]
    pub rich_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UinKey {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub member_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub gen_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub valid_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub rand_num: ::core::option::Option<u32>,
}
