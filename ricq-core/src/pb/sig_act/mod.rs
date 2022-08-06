#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Platform {
    #[prost(int64, optional, tag="1")]
    pub platform: ::core::option::Option<i64>,
    #[prost(string, optional, tag="2")]
    pub osver: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub mqqver: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqBody {
    #[prost(uint32, optional, tag="1")]
    pub cmd: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub seq: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub plf: ::core::option::Option<Platform>,
    #[prost(message, optional, tag="4")]
    pub req: ::core::option::Option<SigactReq>,
    #[prost(message, optional, tag="5")]
    pub auth_req: ::core::option::Option<SigauthReq>,
    #[prost(uint32, optional, tag="6")]
    pub source: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspBody {
    #[prost(int32, optional, tag="1")]
    pub ret: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub cmd: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub seq: ::core::option::Option<u64>,
    #[prost(message, optional, tag="5")]
    pub rsp: ::core::option::Option<SigactRsp>,
    #[prost(message, optional, tag="6")]
    pub auth_rsp: ::core::option::Option<SigauthRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigactReq {
    #[prost(uint64, optional, tag="1")]
    pub uin_disable: ::core::option::Option<u64>,
    #[prost(int32, optional, tag="2")]
    pub actid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub acttype: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigactRsp {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub rank: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigauthReq {
    #[prost(uint64, optional, tag="1")]
    pub uin_disable: ::core::option::Option<u64>,
    #[prost(int32, optional, tag="2")]
    pub itemid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub len: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="5")]
    pub fontid: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigauthRsp {
    #[prost(bytes="vec", optional, tag="1")]
    pub result: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub tips_info: ::core::option::Option<sigauth_rsp::TipsInfo>,
    #[prost(int32, optional, tag="4")]
    pub authfailed_appid: ::core::option::Option<i32>,
}
/// Nested message and enum types in `SigauthRsp`.
pub mod sigauth_rsp {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TipsInfo {
        #[prost(bool, optional, tag="1")]
        pub valid: ::core::option::Option<bool>,
        #[prost(int32, optional, tag="2")]
        pub ret: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag="3")]
        pub r#type: ::core::option::Option<u32>,
        #[prost(string, optional, tag="4")]
        pub title_wording: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="5")]
        pub wording: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="6")]
        pub right_btn_wording: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="7")]
        pub left_btn_wording: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag="8")]
        pub vip_type: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag="9")]
        pub vip_month: ::core::option::Option<u32>,
        #[prost(string, optional, tag="10")]
        pub url: ::core::option::Option<::prost::alloc::string::String>,
    }
}
