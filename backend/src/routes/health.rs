use axum::Json;
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub ipfs_configured: bool,
    pub arweave_configured: bool,
}

pub async fn health(state: axum::extract::State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        ipfs_configured: state.ipfs.is_some(),
        arweave_configured: state.config.arweave_gateway_url.is_some(),
    })
}
