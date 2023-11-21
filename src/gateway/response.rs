use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum GatewayResponse {
    Data(String),
    Error(String),
}

impl IntoResponse for GatewayResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            GatewayResponse::Data(data) => {
                (StatusCode::OK, Json(ResolveCCIPPostResponse { data })).into_response()
            }
            GatewayResponse::Error(message) => (
                StatusCode::NOT_IMPLEMENTED,
                Json(ResolveCCIPPostErrorResponse { message }),
            )
                .into_response(),
        }
    }
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
