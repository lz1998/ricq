use std::sync::atomic::Ordering;
use std::sync::Arc;

use bytes::{BufMut, Bytes, BytesMut};

use crate::engine::command::wtlogin::*;
use crate::engine::protocol::device::random_string;
use crate::engine::protocol::transport::Transport;
use crate::handler::QEvent;
use crate::Client;

impl Client {
    pub(crate) async fn process_login_response(self: &Arc<Self>, login_response: LoginResponse) {
        // merge transport
        // merge account_info
        // merge oicq_codec
        let mut engine = self.engine.write().await;
        let mut cli_account_info = self.account_info.write().await;
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
                rand_seed.map(|v| engine.transport.sig.rand_seed = v);
                ksid.map(|v| engine.transport.sig.ksid = v);
                account_info.map(|v| {
                    cli_account_info.nickname = v.nick;
                    cli_account_info.age = v.age;
                    cli_account_info.gender = v.gender;
                });
                t512.map(|v| {
                    engine.transport.sig.ps_key_map = v.ps_key_map;
                    engine.transport.sig.pt4_token_map = v.pt4_token_map;
                });
                wt_session_ticket_key
                    .map(|v| engine.transport.oicq_codec.wt_session_ticket_key = v);
                srm_token.map(|v| engine.transport.sig.srm_token = v);
                t133.map(|v| engine.transport.sig.t133 = v);
                encrypt_a1.map(|v| engine.transport.sig.encrypted_a1 = v);
                tgt.map(|v| engine.transport.sig.tgt = v);
                tgt_key.map(|v| engine.transport.sig.tgt_key = v);
                user_st_key.map(|v| engine.transport.sig.user_st_key = v);
                user_st_web_sig.map(|v| engine.transport.sig.user_st_web_sig = v);
                s_key.map(|v| engine.transport.sig.s_key = v);
                engine.transport.sig.s_key_expired_time = s_key_expired_time;
                d2.map(|v| engine.transport.sig.d2 = v);
                d2key.map(|v| engine.transport.sig.d2key = v);
                device_token.map(|v| engine.transport.sig.device_token = v);
                if let Some(v) = t402 {
                    set_t402(&mut engine.transport, v)
                }
                self.handler.handle(QEvent::Login(engine.uin())).await;
            }
            LoginResponse::NeedCaptcha { t104, .. } => {
                t104.map(|v| engine.transport.sig.t104 = v);
            }
            LoginResponse::DeviceLocked {
                t104, t174, t402, ..
            } => {
                t104.map(|v| engine.transport.sig.t104 = v);
                if let Some(v) = t174 {
                    engine.transport.sig.t174 = v
                }
                if let Some(v) = t402 {
                    set_t402(&mut engine.transport, v)
                }
            }
            LoginResponse::DeviceLockLogin {
                rand_seed,
                t104,
                t402,
            } => {
                rand_seed.map(|v| engine.transport.sig.rand_seed = v);
                t104.map(|v| engine.transport.sig.t104 = v);
                if let Some(v) = t402 {
                    set_t402(&mut engine.transport, v)
                }
            }
            _ => {}
        }
    }

    pub(crate) async fn process_trans_emp_response(&self, qrcode_state: QRCodeState) {
        if let QRCodeState::QRCodeConfirmed { uin, tgtgt_key, .. } = qrcode_state {
            let engine = &mut self.engine.write().await;
            engine.transport.sig.tgtgt_key = tgtgt_key;
            engine.uin.store(uin, Ordering::SeqCst);
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
