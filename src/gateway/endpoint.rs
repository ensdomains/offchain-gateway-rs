use axum::{
    extract::Path,
    http::StatusCode,
    Json
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

pub async fn route(
    // Ommiting sender from path awaiting viem patch
    // Path(sender): Path<String>,
    Json(request_payload): Json<ResolveCCIPPostPayload>,
) -> (StatusCode, Json<ResolveCCIPPostResponse>) {
    // info!("Received request from {}", sender);
    info!("Received request with {}", request_payload.data);

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ResolveCCIPPostResponse::default()),
    )
    // match resolve::resolve(request_payload) {
    //     Ok(x) => (StatusCode::OK, Json(x)),
    //     Err(e) => {
    //         error!("Error: {:?}", e);
    //         (e.into(), Json(ResolveCCIPPostResponse::default()))
    //     }
    // }
}

#[derive(Deserialize, Debug)]
pub struct ResolveCCIPPostPayload {
    data: String,
    sender: String,
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
