use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};

use crate::state::GlobalState;

use super::{payload::ResolveCCIPPostPayload, response::GatewayResponse};

pub async fn route(
    // Ommiting sender from path awaiting viem patch
    // Path(sender): Path<String>,
    State(state): State<Arc<GlobalState>>,
    Json(request_payload): Json<ResolveCCIPPostPayload>,
) -> impl IntoResponse {
    async fn handle(
        r: ResolveCCIPPostPayload,
        s: Arc<GlobalState>,
    ) -> Result<GatewayResponse, Response> {
        Ok(r.decode()
            .unwrap()
            .resolve(s.clone())
            .await
            .unwrap()
            .sign(s.clone())
            .unwrap())
    }

    handle(request_payload, state)
        .await
        .map_err(|x| x.into_response())
}
