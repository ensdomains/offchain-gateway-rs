use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use ethers::core::utils::rlp::Decodable;
use ethers::{core::k256::ecdsa, providers::namehash, types::Signature};
use ethers_contract_derive::{Eip712, EthAbiType};
use ethers_core::types::{transaction::eip712::Eip712, H160};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use tracing::info;

use crate::state::GlobalState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNamePayload {
    payload: String,
    // Arbitrary auth payload
    auth: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignableUpdateNamePayload {
    name: String,
    records: HashMap<String, String>,
    addresses: HashMap<String, String>,
    time: u64,
}

fn convert_hashmap(a: HashMap<String, String>) -> HashMap<String, Option<String>> {
    a.into_iter()
        .map(|(k, v)| (k, Some(v)))
        .collect::<HashMap<String, Option<String>>>()
}

pub async fn route(
    State(state): State<Arc<GlobalState>>,
    Json(payload): Json<UpdateNamePayload>,
) -> impl IntoResponse {
    info!("Update name: {:?}", payload);

    let auth = payload.auth;
    let raw_payload = &payload.payload;
    let payload: SignableUpdateNamePayload = serde_json::from_str(raw_payload).unwrap();

    let hash = namehash(&payload.name);
    let owner = state
        .db
        .get_addresses(&hash.to_fixed_bytes().to_vec(), &[&"60"])
        .await
        .get("60")
        .unwrap()
        .clone()
        .unwrap();

    let owner: H160 = owner.parse().unwrap();

    #[cfg(feature = "eoa-auth")]
    if verify_eoa_payload(&auth, &raw_payload, &owner).is_err() {
        return (StatusCode::FORBIDDEN, "auth error");
    }

    state
        .db
        .upsert(
            &hash.to_fixed_bytes().to_vec(),
            &convert_hashmap(payload.records),
            &convert_hashmap(payload.addresses),
        )
        .await;
    (StatusCode::OK, "ok")
}

pub fn verify_eoa_payload(auth: &str, message: &str, owner: &H160) -> Result<(), ()> {
    let signature = ethers::types::Signature::from_str(&auth).unwrap();

    let payload = signature.recover(message).unwrap();

    info!("Recovered payload: {:?}", payload);
    info!("Owner: {:?}", owner);

    if payload.eq(owner) {
        Ok(())
    } else {
        Err(())
    }
}
