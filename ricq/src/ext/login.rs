use std::sync::Arc;
use std::time::Duration;

use ricq_core::command::wtlogin::{LoginResponse, QRCodeConfirmed, QRCodeState};
use ricq_core::{RQError, RQResult};

use crate::Client;

/// 扫码登录：自动查询二维码状态，忽略中间结果，成功或失败返回
pub async fn auto_query_qrcode(client: &Arc<Client>, sig: &[u8]) -> RQResult<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let qrcode_state = client.query_qrcode_result(sig).await?;
        match qrcode_state {
            QRCodeState::Timeout => return Err(RQError::Timeout),
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
                        match client.device_lock_login().await? {
                            LoginResponse::Success { .. } => Ok(()),
                            other => Err(RQError::Other(format!(
                                "device_lock_login failed {other:?}"
                            ))),
                        }
                    }
                    other => Err(RQError::Other(format!("invalid login resp: {other:?}"))),
                };
            }
            _ => {
                // do nothing
            }
        }
        tokio::time::sleep(Duration::from_secs(4)).await;
    }
}
