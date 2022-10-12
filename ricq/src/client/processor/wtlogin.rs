use crate::{handler::RawHandler, Client};
use ricq_core::command::wtlogin::*;

impl<H: RawHandler> Client<H> {
    pub(crate) async fn process_login_response(&self, login_response: LoginResponse) {
        if let LoginResponse::Success(ref success) = login_response {
            if let Some(info) = success.account_info.clone() {
                let mut account_info = self.account_info.write().await;
                account_info.nickname = info.nick;
                account_info.age = info.age;
                account_info.gender = info.gender;
            }
        }
        self.engine
            .write()
            .await
            .process_login_response(login_response);
        self.handler.handle_login(self.uin().await).await;
    }

    pub(crate) async fn process_trans_emp_response(&self, qrcode_state: QRCodeState) {
        if let QRCodeState::Confirmed(resp) = qrcode_state {
            self.engine.write().await.process_qrcode_confirmed(resp);
        }
    }
}
