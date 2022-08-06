#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqBody {
    #[prost(uint32, optional, tag="1")]
    pub subcmd: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="2")]
    pub tryup_img_req: ::prost::alloc::vec::Vec<D352TryUpImgReq>,
    /// repeated GetImgUrlReq getimgUrlReq = 3;
    /// repeated DelImgReq delImgReq = 4;
    #[prost(uint32, optional, tag="10")]
    pub net_type: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspBody {
    #[prost(uint32, optional, tag="1")]
    pub subcmd: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="2")]
    pub tryup_img_rsp: ::prost::alloc::vec::Vec<TryUpImgRsp>,
    /// repeated GetImgUrlRsp getimgUrlRsp = 3;
    #[prost(bool, optional, tag="4")]
    pub new_bigchan: ::core::option::Option<bool>,
    /// repeated DelImgRsp delImgRsp = 5;
    #[prost(bytes="vec", optional, tag="10")]
    pub fail_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct D352TryUpImgReq {
    #[prost(uint64, optional, tag="1")]
    pub src_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub dst_uin: ::core::option::Option<u64>,
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
    pub inner_ip: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="10")]
    pub address_book: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="11")]
    pub retry: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub bu_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="13")]
    pub pic_original: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="14")]
    pub pic_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub pic_height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub pic_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="17")]
    pub build_ver: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="18")]
    pub file_index: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="19")]
    pub store_days: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="20")]
    pub tryup_stepflag: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="21")]
    pub reject_tryfast: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="22")]
    pub srv_upload: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="23")]
    pub transfer_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TryUpImgRsp {
    #[prost(uint64, optional, tag="1")]
    pub file_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub client_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub result: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub fail_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag="5")]
    pub file_exit: ::core::option::Option<bool>,
    /// optional ImgInfo imgInfo = 6;
    #[prost(uint32, repeated, packed="false", tag="7")]
    pub up_ip: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="8")]
    pub up_port: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes="vec", optional, tag="9")]
    pub up_ukey: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="10")]
    pub up_resid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="11")]
    pub up_uuid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="12")]
    pub up_offset: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="13")]
    pub block_size: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="14")]
    pub encrypt_dstip: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="15")]
    pub roamdays: ::core::option::Option<u32>,
    /// repeated IPv6Info upIp6 = 26;
    #[prost(bytes="vec", optional, tag="27")]
    pub client_ip6: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="60")]
    pub thumb_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="61")]
    pub original_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="62")]
    pub down_domain: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="64")]
    pub big_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="65")]
    pub big_thumb_down_para: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// optional TryUpInfo4Busi info4Busi = 1001;
    #[prost(uint32, optional, tag="66")]
    pub https_url_flag: ::core::option::Option<u32>,
}
