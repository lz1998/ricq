use crate::client::income::decoder::wtlogin::LoginResponse;
use crate::{Client, RQResult};

impl Client {
    pub async fn process_login_response(&self, login_response: LoginResponse) -> RQResult<()> {
        match login_response {
            LoginResponse::Success {
                rollback_sig: _,
                rand_seed,
                ksid,
                account_info,
                t512,
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
                // merge transport
                // merge account_info
                // merge codec
                let mut transport = self.transport.write().await;
                let mut cli_account_info = self.account_info.write().await;
                let mut codec = self.oicq_codec.write().await;
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
                // TODO dispatch login success event
            }
            LoginResponse::NeedCaptcha { .. } => {
                // TODO dispatch need captcha event
            }
            LoginResponse::DeviceLocked { .. } => {
                // TODO dispatch device locked event
            }
            LoginResponse::DeviceLockLogin { rand_seed, t104 } => {
                {
                    let mut transport = self.transport.write().await;
                    rand_seed.map(|v| transport.sig.rand_seed = v);
                    t104.map(|v| transport.sig.t104 = v);
                }
                // TODO dispatch device lock login event (internal)
            }
            LoginResponse::AccountFrozen => {
                // TODO dispatch account frozen event
            }
            LoginResponse::TooManySMSRequest => {
                // TODO dispatch too many sms request event
            }
            LoginResponse::UnknownLoginStatus { .. } => {}
        }
        Ok(())
    }
}
