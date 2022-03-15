use std::sync::Arc;
use std::time::Duration;

use crate::engine::command::wtlogin::{LoginResponse, QRCodeConfirmed, QRCodeState};
use crate::engine::{RQError, RQResult};

use crate::Client;

/// 扫码登录：自动查询二维码状态，忽略中间结果，成功或失败返回
pub async fn auto_query_qrcode(client: &Arc<Client>, sig: &[u8]) -> RQResult<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let qrcode_state = client.query_qrcode_result(sig).await?;
        match qrcode_state {
            QRCodeState::Timeout => return Err(RQError::Other("timeout".into())),
            QRCodeState::Canceled => return Err(RQError::Other("canceled".into())),
            QRCodeState::Confirmed(QRCodeConfirmed {
                ref tmp_pwd,
                ref tmp_no_pic_sig,
                ref tgt_qr,
                ..
            }) => {
                let login_resp = client.qrcode_login(tmp_pwd, tmp_no_pic_sig, tgt_qr).await?;
                return match login_resp {
                    LoginResponse::Success { .. } => Ok(()),
                    LoginResponse::DeviceLockLogin { .. } => {
                        let login_resp = client.device_lock_login().await?;
                        if let LoginResponse::Success { .. } = login_resp {
                            Ok(())
                        } else {
                            Err(RQError::Other("unknown error".into()))
                        }
                    }
                    _ => Err(RQError::Other("unknown error".into())),
                };
            }
            _ => {
                // do nothing
            }
        }
    }
}
