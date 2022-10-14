use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Extension, Json};
use bytes::Bytes;
use rand::{prelude::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};

use ricq::client::NetworkStatus;
use ricq::client::{Connector as _, DefaultConnector};
use ricq::device::Device;
use ricq::ext::reconnect::Credential;
use ricq::version::{get_version, Protocol};
use ricq::{Client, LoginResponse, QRCodeState};

use crate::processor::Processor;
use crate::u8_protocol::U8Protocol;
use crate::QRCodeClient;
use crate::RicqAxumApi;

mod base64 {
    extern crate base64;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode(s).map_err(de::Error::custom)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateClientReq {
    pub device_seed: Option<u64>,
    pub protocol: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreateClientResp {
    #[serde(with = "base64")]
    pub sig: Vec<u8>,
    #[serde(with = "base64")]
    pub image: Vec<u8>,
}

pub async fn create<P: Processor>(
    Json(req): Json<CreateClientReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<CreateClientResp>, StatusCode> {
    let rand_seed = req.device_seed.unwrap_or_else(rand::random);
    let device = Device::random_with_rng(&mut StdRng::seed_from_u64(rand_seed));
    let protocol = match Protocol::from_u8(req.protocol) {
        Protocol::MacOS => Protocol::MacOS,
        Protocol::AndroidWatch => Protocol::AndroidWatch,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    let (sender, receiver) = tokio::sync::broadcast::channel(10);
    let cli = Arc::new(Client::new(device, get_version(protocol), sender));
    let connector = DefaultConnector;
    let stream = connector
        .connect(&cli)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let c = cli.clone();
    let network_join_handle = tokio::spawn(async move { c.start(stream).await });
    tokio::task::yield_now().await;
    let resp = cli
        .fetch_qrcode()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let QRCodeState::ImageFetch(image_fetch) = resp {
        ricq_axum_api.qrcode_clients.insert(
            image_fetch.sig.clone(),
            QRCodeClient {
                sig: image_fetch.sig.to_vec(),
                image: image_fetch.image_data.to_vec(),
                state: QRCodeState::ImageFetch(image_fetch.clone()),
                client: cli,
                event_receiver: receiver,
                network_join_handle,
            },
        );
        Ok(Json(CreateClientResp {
            sig: image_fetch.sig.to_vec(),
            image: image_fetch.image_data.to_vec(),
        }))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueryQRCodeReq {
    #[serde(with = "base64")]
    pub sig: Vec<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct QueryQRCodeResp {
    pub state: String,
}

pub async fn query<P: Processor>(
    Json(req): Json<QueryQRCodeReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<QueryQRCodeResp>, StatusCode> {
    let sig = Bytes::from(req.sig);

    let resp = ricq_axum_api
        .qrcode_clients
        .get(&sig)
        .ok_or(StatusCode::BAD_REQUEST)?
        .client
        .query_qrcode_result(&sig)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let state = match resp {
        QRCodeState::ImageFetch(_) => "image_fetch",
        QRCodeState::WaitingForScan => "waiting_for_scan",
        QRCodeState::WaitingForConfirm => "waiting_for_confirm",
        QRCodeState::Timeout => "timeout",
        QRCodeState::Confirmed(_) => "confirmed",
        QRCodeState::Canceled => "canceled",
    }
    .to_string();
    ricq_axum_api
        .qrcode_clients
        .get_mut(&sig)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .state = resp.clone();
    if let QRCodeState::Confirmed(confirmed) = resp {
        let (_, cli) = ricq_axum_api.qrcode_clients.remove(&sig).unwrap();
        let mut resp = cli
            .client
            .qrcode_login(
                &confirmed.tmp_pwd,
                &confirmed.tmp_no_pic_sig,
                &confirmed.tgt_qr,
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let LoginResponse::DeviceLockLogin(_) = resp {
            resp = cli
                .client
                .device_lock_login()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        if let LoginResponse::Success(_) = resp {
            let uin = cli.client.uin().await;
            let credential = Credential::Token(cli.client.gen_token().await);
            tracing::info!("login success: {}", uin);
            ricq_axum_api
                .processor
                .on_login_success(
                    cli.client,
                    cli.event_receiver,
                    credential,
                    cli.network_join_handle,
                )
                .await;
        }
    }
    Ok(Json(QueryQRCodeResp { state }))
}

#[derive(Default, Serialize)]
pub struct ListClientResp {
    pub clients: Vec<ListClientRespClient>,
}

#[derive(Default, Serialize)]
pub struct ListClientRespClient {
    #[serde(with = "base64")]
    pub sig: Vec<u8>,
    #[serde(with = "base64")]
    pub image: Vec<u8>,
    pub protocol: u8,
    pub state: String,
}

pub async fn list<P: Processor>(
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<ListClientResp>, StatusCode> {
    let mut clients = Vec::new();
    for c in ricq_axum_api.qrcode_clients.iter() {
        clients.push(ListClientRespClient {
            sig: c.sig.to_vec(),
            image: c.image.clone(),
            protocol: c.client.version().await.protocol.to_u8(),
            state: match c.state {
                QRCodeState::ImageFetch(_) => "image_fetch",
                QRCodeState::WaitingForScan => "waiting_for_scan",
                QRCodeState::WaitingForConfirm => "waiting_for_confirm",
                QRCodeState::Timeout => "timeout",
                QRCodeState::Confirmed(_) => "confirmed",
                QRCodeState::Canceled => "canceled",
            }
            .into(),
        })
    }
    Ok(Json(ListClientResp { clients }))
}

#[derive(Default, Serialize, Deserialize)]
pub struct DeleteClientReq {
    #[serde(with = "base64")]
    pub sig: Vec<u8>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct DeleteClientResp {}

pub async fn delete<P: Processor>(
    Json(req): Json<DeleteClientReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<DeleteClientResp>, StatusCode> {
    if let Some((_, cli)) = ricq_axum_api.qrcode_clients.remove(&Bytes::from(req.sig)) {
        cli.client.stop(NetworkStatus::Stop);
    }
    Ok(Json(DeleteClientResp {}))
}
