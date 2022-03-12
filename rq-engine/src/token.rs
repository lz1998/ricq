use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub uin: i64,
    pub d2: Vec<u8>,
    pub d2key: Vec<u8>,
    pub tgt: Vec<u8>,
    pub srm_token: Vec<u8>,
    pub t133: Vec<u8>,
    pub encrypted_a1: Vec<u8>,
    pub out_packet_session_id: Vec<u8>,
    pub tgtgt_key: Vec<u8>,
    pub wt_session_ticket_key: Vec<u8>, // oicq
}
