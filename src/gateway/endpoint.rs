use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::state::GlobalState;

use super::{payload::ResolveCCIPPostPayload, response::GatewayResponse};

pub async fn route(
    // Ommiting sender from path awaiting viem patch
    // Path(sender): Path<String>,
    State(state): State<Arc<GlobalState>>,
    Json(request_payload): Json<ResolveCCIPPostPayload>,
) -> impl IntoResponse {
    handle(request_payload, state)
        .await
        .map_err(|x| x.into_response())
}

#[derive(Debug, Error)]
pub enum CCIPEndpointError {
    #[error("Invalid prefix: {0}")]
    DecodeError(#[from] super::payload::ResolverDecodeError),
    #[error("Resolve error: {0}")]
    ResolveError(#[from] super::resolution::ResolveError),
    #[error("Sign error: {0}")]
    SignError(#[from] super::signing::SignError),
}

impl IntoResponse for CCIPEndpointError {
    fn into_response(self) -> Response {
        // TODO: remove
        if let CCIPEndpointError::ResolveError(x) = &self {
            if let super::resolution::ResolveError::Unparsable = x {
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, x.to_string())
                    .into_response();
            }
        }

        GatewayResponse::Error(self.to_string()).into_response()
    }
}

async fn handle(
    payload: ResolveCCIPPostPayload,
    state: Arc<GlobalState>,
) -> Result<GatewayResponse, CCIPEndpointError> {
    Ok(payload
        .decode()?
        .resolve(state.clone())
        .await?
        .sign(state.clone())?)
}
