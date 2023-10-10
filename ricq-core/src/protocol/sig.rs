use std::collections::HashMap;

use bytes::Bytes;

use crate::protocol::device::Device;

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
    // TODO 是不是可能None？
    pub device_token: Bytes,
    pub ps_key_map: HashMap<String, Bytes>,
    pub pt4_token_map: HashMap<String, Bytes>,

    pub out_packet_session_id: Bytes,
    pub dpwd: Bytes,
    pub t104: Bytes,
    pub t547: Bytes,
    pub t174: Bytes,
    pub g: Bytes,
    pub t402: Bytes,
    pub rand_seed: Bytes, // t403

    pub sync_const1: u32,
    pub sync_const2: u32,
    pub sync_const3: u32,
    pub sync_cookie: Bytes,
    pub pub_account_cookie: Bytes,

    // device?
    pub guid: Bytes,
    pub tgtgt_key: Bytes,
    pub ksid: Bytes,
}

impl Sig {
    pub fn new(device: &Device) -> Self {
        let mut sig = Self::default();
        sig.guid =
            Bytes::from(md5::compute(device.android_id.to_owned() + &device.mac_address).to_vec());
        sig.tgtgt_key = Bytes::from(md5::compute(&sig.guid).to_vec());
        sig.ksid = Bytes::from(format!("|{}|A8.2.7.27f6ea96", device.imei));
        sig.sync_const1 = rand::random::<u32>();
        sig.sync_const2 = rand::random::<u32>();
        sig.sync_const3 = rand::random::<u32>();
        sig
    }
}
