#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongMsgDeleteReq {
    #[prost(bytes="vec", tag="1")]
    pub msg_resid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub msg_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongMsgDeleteRsp {
    #[prost(int32, tag="1")]
    pub result: i32,
    #[prost(bytes="vec", tag="2")]
    pub msg_resid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongMsgDownReq {
    #[prost(int32, tag="1")]
    pub src_uin: i32,
    #[prost(bytes="vec", tag="2")]
    pub msg_resid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="3")]
    pub msg_type: i32,
    #[prost(int32, tag="4")]
    pub need_cache: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongMsgDownRsp {
    #[prost(int32, tag="1")]
    pub result: i32,
    #[prost(bytes="vec", tag="2")]
    pub msg_resid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub msg_content: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongMsgUpReq {
    #[prost(int32, tag="1")]
    pub msg_type: i32,
    #[prost(int64, tag="2")]
    pub dst_uin: i64,
    #[prost(int32, tag="3")]
    pub msg_id: i32,
    #[prost(bytes="vec", tag="4")]
    pub msg_content: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="5")]
    pub store_type: i32,
    #[prost(bytes="vec", tag="6")]
    pub msg_ukey: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="7")]
    pub need_cache: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongMsgUpRsp {
    #[prost(int32, tag="1")]
    pub result: i32,
    #[prost(int32, tag="2")]
    pub msg_id: i32,
    #[prost(bytes="vec", tag="3")]
    pub msg_resid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongReqBody {
    #[prost(int32, tag="1")]
    pub subcmd: i32,
    #[prost(int32, tag="2")]
    pub term_type: i32,
    #[prost(int32, tag="3")]
    pub platform_type: i32,
    #[prost(message, repeated, tag="4")]
    pub msg_up_req: ::prost::alloc::vec::Vec<LongMsgUpReq>,
    #[prost(message, repeated, tag="5")]
    pub msg_down_req: ::prost::alloc::vec::Vec<LongMsgDownReq>,
    #[prost(message, repeated, tag="6")]
    pub msg_del_req: ::prost::alloc::vec::Vec<LongMsgDeleteReq>,
    #[prost(int32, tag="10")]
    pub agent_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongRspBody {
    #[prost(int32, tag="1")]
    pub subcmd: i32,
    #[prost(message, repeated, tag="2")]
    pub msg_up_rsp: ::prost::alloc::vec::Vec<LongMsgUpRsp>,
    #[prost(message, repeated, tag="3")]
    pub msg_down_rsp: ::prost::alloc::vec::Vec<LongMsgDownRsp>,
    #[prost(message, repeated, tag="4")]
    pub msg_del_rsp: ::prost::alloc::vec::Vec<LongMsgDeleteRsp>,
}
