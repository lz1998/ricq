use std::sync::atomic::Ordering;

use bytes::{BufMut, Bytes, BytesMut};

use crate::command::wtlogin::{
    LoginDeviceLockLogin, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
    QRCodeConfirmed,
};
use crate::protocol::device::random_string;
use crate::utils::OptionSet;
use crate::Transport;

impl super::Engine {
    pub fn process_qrcode_confirmed(&mut self, resp: &QRCodeConfirmed) {
        self.transport.sig.tgtgt_key = resp.tgtgt_key.clone();
        self.uin.store(resp.uin, Ordering::Relaxed);
    }

    pub fn process_login_response(&mut self, login_response: &LoginResponse) {
        match login_response {
            LoginResponse::Success(resp) => self.process_login_success(resp.clone()),
            LoginResponse::NeedCaptcha(resp) => self.process_need_captcha(resp),
            LoginResponse::DeviceLocked(resp) => self.process_device_locked(resp),
            LoginResponse::DeviceLockLogin(resp) => self.process_device_lock_login(resp.clone()),
            _ => {}
        }
    }

    fn process_login_success(&mut self, resp: LoginSuccess) {
        let sig = &mut self.transport.sig;
        let oicq_codec = &mut self.transport.oicq_codec;

        // update
        sig.rand_seed.option_set(resp.rand_seed);
        sig.ksid.option_set(resp.ksid);

        if let Some(v) = resp.t512 {
            sig.ps_key_map = v.ps_key_map;
            sig.pt4_token_map = v.pt4_token_map;
        }

        oicq_codec
            .wt_session_ticket_key
            .option_set(resp.wt_session_ticket_key);

        sig.srm_token.option_set(resp.srm_token);
        sig.t133.option_set(resp.t133);
        sig.encrypted_a1.option_set(resp.encrypt_a1);
        sig.tgt.option_set(resp.tgt);
        sig.tgt_key.option_set(resp.tgt_key);
        sig.user_st_key.option_set(resp.user_st_key);
        sig.user_st_web_sig.option_set(resp.user_st_web_sig);
        sig.s_key.option_set(resp.s_key);
        sig.s_key_expired_time = resp.s_key_expired_time;
        sig.d2.option_set(resp.d2);
        sig.d2key.option_set(resp.d2key);
        sig.device_token.option_set(resp.device_token);

        if let Some(v) = resp.t402 {
            set_t402(&mut self.transport, v)
        }
    }

    fn process_need_captcha(&mut self, resp: &LoginNeedCaptcha) {
        self.transport.sig.t104.option_set(resp.t104.clone());
    }

    fn process_device_locked(&mut self, resp: &LoginDeviceLocked) {
        self.transport.sig.t104.option_set(resp.t104.clone());
        self.transport.sig.t174.option_set(resp.t174.clone());

        if let Some(v) = &resp.t402 {
            set_t402(&mut self.transport, v.clone())
        }
    }
    fn process_device_lock_login(&mut self, resp: LoginDeviceLockLogin) {
        self.transport.sig.rand_seed.option_set(resp.rand_seed);
        self.transport.sig.t104.option_set(resp.t104);

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
