use bytes::Bytes;
use jce_struct::*;
use std::collections::HashMap;

mod test;

JceStruct!(RequestPacket {
    1  => i_version: i16,
    2  => c_packet_type: u8,
    3  => i_message_type: i32,
    4  => i_request_id: i32,
    5  => s_servant_name: String,
    6  => s_func_name: String,
    7  => s_buffer: Bytes,
    8  => i_timeout: i32,
    9  => context: HashMap<String,String>,
    10 => status: HashMap<String,String>,
});

JceStruct!(RequestDataVersion3 {
    0 => map: HashMap<String,Bytes>,
});

// Recursive expansion of JceStruct! macro
// ========================================

// pub struct RequestDataVersion3 {
//     pub map: HashMap<String, Bytes>,
// }
// impl JcePut for RequestDataVersion3 {
//     fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
//         jce_mut.put_head(10, tag);
//         self.put_raw(jce_mut);
//         jce_mut.put_head(11, 0)
//     }
//     fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
//         self.map.put(jce_mut, 0);
//         jce_mut
//     }
// }
// impl JceGet for RequestDataVersion3 {
//     fn get(jce: &mut Jce) -> Self {
//         let map = jce.get_by_tag(0);
//         jce.end_object();
//         RequestDataVersion3 { map }
//     }
//     fn empty() -> Self {
//         {
//             panic!("jce get empty, should have a object")
//         }
//     }
// }

JceStruct!(RequestDataVersion2 {
    0 => map: HashMap<String,HashMap<String,Bytes>>,
});

// Recursive expansion of JceStruct! macro
// ========================================

// pub struct RequestDataVersion2 {
//     pub map: HashMap<String, HashMap<String, Bytes>>,
// }
// impl JcePut for RequestDataVersion2 {
//     fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
//         jce_mut.put_head(10, tag);
//         self.put_raw(jce_mut);
//         jce_mut.put_head(11, 0)
//     }
//     fn put_raw(self, jce_mut: &mut JceMut) -> &mut JceMut {
//         self.map.put(jce_mut, 0);
//         jce_mut
//     }
// }
// impl JceGet for RequestDataVersion2 {
//     fn get(jce: &mut Jce) -> Self {
//         let map = jce.get_by_tag(0);
//         jce.end_object();
//         RequestDataVersion2 { map }
//     }
//     fn empty() -> Self {
//         {
//             panic!("jce get empty, should have a object")
//         }
//     }
// }

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
    0 => p_c_stat: i32,
    1 => is_support_c2c_roam_msg: i32,
    2 => is_support_data_line: i32,
    3 => is_support_printable: i32,
    4 => is_support_view_p_c_file: i32,
    5 => pc_version: i32,
    6 => roam_flag: i64,
    7 => online_infos: Vec<OnlineInfo>,
    8 => p_c_client_type: i32,
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
    4 => sub_platform: String,
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
    0 => AppId: i64,
    1 => Guid: Bytes,
    2 => LoginTime: i64,
    3 => LoginPlatform: i64,
    4 => LoginLocation: String,
    5 => DeviceName: String,
    6 => DeviceTypeInfo: String,
    8 => TerType: i64,
    9 => ProductType: i64,
    10 => CanBeKicked: i64,
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

// JceStruct!(ModifyGroupCardRequest {
//
// 		0 => zero: i64
// 		1 => group_code: i64
// 		2 => new_seq: i64
// 		3 => uin_info: []
// 	});
//
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