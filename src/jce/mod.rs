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
    0 => map: HashMap<String,Bytes>,
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
    // 9  => domain_ip_channel : DomainIpChannel,
    10 => ptt_list: Bytes,
});

JceStruct!(FileStorageServerInfo {
    1 => server: String,
    2 => port: i32,
});

JceStruct!(BigDataChannel {
    0 => ip_list: Vec<BigDataIp>, // gocq BigDataIPList
    1 => sig_session: Bytes,
    2 => key_session: Bytes,
    3 => sig_uin: i64,
    4 => connect_flag: i32,
    5 => pb_buf: Bytes,
});

JceStruct!(BigDataIp {
    0 => service_type: i64,
    1 => ip_list: BigDataIpInfo,
    2 => fragment_size: i64,
});

JceStruct!(BigDataIpInfo {
    0 => type_: i64,
    1 => server: String,
    2 => port: i64,
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
        10 => timestamp : i64,
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
        22 => open_push : u8,
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

// JceStruct!(SvcReqRegisterNew {
//     0  => request_optional: i64,
//     1  => c2c_msg:
// });

// todo
// 抄不动了，毁灭吧
