use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::processor::Processor;
use crate::{ClientInfo, RicqAxumApi};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListBotResp {
    pub bots: Vec<ClientInfo>,
}

pub async fn list<P: Processor>(
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<ListBotResp>, StatusCode> {
    Ok(Json(ListBotResp {
        bots: ricq_axum_api.processor.list_client().await,
    }))
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBotReq {
    uin: i64,
    protocol: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeleteBotResp {}

pub async fn delete<P: Processor>(
    Json(req): Json<DeleteBotReq>,
    ricq_axum_api: Extension<Arc<RicqAxumApi<P>>>,
) -> Result<Json<DeleteBotResp>, StatusCode> {
    ricq_axum_api
        .processor
        .delete_client(req.uin, req.protocol)
        .await;
    Ok(Json(DeleteBotResp {}))
}
