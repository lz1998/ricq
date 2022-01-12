use std::collections::HashMap;

use bytes::Bytes;

#[derive(Default, Debug)]
pub struct Sig {
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
    pub device_token: Option<Bytes>,
    // TODO 是不是可能None？
    pub ps_key_map: HashMap<String, Bytes>,
    pub pt4token_map: HashMap<String, Bytes>,

    pub out_packet_session_id: Bytes,
    pub dpwd: Bytes,
    pub t104: Bytes,
    pub t174: Bytes,
    pub g: Bytes,
    pub t402: Bytes,
    pub rand_seed: Bytes, // t403

    // TODO...
    pub ksid: Bytes,
}
