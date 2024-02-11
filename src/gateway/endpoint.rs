use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::state::GlobalState;
use crate::utils;

use super::{payload::ResolveCCIPPostPayload, response::GatewayResponse};

pub async fn route(
    // Omitting sender from path awaiting viem patch
    // Path(sender): Path<String>,
    State(state): State<Arc<GlobalState>>,
    // custom less strict json implementation because viem makes the request wrong
    utils::axum_json::Json(request_payload): utils::axum_json::Json<ResolveCCIPPostPayload>,
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
