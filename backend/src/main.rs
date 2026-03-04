mod config;
mod middleware;
mod models;
mod routes;
mod services;

use axum::{
    middleware as axum_mw,
    routing::{delete, get, post},
    Router,
};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use crate::config::AppConfig;
use crate::services::{ipfs::IpfsService, pinata::PinataService, store::ProjectStore};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub store: ProjectStore,
    pub ipfs: Option<IpfsService>,
    pub pinata: Option<PinataService>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let config = AppConfig::from_env();
    let ipfs = config
        .ipfs_api_url
        .as_ref()
        .map(|url| IpfsService::new(url.clone()));
    let pinata = config
        .pinata_jwt
        .as_ref()
        .map(|jwt| PinataService::new(jwt.clone(), config.ipfs_gateway.clone()));

    let store = ProjectStore::open(&config.db_path)?;
    tracing::info!("database opened at {}", config.db_path);

    if pinata.is_some() {
        tracing::info!("Pinata IPFS pinning configured");
    }

    let state = AppState {
        config: config.clone(),
        store,
        ipfs,
        pinata,
    };

    // Public routes (no auth)
    let public_api = Router::new()
        .route("/health", get(routes::health::health))
        .route("/auth/login", post(routes::auth::login))
        .with_state(state.clone());

    // Protected routes (require auth)
    let protected_api = Router::new()
        .route("/targets", get(routes::targets::list_targets))
        .route("/projects", get(routes::projects::list_projects))
        .route("/projects", post(routes::projects::create_project))
        .route("/projects/{id}", get(routes::projects::get_project))
        .route("/projects/{id}", delete(routes::projects::delete_project))
        .route("/projects/{id}/deploy", post(routes::deploy::deploy_upload))
        .route("/projects/{id}/deployments", get(routes::deploy::list_deployments))
        .layer(axum_mw::from_fn_with_state(
            state.clone(),
            middleware::require_auth,
        ))
        .with_state(state);

    let app = Router::new()
        .nest("/api", public_api)
        .nest("/api", protected_api)
        .fallback_service(ServeDir::new("../public"))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = config.listen_addr();
    tracing::info!("weavefront v{} listening on {}", env!("CARGO_PKG_VERSION"), addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
