use std::sync::atomic::Ordering;

use bytes::{BufMut, Bytes, BytesMut};

use crate::command::wtlogin::{
    LoginDeviceLockLogin, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
    QRCodeConfirmed,
};
use crate::protocol::device::random_string;
use crate::Transport;

impl super::Engine {
    pub fn process_qrcode_confirmed(&mut self, resp: QRCodeConfirmed) {
        self.transport.sig.tgtgt_key = resp.tgtgt_key;
        self.uin.store(resp.uin, Ordering::Relaxed);
    }

    pub fn process_login_response(&mut self, login_response: LoginResponse) {
        match login_response {
            LoginResponse::Success(resp) => self.process_login_success(resp),
            LoginResponse::NeedCaptcha(resp) => self.process_need_captcha(resp),
            LoginResponse::DeviceLocked(resp) => self.process_device_locked(resp),
            LoginResponse::DeviceLockLogin(resp) => self.process_device_lock_login(resp),
            _ => {}
        }
    }

    fn process_login_success(&mut self, resp: LoginSuccess) {
        if let Some(v) = resp.rand_seed {
            self.transport.sig.rand_seed = v;
        }
        if let Some(v) = resp.ksid {
            self.transport.sig.ksid = v;
        }
        if let Some(v) = resp.t512 {
            self.transport.sig.ps_key_map = v.ps_key_map;
            self.transport.sig.pt4_token_map = v.pt4_token_map;
        }
        if let Some(v) = resp.wt_session_ticket_key {
            self.transport.oicq_codec.wt_session_ticket_key = v;
        }
        if let Some(v) = resp.srm_token {
            self.transport.sig.srm_token = v;
        }
        if let Some(v) = resp.t133 {
            self.transport.sig.t133 = v;
        }
        if let Some(v) = resp.encrypt_a1 {
            self.transport.sig.encrypted_a1 = v;
        }
        if let Some(v) = resp.tgt {
            self.transport.sig.tgt = v;
        }
        if let Some(v) = resp.tgt_key {
            self.transport.sig.tgt_key = v;
        }
        if let Some(v) = resp.user_st_key {
            self.transport.sig.user_st_key = v;
        }
        if let Some(v) = resp.user_st_web_sig {
            self.transport.sig.user_st_web_sig = v;
        }
        if let Some(v) = resp.s_key {
            self.transport.sig.s_key = v;
        }
        self.transport.sig.s_key_expired_time = resp.s_key_expired_time;
        if let Some(v) = resp.d2 {
            self.transport.sig.d2 = v;
        }
        if let Some(v) = resp.d2key {
            self.transport.sig.d2key = v;
        }
        if let Some(v) = resp.device_token {
            self.transport.sig.device_token = v;
        }
        if let Some(v) = resp.t402 {
            set_t402(&mut self.transport, v)
        }
    }

    fn process_need_captcha(&mut self, resp: LoginNeedCaptcha) {
        if let Some(v) = resp.t104 {
            self.transport.sig.t104 = v;
        }
    }

    fn process_device_locked(&mut self, resp: LoginDeviceLocked) {
        if let Some(v) = resp.t104 {
            self.transport.sig.t104 = v
        }
        if let Some(v) = resp.t174 {
            self.transport.sig.t174 = v
        }
        if let Some(v) = resp.t402 {
            set_t402(&mut self.transport, v)
        }
    }
    fn process_device_lock_login(&mut self, resp: LoginDeviceLockLogin) {
        if let Some(v) = resp.rand_seed {
            self.transport.sig.rand_seed = v;
        }
        if let Some(v) = resp.t104 {
            self.transport.sig.t104 = v
        }
        if let Some(v) = resp.t402 {
            set_t402(&mut self.transport, v)
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
