#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoStateBizInfo {
    #[prost(uint64, optional, tag="1")]
    pub update_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomStatus {
    #[prost(uint64, optional, tag="1")]
    pub face_index: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub wording: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub face_type: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeatherBizInfo {
    #[prost(string, optional, tag="1")]
    pub weather_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub weather_type_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub adcode: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="4")]
    pub update_time: ::core::option::Option<u64>,
    #[prost(string, optional, tag="5")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub area: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub temper: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="8")]
    pub flag: ::core::option::Option<u32>,
    #[prost(string, optional, tag="9")]
    pub weather_desc: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZodiacBizInfo {
    #[prost(string, optional, tag="1")]
    pub today_trend: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub tomorrow_trend: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub miniapp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub today_date: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub lucky_color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub lucky_number: ::core::option::Option<::prost::alloc::string::String>,
}
