use std::net::SocketAddr;

use crate::database::Database;
use axum::{
    routing::{get, post},
    Router, Server,
};
use tower_http::cors::CorsLayer;
use tracing::{debug, info};

/// Starts the HTTP Server
pub async fn serve(_db: Database) {
    info!("Starting webserver");
    let app = Router::new()
        .route("/", get(root))
        .route("/gateway", post(crate::gateway::endpoint::route))
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    debug!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Self Endpoint on the Gateway
async fn root() -> &'static str {
    "CCIP Rust Gateway v0.0.1!"
}
