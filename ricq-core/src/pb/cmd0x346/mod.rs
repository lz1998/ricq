#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCleanTrafficRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCopyFromReq {
    #[prost(int64, tag="10")]
    pub src_uin: i64,
    #[prost(int64, tag="20")]
    pub src_group: i64,
    #[prost(int32, tag="30")]
    pub src_svcid: i32,
    #[prost(bytes="vec", tag="40")]
    pub src_parentfolder: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="50")]
    pub src_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="60")]
    pub file_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="70")]
    pub dst_uin: i64,
    #[prost(int64, tag="80")]
    pub file_size: i64,
    #[prost(string, tag="90")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(int32, tag="100")]
    pub danger_level: i32,
    #[prost(int64, tag="110")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCopyFromRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="30")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="40")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCopyToReq {
    #[prost(int64, tag="10")]
    pub dst_id: i64,
    #[prost(int64, tag="20")]
    pub dst_uin: i64,
    #[prost(int32, tag="30")]
    pub dst_svcid: i32,
    #[prost(int64, tag="40")]
    pub src_uin: i64,
    #[prost(int64, tag="50")]
    pub file_size: i64,
    #[prost(string, tag="60")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag="70")]
    pub local_filepath: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="80")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyCopyToRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub file_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyDownloadAbsReq {
    #[prost(int64, tag="10")]
    pub uin: i64,
    #[prost(bytes="vec", tag="20")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyDownloadAbsRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="30")]
    pub download_info: ::core::option::Option<DownloadInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyDownloadReq {
    #[prost(int64, tag="10")]
    pub uin: i64,
    #[prost(bytes="vec", tag="20")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="30")]
    pub owner_type: i32,
    #[prost(int32, tag="500")]
    pub ext_intype: i32,
    #[prost(int32, tag="501")]
    pub need_https_url: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyDownloadRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="30")]
    pub download_info: ::core::option::Option<DownloadInfo>,
    #[prost(message, optional, tag="40")]
    pub file_info: ::core::option::Option<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyForwardFileReq {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(bytes="vec", tag="30")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="40")]
    pub danger_level: i32,
    #[prost(int64, tag="50")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyForwardFileRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="30")]
    pub total_space: i64,
    #[prost(int64, tag="40")]
    pub used_space: i64,
    #[prost(bytes="vec", tag="50")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyGetTrafficReq {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyGetTrafficRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="30")]
    pub use_file_size: i64,
    #[prost(int32, tag="40")]
    pub use_file_num: i32,
    #[prost(int64, tag="50")]
    pub all_file_size: i64,
    #[prost(int32, tag="60")]
    pub all_file_num: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyListDownloadReq {
    #[prost(int64, tag="10")]
    pub uin: i64,
    #[prost(int32, tag="20")]
    pub begin_index: i32,
    #[prost(int32, tag="30")]
    pub req_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyListDownloadRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int32, tag="30")]
    pub total_count: i32,
    #[prost(int32, tag="40")]
    pub begin_index: i32,
    #[prost(int32, tag="50")]
    pub rsp_count: i32,
    #[prost(int32, tag="60")]
    pub is_end: i32,
    #[prost(message, repeated, tag="70")]
    pub file_list: ::prost::alloc::vec::Vec<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadHitReq {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(int64, tag="30")]
    pub file_size: i64,
    #[prost(string, tag="40")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="50")]
    pub bytes_10m_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="60")]
    pub local_filepath: ::prost::alloc::string::String,
    #[prost(int32, tag="70")]
    pub danger_level: i32,
    #[prost(int64, tag="80")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadHitReqV2 {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(int64, tag="30")]
    pub file_size: i64,
    #[prost(string, tag="40")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="50")]
    pub bytes_10m_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="60")]
    pub bytes_3sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="70")]
    pub sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="80")]
    pub local_filepath: ::prost::alloc::string::String,
    #[prost(int32, tag="90")]
    pub danger_level: i32,
    #[prost(int64, tag="100")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadHitReqV3 {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(int64, tag="30")]
    pub file_size: i64,
    #[prost(string, tag="40")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="50")]
    pub bytes_10m_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="60")]
    pub sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="70")]
    pub local_filepath: ::prost::alloc::string::String,
    #[prost(int32, tag="80")]
    pub danger_level: i32,
    #[prost(int64, tag="90")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadHitRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub upload_ip: ::prost::alloc::string::String,
    #[prost(int32, tag="40")]
    pub upload_port: i32,
    #[prost(string, tag="50")]
    pub upload_domain: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="60")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="70")]
    pub upload_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="80")]
    pub total_space: i64,
    #[prost(int64, tag="90")]
    pub used_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadHitRspV2 {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub upload_ip: ::prost::alloc::string::String,
    #[prost(int32, tag="40")]
    pub upload_port: i32,
    #[prost(string, tag="50")]
    pub upload_domain: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="60")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="70")]
    pub upload_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="80")]
    pub total_space: i64,
    #[prost(int64, tag="90")]
    pub used_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadHitRspV3 {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub upload_ip: ::prost::alloc::string::String,
    #[prost(int32, tag="40")]
    pub upload_port: i32,
    #[prost(string, tag="50")]
    pub upload_domain: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="60")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="70")]
    pub upload_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="80")]
    pub total_space: i64,
    #[prost(int64, tag="90")]
    pub used_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadReq {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(int32, tag="30")]
    pub file_type: i32,
    #[prost(int64, tag="40")]
    pub file_size: i64,
    #[prost(string, tag="50")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="60")]
    pub bytes_10m_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="70")]
    pub local_filepath: ::prost::alloc::string::String,
    #[prost(int32, tag="80")]
    pub danger_level: i32,
    #[prost(int64, tag="90")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadReqV2 {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(int64, tag="30")]
    pub file_size: i64,
    #[prost(string, tag="40")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="50")]
    pub bytes_10m_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="60")]
    pub bytes_3sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="70")]
    pub local_filepath: ::prost::alloc::string::String,
    #[prost(int32, tag="80")]
    pub danger_level: i32,
    #[prost(int64, tag="90")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadReqV3 {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(int64, tag="30")]
    pub file_size: i64,
    #[prost(string, tag="40")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="50")]
    pub bytes_10m_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="60")]
    pub sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="70")]
    pub local_filepath: ::prost::alloc::string::String,
    #[prost(int32, tag="80")]
    pub danger_level: i32,
    #[prost(int64, tag="90")]
    pub total_space: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="30")]
    pub total_space: i64,
    #[prost(int64, tag="40")]
    pub used_space: i64,
    #[prost(int64, tag="50")]
    pub uploaded_size: i64,
    #[prost(string, tag="60")]
    pub upload_ip: ::prost::alloc::string::String,
    #[prost(string, tag="70")]
    pub upload_domain: ::prost::alloc::string::String,
    #[prost(int32, tag="80")]
    pub upload_port: i32,
    #[prost(bytes="vec", tag="90")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="100")]
    pub upload_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="110")]
    pub bool_file_exist: bool,
    #[prost(int32, tag="120")]
    pub pack_size: i32,
    #[prost(string, repeated, tag="130")]
    pub uploadip_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadRspV2 {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="30")]
    pub total_space: i64,
    #[prost(int64, tag="40")]
    pub used_space: i64,
    #[prost(int64, tag="50")]
    pub uploaded_size: i64,
    #[prost(string, tag="60")]
    pub upload_ip: ::prost::alloc::string::String,
    #[prost(string, tag="70")]
    pub upload_domain: ::prost::alloc::string::String,
    #[prost(int32, tag="80")]
    pub upload_port: i32,
    #[prost(bytes="vec", tag="90")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="100")]
    pub upload_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="110")]
    pub bool_file_exist: bool,
    #[prost(int32, tag="120")]
    pub pack_size: i32,
    #[prost(string, repeated, tag="130")]
    pub uploadip_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag="140")]
    pub httpsvr_api_ver: i32,
    #[prost(bytes="vec", tag="141")]
    pub sha: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyUploadRspV3 {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int64, tag="30")]
    pub total_space: i64,
    #[prost(int64, tag="40")]
    pub used_space: i64,
    #[prost(int64, tag="50")]
    pub uploaded_size: i64,
    #[prost(string, tag="60")]
    pub upload_ip: ::prost::alloc::string::String,
    #[prost(string, tag="70")]
    pub upload_domain: ::prost::alloc::string::String,
    #[prost(int32, tag="80")]
    pub upload_port: i32,
    #[prost(bytes="vec", tag="90")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="100")]
    pub upload_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="110")]
    pub bool_file_exist: bool,
    #[prost(int32, tag="120")]
    pub pack_size: i32,
    #[prost(string, repeated, tag="130")]
    pub upload_ip_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag="140")]
    pub upload_https_port: i32,
    #[prost(string, tag="150")]
    pub upload_https_domain: ::prost::alloc::string::String,
    #[prost(string, tag="160")]
    pub upload_dns: ::prost::alloc::string::String,
    #[prost(string, tag="170")]
    pub upload_lanip: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelMessageReq {
    #[prost(int64, tag="1")]
    pub uin_sender: i64,
    #[prost(int64, tag="2")]
    pub uin_receiver: i64,
    #[prost(int32, tag="10")]
    pub time: i32,
    #[prost(int32, tag="20")]
    pub random: i32,
    #[prost(int32, tag="30")]
    pub seq_no: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFileReq {
    #[prost(int64, tag="10")]
    pub uin: i64,
    #[prost(int64, tag="20")]
    pub peer_uin: i64,
    #[prost(int32, tag="30")]
    pub delete_type: i32,
    #[prost(bytes="vec", tag="40")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFileRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadInfo {
    #[prost(bytes="vec", tag="10")]
    pub download_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="20")]
    pub download_ip: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub download_domain: ::prost::alloc::string::String,
    #[prost(int32, tag="40")]
    pub port: i32,
    #[prost(string, tag="50")]
    pub download_url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="60")]
    pub downloadip_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="70")]
    pub cookie: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadSuccReq {
    #[prost(int64, tag="10")]
    pub uin: i64,
    #[prost(bytes="vec", tag="20")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadSuccRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int32, tag="30")]
    pub down_stat: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionReq {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(int64, tag="2")]
    pub r#type: i64,
    #[prost(string, tag="3")]
    pub dst_phonenum: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub phone_convert_type: i32,
    #[prost(bytes="vec", tag="20")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="100")]
    pub route_id: i64,
    #[prost(message, optional, tag="90100")]
    pub del_message_req: ::core::option::Option<DelMessageReq>,
    #[prost(int32, tag="90200")]
    pub download_url_type: i32,
    #[prost(int32, tag="90300")]
    pub ptt_format: i32,
    #[prost(int32, tag="90400")]
    pub is_need_inner_ip: i32,
    #[prost(int32, tag="90500")]
    pub net_type: i32,
    #[prost(int32, tag="90600")]
    pub voice_type: i32,
    #[prost(int32, tag="90700")]
    pub file_type: i32,
    #[prost(int32, tag="90800")]
    pub ptt_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionRsp {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(int32, tag="2")]
    pub danger_evel: i32,
    #[prost(int64, tag="3")]
    pub file_size: i64,
    #[prost(int32, tag="4")]
    pub life_time: i32,
    #[prost(int32, tag="5")]
    pub upload_time: i32,
    #[prost(bytes="vec", tag="6")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(int32, tag="90")]
    pub abs_file_type: i32,
    #[prost(bytes="vec", tag="100")]
    pub bytes_10m_md5: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="101")]
    pub sha: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="110")]
    pub client_type: i32,
    #[prost(int64, tag="120")]
    pub owner_uin: i64,
    #[prost(int64, tag="121")]
    pub peer_uin: i64,
    #[prost(int32, tag="130")]
    pub expire_time: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileQueryReq {
    #[prost(int64, tag="10")]
    pub uin: i64,
    #[prost(bytes="vec", tag="20")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileQueryRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="30")]
    pub file_info: ::core::option::Option<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecallFileReq {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(bytes="vec", tag="2")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecallFileRsp {
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecvListQueryReq {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(int32, tag="2")]
    pub begin_index: i32,
    #[prost(int32, tag="3")]
    pub req_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecvListQueryRsp {
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub file_tot_count: i32,
    #[prost(int32, tag="4")]
    pub begin_index: i32,
    #[prost(int32, tag="5")]
    pub rsp_file_count: i32,
    #[prost(int32, tag="6")]
    pub is_end: i32,
    #[prost(message, repeated, tag="7")]
    pub file_list: ::prost::alloc::vec::Vec<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenewFileReq {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(bytes="vec", tag="2")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="3")]
    pub add_ttl: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenewFileRsp {
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C346ReqBody {
    #[prost(int32, tag="1")]
    pub cmd: i32,
    #[prost(int32, tag="2")]
    pub seq: i32,
    #[prost(message, optional, tag="3")]
    pub recv_list_query_req: ::core::option::Option<RecvListQueryReq>,
    #[prost(message, optional, tag="4")]
    pub send_list_query_req: ::core::option::Option<SendListQueryReq>,
    #[prost(message, optional, tag="5")]
    pub renew_file_req: ::core::option::Option<RenewFileReq>,
    #[prost(message, optional, tag="6")]
    pub recall_file_req: ::core::option::Option<RecallFileReq>,
    #[prost(message, optional, tag="7")]
    pub apply_upload_req: ::core::option::Option<ApplyUploadReq>,
    #[prost(message, optional, tag="8")]
    pub apply_upload_hit_req: ::core::option::Option<ApplyUploadHitReq>,
    #[prost(message, optional, tag="9")]
    pub apply_forward_file_req: ::core::option::Option<ApplyForwardFileReq>,
    #[prost(message, optional, tag="10")]
    pub upload_succ_req: ::core::option::Option<UploadSuccReq>,
    #[prost(message, optional, tag="11")]
    pub delete_file_req: ::core::option::Option<DeleteFileReq>,
    #[prost(message, optional, tag="12")]
    pub download_succ_req: ::core::option::Option<DownloadSuccReq>,
    #[prost(message, optional, tag="13")]
    pub apply_download_abs_req: ::core::option::Option<ApplyDownloadAbsReq>,
    #[prost(message, optional, tag="14")]
    pub apply_download_req: ::core::option::Option<ApplyDownloadReq>,
    #[prost(message, optional, tag="15")]
    pub apply_list_download_req: ::core::option::Option<ApplyListDownloadReq>,
    #[prost(message, optional, tag="16")]
    pub file_query_req: ::core::option::Option<FileQueryReq>,
    #[prost(message, optional, tag="17")]
    pub apply_copy_from_req: ::core::option::Option<ApplyCopyFromReq>,
    #[prost(message, optional, tag="18")]
    pub apply_upload_req_v2: ::core::option::Option<ApplyUploadReqV2>,
    #[prost(message, optional, tag="19")]
    pub apply_upload_req_v3: ::core::option::Option<ApplyUploadReqV3>,
    #[prost(message, optional, tag="20")]
    pub apply_upload_hit_req_v2: ::core::option::Option<ApplyUploadHitReqV2>,
    #[prost(message, optional, tag="21")]
    pub apply_upload_hit_req_v3: ::core::option::Option<ApplyUploadHitReqV3>,
    #[prost(int32, tag="101")]
    pub business_id: i32,
    #[prost(int32, tag="102")]
    pub client_type: i32,
    #[prost(message, optional, tag="90000")]
    pub apply_copy_to_req: ::core::option::Option<ApplyCopyToReq>,
    ///ApplyCleanTrafficReq applyCleanTrafficReq = 90001; empty message
    #[prost(message, optional, tag="90002")]
    pub apply_get_traffic_req: ::core::option::Option<ApplyGetTrafficReq>,
    #[prost(message, optional, tag="99999")]
    pub extension_req: ::core::option::Option<ExtensionReq>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C346RspBody {
    #[prost(int32, tag="1")]
    pub cmd: i32,
    #[prost(int32, tag="2")]
    pub seq: i32,
    #[prost(message, optional, tag="3")]
    pub recv_list_query_rsp: ::core::option::Option<RecvListQueryRsp>,
    #[prost(message, optional, tag="4")]
    pub send_list_query_rsp: ::core::option::Option<SendListQueryRsp>,
    #[prost(message, optional, tag="5")]
    pub renew_file_rsp: ::core::option::Option<RenewFileRsp>,
    #[prost(message, optional, tag="6")]
    pub recall_file_rsp: ::core::option::Option<RecallFileRsp>,
    #[prost(message, optional, tag="7")]
    pub apply_upload_rsp: ::core::option::Option<ApplyUploadRsp>,
    #[prost(message, optional, tag="8")]
    pub apply_upload_hit_rsp: ::core::option::Option<ApplyUploadHitRsp>,
    #[prost(message, optional, tag="9")]
    pub apply_forward_file_rsp: ::core::option::Option<ApplyForwardFileRsp>,
    #[prost(message, optional, tag="10")]
    pub upload_succ_rsp: ::core::option::Option<UploadSuccRsp>,
    #[prost(message, optional, tag="11")]
    pub delete_file_rsp: ::core::option::Option<DeleteFileRsp>,
    #[prost(message, optional, tag="12")]
    pub download_succ_rsp: ::core::option::Option<DownloadSuccRsp>,
    #[prost(message, optional, tag="13")]
    pub apply_download_abs_rsp: ::core::option::Option<ApplyDownloadAbsRsp>,
    #[prost(message, optional, tag="14")]
    pub apply_download_rsp: ::core::option::Option<ApplyDownloadRsp>,
    #[prost(message, optional, tag="15")]
    pub apply_list_download_rsp: ::core::option::Option<ApplyListDownloadRsp>,
    #[prost(message, optional, tag="16")]
    pub file_query_rsp: ::core::option::Option<FileQueryRsp>,
    #[prost(message, optional, tag="17")]
    pub apply_copy_from_rsp: ::core::option::Option<ApplyCopyFromRsp>,
    #[prost(message, optional, tag="18")]
    pub apply_upload_rsp_v2: ::core::option::Option<ApplyUploadRspV2>,
    #[prost(message, optional, tag="19")]
    pub apply_upload_rsp_v3: ::core::option::Option<ApplyUploadRspV3>,
    #[prost(message, optional, tag="20")]
    pub apply_upload_hit_rsp_v2: ::core::option::Option<ApplyUploadHitRspV2>,
    #[prost(message, optional, tag="21")]
    pub apply_upload_hit_rsp_v3: ::core::option::Option<ApplyUploadHitRspV3>,
    #[prost(int32, tag="101")]
    pub business_id: i32,
    #[prost(int32, tag="102")]
    pub client_type: i32,
    #[prost(message, optional, tag="90000")]
    pub apply_copy_to_rsp: ::core::option::Option<ApplyCopyToRsp>,
    #[prost(message, optional, tag="90001")]
    pub apply_clean_traffic_rsp: ::core::option::Option<ApplyCleanTrafficRsp>,
    #[prost(message, optional, tag="90002")]
    pub apply_get_traffic_rsp: ::core::option::Option<ApplyGetTrafficRsp>,
    #[prost(message, optional, tag="99999")]
    pub extension_rsp: ::core::option::Option<ExtensionRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendListQueryReq {
    #[prost(int64, tag="1")]
    pub uin: i64,
    #[prost(int32, tag="2")]
    pub begin_index: i32,
    #[prost(int32, tag="3")]
    pub req_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendListQueryRsp {
    #[prost(int32, tag="1")]
    pub ret_code: i32,
    #[prost(string, tag="2")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub file_tot_count: i32,
    #[prost(int32, tag="4")]
    pub begin_index: i32,
    #[prost(int32, tag="5")]
    pub rsp_file_count: i32,
    #[prost(int32, tag="6")]
    pub is_end: i32,
    #[prost(int64, tag="7")]
    pub tot_limit: i64,
    #[prost(int64, tag="8")]
    pub used_limit: i64,
    #[prost(message, repeated, tag="9")]
    pub file_list: ::prost::alloc::vec::Vec<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadSuccReq {
    #[prost(int64, tag="10")]
    pub sender_uin: i64,
    #[prost(int64, tag="20")]
    pub recver_uin: i64,
    #[prost(bytes="vec", tag="30")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadSuccRsp {
    #[prost(int32, tag="10")]
    pub ret_code: i32,
    #[prost(string, tag="20")]
    pub ret_msg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="30")]
    pub file_info: ::core::option::Option<FileInfo>,
}
