use std::{env, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{debug, info};

use crate::state::GlobalState;

/// Starts the HTTP Server
pub async fn serve(state: GlobalState) {
    info!("Starting webserver");

    let app = Router::new()
        .route("/", get(root))
        .route("/gateway", post(crate::gateway::endpoint::route));

    #[cfg(feature = "selfservice")]
    let app = app.route("/update", post(crate::selfservice::endpoint::route));

    let app = app
        .route("/view/:name", get(crate::selfservice::view::route))
        .with_state(Arc::new(state))
        .layer(CorsLayer::very_permissive());

    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse::<u16>()
            .expect("port should fit in u16")
    ))
    .await
    .unwrap();

    debug!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

/// Self Endpoint on the Gateway
async fn root() -> &'static str {
    "CCIP Rust Gateway v0.0.1!"
}
