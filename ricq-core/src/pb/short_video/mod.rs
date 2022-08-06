#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoReqBody {
    #[prost(int32, tag="1")]
    pub cmd: i32,
    #[prost(int32, tag="2")]
    pub seq: i32,
    #[prost(message, optional, tag="3")]
    pub ptt_short_video_upload_req: ::core::option::Option<ShortVideoUploadReq>,
    #[prost(message, optional, tag="4")]
    pub ptt_short_video_download_req: ::core::option::Option<ShortVideoDownloadReq>,
    #[prost(message, repeated, tag="100")]
    pub extension_req: ::prost::alloc::vec::Vec<ShortVideoExtensionReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoRspBody {
    #[prost(int32, tag="1")]
    pub cmd: i32,
    #[prost(int32, tag="2")]
    pub seq: i32,
    #[prost(message, optional, tag="3")]
    pub ptt_short_video_upload_rsp: ::core::option::Option<ShortVideoUploadRsp>,
    #[prost(message, optional, tag="4")]
    pub ptt_short_video_download_rsp: ::core::option::Option<ShortVideoDownloadRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoUploadReq {
    #[prost(int64, tag="1")]
    pub from_uin: i64,
    #[prost(int64, tag="2")]
    pub to_uin: i64,
    #[prost(int32, tag="3")]
    pub chat_type: i32,
    #[prost(int32, tag="4")]
    pub client_type: i32,
    #[prost(message, optional, tag="5")]
    pub info: ::core::option::Option<ShortVideoFileInfo>,
    #[prost(int64, tag="6")]
    pub group_code: i64,
    #[prost(int32, tag="7")]
    pub agent_type: i32,
    #[prost(int32, tag="8")]
    pub business_type: i32,
    #[prost(int32, tag="20")]
    pub support_large_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoDownloadReq {
    #[prost(int64, tag="1")]
    pub from_uin: i64,
    #[prost(int64, tag="2")]
    pub to_uin: i64,
    #[prost(int32, tag="3")]
    pub chat_type: i32,
    #[prost(int32, tag="4")]
    pub client_type: i32,
    #[prost(string, tag="5")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(int64, tag="6")]
    pub group_code: i64,
    #[prost(int32, tag="7")]
    pub agent_type: i32,
    #[prost(bytes="vec", tag="8")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="9")]
    pub business_type: i32,
    #[prost(int32, tag="10")]
    pub file_type: i32,
    #[prost(int32, tag="11")]
    pub down_type: i32,
    #[prost(int32, tag="12")]
    pub scene_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoDownloadRsp {
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub same_area_out_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(message, repeated, tag="4")]
    pub diff_area_out_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(bytes="vec", tag="5")]
    pub download_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="7")]
    pub same_area_inner_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(message, repeated, tag="8")]
    pub diff_area_inner_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(message, optional, tag="9")]
    pub download_addr: ::core::option::Option<ShortVideoAddr>,
    #[prost(bytes="vec", tag="10")]
    pub encrypt_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoUploadRsp {
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub same_area_out_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(message, repeated, tag="4")]
    pub diff_area_out_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(bytes="vec", tag="5")]
    pub file_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub u_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="7")]
    pub file_exists: i32,
    #[prost(message, repeated, tag="8")]
    pub same_area_inner_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(message, repeated, tag="9")]
    pub diff_area_inner_addr: ::prost::alloc::vec::Vec<ShortVideoIpList>,
    #[prost(message, repeated, tag="10")]
    pub data_hole: ::prost::alloc::vec::Vec<DataHole>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoFileInfo {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub thumb_file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub file_size: i64,
    #[prost(int32, tag="5")]
    pub file_res_length: i32,
    #[prost(int32, tag="6")]
    pub file_res_width: i32,
    #[prost(int32, tag="7")]
    pub file_format: i32,
    #[prost(int32, tag="8")]
    pub file_time: i32,
    #[prost(int64, tag="9")]
    pub thumb_file_size: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataHole {
    #[prost(int64, tag="1")]
    pub begin: i64,
    #[prost(int64, tag="2")]
    pub end: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoIpList {
    #[prost(int32, tag="1")]
    pub ip: i32,
    #[prost(int32, tag="2")]
    pub port: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoAddr {
    #[prost(string, repeated, tag="10")]
    pub host: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///repeated string domain = 13;
    #[prost(string, tag="11")]
    pub url_args: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortVideoExtensionReq {
    #[prost(int32, tag="1")]
    pub sub_busi_type: i32,
    #[prost(int32, tag="2")]
    pub user_cnt: i32,
}
