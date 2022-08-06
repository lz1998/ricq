#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGroup {
    #[prost(uint32, optional, tag="1")]
    pub groupid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub sortid: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub groupname: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppointmentNotify {
    #[prost(uint64, optional, tag="1")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub appoint_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="3")]
    pub notifytype: ::core::option::Option<u32>,
    #[prost(string, optional, tag="4")]
    pub tips_content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="5")]
    pub unread_count: ::core::option::Option<u32>,
    #[prost(string, optional, tag="6")]
    pub join_wording: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub view_wording: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="8")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="9")]
    pub event_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="10")]
    pub nearby_event_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="11")]
    pub feed_event_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryMsg {
    #[prost(uint32, optional, tag="1")]
    pub op_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub op_value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMatchInfo {
    #[prost(bytes="vec", optional, tag="1")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="2")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub match_uin: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub tips_wording: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="5")]
    pub left_chat_time: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="6")]
    pub time_stamp: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="7")]
    pub match_expired_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub c2_c_expired_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub match_count: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="10")]
    pub nick: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfMsgRoamFlag {
    #[prost(uint64, optional, tag="1")]
    pub confid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub flag: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="3")]
    pub timestamp: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaRenNotify {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub login_days: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub days: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub is_yestoday_login: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub is_today_login: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelFriend {
    #[prost(uint64, repeated, packed="false", tag="1")]
    pub uins: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelGroup {
    #[prost(uint32, optional, tag="1")]
    pub groupid: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FanpaiziNotify {
    #[prost(uint64, optional, tag="1")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub from_nick: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="3")]
    pub tips_content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub sig: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardBody {
    #[prost(uint32, optional, tag="1")]
    pub notify_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub op_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="3")]
    pub add_group: ::core::option::Option<AddGroup>,
    #[prost(message, optional, tag="4")]
    pub del_group: ::core::option::Option<DelGroup>,
    #[prost(message, optional, tag="5")]
    pub mod_group_name: ::core::option::Option<ModGroupName>,
    #[prost(message, optional, tag="6")]
    pub mod_group_sort: ::core::option::Option<ModGroupSort>,
    #[prost(message, optional, tag="7")]
    pub mod_friend_group: ::core::option::Option<ModFriendGroup>,
    #[prost(message, optional, tag="8")]
    pub mod_profile: ::core::option::Option<ModProfile>,
    #[prost(message, optional, tag="9")]
    pub mod_friend_remark: ::core::option::Option<ModFriendRemark>,
    #[prost(message, optional, tag="10")]
    pub mod_long_nick: ::core::option::Option<ModLongNick>,
    #[prost(message, optional, tag="11")]
    pub mod_custom_face: ::core::option::Option<ModCustomFace>,
    #[prost(message, optional, tag="12")]
    pub mod_group_profile: ::core::option::Option<ModGroupProfile>,
    #[prost(message, optional, tag="13")]
    pub mod_group_member_profile: ::core::option::Option<ModGroupMemberProfile>,
    #[prost(message, optional, tag="14")]
    pub del_friend: ::core::option::Option<DelFriend>,
    #[prost(message, optional, tag="15")]
    pub roam_priv: ::core::option::Option<ModFrdRoamPriv>,
    #[prost(message, optional, tag="16")]
    pub grp_msg_roam_flag: ::core::option::Option<GrpMsgRoamFlag>,
    #[prost(message, optional, tag="17")]
    pub conf_msg_roam_flag: ::core::option::Option<ConfMsgRoamFlag>,
    #[prost(message, optional, tag="18")]
    pub mod_rich_long_nick: ::core::option::Option<ModLongNick>,
    #[prost(message, optional, tag="19")]
    pub bin_pkg: ::core::option::Option<BinaryMsg>,
    #[prost(message, optional, tag="20")]
    pub mod_friend_rings: ::core::option::Option<ModSnsGeneralInfo>,
    #[prost(message, optional, tag="21")]
    pub mod_conf_profile: ::core::option::Option<ModConfProfile>,
    #[prost(message, optional, tag="22")]
    pub mod_friend_flag: ::core::option::Option<SnsUpdateFlag>,
    #[prost(message, optional, tag="23")]
    pub appointment_notify: ::core::option::Option<AppointmentNotify>,
    #[prost(message, optional, tag="25")]
    pub daren_notify: ::core::option::Option<DaRenNotify>,
    #[prost(message, optional, tag="26")]
    pub new_comein_user_notify: ::core::option::Option<NewComeinUserNotify>,
    #[prost(message, optional, tag="200")]
    pub push_search_dev: ::core::option::Option<PushSearchDev>,
    #[prost(message, optional, tag="201")]
    pub push_report_dev: ::core::option::Option<PushReportDev>,
    #[prost(message, optional, tag="202")]
    pub qq_pay_push: ::core::option::Option<QqPayPush>,
    #[prost(bytes="vec", optional, tag="203")]
    pub redpoint_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="204")]
    pub hot_friend_notify: ::core::option::Option<HotFriendNotify>,
    #[prost(message, optional, tag="205")]
    pub praise_rank_notify: ::core::option::Option<PraiseRankNotify>,
    #[prost(message, optional, tag="210")]
    pub campus_notify: ::core::option::Option<MqqCampusNotify>,
    #[prost(message, optional, tag="211")]
    pub mod_rich_long_nick_ex: ::core::option::Option<ModLongNick>,
    #[prost(message, optional, tag="212")]
    pub chat_match_info: ::core::option::Option<ChatMatchInfo>,
    #[prost(message, optional, tag="214")]
    pub frd_custom_online_status_change: ::core::option::Option<FrdCustomOnlineStatusChange>,
    #[prost(message, optional, tag="2000")]
    pub fanpanzi_notify: ::core::option::Option<FanpaiziNotify>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrdCustomOnlineStatusChange {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendGroup {
    #[prost(uint64, optional, tag="1")]
    pub fuin: ::core::option::Option<u64>,
    #[prost(uint32, repeated, packed="false", tag="2")]
    pub old_group_id: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed="false", tag="3")]
    pub new_group_id: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendRemark {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub fuin: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="3")]
    pub rmk_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="4")]
    pub group_code: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gps {
    #[prost(int32, optional, tag="1")]
    pub lat: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub lon: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub alt: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub r#type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupMemberProfileInfo {
    #[prost(uint32, optional, tag="1")]
    pub field: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupProfileInfo {
    #[prost(uint32, optional, tag="1")]
    pub field: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupSort {
    #[prost(uint32, optional, tag="1")]
    pub groupid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2")]
    pub sortid: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpMsgRoamFlag {
    #[prost(uint64, optional, tag="1")]
    pub groupcode: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub flag: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="3")]
    pub timestamp: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotFriendNotify {
    #[prost(uint64, optional, tag="1")]
    pub dst_uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub praise_hot_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub chat_hot_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub praise_hot_days: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub chat_hot_days: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub close_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub close_days: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="8")]
    pub praise_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="9")]
    pub chat_flag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub close_flag: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="11")]
    pub notify_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="12")]
    pub last_praise_time: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="13")]
    pub last_chat_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="14")]
    pub qzone_hot_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="15")]
    pub qzone_hot_days: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="16")]
    pub qzone_flag: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="17")]
    pub last_qzone_time: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MqqCampusNotify {
    #[prost(uint64, optional, tag="1")]
    pub from_uin: ::core::option::Option<u64>,
    #[prost(string, optional, tag="2")]
    pub wording: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub target: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="4")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(string, optional, tag="5")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModConfProfile {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub conf_uin: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="3")]
    pub profile_infos: ::prost::alloc::vec::Vec<ProfileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModCustomFace {
    #[prost(uint32, optional, tag="1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="2")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub cmd_uin: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModFrdRoamPriv {
    #[prost(message, repeated, tag="1")]
    pub roam_priv: ::prost::alloc::vec::Vec<OneRoamPriv>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModFriendGroup {
    #[prost(message, repeated, tag="1")]
    pub frd_group: ::prost::alloc::vec::Vec<FriendGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModFriendRemark {
    #[prost(message, repeated, tag="1")]
    pub frd_rmk: ::prost::alloc::vec::Vec<FriendRemark>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModGroupMemberProfile {
    #[prost(uint64, optional, tag="1")]
    pub group_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub uin: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="3")]
    pub group_member_profile_infos: ::prost::alloc::vec::Vec<GroupMemberProfileInfo>,
    #[prost(uint64, optional, tag="4")]
    pub group_code: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModGroupName {
    #[prost(uint32, optional, tag="1")]
    pub groupid: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub groupname: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModGroupProfile {
    #[prost(uint64, optional, tag="1")]
    pub group_uin: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub group_profile_infos: ::prost::alloc::vec::Vec<GroupProfileInfo>,
    #[prost(uint64, optional, tag="3")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub cmd_uin: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModGroupSort {
    #[prost(message, repeated, tag="1")]
    pub groupsort: ::prost::alloc::vec::Vec<GroupSort>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModLongNick {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModProfile {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(message, repeated, tag="2")]
    pub profile_infos: ::prost::alloc::vec::Vec<ProfileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModSnsGeneralInfo {
    #[prost(message, repeated, tag="1")]
    pub sns_general_infos: ::prost::alloc::vec::Vec<SnsUpateBuffer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubMsg0x27Body {
    #[prost(message, repeated, tag="1")]
    pub mod_infos: ::prost::alloc::vec::Vec<ForwardBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewComeinUser {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub is_frd: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="3")]
    pub remark: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub nick: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewComeinUserNotify {
    #[prost(uint32, optional, tag="1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="2")]
    pub ong_notify: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag="3")]
    pub push_time: ::core::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub new_comein_user: ::core::option::Option<NewComeinUser>,
    #[prost(message, optional, tag="5")]
    pub new_group: ::core::option::Option<NewGroup>,
    #[prost(message, optional, tag="6")]
    pub new_group_user: ::core::option::Option<NewGroupUser>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGroup {
    #[prost(uint64, optional, tag="1")]
    pub group_code: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="2")]
    pub group_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="3")]
    pub owner_uin: ::core::option::Option<u64>,
    #[prost(bytes="vec", optional, tag="4")]
    pub owner_nick: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub distance: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGroupUser {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(int32, optional, tag="2")]
    pub sex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub age: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub nick: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="5")]
    pub distance: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneRoamPriv {
    #[prost(uint64, optional, tag="1")]
    pub fuin: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="2")]
    pub priv_tag: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub priv_value: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PraiseRankNotify {
    #[prost(uint32, optional, tag="11")]
    pub is_champion: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="12")]
    pub rank_num: ::core::option::Option<u32>,
    #[prost(string, optional, tag="13")]
    pub msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileInfo {
    #[prost(uint32, optional, tag="1")]
    pub field: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushReportDev {
    #[prost(uint32, optional, tag="1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub cookie: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="5")]
    pub report_max_num: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub sn: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushSearchDev {
    #[prost(uint32, optional, tag="1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub gps_info: ::core::option::Option<Gps>,
    #[prost(uint32, optional, tag="3")]
    pub dev_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub push_time: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag="5")]
    pub din: ::core::option::Option<u64>,
    #[prost(string, optional, tag="6")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QqPayPush {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(bool, optional, tag="2")]
    pub pay_ok: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnsUpateBuffer {
    #[prost(uint64, optional, tag="1")]
    pub uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub code: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub result: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="400")]
    pub sns_update_item: ::prost::alloc::vec::Vec<SnsUpdateItem>,
    #[prost(uint32, repeated, packed="false", tag="401")]
    pub idlist: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnsUpdateFlag {
    #[prost(message, repeated, tag="1")]
    pub update_sns_flag: ::prost::alloc::vec::Vec<SnsUpdateOneFlag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnsUpdateItem {
    #[prost(uint32, optional, tag="1")]
    pub update_sns_type: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnsUpdateOneFlag {
    #[prost(uint64, optional, tag="1")]
    pub x_uin: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub flag: ::core::option::Option<u32>,
}
