#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C501ReqBody {
    #[prost(message, optional, tag="1281")]
    pub req_body: ::core::option::Option<SubCmd0x501ReqBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C501RspBody {
    #[prost(message, optional, tag="1281")]
    pub rsp_body: ::core::option::Option<SubCmd0x501RspBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubCmd0x501ReqBody {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub idc_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub appid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub login_sig_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="5")]
    pub login_sig_ticket: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="6")]
    pub request_flag: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed="false", tag="7")]
    pub service_types: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag="8")]
    pub bid: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubCmd0x501RspBody {
    #[prost(bytes="vec", optional, tag="1")]
    pub sig_session: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub session_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="3")]
    pub addrs: ::prost::alloc::vec::Vec<SrvAddrs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SrvAddrs {
    #[prost(uint32, optional, tag="1")]
    pub service_type: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="2")]
    pub addrs: ::prost::alloc::vec::Vec<IpAddr>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddr {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="2")]
    pub ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub port: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub area: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C519crmMsgHead {
    #[prost(uint32, optional, tag="1")]
    pub crm_sub_cmd: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub head_len: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub ver_no: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub kf_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="5")]
    pub seq: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub pack_num: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub cur_pack: ::core::option::Option<u32>,
    #[prost(string, optional, tag="8")]
    pub buf_sig: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="9")]
    pub pub_qq: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="10")]
    pub clienttype: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="11")]
    pub labor_uin: ::core::option::Option<u64>,
    #[prost(string, optional, tag="12")]
    pub labor_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="13")]
    pub puin: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNavigationMenuReqBody {
    #[prost(uint64, optional, tag="1")]
    pub puin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub ver_no: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNavigationMenuRspBody {
    #[prost(message, optional, tag="1")]
    pub ret: ::core::option::Option<C519RetInfo>,
    #[prost(int32, optional, tag="2")]
    pub is_show: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub uct_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="4")]
    pub ver_no: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C519ReqBody {
    #[prost(uint32, optional, tag="1")]
    pub sub_cmd: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub crm_common_head: ::core::option::Option<C519crmMsgHead>,
    #[prost(message, optional, tag="33")]
    pub get_address_detail_list_req_body: ::core::option::Option<GetAddressDetailListReqBody>,
    #[prost(message, optional, tag="35")]
    pub get_navigation_menu_req: ::core::option::Option<GetNavigationMenuReqBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C519RetInfo {
    #[prost(uint32, optional, tag="1")]
    pub ret_code: ::core::option::Option<u32>,
    #[prost(string, optional, tag="2")]
    pub error_msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C519RspBody {
    #[prost(uint32, optional, tag="1")]
    pub sub_cmd: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub crm_common_head: ::core::option::Option<C519crmMsgHead>,
    #[prost(message, optional, tag="33")]
    pub get_address_detail_list_rsp_body: ::core::option::Option<GetAddressDetailListRspBody>,
    #[prost(message, optional, tag="35")]
    pub get_navigation_menu_rsp: ::core::option::Option<GetNavigationMenuRspBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressDetailListReqBody {
    #[prost(fixed32, optional, tag="1")]
    pub timestamp: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag="2")]
    pub timestamp2: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAddressDetailListRspBody {
    #[prost(message, optional, tag="1")]
    pub ret: ::core::option::Option<C519RetInfo>,
    #[prost(fixed32, optional, tag="2")]
    pub timestamp: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="3")]
    pub full: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="4")]
    pub address_detail: ::prost::alloc::vec::Vec<AddressDetail>,
    #[prost(fixed64, optional, tag="5")]
    pub timestamp2: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressDetail {
    #[prost(uint32, optional, tag="1")]
    pub aid: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="2")]
    pub modify_time: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="3")]
    pub create_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub status: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub groupid: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub add_group_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="8")]
    pub gender: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag="9")]
    pub birthday: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub company0: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub company_position0: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="12")]
    pub company1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="13")]
    pub company_position1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="14")]
    pub fixed_phone0: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="15")]
    pub fixed_phone1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="16")]
    pub email0: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="17")]
    pub email1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="18")]
    pub fax0: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="19")]
    pub fax1: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="20")]
    pub comment: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="21")]
    pub head_url: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="22")]
    pub mobile_phone: ::prost::alloc::vec::Vec<AddressMobileInfo>,
    #[prost(bool, optional, tag="23")]
    pub mobile_phone_updated: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="24")]
    pub qq: ::prost::alloc::vec::Vec<AddressQQinfo>,
    #[prost(bool, optional, tag="25")]
    pub qq_phone_updated: ::core::option::Option<bool>,
    #[prost(fixed64, optional, tag="26")]
    pub modify_time2: ::core::option::Option<u64>,
    #[prost(message, optional, tag="27")]
    pub client_region: ::core::option::Option<NewBizClientRegion>,
    #[prost(message, optional, tag="28")]
    pub client_region_code: ::core::option::Option<NewBizClientRegionCode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressMobileInfo {
    #[prost(uint32, optional, tag="1")]
    pub index: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub account: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub formatted_account: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressQQinfo {
    #[prost(uint32, optional, tag="1")]
    pub index: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub account: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBizClientRegion {
    #[prost(string, optional, tag="1")]
    pub client_nation: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub client_province: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub client_city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub client_region: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBizClientRegionCode {
    #[prost(uint64, optional, tag="1")]
    pub nationid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub provinceid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub cityid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub regionid: ::core::option::Option<u64>,
}
