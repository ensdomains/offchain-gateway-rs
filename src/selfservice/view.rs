use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use ethers::providers::namehash;
use serde::{Deserialize, Serialize};

use crate::state::GlobalState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewPayload {
    name: String,
    records: HashMap<String, Option<String>>,
    addresses: HashMap<String, Option<String>>,
}

pub async fn route(
    Path(name): Path<String>,
    State(state): State<Arc<GlobalState>>,
) -> impl IntoResponse {
    let hash = namehash(&name);

    let (records, addresses) = state.db.get_all(hash.to_fixed_bytes().as_ref()).await;

    Json(ViewPayload {
        name,
        records,
        addresses,
    })
}
