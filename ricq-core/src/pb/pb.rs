#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    #[prost(string, tag="1")]
    pub bootloader: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub proc_version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub codename: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub incremental: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub fingerprint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub boot_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub android_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub base_band: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub inner_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBody {
    #[prost(message, repeated, tag="1")]
    pub rpt_config_list: ::prost::alloc::vec::Vec<ConfigSeq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSeq {
    #[prost(int32, tag="1")]
    pub r#type: i32,
    #[prost(int32, tag="2")]
    pub version: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D50ReqBody {
    #[prost(int64, tag="1")]
    pub appid: i64,
    #[prost(int32, tag="2")]
    pub max_pkg_size: i32,
    #[prost(int32, tag="3")]
    pub start_time: i32,
    #[prost(int32, tag="4")]
    pub start_index: i32,
    #[prost(int32, tag="5")]
    pub req_num: i32,
    #[prost(int64, repeated, tag="6")]
    pub uin_list: ::prost::alloc::vec::Vec<i64>,
    #[prost(int32, tag="91001")]
    pub req_music_switch: i32,
    #[prost(int32, tag="101001")]
    pub req_mutualmark_alienation: i32,
    #[prost(int32, tag="141001")]
    pub req_mutualmark_score: i32,
    #[prost(int32, tag="151001")]
    pub req_ksing_switch: i32,
    #[prost(int32, tag="181001")]
    pub req_mutualmark_lbsshare: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D388ReqBody {
    #[prost(int32, tag="1")]
    pub net_type: i32,
    #[prost(int32, tag="2")]
    pub subcmd: i32,
    #[prost(message, repeated, tag="3")]
    pub msg_try_up_img_req: ::prost::alloc::vec::Vec<TryUpImgReq>,
    #[prost(message, repeated, tag="5")]
    pub msg_try_up_ptt_req: ::prost::alloc::vec::Vec<TryUpPttReq>,
    #[prost(message, repeated, tag="6")]
    pub msg_get_ptt_req: ::prost::alloc::vec::Vec<GetPttUrlReq>,
    #[prost(int32, tag="7")]
    pub command_id: i32,
    #[prost(bytes="vec", tag="1001")]
    pub extension: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D388RespBody {
    #[prost(int32, tag="1")]
    pub client_ip: i32,
    #[prost(int32, tag="2")]
    pub sub_cmd: i32,
    #[prost(message, repeated, tag="3")]
    pub msg_try_up_img_rsp: ::prost::alloc::vec::Vec<TryUpImgResp>,
    #[prost(message, repeated, tag="5")]
    pub msg_try_up_ptt_rsp: ::prost::alloc::vec::Vec<TryUpPttResp>,
    #[prost(message, repeated, tag="6")]
    pub msg_get_ptt_url_rsp: ::prost::alloc::vec::Vec<GetPttUrlRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPttUrlReq {
    #[prost(int64, tag="1")]
    pub group_code: i64,
    #[prost(int64, tag="2")]
    pub dst_uin: i64,
    #[prost(int64, tag="3")]
    pub file_id: i64,
    #[prost(bytes="vec", tag="4")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="5")]
    pub req_term: i32,
    #[prost(int32, tag="6")]
    pub req_platform_type: i32,
    #[prost(int32, tag="7")]
    pub inner_ip: i32,
    #[prost(int32, tag="8")]
    pub bu_type: i32,
    #[prost(bytes="vec", tag="9")]
    pub build_ver: ::prost::alloc::vec::Vec<u8>,
    ///int64 fileId = 10;
    #[prost(bytes="vec", tag="11")]
    pub file_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="12")]
    pub codec: i32,
    #[prost(int32, tag="13")]
    pub bu_id: i32,
    #[prost(int32, tag="14")]
    pub req_transfer_type: i32,
    #[prost(int32, tag="15")]
    pub is_auto: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPttUrlRsp {
    #[prost(int64, tag="1")]
    pub file_id: i64,
    #[prost(bytes="vec", tag="2")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="3")]
    pub result: i32,
    #[prost(bytes="vec", tag="4")]
    pub fail_msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub bytes_down_url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, repeated, tag="6")]
    pub uint32_down_ip: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="7")]
    pub uint32_down_port: ::prost::alloc::vec::Vec<i32>,
    #[prost(bytes="vec", tag="8")]
    pub down_domain: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub down_para: ::prost::alloc::vec::Vec<u8>,
    ///int64 fileId = 10;
    #[prost(int32, tag="11")]
    pub transfer_type: i32,
    #[prost(int32, tag="12")]
    pub allow_retry: i32,
    ///repeated IPv6Info msgDownIp6 = 26;
    #[prost(bytes="vec", tag="27")]
    pub client_ip6: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="28")]
    pub str_domain: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqDataHighwayHead {
    #[prost(message, optional, tag="1")]
    pub msg_basehead: ::core::option::Option<DataHighwayHead>,
    #[prost(message, optional, tag="2")]
    pub msg_seghead: ::core::option::Option<SegHead>,
    #[prost(bytes="vec", tag="3")]
    pub req_extendinfo: ::prost::alloc::vec::Vec<u8>,
    ///LoginSigHead? msgLoginSigHead = 5;
    #[prost(int64, tag="4")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspDataHighwayHead {
    #[prost(message, optional, tag="1")]
    pub msg_basehead: ::core::option::Option<DataHighwayHead>,
    #[prost(message, optional, tag="2")]
    pub msg_seghead: ::core::option::Option<SegHead>,
    #[prost(int32, tag="3")]
    pub error_code: i32,
    #[prost(int32, tag="4")]
    pub allow_retry: i32,
    #[prost(int32, tag="5")]
    pub cachecost: i32,
    #[prost(int32, tag="6")]
    pub htcost: i32,
    #[prost(bytes="vec", tag="7")]
    pub rsp_extendinfo: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="8")]
    pub timestamp: i64,
    #[prost(int64, tag="9")]
    pub range: i64,
    #[prost(int32, tag="10")]
    pub is_reset: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataHighwayHead {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(string, tag="2")]
    pub uin: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub command: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub seq: i32,
    #[prost(int32, tag="5")]
    pub retry_times: i32,
    #[prost(int32, tag="6")]
    pub appid: i32,
    #[prost(int32, tag="7")]
    pub dataflag: i32,
    #[prost(int32, tag="8")]
    pub command_id: i32,
    #[prost(string, tag="9")]
    pub build_ver: ::prost::alloc::string::String,
    #[prost(int32, tag="10")]
    pub locale_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegHead {
    #[prost(int32, tag="1")]
    pub serviceid: i32,
    #[prost(int64, tag="2")]
    pub filesize: i64,
    #[prost(int64, tag="3")]
    pub dataoffset: i64,
    #[prost(int32, tag="4")]
    pub datalength: i32,
    #[prost(int32, tag="5")]
    pub rtcode: i32,
    #[prost(bytes="vec", tag="6")]
    pub serviceticket: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="7")]
    pub flag: i32,
    #[prost(bytes="vec", tag="8")]
    pub md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="10")]
    pub cache_addr: i32,
    #[prost(int32, tag="11")]
    pub query_times: i32,
    #[prost(int32, tag="12")]
    pub update_cacheip: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpImgReq {
    #[prost(int64, tag="1")]
    pub group_code: i64,
    #[prost(int64, tag="2")]
    pub src_uin: i64,
    #[prost(int64, tag="3")]
    pub file_id: i64,
    #[prost(bytes="vec", tag="4")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub file_size: i64,
    #[prost(string, tag="6")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(int32, tag="7")]
    pub src_term: i32,
    #[prost(int32, tag="8")]
    pub platform_type: i32,
    #[prost(int32, tag="9")]
    pub bu_type: i32,
    #[prost(int32, tag="10")]
    pub pic_width: i32,
    #[prost(int32, tag="11")]
    pub pic_height: i32,
    #[prost(int32, tag="12")]
    pub pic_type: i32,
    #[prost(string, tag="13")]
    pub build_ver: ::prost::alloc::string::String,
    #[prost(int32, tag="14")]
    pub inner_ip: i32,
    #[prost(int32, tag="15")]
    pub app_pic_type: i32,
    #[prost(int32, tag="16")]
    pub original_pic: i32,
    #[prost(bytes="vec", tag="17")]
    pub file_index: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="18")]
    pub dst_uin: i64,
    #[prost(int32, tag="19")]
    pub srv_upload: i32,
    #[prost(bytes="vec", tag="20")]
    pub transfer_url: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpImgResp {
    #[prost(int64, tag="1")]
    pub file_id: i64,
    #[prost(int32, tag="2")]
    pub result: i32,
    #[prost(string, tag="3")]
    pub fail_msg: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub bool_file_exit: bool,
    #[prost(message, optional, tag="5")]
    pub msg_img_info: ::core::option::Option<ImgInfo>,
    #[prost(uint32, repeated, tag="6")]
    pub uint32_up_ip: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag="7")]
    pub uint32_up_port: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", tag="8")]
    pub up_ukey: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="9")]
    pub fid: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpPttReq {
    #[prost(int64, tag="1")]
    pub group_code: i64,
    #[prost(int64, tag="2")]
    pub src_uin: i64,
    #[prost(int64, tag="3")]
    pub file_id: i64,
    #[prost(bytes="vec", tag="4")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub file_size: i64,
    #[prost(bytes="vec", tag="6")]
    pub file_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="7")]
    pub src_term: i32,
    #[prost(int32, tag="8")]
    pub platform_type: i32,
    #[prost(int32, tag="9")]
    pub bu_type: i32,
    #[prost(string, tag="10")]
    pub build_ver: ::prost::alloc::string::String,
    #[prost(int32, tag="11")]
    pub inner_ip: i32,
    #[prost(int32, tag="12")]
    pub voice_length: i32,
    #[prost(bool, tag="13")]
    pub bool_new_up_chan: bool,
    #[prost(int32, tag="14")]
    pub codec: i32,
    #[prost(int32, tag="15")]
    pub voice_type: i32,
    #[prost(int32, tag="16")]
    pub bu_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpPttResp {
    #[prost(int64, tag="1")]
    pub file_id: i64,
    #[prost(int32, tag="2")]
    pub result: i32,
    #[prost(string, tag="3")]
    pub fail_msg: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub bool_file_exit: bool,
    #[prost(int32, repeated, tag="5")]
    pub uint32_up_ip: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag="6")]
    pub uint32_up_port: ::prost::alloc::vec::Vec<i32>,
    #[prost(bytes="vec", tag="7")]
    pub up_ukey: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="8")]
    pub file_id2: i64,
    #[prost(int64, tag="9")]
    pub up_offset: i64,
    #[prost(int64, tag="10")]
    pub block_size: i64,
    #[prost(bytes="vec", tag="11")]
    pub file_key: ::prost::alloc::vec::Vec<u8>,
    ///    List<IPv6Info>? msgUpIp6 = 26;
    ///    bytes clientIp6 = 27;
    #[prost(int32, tag="12")]
    pub channel_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImgInfo {
    #[prost(bytes="vec", tag="1")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub file_type: i32,
    #[prost(int64, tag="3")]
    pub file_size: i64,
    #[prost(int32, tag="4")]
    pub file_width: i32,
    #[prost(int32, tag="5")]
    pub file_height: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMessageRequest {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<MessageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageItem {
    #[prost(int64, tag="1")]
    pub from_uin: i64,
    #[prost(int64, tag="2")]
    pub to_uin: i64,
    #[prost(int32, tag="3")]
    pub msg_type: i32,
    #[prost(int32, tag="4")]
    pub msg_seq: i32,
    #[prost(int64, tag="5")]
    pub msg_uid: i64,
    #[prost(bytes="vec", tag="7")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubD4 {
    #[prost(int64, tag="1")]
    pub uin: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sub8A {
    #[prost(message, repeated, tag="1")]
    pub msg_info: ::prost::alloc::vec::Vec<Sub8AMsgInfo>,
    #[prost(int32, tag="2")]
    pub app_id: i32,
    #[prost(int32, tag="3")]
    pub inst_id: i32,
    #[prost(int32, tag="4")]
    pub long_message_flag: i32,
    #[prost(bytes="vec", tag="5")]
    pub reserved: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sub8AMsgInfo {
    #[prost(int64, tag="1")]
    pub from_uin: i64,
    #[prost(int64, tag="2")]
    pub to_uin: i64,
    #[prost(int32, tag="3")]
    pub msg_seq: i32,
    #[prost(int64, tag="4")]
    pub msg_uid: i64,
    #[prost(int64, tag="5")]
    pub msg_time: i64,
    #[prost(int32, tag="6")]
    pub msg_random: i32,
    #[prost(int32, tag="7")]
    pub pkg_num: i32,
    #[prost(int32, tag="8")]
    pub pkg_index: i32,
    #[prost(int32, tag="9")]
    pub dev_seq: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubB3 {
    #[prost(int32, tag="1")]
    pub r#type: i32,
    #[prost(message, optional, tag="2")]
    pub msg_add_frd_notify: ::core::option::Option<SubB3AddFrdNotify>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubB3AddFrdNotify {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(string, tag="5")]
    pub nick: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sub44 {
    #[prost(message, optional, tag="1")]
    pub friend_sync_msg: ::core::option::Option<Sub44FriendSyncMsg>,
    #[prost(message, optional, tag="2")]
    pub group_sync_msg: ::core::option::Option<Sub44GroupSyncMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sub44FriendSyncMsg {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(int64, tag="2")]
    pub f_uin: i64,
    #[prost(int32, tag="3")]
    pub process_type: i32,
    #[prost(int32, tag="4")]
    pub time: i32,
    #[prost(int32, tag="5")]
    pub process_flag: i32,
    #[prost(int32, tag="6")]
    pub source_id: i32,
    #[prost(int32, tag="7")]
    pub source_sub_id: i32,
    #[prost(string, repeated, tag="8")]
    pub str_wording: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sub44GroupSyncMsg {
    #[prost(int32, tag="1")]
    pub msg_type: i32,
    #[prost(int64, tag="2")]
    pub msg_seq: i64,
    #[prost(int64, tag="3")]
    pub grp_code: i64,
    #[prost(int64, tag="4")]
    pub ga_code: i64,
    #[prost(int64, tag="5")]
    pub opt_uin1: i64,
    #[prost(int64, tag="6")]
    pub opt_uin2: i64,
    #[prost(bytes="vec", tag="7")]
    pub msg_buf: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub auth_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="9")]
    pub msg_status: i32,
    #[prost(int64, tag="10")]
    pub action_uin: i64,
    #[prost(int64, tag="11")]
    pub action_time: i64,
    #[prost(int32, tag="12")]
    pub cur_max_mem_count: i32,
    #[prost(int32, tag="13")]
    pub next_max_mem_count: i32,
    #[prost(int32, tag="14")]
    pub cur_mem_count: i32,
    #[prost(int32, tag="15")]
    pub req_src_id: i32,
    #[prost(int32, tag="16")]
    pub req_src_sub_id: i32,
    #[prost(int32, tag="17")]
    pub inviter_role: i32,
    #[prost(int32, tag="18")]
    pub ext_admin_num: i32,
    #[prost(int32, tag="19")]
    pub process_flag: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberReqBody {
    #[prost(int64, tag="1")]
    pub group_code: i64,
    #[prost(int64, tag="2")]
    pub uin: i64,
    #[prost(bool, tag="3")]
    pub new_client: bool,
    #[prost(int32, tag="4")]
    pub client_type: i32,
    #[prost(int32, tag="5")]
    pub rich_card_name_ver: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberRspBody {
    #[prost(int64, tag="1")]
    pub group_code: i64,
    #[prost(int32, tag="2")]
    pub self_role: i32,
    #[prost(message, optional, tag="3")]
    pub mem_info: ::core::option::Option<GroupMemberInfo>,
    #[prost(bool, tag="4")]
    pub bool_self_location_shared: bool,
    #[prost(int32, tag="5")]
    pub group_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberInfo {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(int32, tag="2")]
    pub result: i32,
    #[prost(bytes="vec", tag="3")]
    pub errmsg: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="4")]
    pub is_friend: bool,
    #[prost(bytes="vec", tag="5")]
    pub remark: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub is_concerned: bool,
    #[prost(int32, tag="7")]
    pub credit: i32,
    #[prost(bytes="vec", tag="8")]
    pub card: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="9")]
    pub sex: i32,
    #[prost(bytes="vec", tag="10")]
    pub location: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="11")]
    pub nick: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="12")]
    pub age: i32,
    #[prost(bytes="vec", tag="13")]
    pub lev: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="14")]
    pub join: i64,
    #[prost(int64, tag="15")]
    pub last_speak: i64,
    ///repeated CustomEntry customEnties = 16;
    ///repeated GBarInfo gbarConcerned = 17;
    #[prost(bytes="vec", tag="18")]
    pub gbar_title: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="19")]
    pub gbar_url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="20")]
    pub gbar_cnt: i32,
    #[prost(bool, tag="21")]
    pub is_allow_mod_card: bool,
    #[prost(bool, tag="22")]
    pub is_vip: bool,
    #[prost(bool, tag="23")]
    pub is_year_vip: bool,
    #[prost(bool, tag="24")]
    pub is_super_vip: bool,
    #[prost(bool, tag="25")]
    pub is_super_qq: bool,
    #[prost(int32, tag="26")]
    pub vip_lev: i32,
    #[prost(int32, tag="27")]
    pub role: i32,
    #[prost(bool, tag="28")]
    pub location_shared: bool,
    #[prost(int64, tag="29")]
    pub int64_distance: i64,
    #[prost(int32, tag="30")]
    pub concern_type: i32,
    #[prost(bytes="vec", tag="31")]
    pub special_title: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="32")]
    pub special_title_expire_time: i32,
    ///FlowersEntry flowerEntry = 33;
    ///TeamEntry teamEntry = 34;
    #[prost(bytes="vec", tag="35")]
    pub phone_num: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="36")]
    pub job: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="37")]
    pub medal_id: i32,
    #[prost(int32, tag="39")]
    pub level: i32,
    #[prost(string, tag="41")]
    pub honor: ::prost::alloc::string::String,
}
