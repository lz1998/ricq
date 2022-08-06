#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscussList {
    #[prost(uint64, optional, tag="1")]
    pub discuss_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub discuss_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub member_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub info_seq: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="5")]
    pub b_hot_group: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag="6")]
    pub redpack_time: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="7")]
    pub has_msg: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="8")]
    pub dicuss_flag: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupList {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub group_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub member_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub mask: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub redpack_time: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="6")]
    pub has_msg: ::core::option::Option<bool>,
    #[prost(int64, optional, tag="7")]
    pub group_flag: ::core::option::Option<i64>,
    #[prost(uint64, optional, tag="8")]
    pub group_type: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="9")]
    pub group_name_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub group_member_seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub uin_flag_ex2: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub important_msg_latest_seq: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SvcPbResponsePullDisMsgProxy {
    #[prost(uint64, optional, tag="1")]
    pub member_seq: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SvcRegisterProxyMsgResp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub seq: ::core::option::Option<u32>,
    #[prost(message, optional, tag="5")]
    pub info: ::core::option::Option<SvcResponseMsgInfo>,
    #[prost(message, repeated, tag="6")]
    pub group_list: ::prost::alloc::vec::Vec<GroupList>,
    #[prost(message, repeated, tag="7")]
    pub discuss_list: ::prost::alloc::vec::Vec<DiscussList>,
    #[prost(message, repeated, tag="8")]
    pub group_msg: ::prost::alloc::vec::Vec<SvcResponsePbPullGroupMsgProxy>,
    #[prost(message, repeated, tag="9")]
    pub discuss_msg: ::prost::alloc::vec::Vec<SvcPbResponsePullDisMsgProxy>,
    #[prost(bytes="vec", optional, tag="10")]
    pub c2_c_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub pub_account_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="12")]
    pub discuss_list_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SvcResponseMsgInfo {
    #[prost(uint32, optional, tag="1")]
    pub group_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub discuss_num: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SvcResponsePbPullGroupMsgProxy {
    #[prost(uint64, optional, tag="1")]
    pub member_seq: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
