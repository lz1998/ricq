#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFrdSnInfo {
    #[prost(int32, tag="1")]
    pub not_see_dynamic: i32,
    #[prost(int32, tag="2")]
    pub set_sn: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlagInfo {
    #[prost(int32, tag="1")]
    pub grp_msg_kick_admin: i32,
    #[prost(int32, tag="2")]
    pub grp_msg_hidden_grp: i32,
    #[prost(int32, tag="3")]
    pub grp_msg_wording_down: i32,
    #[prost(int32, tag="4")]
    pub frd_msg_get_busi_card: i32,
    #[prost(int32, tag="5")]
    pub grp_msg_get_official_account: i32,
    #[prost(int32, tag="6")]
    pub grp_msg_get_pay_in_group: i32,
    #[prost(int32, tag="7")]
    pub frd_msg_discuss2_many_chat: i32,
    #[prost(int32, tag="8")]
    pub grp_msg_not_allow_join_grp_invite_not_frd: i32,
    #[prost(int32, tag="9")]
    pub frd_msg_need_waiting_msg: i32,
    #[prost(int32, tag="10")]
    pub frd_msg_uint32_need_all_unread_msg: i32,
    #[prost(int32, tag="11")]
    pub grp_msg_need_auto_admin_wording: i32,
    #[prost(int32, tag="12")]
    pub grp_msg_get_transfer_group_msg_flag: i32,
    #[prost(int32, tag="13")]
    pub grp_msg_get_quit_pay_group_msg_flag: i32,
    #[prost(int32, tag="14")]
    pub grp_msg_support_invite_auto_join: i32,
    #[prost(int32, tag="15")]
    pub grp_msg_mask_invite_auto_join: i32,
    #[prost(int32, tag="16")]
    pub grp_msg_get_disbanded_by_admin: i32,
    #[prost(int32, tag="17")]
    pub grp_msg_get_c2c_invite_join_group: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendInfo {
    #[prost(string, tag="1")]
    pub msg_joint_friend: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub msg_blacklist: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SGroupInfo {
    #[prost(int32, tag="1")]
    pub group_auth_type: i32,
    #[prost(int32, tag="2")]
    pub display_action: i32,
    #[prost(string, tag="3")]
    pub msg_alert: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub msg_detail_alert: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub msg_other_admin_done: ::prost::alloc::string::String,
    #[prost(int32, tag="6")]
    pub app_privilege_flag: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInviteExt {
    #[prost(int32, tag="1")]
    pub src_type: i32,
    #[prost(int64, tag="2")]
    pub src_code: i64,
    #[prost(int32, tag="3")]
    pub wait_state: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayGroupExt {
    #[prost(int64, tag="1")]
    pub join_grp_time: i64,
    #[prost(int64, tag="2")]
    pub quit_grp_time: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqNextSystemMsg {
    #[prost(int32, tag="1")]
    pub msg_num: i32,
    #[prost(int64, tag="2")]
    pub following_friend_seq: i64,
    #[prost(int64, tag="3")]
    pub following_group_seq: i64,
    #[prost(int32, tag="4")]
    pub checktype: i32,
    #[prost(message, optional, tag="5")]
    pub flag: ::core::option::Option<FlagInfo>,
    #[prost(int32, tag="6")]
    pub language: i32,
    #[prost(int32, tag="7")]
    pub version: i32,
    #[prost(int64, tag="8")]
    pub friend_msg_type_flag: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSystemMsg {
    #[prost(int32, tag="1")]
    pub msg_num: i32,
    #[prost(int64, tag="2")]
    pub latest_friend_seq: i64,
    #[prost(int64, tag="3")]
    pub latest_group_seq: i64,
    #[prost(int32, tag="4")]
    pub version: i32,
    #[prost(int32, tag="5")]
    pub language: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSystemMsgAction {
    #[prost(int32, tag="1")]
    pub msg_type: i32,
    #[prost(int64, tag="2")]
    pub msg_seq: i64,
    #[prost(int64, tag="3")]
    pub req_uin: i64,
    #[prost(int32, tag="4")]
    pub sub_type: i32,
    #[prost(int32, tag="5")]
    pub src_id: i32,
    #[prost(int32, tag="6")]
    pub sub_src_id: i32,
    #[prost(int32, tag="7")]
    pub group_msg_type: i32,
    #[prost(message, optional, tag="8")]
    pub action_info: ::core::option::Option<SystemMsgActionInfo>,
    #[prost(int32, tag="9")]
    pub language: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSystemMsgNew {
    #[prost(int32, tag="1")]
    pub msg_num: i32,
    #[prost(int64, tag="2")]
    pub latest_friend_seq: i64,
    #[prost(int64, tag="3")]
    pub latest_group_seq: i64,
    #[prost(int32, tag="4")]
    pub version: i32,
    #[prost(int32, tag="5")]
    pub checktype: i32,
    #[prost(message, optional, tag="6")]
    pub flag: ::core::option::Option<FlagInfo>,
    #[prost(int32, tag="7")]
    pub language: i32,
    #[prost(bool, tag="8")]
    pub is_get_frd_ribbon: bool,
    #[prost(bool, tag="9")]
    pub is_get_grp_ribbon: bool,
    #[prost(int64, tag="10")]
    pub friend_msg_type_flag: i64,
    #[prost(int32, tag="11")]
    pub req_msg_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqSystemMsgRead {
    #[prost(int64, tag="1")]
    pub latest_friend_seq: i64,
    #[prost(int64, tag="2")]
    pub latest_group_seq: i64,
    #[prost(int32, tag="3")]
    pub r#type: i32,
    #[prost(int32, tag="4")]
    pub checktype: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspHead {
    #[prost(int32, tag="1")]
    pub result: i32,
    #[prost(string, tag="2")]
    pub msg_fail: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspNextSystemMsg {
    #[prost(message, optional, tag="1")]
    pub head: ::core::option::Option<RspHead>,
    #[prost(message, repeated, tag="2")]
    pub msgs: ::prost::alloc::vec::Vec<StructMsg>,
    #[prost(int64, tag="3")]
    pub following_friend_seq: i64,
    #[prost(int64, tag="4")]
    pub following_group_seq: i64,
    #[prost(int32, tag="5")]
    pub checktype: i32,
    #[prost(string, tag="100")]
    pub game_nick: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="101")]
    pub undecid_for_qim: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="102")]
    pub un_read_count3: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSystemMsg {
    #[prost(message, optional, tag="1")]
    pub head: ::core::option::Option<RspHead>,
    #[prost(message, repeated, tag="2")]
    pub msgs: ::prost::alloc::vec::Vec<StructMsg>,
    #[prost(int32, tag="3")]
    pub unread_count: i32,
    #[prost(int64, tag="4")]
    pub latest_friend_seq: i64,
    #[prost(int64, tag="5")]
    pub latest_group_seq: i64,
    #[prost(int64, tag="6")]
    pub following_friend_seq: i64,
    #[prost(int64, tag="7")]
    pub following_group_seq: i64,
    #[prost(string, tag="8")]
    pub msg_display: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSystemMsgAction {
    #[prost(message, optional, tag="1")]
    pub head: ::core::option::Option<RspHead>,
    #[prost(string, tag="2")]
    pub msg_detail: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub r#type: i32,
    #[prost(string, tag="5")]
    pub msg_invalid_decided: ::prost::alloc::string::String,
    #[prost(int32, tag="6")]
    pub remark_result: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSystemMsgNew {
    #[prost(message, optional, tag="1")]
    pub head: ::core::option::Option<RspHead>,
    #[prost(int32, tag="2")]
    pub unread_friend_count: i32,
    #[prost(int32, tag="3")]
    pub unread_group_count: i32,
    #[prost(int64, tag="4")]
    pub latest_friend_seq: i64,
    #[prost(int64, tag="5")]
    pub latest_group_seq: i64,
    #[prost(int64, tag="6")]
    pub following_friend_seq: i64,
    #[prost(int64, tag="7")]
    pub following_group_seq: i64,
    #[prost(message, repeated, tag="9")]
    pub friendmsgs: ::prost::alloc::vec::Vec<StructMsg>,
    #[prost(message, repeated, tag="10")]
    pub groupmsgs: ::prost::alloc::vec::Vec<StructMsg>,
    #[prost(message, optional, tag="11")]
    pub msg_ribbon_friend: ::core::option::Option<StructMsg>,
    #[prost(message, optional, tag="12")]
    pub msg_ribbon_group: ::core::option::Option<StructMsg>,
    #[prost(string, tag="13")]
    pub msg_display: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub grp_msg_display: ::prost::alloc::string::String,
    #[prost(int32, tag="15")]
    pub over: i32,
    #[prost(int32, tag="20")]
    pub checktype: i32,
    #[prost(string, tag="100")]
    pub game_nick: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="101")]
    pub undecid_for_qim: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="102")]
    pub un_read_count3: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RspSystemMsgRead {
    #[prost(message, optional, tag="1")]
    pub head: ::core::option::Option<RspHead>,
    #[prost(int32, tag="2")]
    pub r#type: i32,
    #[prost(int32, tag="3")]
    pub checktype: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructMsg {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(int32, tag="2")]
    pub msg_type: i32,
    #[prost(int64, tag="3")]
    pub msg_seq: i64,
    #[prost(int64, tag="4")]
    pub msg_time: i64,
    #[prost(int64, tag="5")]
    pub req_uin: i64,
    #[prost(int32, tag="6")]
    pub unread_flag: i32,
    #[prost(message, optional, tag="50")]
    pub msg: ::core::option::Option<SystemMsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemMsg {
    #[prost(int32, tag="1")]
    pub sub_type: i32,
    #[prost(string, tag="2")]
    pub msg_title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub msg_describe: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub msg_additional: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub msg_source: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub msg_decided: ::prost::alloc::string::String,
    #[prost(int32, tag="7")]
    pub src_id: i32,
    #[prost(int32, tag="8")]
    pub sub_src_id: i32,
    #[prost(message, repeated, tag="9")]
    pub actions: ::prost::alloc::vec::Vec<SystemMsgAction>,
    #[prost(int64, tag="10")]
    pub group_code: i64,
    #[prost(int64, tag="11")]
    pub action_uin: i64,
    #[prost(int32, tag="12")]
    pub group_msg_type: i32,
    #[prost(int32, tag="13")]
    pub group_inviter_role: i32,
    #[prost(message, optional, tag="14")]
    pub friend_info: ::core::option::Option<FriendInfo>,
    #[prost(message, optional, tag="15")]
    pub group_info: ::core::option::Option<SGroupInfo>,
    #[prost(int64, tag="16")]
    pub actor_uin: i64,
    #[prost(string, tag="17")]
    pub msg_actor_describe: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub msg_additional_list: ::prost::alloc::string::String,
    #[prost(int32, tag="19")]
    pub relation: i32,
    #[prost(int32, tag="20")]
    pub reqsubtype: i32,
    #[prost(int64, tag="21")]
    pub clone_uin: i64,
    #[prost(int64, tag="22")]
    pub discuss_uin: i64,
    #[prost(int64, tag="23")]
    pub eim_group_id: i64,
    #[prost(message, optional, tag="24")]
    pub msg_invite_extinfo: ::core::option::Option<MsgInviteExt>,
    #[prost(message, optional, tag="25")]
    pub msg_pay_group_extinfo: ::core::option::Option<MsgPayGroupExt>,
    #[prost(int32, tag="26")]
    pub source_flag: i32,
    #[prost(bytes="vec", tag="27")]
    pub game_nick: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="28")]
    pub game_msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="29")]
    pub group_flagext3: i32,
    #[prost(int64, tag="30")]
    pub group_owner_uin: i64,
    #[prost(int32, tag="31")]
    pub doubt_flag: i32,
    #[prost(bytes="vec", tag="32")]
    pub warning_tips: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="33")]
    pub name_more: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="50")]
    pub req_uin_faceid: i32,
    #[prost(string, tag="51")]
    pub req_uin_nick: ::prost::alloc::string::String,
    #[prost(string, tag="52")]
    pub group_name: ::prost::alloc::string::String,
    #[prost(string, tag="53")]
    pub action_uin_nick: ::prost::alloc::string::String,
    #[prost(string, tag="54")]
    pub msg_qna: ::prost::alloc::string::String,
    #[prost(string, tag="55")]
    pub msg_detail: ::prost::alloc::string::String,
    #[prost(int32, tag="57")]
    pub group_ext_flag: i32,
    #[prost(string, tag="58")]
    pub actor_uin_nick: ::prost::alloc::string::String,
    #[prost(string, tag="59")]
    pub pic_url: ::prost::alloc::string::String,
    #[prost(string, tag="60")]
    pub clone_uin_nick: ::prost::alloc::string::String,
    #[prost(string, tag="61")]
    pub req_uin_business_card: ::prost::alloc::string::String,
    #[prost(string, tag="63")]
    pub eim_group_id_name: ::prost::alloc::string::String,
    #[prost(string, tag="64")]
    pub req_uin_pre_remark: ::prost::alloc::string::String,
    #[prost(string, tag="65")]
    pub action_uin_qq_nick: ::prost::alloc::string::String,
    #[prost(string, tag="66")]
    pub action_uin_remark: ::prost::alloc::string::String,
    #[prost(int32, tag="67")]
    pub req_uin_gender: i32,
    #[prost(int32, tag="68")]
    pub req_uin_age: i32,
    #[prost(int32, tag="69")]
    pub c2c_invite_join_group_flag: i32,
    #[prost(int32, tag="101")]
    pub card_switch: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemMsgAction {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub result: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub action: i32,
    #[prost(message, optional, tag="4")]
    pub action_info: ::core::option::Option<SystemMsgActionInfo>,
    #[prost(string, tag="5")]
    pub detail_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemMsgActionInfo {
    #[prost(int32, tag="1")]
    pub r#type: i32,
    #[prost(int64, tag="2")]
    pub group_code: i64,
    #[prost(bytes="vec", tag="3")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="50")]
    pub msg: ::prost::alloc::string::String,
    #[prost(int32, tag="51")]
    pub group_id: i32,
    #[prost(string, tag="52")]
    pub remark: ::prost::alloc::string::String,
    #[prost(bool, tag="53")]
    pub blacklist: bool,
    #[prost(message, optional, tag="54")]
    pub add_frd_sn_info: ::core::option::Option<AddFrdSnInfo>,
}
