#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternMsg {
    #[prost(int32, tag="1")]
    pub channel_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiMsgApplyDownReq {
    #[prost(bytes="vec", tag="1")]
    pub msg_resid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub msg_type: i32,
    #[prost(int64, tag="3")]
    pub src_uin: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiMsgApplyDownRsp {
    #[prost(int32, tag="1")]
    pub result: i32,
    #[prost(bytes="vec", tag="2")]
    pub thumb_down_para: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub msg_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, repeated, tag="4")]
    pub down_ip: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag="5")]
    pub down_port: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", tag="6")]
    pub msg_resid: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub msg_extern_info: ::core::option::Option<ExternMsg>,
    #[prost(bytes="vec", repeated, tag="8")]
    pub bytes_down_ip_v6: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, repeated, tag="9")]
    pub uint32_down_v6_port: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiMsgApplyUpReq {
    #[prost(int64, tag="1")]
    pub dst_uin: i64,
    #[prost(int64, tag="2")]
    pub msg_size: i64,
    #[prost(bytes="vec", tag="3")]
    pub msg_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="4")]
    pub msg_type: i32,
    #[prost(int32, tag="5")]
    pub apply_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiMsgApplyUpRsp {
    #[prost(int32, tag="1")]
    pub result: i32,
    #[prost(string, tag="2")]
    pub msg_resid: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub msg_ukey: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, repeated, tag="4")]
    pub uint32_up_ip: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="5")]
    pub uint32_up_port: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, tag="6")]
    pub block_size: i64,
    #[prost(int64, tag="7")]
    pub up_offset: i64,
    #[prost(int32, tag="8")]
    pub apply_id: i32,
    #[prost(bytes="vec", tag="9")]
    pub msg_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub msg_sig: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="11")]
    pub msg_extern_info: ::core::option::Option<ExternMsg>,
    #[prost(bytes="vec", repeated, tag="12")]
    pub bytes_up_ip_v6: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, repeated, tag="13")]
    pub uint32_up_v6_port: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiReqBody {
    #[prost(int32, tag="1")]
    pub subcmd: i32,
    #[prost(int32, tag="2")]
    pub term_type: i32,
    #[prost(int32, tag="3")]
    pub platform_type: i32,
    #[prost(int32, tag="4")]
    pub net_type: i32,
    #[prost(string, tag="5")]
    pub build_ver: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub multimsg_applyup_req: ::prost::alloc::vec::Vec<MultiMsgApplyUpReq>,
    #[prost(message, repeated, tag="7")]
    pub multimsg_applydown_req: ::prost::alloc::vec::Vec<MultiMsgApplyDownReq>,
    #[prost(int32, tag="8")]
    pub bu_type: i32,
    #[prost(int32, tag="9")]
    pub req_channel_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiRspBody {
    #[prost(int32, tag="1")]
    pub subcmd: i32,
    #[prost(message, repeated, tag="2")]
    pub multimsg_applyup_rsp: ::prost::alloc::vec::Vec<MultiMsgApplyUpRsp>,
    #[prost(message, repeated, tag="3")]
    pub multimsg_applydown_rsp: ::prost::alloc::vec::Vec<MultiMsgApplyDownRsp>,
}
