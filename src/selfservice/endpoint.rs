use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use ethers::providers::namehash;
use tracing::info;

use crate::state::GlobalState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNamePayload {
    name: String,
    records: HashMap<String, Option<String>>,
    addresses: HashMap<String, Option<String>>,
    // Arbitrary auth payload
    auth: String,
}

pub async fn route(
    State(state): State<Arc<GlobalState>>,
    Json(payload): Json<UpdateNamePayload>,
) -> impl IntoResponse {
    info!("Update name: {:?}", payload);

    if payload.auth != "yes" {
        return "auth error";
    }

    let hash = namehash(&payload.name);

    state
        .db
        .upsert(&hash.to_fixed_bytes().to_vec(), &payload.records, &payload.addresses)
        .await;

    // handle(request_payload, state)
    //     .await
    //     .map_err(|x| x.into_response())
    "ok"
}
