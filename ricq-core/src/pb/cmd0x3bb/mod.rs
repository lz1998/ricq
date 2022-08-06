#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnonyMsg {
    #[prost(uint32, optional, tag="1")]
    pub cmd: ::core::option::Option<u32>,
    #[prost(message, optional, tag="10")]
    pub anony_req: ::core::option::Option<C3bbReqBody>,
    #[prost(message, optional, tag="11")]
    pub anony_rsp: ::core::option::Option<C3bbRspBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnonyStatus {
    #[prost(uint32, optional, tag="1")]
    pub forbid_talking: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub err_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C3bbReqBody {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub group_code: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C3bbRspBody {
    #[prost(int32, optional, tag="1")]
    pub ret: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag="2")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="3")]
    pub anony_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="4")]
    pub portrait_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub bubble_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub expired_time: ::core::option::Option<u32>,
    #[prost(message, optional, tag="10")]
    pub anony_status: ::core::option::Option<AnonyStatus>,
    #[prost(string, optional, tag="15")]
    pub color: ::core::option::Option<::prost::alloc::string::String>,
}
