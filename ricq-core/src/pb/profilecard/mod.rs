#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateCommTaskInfo {
    #[prost(int32, optional, tag="1")]
    pub appid: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub task_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateGetGiftListReq {
    #[prost(int32, optional, tag="1")]
    pub uin: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateGetGiftListRsp {
    #[prost(string, repeated, tag="1")]
    pub gift_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub custom_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="4")]
    pub is_on: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateGetVipCareReq {
    #[prost(int64, optional, tag="1")]
    pub uin: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateGetVipCareRsp {
    #[prost(int32, optional, tag="1")]
    pub buss: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub notice: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateOidbFlagInfo {
    #[prost(int32, optional, tag="1")]
    pub fieled: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub byets_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatePrivilegeBaseInfoReq {
    #[prost(int64, optional, tag="1")]
    pub u_req_uin: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatePrivilegeBaseInfoRsp {
    #[prost(bytes="vec", optional, tag="1")]
    pub msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub jump_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="3")]
    pub v_open_priv: ::prost::alloc::vec::Vec<GatePrivilegeInfo>,
    #[prost(message, repeated, tag="4")]
    pub v_close_priv: ::prost::alloc::vec::Vec<GatePrivilegeInfo>,
    #[prost(int32, optional, tag="5")]
    pub u_is_gray_usr: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatePrivilegeInfo {
    #[prost(int32, optional, tag="1")]
    pub i_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub i_sort: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub i_fee_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub i_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub i_flag: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub icon_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub deluxe_icon_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub jump_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="9")]
    pub i_is_big: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateVaProfileGateReq {
    #[prost(int32, optional, tag="1")]
    pub u_cmd: ::core::option::Option<i32>,
    #[prost(message, optional, tag="2")]
    pub st_privilege_req: ::core::option::Option<GatePrivilegeBaseInfoReq>,
    #[prost(message, optional, tag="3")]
    pub st_gift_req: ::core::option::Option<GateGetGiftListReq>,
    #[prost(message, repeated, tag="4")]
    pub task_item: ::prost::alloc::vec::Vec<GateCommTaskInfo>,
    #[prost(message, repeated, tag="5")]
    pub oidb_flag: ::prost::alloc::vec::Vec<GateOidbFlagInfo>,
    #[prost(message, optional, tag="6")]
    pub st_vip_care: ::core::option::Option<GateGetVipCareReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateQidInfoItem {
    #[prost(string, optional, tag="1")]
    pub qid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub logo_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateVaProfileGateRsp {
    #[prost(int32, optional, tag="1")]
    pub i_ret_code: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub s_ret_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="3")]
    pub st_privilege_rsp: ::core::option::Option<GatePrivilegeBaseInfoRsp>,
    #[prost(message, optional, tag="4")]
    pub st_gift_rsp: ::core::option::Option<GateGetGiftListRsp>,
    #[prost(message, repeated, tag="5")]
    pub task_item: ::prost::alloc::vec::Vec<GateCommTaskInfo>,
    #[prost(message, repeated, tag="6")]
    pub oidb_flag: ::prost::alloc::vec::Vec<GateOidbFlagInfo>,
    #[prost(message, optional, tag="7")]
    pub st_vip_care: ::core::option::Option<GateGetVipCareRsp>,
    #[prost(message, optional, tag="9")]
    pub qid_info: ::core::option::Option<GateQidInfoItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiColor {
    #[prost(int32, optional, tag="1")]
    pub r: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub g: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub b: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiComm {
    #[prost(int32, optional, tag="1")]
    pub ver: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub seq: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="3")]
    pub fromuin: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub touin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="5")]
    pub service: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub session_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub session_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="8")]
    pub client_ip: ::core::option::Option<i32>,
    #[prost(message, optional, tag="9")]
    pub display: ::core::option::Option<BusiUi>,
    #[prost(int32, optional, tag="10")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag="11")]
    pub err_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="12")]
    pub platform: ::core::option::Option<i32>,
    #[prost(string, optional, tag="13")]
    pub qqver: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="14")]
    pub build: ::core::option::Option<i32>,
    #[prost(message, optional, tag="15")]
    pub msg_login_sig: ::core::option::Option<BusiLoginSig>,
    #[prost(int32, optional, tag="17")]
    pub version: ::core::option::Option<i32>,
    #[prost(message, optional, tag="18")]
    pub msg_uin_info: ::core::option::Option<BusiUinInfo>,
    #[prost(message, optional, tag="19")]
    pub msg_rich_display: ::core::option::Option<BusiRichUi>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiCommonReq {
    #[prost(string, optional, tag="1")]
    pub service_cmd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub vc_req: ::core::option::Option<BusiVisitorCountReq>,
    #[prost(message, optional, tag="3")]
    pub hr_req: ::core::option::Option<BusiHideRecordsReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiDetailRecord {
    #[prost(int32, optional, tag="1")]
    pub fuin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub source: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub vtime: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub r#mod: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub hide_flag: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiHideRecordsReq {
    #[prost(int32, optional, tag="1")]
    pub huin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub fuin: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="3")]
    pub records: ::prost::alloc::vec::Vec<BusiDetailRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiLabel {
    #[prost(bytes="vec", optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="2")]
    pub enum_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub text_color: ::core::option::Option<BusiColor>,
    #[prost(message, optional, tag="4")]
    pub edging_color: ::core::option::Option<BusiColor>,
    #[prost(int32, optional, tag="5")]
    pub label_attr: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub label_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiLoginSig {
    #[prost(int32, optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="3")]
    pub appid: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiRichUi {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    ///repeated UiInfo uiList = 3;
    #[prost(string, optional, tag="2")]
    pub service_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiUi {
    #[prost(string, optional, tag="1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub jump_url: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiUinInfo {
    #[prost(int64, optional, tag="1")]
    pub int64_longitude: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub int64_latitude: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiVisitorCountReq {
    #[prost(int32, optional, tag="1")]
    pub requireuin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub operuin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub r#mod: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub report_flag: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusiVisitorCountRsp {
    #[prost(int32, optional, tag="1")]
    pub requireuin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub total_like: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub total_view: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub hot_value: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub red_value: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub hot_diff: ::core::option::Option<i32>,
}
