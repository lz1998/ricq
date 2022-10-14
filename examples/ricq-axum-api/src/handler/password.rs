use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Extension, Json};
use rand::{prelude::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};

use ricq::client::{Connector as _, DefaultConnector, NetworkStatus};
use ricq::ext::reconnect::{Credential, Password};
use ricq::version::get_version;
use ricq::{Client, Device, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, Protocol};

use crate::processor::Processor;
use crate::u8_protocol::U8Protocol;
use crate::{PasswordClient, RicqAxumApi};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateClientReq {
    pub uin: i64,
    pub protocol: u8,
    pub password: String,
    pub device_seed: Option<u64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubmitTicketReq {
    pub uin: i64,
    pub protocol: u8,
    pub ticket: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RequestSmsReq {
    pub uin: i64,
    pub protocol: u8,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SubmitSmsReq {
    pub uin: i64,
    pub protocol: u8,
    pub sms: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct PasswordLoginResp {
    pub state: String,
    pub captcha_url: Option<String>,
    pub verify_url: Option<String>,
    pub sms_phone: Option<String>,
    pub message: Option<String>,
}

impl From<LoginResponse> for PasswordLoginResp {
    fn from(login_response: LoginResponse) -> Self {
        let mut resp = PasswordLoginResp::default();
        match login_response {
            LoginResponse::Success(_) => {
                resp.state = "success".into();
            }
            LoginResponse::NeedCaptcha(LoginNeedCaptcha { ref verify_url, .. }) => {
                resp.state = "need_captcha".into();
                resp.captcha_url = verify_url.clone();
            }
            LoginResponse::AccountFrozen => {
                resp.state = "account_frozen".into();
            }
            LoginResponse::DeviceLocked(LoginDeviceLocked {
                ref verify_url,
                ref message,
                ref sms_phone,
                ..
            }) => {
                resp.state = "device_locked".into();
                resp.verify_url = verify_url.clone();
                resp.sms_phone = sms_phone.clone();
                resp.message = message.clone();
            }
            LoginResponse::TooManySMSRequest => {
                resp.state = "too_many_sms_request".into();
            }
            LoginResponse::DeviceLockLogin(_) => {
                resp.state = "device_lock_login".into();
            }
            LoginResponse::UnknownStatus(status) => {
                resp.state = "unknown".into();
                resp.message = Some(format!(
                    "status: {} message: {}",
                    status.status, status.message
                ));
            }
        };
        resp
    }
}

pub async fn login<P: Processor>(
    Json(req): Json<CreateClientReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<PasswordLoginResp>, StatusCode> {
    let mut rand_seed = req.device_seed.unwrap_or(req.uin as u64);
    if rand_seed == 0 {
        rand_seed = req.uin as u64;
    }
    let device = Device::random_with_rng(&mut StdRng::seed_from_u64(rand_seed));
    let protocol = Protocol::from_u8(req.protocol);
    let (sender, receiver) = tokio::sync::broadcast::channel(10);
    let cli = Arc::new(Client::new(device, get_version(protocol.clone()), sender));
    let connector = DefaultConnector;
    let stream = connector
        .connect(&cli)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let c = cli.clone();
    let network_join_handle = tokio::spawn(async move { c.start(stream).await });
    tokio::task::yield_now().await;
    let mut resp = cli
        .password_login(req.uin, &req.password)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let LoginResponse::DeviceLockLogin(_) = resp {
        resp = cli
            .device_lock_login()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    let credential = Credential::Password(Password {
        uin: req.uin,
        password: req.password,
    });
    if let LoginResponse::Success(_) = resp {
        tracing::info!("login success: {} {:?}", req.uin, req.protocol);
        ricq_axum_api
            .processor
            .on_login_success(cli, receiver, credential, network_join_handle)
            .await;
    } else if let Some(old) = ricq_axum_api.password_clients.insert(
        (req.uin, protocol.to_u8()),
        PasswordClient {
            client: cli,
            login_response: resp.clone(),
            event_receiver: receiver,
            network_join_handle,
            credential,
        },
    ) {
        old.client.stop(NetworkStatus::Stop);
    }
    Ok(Json(PasswordLoginResp::from(resp)))
}

pub async fn submit_ticket<P: Processor>(
    Json(req): Json<SubmitTicketReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<PasswordLoginResp>, StatusCode> {
    let mut resp = ricq_axum_api
        .password_clients
        .get(&(req.uin, req.protocol))
        .ok_or(StatusCode::BAD_REQUEST)?
        .client
        .submit_ticket(&req.ticket)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let LoginResponse::DeviceLockLogin(_) = resp {
        resp = ricq_axum_api
            .password_clients
            .get(&(req.uin, req.protocol))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .client
            .device_lock_login()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let LoginResponse::Success(_) = resp {
        if let Some(((uin, protocol), client)) = ricq_axum_api
            .password_clients
            .remove(&(req.uin, req.protocol))
        {
            tracing::info!("login success: {} {:?}", uin, Protocol::from_u8(protocol));
            ricq_axum_api
                .processor
                .on_login_success(
                    client.client,
                    client.event_receiver,
                    client.credential,
                    client.network_join_handle,
                )
                .await;
        } else {
            tracing::warn!("failed to remove client: {}", req.uin);
        }
    } else {
        ricq_axum_api
            .password_clients
            .get_mut(&(req.uin, req.protocol))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .login_response = resp.clone();
    }
    Ok(Json(PasswordLoginResp::from(resp)))
}

pub async fn request_sms<P: Processor>(
    Json(req): Json<RequestSmsReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<PasswordLoginResp>, StatusCode> {
    let resp = ricq_axum_api
        .password_clients
        .get(&(req.uin, req.protocol))
        .ok_or(StatusCode::BAD_REQUEST)?
        .client
        .request_sms()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    ricq_axum_api
        .password_clients
        .get_mut(&(req.uin, req.protocol))
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .login_response = resp.clone();
    Ok(Json(PasswordLoginResp::from(resp)))
}

pub async fn submit_sms<P: Processor>(
    Json(req): Json<SubmitSmsReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<PasswordLoginResp>, StatusCode> {
    let mut resp = ricq_axum_api
        .password_clients
        .get(&(req.uin, req.protocol))
        .ok_or(StatusCode::BAD_REQUEST)?
        .client
        .submit_sms_code(&req.sms)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let LoginResponse::DeviceLockLogin(_) = resp {
        resp = ricq_axum_api
            .password_clients
            .get(&(req.uin, req.protocol))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .client
            .device_lock_login()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    if let LoginResponse::Success(_) = resp {
        let cli = ricq_axum_api
            .password_clients
            .remove(&(req.uin, req.protocol));
        if let Some(((uin, protocol), client)) = cli {
            tracing::info!("login success: {} {:?}", uin, Protocol::from_u8(protocol));
            ricq_axum_api
                .processor
                .on_login_success(
                    client.client,
                    client.event_receiver,
                    client.credential,
                    client.network_join_handle,
                )
                .await;
        } else {
            tracing::warn!("failed to remove client: {}", req.uin);
        }
    } else {
        ricq_axum_api
            .password_clients
            .get_mut(&(req.uin, req.protocol))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            .login_response = resp.clone();
    }
    Ok(Json(PasswordLoginResp::from(resp)))
}

#[derive(Default, Serialize)]
pub struct ListClientResp {
    pub clients: Vec<ListClientRespClient>,
}

#[derive(Default, Serialize)]
pub struct ListClientRespClient {
    pub uin: i64,
    pub protocol: u8,
    pub resp: PasswordLoginResp,
}

pub async fn list<P: Processor>(
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<ListClientResp>, StatusCode> {
    let mut clients = Vec::new();
    for c in ricq_axum_api.password_clients.iter() {
        clients.push(ListClientRespClient {
            uin: c.key().0,
            protocol: c.client.version().await.protocol.to_u8(),
            resp: PasswordLoginResp::from(c.login_response.clone()),
        })
    }
    Ok(Json(ListClientResp { clients }))
}

#[derive(Default, Serialize, Deserialize)]
pub struct DeleteClientReq {
    pub uin: i64,
    pub protocol: u8,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DeleteClientResp {}

pub async fn delete<P: Processor>(
    Json(req): Json<DeleteClientReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<DeleteClientResp>, StatusCode> {
    if let Some((_, cli)) = ricq_axum_api
        .password_clients
        .remove(&(req.uin, req.protocol))
    {
        cli.client.stop(NetworkStatus::Stop);
    }
    Ok(Json(DeleteClientResp {}))
}
