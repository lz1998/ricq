use std::collections::HashMap;
use bytes::Bytes;

#[derive(Default)]
pub struct LoginSigInfo {
    pub login_bitmap: u64,
    pub tgt: Bytes,
    pub tgt_key: Bytes,
    // study room manager | 0x16a
    pub srm_token: Bytes,
    pub t133: Bytes,
    pub encrypted_a1: Bytes,
    pub user_st_key: Bytes,
    pub user_st_web_sig: Bytes,
    pub s_key: Bytes,
    pub s_key_expired_time: i64,
    pub d2: Bytes,
    pub d2key: Bytes,
    pub wt_session_ticket_key: Bytes,
    // TODO 是不是可能None？
    pub device_token: Option<Bytes>,
    pub ps_key_map: HashMap<String, Bytes>,
    pub pt4token_map: HashMap<String, Bytes>,
}

pub struct QiDianAccountInfo {
    pub master_uin: i64,
    pub ext_name: String,
    pub create_time: i64,

    pub big_data_req_addrs: Vec<String>,
    pub big_data_req_session: BigDataReqSessionInfo,
}

pub struct BigDataReqSessionInfo {
    pub sig_session: Bytes,
    pub session_key: Bytes,
}