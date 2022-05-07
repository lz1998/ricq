use std::collections::HashMap;

use bytes::Bytes;
use jcers::{JceGet, JcePut};

macro_rules! JceStruct {
    ($struct_name: ident {$($tag: expr => $field: ident: $field_t: ty,)*}) => {
        #[derive(Debug, Clone, PartialEq, JceGet, JcePut, Default)]
        pub struct $struct_name {
            $(#[jce($tag)]
            pub $field: $field_t),*
        }
    };
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct RequestPacket {
    #[jce(1)]
    pub i_version: i16,
    #[jce(2)]
    pub c_packet_type: u8,
    #[jce(3)]
    pub i_message_type: i32,
    #[jce(4)]
    pub i_request_id: i32,
    #[jce(5)]
    pub s_servant_name: String,
    #[jce(6)]
    pub s_func_name: String,
    #[jce(7)]
    pub s_buffer: Bytes,
    #[jce(8)]
    pub i_timeout: i32,
    #[jce(9)]
    pub context: HashMap<String, String>,
    #[jce(10)]
    pub status: HashMap<String, String>,
}

JceStruct!(RequestDataVersion3 {
    0 => map: HashMap<String,Bytes>,
});

JceStruct!(RequestDataVersion2 {
    0 => map: HashMap<String,HashMap<String,Bytes>>,
});

JceStruct!(HttpServerListRes {
    2 => sso_server_infos: Vec<SsoServerInfo>,
});

JceStruct!(SsoServerInfo {
    1 => server: String,
    2 => port: i32,
    8 => location: String,
});

JceStruct!(FileStoragePushFSSvcList {
    0  => upload_list: Vec<FileStorageServerInfo>,
    1  => pic_download_list: Vec<FileStorageServerInfo>,
    2  => g_pic_download_list: Vec<FileStorageServerInfo>,
    3  => q_zone_proxy_service_list: Vec<FileStorageServerInfo>,
    4  => url_encode_service_list: Vec<FileStorageServerInfo>,
    5  => big_data_channel: BigDataChannel,
    6  => vip_emotion_list: Vec<FileStorageServerInfo>,
    7  => c2c_pic_down_list: Vec<FileStorageServerInfo>,
    // 8  => fmt_ip_info: FmtIpInfo,
    // 9  => domain_ip_channel: DomainIpChannel,
    10 => ptt_list: Bytes,
});

JceStruct!(FileStorageServerInfo {
    1 => server: String,
    2 => port: i32,
});

JceStruct!(BigDataChannel {
    0 => ip_lists: Vec<BigDataIPList>,
    1 => sig_session: Bytes,
    2 => key_session: Bytes,
    3 => sig_uin: i64,
    4 => connect_flag: i32,
    5 => pb_buf: Bytes,
});

JceStruct!(BigDataIPList {
    0 => service_type: i64,
    1 => ip_list: Vec<BigDataIPInfo>,
    3 => fragment_size: i64,
});

JceStruct!(BigDataIPInfo {
    0 => r#type: i64,
    1 => server: String,
    2 => port: i64,
});

JceStruct!(SvcReqPullGroupMsgSeq {
    0 => group_info: Vec<PullGroupSeqParam>,
    1 => verify_type: u8,
    2 => filter: i32,
});

JceStruct!(SvcReqRegister {
    0  => uin: i64,
    1  => bid: i64,
    2  => conn_type: u8,
    3  => other: String,
    4  => status: i32,
    5  => online_push: u8,
    6  => is_online: u8,
    7  => is_show_online: u8,
    8  => kick_pc: u8,
    9  => kick_weak: u8,
    10 => timestamp: i64,
    11 => ios_version: i64,
    12 => net_type: u8,
    13 => build_ver: String,
    14 => reg_type: u8,
    15 => dev_param: Bytes ,
    16 => guid: Bytes,
    17 => locale_id: i32,
    18 => silent_push: u8,
    19 => dev_name: String,
    20 => dev_type: String,
    21 => os_ver: String,
    22 => open_push: u8,
    23 => large_seq: i64,
    24 => last_watch_start_time: i64,
    26 => old_sso_ip: i64,
    27 => new_sso_ip: i64,
    28 => channel_no: String,
    29 => cpid: i64,
    30 => vendor_name: String,
    31 => vendor_os_name: String,
    32 => ios_idfa: String,
    33 => b769: Bytes,
    34 => is_set_status: u8,
    35 => server_buf: Bytes,
    36 => set_mute: u8,
    38 => ext_online_status: i64,
    39 => battery_status: i32,
    40 => tim_active_flag:u8,
    41 => bind_uin_notify_switch:u8,
    // 42 => stVendorPushInfo:struct,
    43 => vendor_dev_id:i64,
    45 => custom_status: Bytes, // 自定义状态 protobuf
});

JceStruct!(SvcRespRegister {
    0  => uin: i64,
    1  => bid: i64,
    2  => reply_code: u8,
    3  => result: String,
    4  => server_time: i64,
    5  => log_qq: u8,
    6  => need_kik: u8,
    7  => update_flag: u8,
    8  => timestamp: i64,
    9  => crash_flag: u8,
    10 => client_ip: String,
    11 => client_port: i32,
    12 => hello_interval: i32,
    13 => large_seq: i32,
    14 => large_seq_update: u8,
    15 => d769_rsp_body: Bytes,
    16 => status: i32,
    17 => ext_online_status: i64,
    18 => client_battery_get_interval: i64,
    19 => client_auto_status_interval: i64,
});

JceStruct!(SvcReqRegisterNew {
    0  => request_optional: i64,
    1  => c2c_msg: SvcReqGetMsgV2,
    2  => group_msg: SvcReqPullGroupMsgSeq,
    14 => dis_group_msg_filter: u8,
    15 => group_mask: u8,
    16 => end_seq: i64,
    20 => _0769_body: Bytes,
});

JceStruct!(SvcReqGetMsgV2 {
    0 => uin: i64,
    1 => date_time: i32,
    4 => recive_pic: u8,
    6 => ability: i16,
    9 => channel: u8,
    16 => inst: u8,
    17 => channel_ex: u8,
    18 => sync_cookie: Bytes,
    19 => sync_flag: i32,
    20 => ramble_flag: u8,
    26 => general_abi: i64,
    27 => pub_account_cookie: Bytes,
});

JceStruct!(PullGroupSeqParam {
    0 => group_code: i64,
    1 => last_seq_id: i64,
});

JceStruct!(SvcRespParam {
    0 => pc_stat: i32,
    1 => is_support_c2c_roam_msg: i32,
    2 => is_support_data_line: i32,
    3 => is_support_printable: i32,
    4 => is_support_view_p_c_file: i32,
    5 => pc_version: i32,
    6 => roam_flag: i64,
    7 => online_infos: Vec<OnlineInfo>,
    8 => pc_client_type: i32,
});

JceStruct!(RequestPushNotify {
    0 => uin: i64,
    1 => r#type: u8,
    2 => service: String,
    3 => cmd: String,
    4 => notify_cookie: Bytes,
    5 => msg_type: i32,
    6 => user_active: i32,
    7 => general_flag: i32,
    8 => binded_uin: i64,
});

JceStruct!(OnlineInfo {
    0 => instance_id: i32,
    1 => client_type: i32,
    2 => online_status: i32,
    3 => platform_id: i32,
    4 => sub_platform: Bytes,
    5 => u_client_type: i64,
});

JceStruct!(SvcReqMSFLoginNotify {
    0 => app_id: i64,
    1 => status: u8,
    2 => tablet: u8,
    3 => platform: i64,
    4 => title: String,
    5 => info: String,
    6 => product_type: i64,
    7 => client_type: i64,
    8 => instance_list: Vec<InstanceInfo>,
});

JceStruct!(InstanceInfo {
    0 => app_id: i32,
    1 => tablet: u8,
    2 => platform: i64,
    3 => product_type: i64,
    4 => client_type: i64,
});

JceStruct!(PushMessageInfo {
    0 => from_uin: i64,
    1 => msg_time: i64,
    2 => msg_type: i16,
    3 => msg_seq: i16,
    4 => msg: String,
    5 => real_msg_time: i32,
    6 => v_msg: Bytes,
    7 => app_share_id: i64,
    8 => msg_cookies: Bytes,
    9 => app_share_cookie: Bytes,
    10 => msg_uid: i64,
    11 => last_change_time: i64,
    14 => from_inst_id: i64,
    15 => remark_of_sender: Bytes,
    16 => from_mobile: String,
    17 => from_name: String,
});

JceStruct!(SvcRespPushMsg {
    0 => uin: i64,
    1 => del_infos: Vec<DelMsgInfo>,
    2 => svrip: i32,
    3 => push_token: Bytes,
    4 => service_type: i32,
});

JceStruct!(SvcReqGetDevLoginInfo {
    0 => guid: Bytes,
    1 => app_name: String,
    2 => login_type: i64,
    3 => timestamp: i64,
    4 => next_item_index: i64,
    5 => require_max: i64,
    6 => get_dev_list_type: i64, // 1: getLoginDevList 2: getRecentLoginDevList 4: getAuthLoginDevList
});

JceStruct!(SvcDevLoginInfo {
    0 => app_id: i64,
    1 => guid: Bytes,
    2 => login_time: i64,
    3 => login_platform: i64,
    4 => login_location: String,
    5 => device_name: String,
    6 => device_type_info: String,
    8 => ter_type: i64,
    9 => product_type: i64,
    10 => can_be_kicked: i64,
});

JceStruct!(DelMsgInfo {
    0 => from_uin: i64,
    1 => msg_time: i64,
    2 => msg_seq: i16,
    3 => msg_cookies: Bytes,
    4 => cmd: i16,
    5 => msg_type: i64,
    6 => app_id: i64,
    7 => send_time: i64,
    8 => sso_seq: i32,
    9 => sso_ip: i32,
    10 => client_ip: i32,
});

// 下面是生成的
//
// JceStruct!(RequestPacket {
//     1 => i_version: i16
//     2 => c_packet_type: u8
//     3 => i_message_type: i32
//     4 => i_request_id: i32
//     5 => s_servant_name: String
//     6 => s_func_name: String
//     7 => s_buffer: Bytes
//     8 => i_timeout: i32
//     9 => context: map[String]String
//     10 => status: map[String]String
// });
//
// JceStruct!(RequestDataVersion3 {
// 		0 => map: map[String]Bytes
// 	});
//
// JceStruct!(RequestDataVersion2 {
// 		0 => map: map[String]map[String]Bytes
// 	});
//
// JceStruct!(SsoServerInfo {
// 		1 => server: String
// 		2 => port: i32
// 		8 => location: String
// 	});
//
// JceStruct!(FileStoragePushFSSvcList {
// 		0 => upload_list: []FileStorageServerInfo
// 		1 => pic_download_list: []FileStorageServerInfo
// 		2 => g_pic_download_list: []FileStorageServerInfo
// 		3 => q_zone_proxy_service_list: []FileStorageServerInfo
// 		4 => url_encode_service_list: []FileStorageServerInfo
// 		5 => big_data_channel: BigDataChannel
// 		6 => vip_emotion_list: []FileStorageServerInfo
// 		7 => c2CPicDownList: []FileStorageServerInfo
// 		10 => ptt_list: Bytes
// 	});
//
// JceStruct!(FileStorageServerInfo {
// 		1 => server: String
// 		2 => port: i32
// 	});
//
// JceStruct!(BigDataChannel {
// 		0 => i_p_lists: []BigDataIPList
// 		1 => sig_session: Bytes
// 		2 => key_session: Bytes
// 		3 => sig_uin: i64
// 		4 => connect_flag: i32
// 		5 => pb_buf: Bytes
// 	});
//
// JceStruct!(BigDataIPList {
// 		0 => service_type: i64
// 		1 => i_p_list: []BigDataIPInfo
// 		3 => fragment_size: i64
// 	});
//
// JceStruct!(BigDataIPInfo {
// 		0 => type: i64
// 		1 => server: String
// 		2 => port: i64
// 	});
//
// JceStruct!(SvcReqRegister {
//
// 		0 => uin: i64
// 		1 => bid: i64
// 		2 => conn_type: u8
// 		3 => other: String
// 		4 => status: i32
// 		5 => online_push: u8
// 		6 => is_online: u8
// 		7 => is_show_online: u8
// 		8 => kick_p_c: u8
// 		9 => kick_weak: u8
// 		10 => timestamp: i64
// 		11 => i_o_s_version: i64
// 		12 => net_type: u8
// 		13 => build_ver: String
// 		14 => reg_type: u8
// 		15 => dev_param: Bytes
// 		16 => guid: Bytes
// 		17 => locale_id: i32
// 		18 => silent_push: u8
// 		19 => dev_name: String
// 		20 => dev_type: String
// 		21 => o_s_ver: String
// 		22 => open_push: u8
// 		23 => large_seq: i64
// 		24 => last_watch_start_time: i64
// 		26 => old_s_s_o_ip: i64
// 		27 => new_s_s_o_ip: i64
// 		28 => channel_no: String
// 		29 => c_p_i_d: i64
// 		30 => vendor_name: String
// 		31 => vendor_o_s_name: String
// 		32 => i_o_s_idfa: String
// 		33 => b769: Bytes
// 		34 => is_set_status: u8
// 		35 => server_buf: Bytes
// 		36 => set_mute: u8
// 		38 => ext_online_status: i64
// 		39 => battery_status: i32
// 	});
//
// JceStruct!(SvcRespRegister {
// 		0 => uin: i64
// 		1 => bid: i64
// 		2 => reply_code: u8
// 		3 => result: String
// 		4 => server_time: i64
// 		5 => log_q_q: u8
// 		6 => need_kik: u8
// 		7 => update_flag: u8
// 		8 => timestamp: i64
// 		9 => crash_flag: u8
// 		10 => client_ip: String
// 		11 => client_port: i32
// 		12 => hello_interval: i32
// 		13 => large_seq: i32
// 		14 => large_seq_update: u8
// 		15 => d769RspBody: Bytes
// 		16 => status: i32
// 		17 => ext_online_status: i64
// 		18 => client_battery_get_interval: i64
// 		19 => client_auto_status_interval: i64
// 	});
//
// JceStruct!(SvcReqRegisterNew {
//
// 		0 => request_optional: i64
// 		1 => c2CMsg:  // SvcReqGetMsgV2
// 		2 => group_msg:  // SvcReqPullGroupMsgSeq
// 		14 => dis_group_msg_filter: u8
// 		15 => group_mask: u8
// 		16 => end_seq: i64
// 		20 => o769Body: Bytes
// 	});
//
// JceStruct!(SvcReqGetMsgV2 {
//
// 		0 => uin: i64
// 		1 => date_time: i32
// 		4 => recive_pic: u8
// 		6 => ability: i16
// 		9 => channel: u8
// 		16 => inst: u8
// 		17 => channel_ex: u8
// 		18 => sync_cookie: Bytes
// 		19 => sync_flag: int
// 		20 => ramble_flag: u8
// 		26 => general_abi: i64
// 		27 => pub_account_cookie: Bytes
// 	});
//
// JceStruct!(SvcReqPullGroupMsgSeq {
//
// 		0 => group_info: [] // PullGroupSeqParam
// 		1 => verify_type: u8
// 		2 => filter: i32
// 	});
//
// JceStruct!(PullGroupSeqParam {
//
// 		0 => group_code: i64
// 		1 => last_seq_id: i64
// 	});
//
// JceStruct!(SvcRespParam {
// 		0 => p_c_stat: i32
// 		1 => is_support_c2CRoamMsg: i32
// 		2 => is_support_data_line: i32
// 		3 => is_support_printable: i32
// 		4 => is_support_view_p_c_file: i32
// 		5 => pc_version: i32
// 		6 => roam_flag: i64
// 		7 => online_infos: []OnlineInfo
// 		8 => p_c_client_type: i32
// 	});
//
// JceStruct!(RequestPushNotify {
// 		0 => uin: i64
// 		1 => type: u8
// 		2 => service: String
// 		3 => cmd: String
// 		4 => notify_cookie: Bytes
// 		5 => msg_type: i32
// 		6 => user_active: i32
// 		7 => general_flag: i32
// 		8 => binded_uin: i64
// 	});
//
// JceStruct!(OnlineInfo {
// 		0 => instance_id: i32
// 		1 => client_type: i32
// 		2 => online_status: i32
// 		3 => platform_id: i32
// 		4 => sub_platform: String
// 		5 => u_client_type: i64
// 	});
//
// JceStruct!(SvcReqMSFLoginNotify {
// 		0 => app_id: i64
// 		1 => status: u8
// 		2 => tablet: u8
// 		3 => platform: i64
// 		4 => title: String
// 		5 => info: String
// 		6 => product_type: i64
// 		7 => client_type: i64
// 		8 => instance_list: []InstanceInfo
// 	});
//
// JceStruct!(InstanceInfo {
// 		0 => app_id: i32
// 		1 => tablet: u8
// 		2 => platform: i64
// 		3 => product_type: i64
// 		4 => client_type: i64
// 	});
//
// JceStruct!(PushMessageInfo {
// 		0 => from_uin: i64
// 		1 => msg_time: i64
// 		2 => msg_type: i16
// 		3 => msg_seq: i16
// 		4 => msg: String
// 		5 => real_msg_time: i32
// 		6 => v_msg: Bytes
// 		7 => app_share_i_d: i64
// 		8 => msg_cookies: Bytes
// 		9 => app_share_cookie: Bytes
// 		10 => msg_uid: i64
// 		11 => last_change_time: i64
// 		14 => from_inst_id: i64
// 		15 => remark_of_sender: Bytes
// 		16 => from_mobile: String
// 		17 => from_name: String
// 	});
//
// JceStruct!(SvcRespPushMsg {
//
// 		0 => uin: i64
// 		1 => del_infos: []
// 		2 => svrip: i32
// 		3 => push_token: Bytes
// 		4 => service_type: i32
// 	});
//
// JceStruct!(SvcReqGetDevLoginInfo {
//
// 		0 => guid: Bytes
// 		1 => app_name: String
// 		2 => login_type: i64
// 		3 => timestamp: i64
// 		4 => next_item_index: i64
// 		5 => require_max: i64
// 		6 => get_dev_list_type: i64 // 1: getLoginDevList 2: getRecentLoginDevList 4: getAuthLoginDevList
// 	});
//
// JceStruct!(SvcDevLoginInfo {
// 		AppId          i64
// 		Guid           Bytes
// 		LoginTime      i64
// 		LoginPlatform  i64
// 		LoginLocation  String
// 		DeviceName     String
// 		DeviceTypeInfo String
// 		TerType        i64
// 		ProductType    i64
// 		CanBeKicked    i64
// 	});
//
// JceStruct!(DelMsgInfo {
//
// 		0 => from_uin: i64
// 		1 => msg_time: i64
// 		2 => msg_seq: i16
// 		3 => msg_cookies: Bytes
// 		4 => cmd: i16
// 		5 => msg_type: i64
// 		6 => app_id: i64
// 		7 => send_time: i64
// 		8 => sso_seq: i32
// 		9 => sso_ip: i32
// 		10 => client_ip: i32
// 	});
//
JceStruct!(FriendListRequest {
    0 => reqtype: i32,
    1 => if_reflush: u8,
    2 => uin: i64,
    3 => start_index: i16,
    4 => friend_count: i16,
    5 => group_id: u8,
    6 => if_get_group_info: u8,
    7 => group_start_index: u8,
    8 => group_count: u8,
    9 => if_get_msf_group: u8,
    10 => if_show_term_type: u8,
    11 => version: i64,
    12 => uin_list: Vec<i64>,
    13 => app_type: i32,
    14 => if_get_dov_id: u8,
    15 => if_get_both_flag: u8,
    16 => d50: Bytes,
    17 => d6b: Bytes,
    18 => sns_type_list: Vec<i64>,
});

JceStruct!(FriendInfo {
    0 => friend_uin: i64,
    1 => group_id: u8,
    2 => face_id: i16,
    3 => remark: String,
    4 => qq_type: u8,
    5 => status: u8,
    6 => member_level: u8,
    7 => is_mqq_online: u8,
    8 => qq_online_state: u8,
    9 => is_iphone_online: u8,
    10 => detail_status_flag: u8,
    11 => qq_online_state_v2: u8,
    12 => show_name: String,
    13 => is_remark: u8,
    14 => nick: String,
    15 => special_flag: u8,
    16 => im_group_id: Bytes,
    17 => msf_group_id: Bytes,
    18 => term_type: i32,
    20 => network: u8,
    21 => ring: Bytes,
    22 => abi_flag: i64,
    23 => face_addon_id: i64,
    24 => network_type: i32,
    25 => vip_font: i64,
    26 => icon_type: i32,
    27 => term_desc: String,
    28 => color_ring: i64,
    29 => apollo_flag: u8,
    30 => apollo_timestamp: i64,
    31 => sex: u8,
    32 => founder_font: i64,
    33 => eim_id: String,
    34 => eim_mobile: String,
    35 => olympic_torch: u8,
    36 => apollo_sign_time: i64,
    37 => lavi_uin: i64,
    38 => tag_update_time: i64,
    39 => game_last_login_time: i64,
    40 => game_app_id: i64,
    41 => card_id: Bytes,
    42 => bit_set: i64,
    43 => king_of_glory_flag: u8,
    44 => king_of_glory_rank: i64,
    45 => master_uin: String,
    46 => last_medal_update_time: i64,
    47 => face_store_id: i64,
    48 => font_effect: i64,
    49 => d_ov_id: String,
    50 => both_flag: i64,
    51 => centi_show_3d_flag: u8,
    52 => intimate_info: Bytes,
    53 => show_nameplate: u8,
    54 => new_lover_diamond_flag: u8,
    55 => ext_sns_frd_data: Bytes,
    56 => mutual_mark_data: Bytes,
});

JceStruct!(TroopListRequest {
    0 => uin: i64,
    1 => get_msf_msg_flag: u8,
    2 => cookies: Bytes,
    3 => group_info: Vec<i64>,
    4 => group_flag_ext: u8,
    5 => version: i32,
    6 => company_id: i64,
    7 => version_num: i64,
    8 => get_long_group_name: u8,
});

JceStruct!(TroopNumber {
    0 => group_uin: i64,
    1 => group_code: i64,
    2 => flag: u8,
    3 => group_info_seq: i64,
    4 => group_name: String,
    5 => group_memo: String,
    6 => group_flag_ext: i64,
    7 => group_rank_seq: i64,
    8 => certification_type: i64,
    9 => shut_up_timestamp: i64,
    10 => my_shut_up_timestamp: i64,
    11 => cmd_uin_uin_flag: i64,
    12 => additional_flag: i64,
    13 => group_type_flag: i64,
    14 => group_sec_type: i64,
    15 => group_sec_type_info: i64,
    16 => group_class_ext: i64,
    17 => app_privilege_flag: i64,
    18 => subscription_uin: i64,
    19 => member_num: i64,
    20 => member_num_seq: i64,
    21 => member_card_seq: i64,
    22 => group_flag_ext3: i64,
    23 => group_owner_uin: i64,
    24 => is_conf_group: u8,
    25 => is_modify_conf_group_face: u8,
    26 => is_modify_conf_group_name: u8,
    27 => cmd_uin_join_time: i64,
    28 => company_id: i64,
    29 => max_group_member_num: i64,
    30 => cmd_uin_group_mask: i64,
    31 => guild_app_id: i64,
    32 => guild_sub_type: i64,
    33 => cmd_uin_ringtone_i_d: i64,
    34 => cmd_uin_flag_ex2: i64,
});

JceStruct!(TroopMemberListRequest {
    0 => uin: i64,
    1 => group_code: i64,
    2 => next_uin: i64,
    3 => group_uin: i64,
    4 => version: i64,
    5 => req_type: i64,
    6 => get_list_appoint_time: i64,
    7 => rich_card_name_ver: u8,
});

JceStruct!(TroopMemberInfo {
    0 => member_uin: i64,
    1 => face_id: i16,
    2 => age: u8,
    3 => gender: u8,
    4 => nick: String,
    5 => status: u8,
    6 => show_name: String,
    8 => name: String,
    12 => memo: String,
    13 => auto_remark: String,
    14 => member_level: i64,
    15 => join_time: i64,
    16 => last_speak_time: i64,
    17 => credit_level: i64,
    18 => flag: i64,
    19 => flag_ext: i64,
    20 => point: i64,
    21 => concerned: u8,
    22 => shielded: u8,
    23 => special_title: String,
    24 => special_title_expire_time: i64,
    25 => job: String,
    26 => apollo_flag: u8,
    27 => apollo_timestamp: i64,
    28 => global_group_level: i64,
    29 => title_id: i64,
    30 => shut_up_timestap: i64,
    31 => global_group_point: i64,
    33 => rich_card_name_ver: u8,
    34 => vip_type: i64,
    35 => vip_level: i64,
    36 => big_club_level: i64,
    37 => big_club_flag: i64,
    38 => nameplate: i64,
    39 => group_honor: Bytes,
});

JceStruct!(ModifyGroupCardRequest {
    0 => zero: i64,
    1 => group_code: i64,
    2 => new_seq: i64,
    3 => uin_info: Vec<UinInfo>,
});

JceStruct!(UinInfo {
    0 => uin: i64,
    1 => flag: i64,
    2 => name: String,
    3 => gender: u8,
    4 => phone: String,
    5 => email: String,
    6 => remark: String,
});

JceStruct!(SummaryCardReq {
    0 => uin: i64,
    1 => come_from: i32,
    2 => qzone_feed_timestamp: i64,
    3 => is_friend: u8,
    4 => group_code: i64,
    5 => group_uin: i64,
    8 => get_control: i64,
    9 => add_friend_source: i32,
    10 => secure_sig: Bytes,
    14 => req_services: Vec<Bytes>, // todo
    15 => tiny_id: i64,
    16 => like_source: i64,
    18 => req_medal_wall_info: u8,
    19 => req_0x5eb_field_id: Vec<i64>,
    20 => req_nearby_god_info: u8,
    22 => req_extend_card: u8,
});

JceStruct!(SummaryCardReqSearch {
    0 => keyword: String,
    1 => country_code: String,
    2 => version: i32,
    3 => req_services:  Vec<Bytes>, // todo // busi
});

JceStruct!(DelFriendReq {
    0 => uin: i64,
    1 => del_uin: i64,
    2 => del_type: u8,
    3 => version: i32,
});

JceStruct!(DelFriendResp{
    0 => uin : i64,
    1 => del_uin : i64,
    2 => result : i32,
    3 => error_code : i16,
});

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct QQServiceReqHead {
    #[jce(0)]
    pub l_uin: i64,
    #[jce(1)]
    pub sh_version: i16,
    #[jce(2)]
    pub i_seq: i32,
    #[jce(3)]
    pub b_req_type: u8,
    #[jce(4)]
    pub b_triggered: u8,
    #[jce(5)]
    pub v_cookies: Bytes,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct ReqFavorite {
    #[jce(0)]
    pub st_header: QQServiceReqHead,
    #[jce(1)]
    pub l_mid: i64,
    #[jce(2)]
    pub c_op_type: i32,
    #[jce(3)]
    pub em_source: i32,
    #[jce(4)]
    pub i_count: i32,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct MsgType0x210 {
    #[jce(0)]
    pub sub_msg_type: i64,
    #[jce(10)]
    pub v_protobuf: Bytes,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct RequestPushForceOffline {
    #[jce(0)]
    pub uin: i64,
    #[jce(1)]
    pub title: String,
    #[jce(2)]
    pub tips: String,
    #[jce(3)]
    pub same_device: u8,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct RequestMSFForceOffline {
    #[jce(0)]
    pub uin: i64,
    #[jce(1)]
    pub seq_no: i64,
    #[jce(2)]
    pub kick_type: u8,
    #[jce(3)]
    pub info: String,
    #[jce(4)]
    pub title: String,
    #[jce(5)]
    pub sig_kick: u8,
    #[jce(6)]
    pub sig_kick_data: Bytes,
    #[jce(7)]
    pub same_device: u8,
}

#[derive(Debug, Clone, JceGet, JcePut, Default)]
pub struct RspMSFForceOffline {
    #[jce(0)]
    pub uin: i64,
    #[jce(1)]
    pub seq_no: i64,
    #[jce(2)]
    pub const_zero: u8,
}

#[cfg(test)]
mod tests {
    use bytes::*;

    use crate::crypto::qqtea_decrypt;
    use crate::hex::decode_hex;

    use super::*;

    #[test]
    fn sso_address_resp_decode() {
        static SSO_ADDRESS_RESP: &str = "6e477b1c09e193f1d6084a1e8a052c259f6cf4e608e614d5db262ba96fceb6e8a5d19d72860d0db5dfac554cb7de435e4400f9cd0e8b14f98d4020962c496393e6947f85adcaf00656782b9512713177b41d8489ddf8952766c9850639e329905911aa989d618be1e14b133d228f187efd0ae3a8bdec00c6028078862e965940ad8acc937c5522abf967de737632f19d4f27b5bdf34a2003b11d8547cd4f82c7f1fcb8ea8219ada296ca5ed38bc38b4bbb475f51974dffb85daf1a12b3be2d853ff7877eba722148612abe85535492dd955ae9ff2b6cd9bb570711acc0869cdab62f0aa7fb1caa9862abd1e3aca39a96a9b45116cc92a065c2736420cab691e540534307dcc2d872da82c35b03f7a94b0a9bd6fbf73caa002702c10c9af38616ed9e6c54912de021ace4d969ca264d7f9d94ea4913a1a2184e77b9a1bc850c38d7de55b82e21c2f0e45e0e12ab602c54641b20409d1013245f79ec151e1ee773b9cca9f6d172084d1a125b9ff0e3d5efac8f0ad9e4cdec94f9366e6346274b6c9994ae62f35060c961948b5aa23eb2402c008603fa2e416d7803c9e466a9658d5e3470abda44c9b00c985ee28ae01e2f837666b8b8c5fb5fdfd263fa4173c948cf282efcd779fc15e21dbc29d4fa826d92d75bd7ebf19036c621d2fb96e6940fcab78ebea48da089893374b67d8341a10d86c33a182a9354268a7a7e168bd5c4a33e8b6110cd7e838c9bf53b8869ddbe747daf94183627e8dab6f71c67f71c621eddce055eb20803c962433baa1c7b0d1caf8a8b8b10ed3d429adaa38afaf1effab224ea16454e6e99b35dbcc55bfa972b8511ca1dee7af4485e45a4c0b5a6940168678dc0471420e4d9c401a514f12423972d6deb28192e7e9f74987282a67c4e1ea7e987ebbac989a72c85faac5d2648a284dfdb9374dfd17887c4b31b3e676436bbd47c5a4258406552187f2ca3586c58b1187c979c9303561fd4308646f30908d8abb19bae79d493100c034e6b34b2cd810ddb700c1a2d5418f556a8c46ed8912b1ea507d289c350e0ac86e2b80cee31263c10f431353d049890e7bbb732fed29353e75c4172193f45c7260157ee6f5cf84f11c3183f9889e30e239cf2fae16f1d65f14b519f407f848bd014c7b95d61ef6393b137b6989979641af5325f53411090c54164148ce3f732e0cdaac01de3b8585594d1d2e8f76723d995497d2c314f377efbc3974363d031e5de9fa4799a42b4acc28f7b2834203a1f3fa00510604801c7777ae1f62e89e08b6a0248d46a2c055e93458498077b175a2f08313ee373d42b861a727935be574838cdc15e654c6a6d01143d23d072bf7ea93e1094d4c5330a1ee3ea421f78127b60ea65dc9628a4e2eb440f21daaa514d30dde9ba5eea69c19a8ae8c65b60bad64859692dcf9c947b99333c31b21418e2984b178ee61011c0a3cec9d97abc4511ee96599e49fa8dc7b27d38d1cac74071c0b840db24d6ad4f2923622f7482ca0de4dc35698b69bbb3692d85f698f629e1476ac54a35502098ebf607ec1ad0d053745c8685e5fa69f91694163ab89c3391b7e0856239486a3206465e521364b4de7d5e5e2124e8c4a2bf9ca140f2af2c65b5e5c0a29f2cbcaaf0cea569250c1ce9c9356f1ad0ace6c9f729319cd61b701d5b8ccdd4509bfed9c2dbdcb7f6e47c97570b6914b43fd14798337dd997ce6d3d520d3db2a1fb0d50730f89a96df64d5f383c7889c405bb28e261f56a4f22c8dd8d5c95ac5d8653bdaa67e826d0e4214559d70e5f82ce0846970399b11e875c08571165b16029c5c575bb21316c19269ffac6db84efb2aa35b1c877bd824e8d00d18aaf4a7dfec14d8bfa3a92029936aea0dd4eb9c3c8b3c633d2b59339ea5865997c5e52facb546a222fa27e9ec00063a620ab44550241f3a115fc6251ad8b244a3588d400ee18ca0769d2ae64e09b8a6c7e439a856093087930e442d036e6bcfeee0c7b6fa908ab1c6906552cf0cd6341ac14bc04ef45c4bd558d45fa51b0ca3465a9c681fbe24fec1d231a73080b067caa5ac0641e6da3e3d105bf6491752e308c026e94e616582150d3a8a52a7bac25d3880c8324c18a92bcdc73c1dfa7f3591754f8944ac6c32d86163edff93de1d3e435367714c390bcb6ee741032e1981450112a1a1303969008c5170c0774dd7a61952778ff2bd6abab52834b7e5fa1d4c2ae47026e4072bea7d4fa359c25b36658c0372b11ad1c5723fb9308ce31ec4a4e0c38244fdfe4918079";
        // static DE_DATA: &'static str = "0000063710032c3c4c560a436f6e666967487474706611487474705365727665724c6973745265737d000106050800010611487474705365727665724c6973745265731d000105ea0a100129000b0a160e3130392e3234342e3139382e3134211f9030014c5c600870018602737a96066f74686572730b0a160f3138302e3130322e3131312e313035211f9030014c5c6008700186027368960374656c0b0a160c3131332e39362e31322e3835211f9030014c5c600870018602737a960374656c0b0a160b31342e32322e332e3132322101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139332e323530205030014c5c600870018602746a960374656c0b0a160e3131342e3232312e3134382e34392136b030014c5c6008700186027368960374656c0b0a160c3131332e39362e31332e39352101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139322e323131211f9030014c5c600870018602746a960374656c0b0a160c3130312e39312e34322e3938205030014c5c6008700186027368960374656c0b0a16116d7366776966692e33672e71712e636f6d211f9030014c5c60087c86066f746865727396066f74686572730b0a160d34322e38312e3137322e323037205030014c5c600870018602746a960374656c0b39000b0a160e3130392e3234342e3139382e3134211f9030014c5c600870018602737a96066f74686572730b0a160f3138302e3130322e3131312e313035211f9030014c5c6008700186027368960374656c0b0a160c3131332e39362e31322e3835211f9030014c5c600870018602737a960374656c0b0a160b31342e32322e332e3132322101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139332e323530205030014c5c600870018602746a960374656c0b0a160e3131342e3232312e3134382e34392136b030014c5c6008700186027368960374656c0b0a160c3131332e39362e31332e39352101bb30014c5c600870018602737a960374656c0b0a160d34322e38312e3139322e323131211f9030014c5c600870018602746a960374656c0b0a160c3130312e39312e34322e3938205030014c5c6008700186027368960374656c0b0a16116d7366776966692e33672e71712e636f6d211f9030014c5c60087c86066f746865727396066f74686572730b0a160d34322e38312e3137322e323037205030014c5c600870018602746a960374656c0b426155ae2b5138406c7c80029005acbcc900080a160e3130392e3234342e3132392e3135205030014c500360087c8602737a96066f74686572730b0a160e3131342e3232312e3134342e3232205030014c500360087c86027368960374656c0b0a160c3131332e39362e31332e3434205030014c500360087c8602737a960374656c0b0a160e3131392e3134372e3139302e3337205030014c500360087c8602737a960374656c0b0a160d34322e38312e3139332e323432205030014c500360087c8602746a960374656c0b0a160d3138302e3130322e35392e3530205030014c500360087c86027368960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0bd900080a160e3130392e3234342e3132392e3135205030014c500360087c8602737a96066f74686572730b0a160e3131342e3232312e3134342e3232205030014c500360087c86027368960374656c0b0a160c3131332e39362e31332e3434205030014c500360087c8602737a960374656c0b0a160e3131392e3134372e3139302e3337205030014c500360087c8602737a960374656c0b0a160d34322e38312e3139332e323432205030014c500360087c8602746a960374656c0b0a160d3138302e3130322e35392e3530205030014c500360087c86027368960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0b0a160d34322e38312e3136392e313035205030014c500360087c8602746a960374656c0bed000cf90f0cf9100cf9110cf01202f113ff38f61428323032312d30392d33302031363a33313a33392064656c6976657279696e67206120706f6c696379fc150b8c980ca80c";

        let key = decode_hex("F0441F5FF42DA58FDCF7949ABA62D411").expect("failed to decode hex");

        let data = decode_hex(SSO_ADDRESS_RESP).expect("failed to decode_hex");
        let mut de_rsp = Bytes::from(qqtea_decrypt(&data, &key));

        de_rsp.advance(4);
        let mut request_packet: RequestPacket =
            jcers::from_buf(&mut de_rsp).expect("failed to decode RequestPacket");

        let mut request_data_version3: RequestDataVersion3 =
            jcers::from_buf(&mut request_packet.s_buffer)
                .expect("failed to decode RequestDataVersion3");

        let sso_server_infos: HttpServerListRes = jcers::from_buf(
            request_data_version3
                .map
                .get_mut("HttpServerListRes")
                .expect("failed to get HttpServerListRes"),
        )
        .unwrap();
        for s in sso_server_infos.sso_server_infos {
            println!("Get Addrs server:{} port:{}", s.server, s.port);
        }
    }
}
