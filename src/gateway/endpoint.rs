use axum::{extract::Path, http::StatusCode, Json};
use ethers::abi::ParamType;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::ccip::lookup::ResolverFunctionCall;

use super::payload::ResolveCCIPPostPayload;

pub async fn route(
    // Ommiting sender from path awaiting viem patch
    // Path(sender): Path<String>,
    Json(request_payload): Json<ResolveCCIPPostPayload>,
) -> (StatusCode, Json<ResolveCCIPPostResponse>) {
    match request_payload.decode() {
        Ok((name, resolver_function_call)) => {
            info!("Decoded name: {}", name);
            info!("Decoded resolver_function_call: {:?}", resolver_function_call);

            (
                StatusCode::NOT_IMPLEMENTED,
                Json(ResolveCCIPPostResponse::default()),
            )        
        }
        Err(e) => {
            info!("Error: {:?}", e);
            info!("Request payload: {:?}", request_payload);
            (
                StatusCode::NOT_IMPLEMENTED,
                Json(ResolveCCIPPostResponse::default()),
            )        
        }
    }

    // match resolve::resolve(request_payload) {
    //     Ok(x) => (StatusCode::OK, Json(x)),
    //     Err(e) => {
    //         error!("Error: {:?}", e);
    //         (e.into(), Json(ResolveCCIPPostResponse::default()))
    //     }
    // }
}

#[derive(Serialize)]
pub struct ResolveCCIPPostResponse {
    data: String,
}

#[derive(Serialize)]
struct ResolveCCIPPostErrorResponse {
    message: String,
}

impl Default for ResolveCCIPPostResponse {
    fn default() -> Self {
        Self {
            data: "0x".to_string(),
        }
    }
}
