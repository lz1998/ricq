use std::sync::atomic::Ordering;

use bytes::{BufMut, Bytes, BytesMut};

use crate::client::income::decoder::wtlogin::{LoginResponse, QRCodeState};
use crate::client::protocol::device::random_string;
use crate::client::protocol::transport::Transport;
use crate::Client;

impl Client {
    pub async fn process_login_response(&self, login_response: LoginResponse) {
        // merge transport
        // merge account_info
        // merge oicq_codec
        let mut transport = self.transport.write().await;
        let mut cli_account_info = self.account_info.write().await;
        let mut codec = self.oicq_codec.write().await;
        match login_response {
            LoginResponse::Success {
                rollback_sig: _,
                rand_seed,
                ksid,
                account_info,
                t512,
                t402,
                wt_session_ticket_key,
                srm_token,
                t133,
                encrypt_a1,
                tgt,
                tgt_key,
                user_st_key,
                user_st_web_sig,
                s_key,
                s_key_expired_time,
                d2,
                d2key,
                device_token,
            } => {
                rand_seed.map(|v| transport.sig.rand_seed = v);
                ksid.map(|v| transport.sig.ksid = v);
                account_info.map(|v| {
                    cli_account_info.nickname = v.nick;
                    cli_account_info.age = v.age;
                    cli_account_info.gender = v.gender;
                });
                t512.map(|v| {
                    transport.sig.ps_key_map = v.ps_key_map;
                    transport.sig.pt4_token_map = v.pt4_token_map;
                });
                wt_session_ticket_key.map(|v| codec.wt_session_ticket_key = v);
                srm_token.map(|v| transport.sig.srm_token = v);
                t133.map(|v| transport.sig.t133 = v);
                encrypt_a1.map(|v| transport.sig.encrypted_a1 = v);
                tgt.map(|v| transport.sig.tgt = v);
                tgt_key.map(|v| transport.sig.tgt_key = v);
                user_st_key.map(|v| transport.sig.user_st_key = v);
                user_st_web_sig.map(|v| transport.sig.user_st_web_sig = v);
                s_key.map(|v| transport.sig.s_key = v);
                transport.sig.s_key_expired_time = s_key_expired_time;
                d2.map(|v| transport.sig.d2 = v);
                d2key.map(|v| transport.sig.d2key = v);
                device_token.map(|v| transport.sig.device_token = v);
                t402.map(|v| set_t402(&mut transport, v));
                // TODO dispatch login success event
            }
            LoginResponse::NeedCaptcha { t104, .. } => {
                t104.map(|v| transport.sig.t104 = v);
            }
            LoginResponse::DeviceLocked {
                t104, t174, t402, ..
            } => {
                t104.map(|v| transport.sig.t104 = v);
                t174.map(|v| transport.sig.t174 = v);
                t402.map(|v| set_t402(&mut transport, v));
            }
            LoginResponse::DeviceLockLogin {
                rand_seed,
                t104,
                t402,
            } => {
                rand_seed.map(|v| transport.sig.rand_seed = v);
                t104.map(|v| transport.sig.t104 = v);
                t402.map(|v| set_t402(&mut transport, v));
            }
            _ => {}
        }
    }

    pub async fn process_trans_emp_response(&self, qrcode_state: QRCodeState) {
        if let QRCodeState::QRCodeConfirmed { uin, tgtgt_key, .. } = qrcode_state {
            let mut transport = self.transport.write().await;
            transport.sig.tgtgt_key = tgtgt_key;
            self.uin.store(uin, Ordering::SeqCst);
        }
    }
}

fn set_t402(transport: &mut Transport, t402: Bytes) {
    transport.sig.dpwd = random_string(16).into();
    transport.sig.t402 = t402;
    let mut v = BytesMut::new();
    v.put_slice(&transport.sig.guid);
    v.put_slice(&transport.sig.dpwd);
    v.put_slice(&transport.sig.t402);
    transport.sig.g = Bytes::from(md5::compute(&v).to_vec())
}
