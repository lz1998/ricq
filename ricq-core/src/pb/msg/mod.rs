#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtChannelInfo {
    #[prost(uint64, optional, tag="1")]
    pub guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub channel_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextResvAttr {
    #[prost(bytes="vec", optional, tag="1")]
    pub wording: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="2")]
    pub text_analysis_result: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub at_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub at_member_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub at_member_tinyid: ::core::option::Option<u64>,
    #[prost(message, optional, tag="6")]
    pub at_member_role_info: ::core::option::Option<ExtRoleInfo>,
    #[prost(message, optional, tag="7")]
    pub at_role_info: ::core::option::Option<ExtRoleInfo>,
    #[prost(message, optional, tag="8")]
    pub at_channel_info: ::core::option::Option<ExtChannelInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtRoleInfo {
    #[prost(uint64, optional, tag="1")]
    pub id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMsgReadedReportReq {
    #[prost(message, repeated, tag="1")]
    pub grp_read_report: ::prost::alloc::vec::Vec<PbGroupReadedReportReq>,
    #[prost(message, repeated, tag="2")]
    pub dis_read_report: ::prost::alloc::vec::Vec<PbDiscussReadedReportReq>,
    ///optional PbBindUinMsgReadedConfirmReq bindUinReadReport = 4;
    #[prost(message, optional, tag="3")]
    pub c2_c_read_report: ::core::option::Option<PbC2cReadedReportReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMsgReadedReportResp {
    #[prost(message, repeated, tag="1")]
    pub grp_read_report: ::prost::alloc::vec::Vec<PbGroupReadedReportResp>,
    #[prost(message, repeated, tag="2")]
    pub dis_read_report: ::prost::alloc::vec::Vec<PbDiscussReadedReportResp>,
    ///optional PbBindUinMsgReadedConfirmResp bindUinReadReport = 4;
    #[prost(message, optional, tag="3")]
    pub c2_c_read_report: ::core::option::Option<PbC2cReadedReportResp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbGroupReadedReportReq {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub last_read_seq: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDiscussReadedReportReq {
    #[prost(uint64, optional, tag="1")]
    pub conf_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub last_read_seq: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbC2cReadedReportReq {
    #[prost(bytes="vec", optional, tag="1")]
    pub sync_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="2")]
    pub pair_info: ::prost::alloc::vec::Vec<UinPairReadInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UinPairReadInfo {
    #[prost(uint64, optional, tag="1")]
    pub peer_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub last_read_time: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub crm_sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="4")]
    pub peer_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub chat_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="6")]
    pub cpid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="7")]
    pub aio_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="9")]
    pub to_tiny_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbGroupReadedReportResp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub errmsg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub member_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub group_msg_seq: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbDiscussReadedReportResp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub errmsg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub conf_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub member_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub conf_seq: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbC2cReadedReportResp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub errmsg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="3")]
    pub sync_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPic {
    #[prost(bytes="vec", tag="1")]
    pub small_pic_url: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub original_pic_url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="3")]
    pub local_pic_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjMsg {
    #[prost(int32, tag="1")]
    pub msg_type: i32,
    #[prost(bytes="vec", tag="2")]
    pub title: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub bytes_abstact: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub title_ext: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="6")]
    pub msg_pic: ::prost::alloc::vec::Vec<MsgPic>,
    #[prost(message, repeated, tag="7")]
    pub msg_content_info: ::prost::alloc::vec::Vec<MsgContentInfo>,
    #[prost(int32, tag="8")]
    pub report_id_show: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgContentInfo {
    #[prost(bytes="vec", tag="1")]
    pub content_info_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub msg_file: ::core::option::Option<MsgFile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFile {
    #[prost(int32, tag="1")]
    pub bus_id: i32,
    #[prost(bytes="vec", tag="2")]
    pub file_path: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub file_size: i64,
    #[prost(string, tag="4")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub int64_dead_time: i64,
    #[prost(bytes="vec", tag="6")]
    pub file_sha1: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub ext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2cHead {
    #[prost(uint64, optional, tag="1")]
    pub to_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub cc_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub cc_cmd: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="5")]
    pub auth_pic_sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="6")]
    pub auth_sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub auth_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="8")]
    pub server_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub client_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub rand: ::core::option::Option<u32>,
    #[prost(string, optional, tag="11")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsHead {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub command: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub retry_times: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub client_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub pubno: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub localid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub timezone: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="10")]
    pub client_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub client_port: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="12")]
    pub conn_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub conn_port: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="14")]
    pub interface_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub interface_port: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="16")]
    pub actual_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="17")]
    pub flag: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="18")]
    pub timestamp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="19")]
    pub subcmd: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="20")]
    pub result: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="21")]
    pub app_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="22")]
    pub instance_id: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="23")]
    pub session_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="24")]
    pub idc_id: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaHead {
    #[prost(uint64, optional, tag="1")]
    pub total_len: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub offset: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub ack_offset: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub ack_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="6")]
    pub result: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub flags: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImHead {
    #[prost(uint32, optional, tag="1")]
    pub head_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub cs_head: ::core::option::Option<CsHead>,
    #[prost(message, optional, tag="3")]
    pub s2_c_head: ::core::option::Option<S2cHead>,
    #[prost(message, optional, tag="4")]
    pub httpconn_head: ::core::option::Option<HttpConnHead>,
    #[prost(uint32, optional, tag="5")]
    pub paint_flag: ::core::option::Option<u32>,
    #[prost(message, optional, tag="6")]
    pub login_sig: ::core::option::Option<LoginSig>,
    #[prost(message, optional, tag="7")]
    pub delta_head: ::core::option::Option<DeltaHead>,
    #[prost(message, optional, tag="8")]
    pub c2_c_head: ::core::option::Option<C2cHead>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpConnHead {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub command: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub sub_command: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub retry_times: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub client_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub pub_no: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub local_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub time_zone: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="11")]
    pub client_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub client_port: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="13")]
    pub qzhttp_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="14")]
    pub qzhttp_port: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="15")]
    pub spp_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub spp_port: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="17")]
    pub flag: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="18")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="19")]
    pub compress_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="20")]
    pub origin_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="21")]
    pub error_code: ::core::option::Option<u32>,
    #[prost(message, optional, tag="22")]
    pub redirect: ::core::option::Option<RedirectMsg>,
    #[prost(uint32, optional, tag="23")]
    pub command_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="24")]
    pub service_cmdid: ::core::option::Option<u32>,
    #[prost(message, optional, tag="25")]
    pub oidbhead: ::core::option::Option<TransOidbHead>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginSig {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedirectMsg {
    #[prost(fixed32, optional, tag="1")]
    pub last_redirect_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub last_redirect_port: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="3")]
    pub redirect_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub redirect_port: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub redirect_count: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2cHead {
    #[prost(uint32, optional, tag="1")]
    pub sub_msgtype: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="3")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub msg_id: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="5")]
    pub relay_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub relay_port: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="7")]
    pub to_uin: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransOidbHead {
    #[prost(uint32, optional, tag="1")]
    pub command: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub service_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub result: ::core::option::Option<u32>,
    #[prost(string, optional, tag="4")]
    pub error_msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageRequest {
    #[prost(enumeration="SyncFlag", optional, tag="1")]
    pub sync_flag: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub sync_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="3")]
    pub ramble_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub latest_ramble_number: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub other_ramble_number: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub online_sync_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub context_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub whisper_session_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub msg_req_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub pubaccount_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub msg_ctrl_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="12")]
    pub server_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageRequest {
    #[prost(message, optional, tag="1")]
    pub routing_head: ::core::option::Option<RoutingHead>,
    #[prost(message, optional, tag="2")]
    pub content_head: ::core::option::Option<ContentHead>,
    #[prost(message, optional, tag="3")]
    pub msg_body: ::core::option::Option<MessageBody>,
    #[prost(int32, optional, tag="4")]
    pub msg_seq: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub msg_rand: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub sync_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    ///MsgComm.AppShareInfo? appShare = 7;
    #[prost(int32, optional, tag="8")]
    pub msg_via: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub data_statist: ::core::option::Option<i32>,
    ///MultiMsgAssist? multiMsgAssist = 10;
    ///PbInputNotifyInfo? inputNotifyInfo = 11;
    #[prost(message, optional, tag="12")]
    pub msg_ctrl: ::core::option::Option<MsgCtrl>,
    ///ImReceipt.ReceiptReq? receiptReq = 13;
    #[prost(int32, optional, tag="14")]
    pub multi_send_seq: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageResponse {
    #[prost(int32, optional, tag="1")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithDrawReq {
    #[prost(message, repeated, tag="1")]
    pub c2c_with_draw: ::prost::alloc::vec::Vec<C2cMsgWithDrawReq>,
    #[prost(message, repeated, tag="2")]
    pub group_with_draw: ::prost::alloc::vec::Vec<GroupMsgWithDrawReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2cMsgWithDrawReq {
    #[prost(message, repeated, tag="1")]
    pub msg_info: ::prost::alloc::vec::Vec<C2cMsgInfo>,
    #[prost(int32, optional, tag="2")]
    pub long_message_flag: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub reserved: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="4")]
    pub sub_cmd: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMsgWithDrawReq {
    #[prost(int32, optional, tag="1")]
    pub sub_cmd: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub group_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="3")]
    pub group_code: ::core::option::Option<i64>,
    #[prost(message, repeated, tag="4")]
    pub msg_list: ::prost::alloc::vec::Vec<GroupMsgInfo>,
    #[prost(bytes="vec", optional, tag="5")]
    pub user_def: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithDrawResp {
    #[prost(message, repeated, tag="1")]
    pub c2c_with_draw: ::prost::alloc::vec::Vec<C2cMsgWithDrawResp>,
    #[prost(message, repeated, tag="2")]
    pub group_with_draw: ::prost::alloc::vec::Vec<GroupMsgWithDrawResp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2cMsgWithDrawResp {
    #[prost(int32, optional, tag="1")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMsgWithDrawResp {
    #[prost(int32, optional, tag="1")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMsgInfo {
    #[prost(int32, optional, tag="1")]
    pub msg_seq: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub msg_random: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub msg_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2cMsgInfo {
    #[prost(int64, optional, tag="1")]
    pub from_uin: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub to_uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="3")]
    pub msg_seq: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="4")]
    pub msg_uid: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="5")]
    pub msg_time: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="6")]
    pub msg_random: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub pkg_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub pkg_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub div_seq: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="20")]
    pub routing_head: ::core::option::Option<RoutingHead>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingHead {
    #[prost(oneof="routing_head::RoutingHead", tags="1, 2, 3, 6")]
    pub routing_head: ::core::option::Option<routing_head::RoutingHead>,
}
/// Nested message and enum types in `RoutingHead`.
pub mod routing_head {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RoutingHead {
        #[prost(message, tag="1")]
        C2c(super::C2c),
        #[prost(message, tag="2")]
        Grp(super::Grp),
        #[prost(message, tag="3")]
        GrpTmp(super::GrpTmp),
        #[prost(message, tag="6")]
        WpaTmp(super::WpaTmp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WpaTmp {
    #[prost(uint64, optional, tag="1")]
    pub to_uin: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2c {
    #[prost(int64, optional, tag="1")]
    pub to_uin: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grp {
    #[prost(int64, optional, tag="1")]
    pub group_code: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpTmp {
    #[prost(int64, optional, tag="1")]
    pub group_uin: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub to_uin: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCtrl {
    #[prost(int32, optional, tag="1")]
    pub msg_flag: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageResponse {
    #[prost(int32, optional, tag="1")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub error_message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="3")]
    pub sync_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="SyncFlag", optional, tag="4")]
    pub sync_flag: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="5")]
    pub uin_pair_msgs: ::prost::alloc::vec::Vec<UinPairMessage>,
    #[prost(int64, optional, tag="6")]
    pub bind_uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="7")]
    pub msg_rsp_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub pub_account_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="9")]
    pub is_partial_sync: ::core::option::Option<bool>,
    #[prost(bytes="vec", optional, tag="10")]
    pub msg_ctrl_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushMessagePacket {
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<Message>,
    #[prost(int32, optional, tag="2")]
    pub svrip: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub push_token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="4")]
    pub ping_f_lag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub general_flag: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UinPairMessage {
    #[prost(int32, optional, tag="1")]
    pub last_read_time: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="2")]
    pub peer_uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="3")]
    pub msg_completed: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="4")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(message, optional, tag="1")]
    pub head: ::core::option::Option<MessageHead>,
    #[prost(message, optional, tag="2")]
    pub content: ::core::option::Option<ContentHead>,
    #[prost(message, optional, tag="3")]
    pub body: ::core::option::Option<MessageBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageBody {
    #[prost(message, optional, tag="1")]
    pub rich_text: ::core::option::Option<RichText>,
    #[prost(bytes="vec", optional, tag="2")]
    pub msg_content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub msg_encrypt_content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RichText {
    #[prost(message, optional, tag="1")]
    pub attr: ::core::option::Option<Attr>,
    #[prost(message, repeated, tag="2")]
    pub elems: ::prost::alloc::vec::Vec<Elem>,
    #[prost(message, optional, tag="3")]
    pub not_online_file: ::core::option::Option<NotOnlineFile>,
    #[prost(message, optional, tag="4")]
    pub ptt: ::core::option::Option<Ptt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Elem {
    #[prost(oneof="elem::Elem", tags="1, 2, 3, 4, 5, 6, 8, 9, 12, 13, 16, 19, 21, 24, 31, 37, 45, 51, 53")]
    pub elem: ::core::option::Option<elem::Elem>,
}
/// Nested message and enum types in `Elem`.
pub mod elem {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Elem {
        #[prost(message, tag="1")]
        Text(super::Text),
        #[prost(message, tag="2")]
        Face(super::Face),
        #[prost(message, tag="3")]
        OnlineImage(super::OnlineImage),
        #[prost(message, tag="4")]
        NotOnlineImage(super::NotOnlineImage),
        #[prost(message, tag="5")]
        TransElemInfo(super::TransElem),
        #[prost(message, tag="6")]
        MarketFace(super::MarketFace),
        ///ElemFlags elemFlags = 7;
        #[prost(message, tag="8")]
        CustomFace(super::CustomFace),
        #[prost(message, tag="9")]
        ElemFlags2(super::ElemFlags2),
        ///FunFace funFace = 10;
        ///SecretFileMsg secretFile = 11;
        #[prost(message, tag="12")]
        RichMsg(super::RichMsg),
        #[prost(message, tag="13")]
        GroupFile(super::GroupFile),
        ///PubGroup pubGroup = 14;
        ///MarketTrans marketTrans = 15;
        #[prost(message, tag="16")]
        ExtraInfo(super::ExtraInfo),
        ///ShakeWindow? shakeWindow = 17;
        ///PubAccount? pubAccount = 18;
        #[prost(message, tag="19")]
        VideoFile(super::VideoFile),
        ///TipsInfo? tipsInfo = 20;
        #[prost(message, tag="21")]
        AnonGroupMsg(super::AnonymousGroupMessage),
        ///QQLiveOld? qqLiveOld = 22;
        ///LifeOnlineAccount? lifeOnline = 23;
        #[prost(message, tag="24")]
        QqWalletMsg(super::QqWalletMsg),
        ///CrmElem? crmElem = 25;
        ///ConferenceTipsInfo? conferenceTipsInfo = 26;
        ///RedBagInfo? redbagInfo = 27;
        ///LowVersionTips? lowVersionTips = 28;
        ///bytes bankcodeCtrlInfo = 29;
        ///NearByMessageType? nearByMsg = 30;
        #[prost(message, tag="31")]
        CustomElem(super::CustomElem),
        ///LocationInfo? locationInfo = 32;
        ///PubAccInfo? pubAccInfo = 33;
        ///SmallEmoji? smallEmoji = 34;
        ///FSJMessageElem? fsjMsgElem = 35;
        ///ArkAppElem? arkApp = 36;
        #[prost(message, tag="37")]
        GeneralFlags(super::GeneralFlags),
        ///CustomFace? hcFlashPic = 38;
        ///DeliverGiftMsg? deliverGiftMsg = 39;
        ///BitAppMsg? bitappMsg = 40;
        ///OpenQQData? openQqData = 41;
        ///ApolloActMsg? apolloMsg = 42;
        ///GroupPubAccountInfo? groupPubAccInfo = 43;
        ///BlessingMessage? blessMsg = 44;
        #[prost(message, tag="45")]
        SrcMsg(super::SourceMsg),
        ///LolaMsg? lolaMsg = 46;
        ///GroupBusinessMsg? groupBusinessMsg = 47;
        ///WorkflowNotifyMsg? msgWorkflowNotify = 48;
        ///PatsElem? patElem = 49;
        ///GroupPostElem? groupPostElem = 50;
        #[prost(message, tag="51")]
        LightApp(super::LightApp),
        ///EIMInfo? eimInfo = 52;
        #[prost(message, tag="53")]
        CommonElem(super::CommonElem),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketFace {
    #[prost(bytes="vec", optional, tag="1")]
    pub face_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="2")]
    pub item_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub face_info: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub face_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="5")]
    pub tab_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub sub_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub param: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="9")]
    pub media_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub image_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub image_height: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="12")]
    pub mobileparam: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="13")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElemFlags2 {
    #[prost(uint32, optional, tag="1")]
    pub color_text_id: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub msg_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub whisper_session_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub ptt_change_bit: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub vip_status: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub compatible_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="7")]
    pub insts: ::prost::alloc::vec::Vec<elem_flags2::Inst>,
    #[prost(uint32, optional, tag="8")]
    pub msg_rpt_cnt: ::core::option::Option<u32>,
    #[prost(message, optional, tag="9")]
    pub src_inst: ::core::option::Option<elem_flags2::Inst>,
    #[prost(uint32, optional, tag="10")]
    pub longtitude: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub latitude: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub custom_font: ::core::option::Option<u32>,
    #[prost(message, optional, tag="13")]
    pub pc_support_def: ::core::option::Option<PcSupportDef>,
    #[prost(uint32, optional, tag="14")]
    pub crm_flags: ::core::option::Option<u32>,
}
/// Nested message and enum types in `ElemFlags2`.
pub mod elem_flags2 {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Inst {
        #[prost(uint32, optional, tag="1")]
        pub app_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag="2")]
        pub inst_id: ::core::option::Option<u32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PcSupportDef {
    #[prost(uint32, optional, tag="1")]
    pub pc_ptl_begin: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub pc_ptl_end: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub mac_ptl_begin: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub mac_ptl_end: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed="false", tag="5")]
    pub ptls_support: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="6")]
    pub ptls_not_support: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonElem {
    #[prost(int32, optional, tag="1")]
    pub service_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub pb_elem: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="3")]
    pub business_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QqWalletMsg {
    #[prost(message, optional, tag="1")]
    pub aio_body: ::core::option::Option<QqWalletAioBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QqWalletAioBody {
    #[prost(uint64, optional, tag="1")]
    pub send_uin: ::core::option::Option<u64>,
    #[prost(message, optional, tag="2")]
    pub sender: ::core::option::Option<QqWalletAioElem>,
    #[prost(message, optional, tag="3")]
    pub receiver: ::core::option::Option<QqWalletAioElem>,
    #[prost(sint32, optional, tag="4")]
    pub channel_id: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="5")]
    pub template_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag="6")]
    pub resend: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub msg_priority: ::core::option::Option<u32>,
    #[prost(sint32, optional, tag="8")]
    pub red_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="9")]
    pub bill_no: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="10")]
    pub auth_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(sint32, optional, tag="11")]
    pub session_type: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="12")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="13")]
    pub envel_ope_id: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="14")]
    pub name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(sint32, optional, tag="15")]
    pub conf_type: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag="16")]
    pub msg_from: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="17")]
    pub pc_body: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="18")]
    pub index: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="19")]
    pub red_channel: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed="false", tag="20")]
    pub grap_uin: ::prost::alloc::vec::Vec<u64>,
    #[prost(bytes="vec", optional, tag="21")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QqWalletAioElem {
    #[prost(uint32, optional, tag="1")]
    pub background: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub icon: ::core::option::Option<u32>,
    #[prost(string, optional, tag="3")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub subtitle: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="6")]
    pub link_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub black_stripe: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub notice: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="9")]
    pub title_color: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub subtitle_color: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="11")]
    pub actions_priority: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="12")]
    pub jump_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="13")]
    pub native_ios: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="14")]
    pub native_android: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="15")]
    pub icon_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="16")]
    pub content_color: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="17")]
    pub content_bg_color: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="18")]
    pub aio_image_left: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="19")]
    pub aio_image_right: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="20")]
    pub cft_image: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="21")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RichMsg {
    #[prost(bytes="vec", optional, tag="1")]
    pub template1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="2")]
    pub service_id: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub msg_res_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="4")]
    pub rand: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub seq: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomElem {
    #[prost(bytes="vec", optional, tag="1")]
    pub desc: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="3")]
    pub enum_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub ext: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub sound: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    #[prost(string, optional, tag="1")]
    pub str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub link: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="3")]
    pub attr6_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub attr7_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="12")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attr {
    #[prost(int32, optional, tag="1")]
    pub code_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub random: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub color: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub effect: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub char_set: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub pitch_and_family: ::core::option::Option<i32>,
    #[prost(string, optional, tag="9")]
    pub font_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="10")]
    pub reserve_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ptt {
    #[prost(int32, optional, tag="1")]
    pub file_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="2")]
    pub src_uin: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="3")]
    pub file_uuid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="5")]
    pub file_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub file_size: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="8")]
    pub file_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub server_ip: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub server_port: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="11")]
    pub bool_valid: ::core::option::Option<bool>,
    #[prost(bytes="vec", optional, tag="12")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="13")]
    pub shortcut: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="14")]
    pub file_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="15")]
    pub magic_ptt_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="16")]
    pub voice_switch: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="17")]
    pub ptt_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="18")]
    pub group_file_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="19")]
    pub time: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="20")]
    pub down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="29")]
    pub format: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="30")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="31")]
    pub bytes_ptt_urls: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="32")]
    pub download_flag: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlineImage {
    #[prost(bytes="vec", optional, tag="1")]
    pub guid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub file_path: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub old_ver_send_file: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotOnlineImage {
    #[prost(string, optional, tag="1")]
    pub file_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="2")]
    pub file_len: ::core::option::Option<u32>,
    #[prost(string, optional, tag="3")]
    pub download_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="4")]
    pub old_ver_send_file: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="5")]
    pub img_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub previews_image: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub pic_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="8")]
    pub pic_height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub pic_width: ::core::option::Option<u32>,
    #[prost(string, optional, tag="10")]
    pub res_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="11")]
    pub flag: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="12")]
    pub thumb_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="13")]
    pub original: ::core::option::Option<i32>,
    #[prost(string, optional, tag="14")]
    pub big_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub orig_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="16")]
    pub biz_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="17")]
    pub result: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="18")]
    pub index: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="19")]
    pub op_face_buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="20")]
    pub old_pic_md5: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="21")]
    pub thumb_width: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="22")]
    pub thumb_height: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="23")]
    pub file_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="24")]
    pub show_len: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="25")]
    pub download_len: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="29")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotOnlineFile {
    #[prost(int32, optional, tag="1")]
    pub file_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub file_uuid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub file_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="6")]
    pub file_size: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="7")]
    pub note: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="8")]
    pub reserved: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub subcmd: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub micro_cloud: ::core::option::Option<i32>,
    #[prost(bytes="vec", repeated, tag="11")]
    pub bytes_file_urls: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="12")]
    pub download_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="50")]
    pub danger_evel: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="51")]
    pub life_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="52")]
    pub upload_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="53")]
    pub abs_file_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="54")]
    pub client_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="55")]
    pub expire_time: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="56")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransElem {
    #[prost(int32, optional, tag="1")]
    pub elem_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub elem_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraInfo {
    #[prost(bytes="vec", optional, tag="1")]
    pub nick: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub group_card: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="3")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub flags: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub group_mask: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub msg_tail_id: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub sender_title: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub apns_tips: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="9")]
    pub uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="10")]
    pub msg_state_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="11")]
    pub apns_sound_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="12")]
    pub new_group_flag: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFile {
    #[prost(bytes="vec", optional, tag="1")]
    pub filename: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="2")]
    pub file_size: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="3")]
    pub file_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub batch_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub file_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="6")]
    pub mark: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="7")]
    pub sequence: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="8")]
    pub batch_item_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="9")]
    pub feed_msg_time: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnonymousGroupMessage {
    #[prost(int32, optional, tag="1")]
    pub flags: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub anon_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub anon_nick: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="4")]
    pub head_portrait: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub expire_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub bubble_id: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub rank_color: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoFile {
    #[prost(bytes="vec", optional, tag="1")]
    pub file_uuid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="3")]
    pub file_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="4")]
    pub file_format: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub file_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub file_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub thumb_width: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub thumb_height: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="9")]
    pub thumb_file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="10")]
    pub source: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="11")]
    pub thumb_file_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="12")]
    pub busi_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="13")]
    pub from_chat_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="14")]
    pub to_chat_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="15")]
    pub bool_support_progressive: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="16")]
    pub file_width: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="17")]
    pub file_height: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="18")]
    pub sub_busi_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="19")]
    pub video_attr: ::core::option::Option<i32>,
    #[prost(bytes="vec", repeated, tag="20")]
    pub bytes_thumb_file_urls: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="21")]
    pub bytes_video_file_urls: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="22")]
    pub thumb_download_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="23")]
    pub video_download_flag: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="24")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceMsg {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub orig_seqs: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, optional, tag="2")]
    pub sender_uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="3")]
    pub time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub flag: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="5")]
    pub elems: ::prost::alloc::vec::Vec<Elem>,
    #[prost(int32, optional, tag="6")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub rich_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="9")]
    pub src_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag="10")]
    pub to_uin: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="11")]
    pub troop_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Face {
    #[prost(int32, optional, tag="1")]
    pub index: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub old: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightApp {
    #[prost(bytes="vec", optional, tag="1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub msg_resid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomFace {
    #[prost(bytes="vec", optional, tag="1")]
    pub guid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="2")]
    pub file_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub shortcut: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="4")]
    pub buffer: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub flag: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="6")]
    pub old_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="7")]
    pub file_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag="8")]
    pub server_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub server_port: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="10")]
    pub file_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="11")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="12")]
    pub useful: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="13")]
    pub md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="14")]
    pub thumb_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub big_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub orig_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="17")]
    pub biz_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="18")]
    pub repeat_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="19")]
    pub repeat_image: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="20")]
    pub image_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="21")]
    pub index: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag="22")]
    pub width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="23")]
    pub height: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="24")]
    pub source: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag="25")]
    pub size: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="26")]
    pub origin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="27")]
    pub thumb_width: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="28")]
    pub thumb_height: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="29")]
    pub show_len: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="30")]
    pub download_len: ::core::option::Option<i32>,
    ///x
    #[prost(string, optional, tag="31")]
    pub x400_url: ::core::option::Option<::prost::alloc::string::String>,
    ///x
    #[prost(int32, optional, tag="32")]
    pub x400_width: ::core::option::Option<i32>,
    ///x
    #[prost(int32, optional, tag="33")]
    pub x400_height: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="34")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentHead {
    #[prost(int32, optional, tag="1")]
    pub pkg_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub pkg_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub div_seq: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub auto_reply: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageHead {
    #[prost(int64, optional, tag="1")]
    pub from_uin: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub to_uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="3")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub c2c_cmd: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub msg_seq: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub msg_time: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="7")]
    pub msg_uid: ::core::option::Option<i64>,
    #[prost(message, optional, tag="8")]
    pub c2c_tmp_msg_head: ::core::option::Option<C2cTempMessageHead>,
    #[prost(message, optional, tag="9")]
    pub group_info: ::core::option::Option<GroupInfo>,
    #[prost(int32, optional, tag="10")]
    pub from_appid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="11")]
    pub from_instid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="12")]
    pub user_active: ::core::option::Option<i32>,
    #[prost(message, optional, tag="13")]
    pub discuss_info: ::core::option::Option<DiscussInfo>,
    #[prost(string, optional, tag="14")]
    pub from_nick: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="15")]
    pub auth_uin: ::core::option::Option<i64>,
    #[prost(string, optional, tag="16")]
    pub auth_nick: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="17")]
    pub msg_flag: ::core::option::Option<i32>,
    #[prost(string, optional, tag="18")]
    pub auth_remark: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub group_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="20")]
    pub mutiltrans_head: ::core::option::Option<MutilTransHead>,
    #[prost(message, optional, tag="21")]
    pub msg_inst_ctrl: ::core::option::Option<InstCtrl>,
    #[prost(int32, optional, tag="22")]
    pub public_account_group_send_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="23")]
    pub wseq_in_c2c_msghead: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="24")]
    pub cpid: ::core::option::Option<i64>,
    #[prost(message, optional, tag="25")]
    pub ext_group_key_info: ::core::option::Option<ExtGroupKeyInfo>,
    #[prost(string, optional, tag="26")]
    pub multi_compatible_text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="27")]
    pub auth_sex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="28")]
    pub is_src_msg: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInfo {
    #[prost(int64, optional, tag="1")]
    pub group_code: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="2")]
    pub group_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="3")]
    pub group_info_seq: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub group_card: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub group_rank: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="6")]
    pub group_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub group_card_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub group_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscussInfo {
    #[prost(int64, optional, tag="1")]
    pub discuss_uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="2")]
    pub discuss_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="3")]
    pub discuss_info_seq: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub discuss_remark: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub discuss_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutilTransHead {
    #[prost(int32, optional, tag="1")]
    pub status: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub msg_id: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2cTempMessageHead {
    #[prost(int32, optional, tag="1")]
    pub c2c_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub service_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="3")]
    pub group_uin: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub group_code: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="5")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="6")]
    pub sig_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="7")]
    pub from_phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub to_phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="9")]
    pub lock_display: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub direction_flag: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="11")]
    pub reserved: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstCtrl {
    #[prost(message, repeated, tag="1")]
    pub msg_send_to_inst: ::prost::alloc::vec::Vec<InstInfo>,
    #[prost(message, repeated, tag="2")]
    pub msg_exclude_inst: ::prost::alloc::vec::Vec<InstInfo>,
    #[prost(message, optional, tag="3")]
    pub msg_from_inst: ::core::option::Option<InstInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstInfo {
    #[prost(int32, optional, tag="1")]
    pub apppid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub instid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub platform: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub enum_device_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtGroupKeyInfo {
    #[prost(int32, optional, tag="1")]
    pub cur_max_seq: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="2")]
    pub cur_time: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncCookie {
    #[prost(int64, optional, tag="1")]
    pub time1: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub time: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub ran1: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub ran2: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="5")]
    pub const1: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="11")]
    pub const2: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="12")]
    pub const3: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="13")]
    pub last_sync_time: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="14")]
    pub const4: ::core::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransMsgInfo {
    #[prost(int64, optional, tag="1")]
    pub from_uin: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub to_uin: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="3")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub msg_subtype: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub msg_seq: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="6")]
    pub msg_uid: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="7")]
    pub msg_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub real_msg_time: ::core::option::Option<i32>,
    #[prost(string, optional, tag="9")]
    pub nick_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="10")]
    pub msg_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="11")]
    pub svr_ip: ::core::option::Option<i32>,
    #[prost(message, optional, tag="12")]
    pub ext_group_key_info: ::core::option::Option<ExtGroupKeyInfo>,
    #[prost(int32, optional, tag="17")]
    pub general_flag: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralFlags {
    #[prost(int32, optional, tag="1")]
    pub bubble_diy_text_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub group_flag_new: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="3")]
    pub uin: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub rp_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="5")]
    pub prp_fold: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub long_text_flag: ::core::option::Option<i32>,
    #[prost(string, optional, tag="7")]
    pub long_text_resid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="8")]
    pub group_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub to_uin_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub glamour_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="11")]
    pub member_level: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="12")]
    pub group_rank_seq: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="13")]
    pub olympic_torch: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="14")]
    pub babyq_guide_msg_cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag="15")]
    pub uin32_expert_flag: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="16")]
    pub bubble_sub_id: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="17")]
    pub pendant_id: ::core::option::Option<i64>,
    #[prost(bytes="vec", optional, tag="18")]
    pub rp_index: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="19")]
    pub pb_reserve: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMultiMsgItem {
    #[prost(string, optional, tag="1")]
    pub file_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub buffer: ::core::option::Option<PbMultiMsgNew>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMultiMsgNew {
    #[prost(message, repeated, tag="1")]
    pub msg: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbMultiMsgTransmit {
    #[prost(message, repeated, tag="1")]
    pub msg: ::prost::alloc::vec::Vec<Message>,
    #[prost(message, repeated, tag="2")]
    pub pb_item_list: ::prost::alloc::vec::Vec<PbMultiMsgItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgElemInfoServtype3 {
    #[prost(message, optional, tag="1")]
    pub flash_troop_pic: ::core::option::Option<CustomFace>,
    #[prost(message, optional, tag="2")]
    pub flash_c2c_pic: ::core::option::Option<NotOnlineImage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgElemInfoServtype33 {
    #[prost(uint32, optional, tag="1")]
    pub index: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub text: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub compat: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub buf: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubMsgType0x4Body {
    #[prost(message, optional, tag="1")]
    pub not_online_file: ::core::option::Option<NotOnlineFile>,
    #[prost(uint32, optional, tag="2")]
    pub msg_time: ::core::option::Option<u32>,
    /// fileImageInfo
    #[prost(uint32, optional, tag="3")]
    pub online_file_for_poly_to_offline: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResvAttr {
    #[prost(uint32, optional, tag="1")]
    pub image_biz_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="7")]
    pub image_show: ::core::option::Option<AnimationImageShow>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnimationImageShow {
    #[prost(int32, optional, tag="1")]
    pub effect_id: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub animation_param: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UinTypeUserDef {
    #[prost(int32, optional, tag="1")]
    pub from_uin_type: ::core::option::Option<i32>,
    #[prost(int64, optional, tag="2")]
    pub from_group_code: ::core::option::Option<i64>,
    #[prost(string, optional, tag="3")]
    pub file_uuid: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupMsgReq {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub begin_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub end_seq: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub filter: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="5")]
    pub member_seq: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="6")]
    pub public_group: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="7")]
    pub shield_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub save_traffic_flag: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupMsgResp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub errmsg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub return_begin_seq: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub return_end_seq: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="6")]
    pub msg: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbGetOneDayRoamMsgReq {
    #[prost(uint64, optional, tag="1")]
    pub peer_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub last_msg_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub random: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub read_cnt: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbGetOneDayRoamMsgResp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub err_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub peer_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub last_msg_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub random: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="6")]
    pub msg: ::prost::alloc::vec::Vec<Message>,
    #[prost(uint32, optional, tag="7")]
    pub is_complete: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPushMsg {
    #[prost(message, optional, tag="1")]
    pub msg: ::core::option::Option<Message>,
    #[prost(int32, optional, tag="2")]
    pub svrip: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub push_token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="4")]
    pub ping_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub general_flag: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="10")]
    pub bind_uin: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgElemInfoServtype37 {
    #[prost(bytes="vec", optional, tag="1")]
    pub packid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub stickerid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub qsid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub sourcetype: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub stickertype: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub resultid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub text: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub surpriseid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="9")]
    pub randomtype: ::core::option::Option<u32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SyncFlag {
    Start = 0,
    Continume = 1,
    Stop = 2,
}
