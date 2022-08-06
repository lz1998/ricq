#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelImgReq {
    #[prost(uint64, optional, tag="1")]
    pub src_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub dst_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub req_term: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub req_platform_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub bu_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub build_ver: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub file_resid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="8")]
    pub pic_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub pic_height: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelImgRsp {
    #[prost(uint32, optional, tag="1")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub fail_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub file_resid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpRoamExtendInfo {
    #[prost(bytes="vec", optional, tag="1")]
    pub resid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpRoamPicInfo {
    #[prost(uint32, optional, tag="1")]
    pub shop_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub pkg_id: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub pic_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionCommPicTryUp {
    #[prost(bytes="vec", repeated, tag="1")]
    pub extinfo: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionExpRoamTryUp {
    #[prost(message, repeated, tag="1")]
    pub exproam_pic_info: ::prost::alloc::vec::Vec<ExpRoamPicInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImgUrlReq {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub dst_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub fileid: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="5")]
    pub url_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub url_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub req_term: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub req_platform_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub inner_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub bu_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="11")]
    pub build_ver: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="12")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="13")]
    pub file_size: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="14")]
    pub original_pic: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub retry_req: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub file_height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="17")]
    pub file_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="18")]
    pub pic_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="19")]
    pub pic_up_timestamp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="20")]
    pub req_transfer_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="21")]
    pub qqmeet_guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="22")]
    pub qqmeet_channel_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="23")]
    pub download_index: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImgUrlRsp {
    #[prost(uint64, optional, tag="1")]
    pub fileid: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub fail_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="5")]
    pub img_info: ::core::option::Option<ImgInfo>,
    #[prost(bytes="vec", repeated, tag="6")]
    pub thumb_down_url: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="7")]
    pub original_down_url: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="8")]
    pub big_down_url: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, packed="false", tag="9")]
    pub down_ip: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="10")]
    pub down_port: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", optional, tag="11")]
    pub down_domain: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="12")]
    pub thumb_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="13")]
    pub original_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="14")]
    pub big_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="15")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="16")]
    pub auto_down_type: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed="false", tag="17")]
    pub order_down_type: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", optional, tag="19")]
    pub big_thumb_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="20")]
    pub https_url_flag: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="26")]
    pub down_ip6: ::prost::alloc::vec::Vec<IPv6Info>,
    #[prost(bytes="vec", optional, tag="27")]
    pub client_ip6: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPttUrlReq {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub dst_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub fileid: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="5")]
    pub req_term: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub req_platform_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub inner_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub bu_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="9")]
    pub build_ver: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="10")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="11")]
    pub file_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="12")]
    pub codec: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="13")]
    pub bu_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="14")]
    pub req_transfer_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub is_auto: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPttUrlRsp {
    #[prost(uint64, optional, tag="1")]
    pub fileid: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="3")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub fail_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="5")]
    pub down_url: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, packed="false", tag="6")]
    pub down_ip: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="7")]
    pub down_port: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub down_domain: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="9")]
    pub down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="10")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="11")]
    pub transfer_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub allow_retry: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="26")]
    pub down_ip6: ::prost::alloc::vec::Vec<IPv6Info>,
    #[prost(bytes="vec", optional, tag="27")]
    pub client_ip6: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="28")]
    pub domain: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IPv6Info {
    #[prost(bytes="vec", optional, tag="1")]
    pub ip6: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="2")]
    pub port: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImgInfo {
    #[prost(bytes="vec", optional, tag="1")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="2")]
    pub file_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="3")]
    pub file_size: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="4")]
    pub file_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub file_height: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PicSize {
    #[prost(uint32, optional, tag="1")]
    pub original: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub thumb: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub high: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D388ReqBody {
    #[prost(uint32, optional, tag="1")]
    pub net_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub subcmd: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="3")]
    pub tryup_img_req: ::prost::alloc::vec::Vec<TryUpImgReq>,
    #[prost(message, repeated, tag="4")]
    pub getimg_url_req: ::prost::alloc::vec::Vec<GetImgUrlReq>,
    #[prost(message, repeated, tag="5")]
    pub tryup_ptt_req: ::prost::alloc::vec::Vec<TryUpPttReq>,
    #[prost(message, repeated, tag="6")]
    pub getptt_url_req: ::prost::alloc::vec::Vec<GetPttUrlReq>,
    #[prost(uint32, optional, tag="7")]
    pub command_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="8")]
    pub del_img_req: ::prost::alloc::vec::Vec<DelImgReq>,
    #[prost(bytes="vec", optional, tag="1001")]
    pub extension: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D388RspBody {
    #[prost(uint32, optional, tag="1")]
    pub client_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub subcmd: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="3")]
    pub tryup_img_rsp: ::prost::alloc::vec::Vec<D388TryUpImgRsp>,
    #[prost(message, repeated, tag="4")]
    pub getimg_url_rsp: ::prost::alloc::vec::Vec<GetImgUrlRsp>,
    #[prost(message, repeated, tag="5")]
    pub tryup_ptt_rsp: ::prost::alloc::vec::Vec<TryUpPttRsp>,
    #[prost(message, repeated, tag="6")]
    pub getptt_url_rsp: ::prost::alloc::vec::Vec<GetPttUrlRsp>,
    #[prost(message, repeated, tag="7")]
    pub del_img_rsp: ::prost::alloc::vec::Vec<DelImgRsp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpImgReq {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub src_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="5")]
    pub file_size: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="6")]
    pub file_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="7")]
    pub src_term: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub platform_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub bu_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub pic_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="11")]
    pub pic_height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub pic_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="13")]
    pub build_ver: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="14")]
    pub inner_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub app_pic_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub original_pic: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="17")]
    pub file_index: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="18")]
    pub dst_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="19")]
    pub srv_upload: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="20")]
    pub transfer_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="21")]
    pub qqmeet_guild_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="22")]
    pub qqmeet_channel_id: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D388TryUpImgRsp {
    #[prost(uint64, optional, tag="1")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub fail_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="4")]
    pub file_exit: ::core::option::Option<bool>,
    #[prost(message, optional, tag="5")]
    pub img_info: ::core::option::Option<ImgInfo>,
    #[prost(uint32, repeated, packed="false", tag="6")]
    pub up_ip: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="7")]
    pub up_port: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", optional, tag="8")]
    pub up_ukey: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="9")]
    pub fileid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="10")]
    pub up_offset: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="11")]
    pub block_size: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="12")]
    pub new_big_chan: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="26")]
    pub up_ip6: ::prost::alloc::vec::Vec<IPv6Info>,
    #[prost(bytes="vec", optional, tag="27")]
    pub client_ip6: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="28")]
    pub download_index: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="1001")]
    pub info4_busi: ::core::option::Option<TryUpInfo4Busi>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpInfo4Busi {
    #[prost(bytes="vec", optional, tag="1")]
    pub down_domain: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub thumb_down_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub original_down_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub big_down_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub file_resid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpPttReq {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub src_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub file_md5: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="5")]
    pub file_size: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="6")]
    pub file_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="7")]
    pub src_term: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub platform_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub bu_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub build_ver: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="11")]
    pub inner_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub voice_length: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="13")]
    pub new_up_chan: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="14")]
    pub codec: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub voice_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub bu_id: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpPttRsp {
    #[prost(uint64, optional, tag="1")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub fail_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="4")]
    pub file_exit: ::core::option::Option<bool>,
    #[prost(uint32, repeated, packed="false", tag="5")]
    pub up_ip: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="6")]
    pub up_port: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", optional, tag="7")]
    pub up_ukey: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="8")]
    pub fileid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="9")]
    pub up_offset: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="10")]
    pub block_size: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="11")]
    pub file_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="12")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="26")]
    pub up_ip6: ::prost::alloc::vec::Vec<IPv6Info>,
    #[prost(bytes="vec", optional, tag="27")]
    pub client_ip6: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
