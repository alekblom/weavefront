use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;

use crate::models::deployment::Deployment;
use crate::routes::projects::ApiError;
use crate::services::pinata::PinataService;
use crate::AppState;

#[derive(Serialize)]
pub struct DeployResponse {
    pub deployment_id: String,
    pub cid: String,
    pub gateway_url: String,
    pub size_bytes: u64,
}

type ApiResult<T> = Result<T, (StatusCode, Json<ApiError>)>;

fn internal_err(e: anyhow::Error) -> (StatusCode, Json<ApiError>) {
    tracing::error!("deploy error: {e:#}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: "Internal server error".to_string(),
        }),
    )
}

pub async fn deploy_upload(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    mut multipart: Multipart,
) -> ApiResult<Json<DeployResponse>> {
    // Verify project exists
    let project = state
        .store
        .get(&project_id)
        .await
        .map_err(internal_err)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError {
                    error: "Project not found".to_string(),
                }),
            )
        })?;

    // Get Pinata service
    let pinata = state.pinata.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiError {
                error: "IPFS deployment not configured. Set PINATA_JWT in environment.".to_string(),
            }),
        )
    })?;

    // Read uploaded file
    let field = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: format!("Failed to read upload: {e}"),
            }),
        )
    })?;

    let field = field.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "No file uploaded".to_string(),
            }),
        )
    })?;

    let filename = field
        .file_name()
        .unwrap_or("site.tar.gz")
        .to_string();
    let data = field.bytes().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: format!("Failed to read file data: {e}"),
            }),
        )
    })?;

    // Create deployment record
    let dep_id = state
        .store
        .create_deployment(&project_id, &project.target)
        .await
        .map_err(internal_err)?;

    // Update project status
    let _ = state.store.update_status(&project_id, "deploying", None).await;

    // Upload to Pinata
    match pinata.pin_file(&filename, data.to_vec()).await {
        Ok((cid, size)) => {
            let gateway_url = PinataService::gateway_url(&cid);
            let _ = state.store.complete_deployment(&dep_id, &cid, &gateway_url, size).await;
            let now = crate::services::store::now_iso_pub();
            let _ = state.store.update_status(&project_id, "live", Some(&now)).await;

            Ok(Json(DeployResponse {
                deployment_id: dep_id,
                cid,
                gateway_url,
                size_bytes: size,
            }))
        }
        Err(e) => {
            let _ = state.store.fail_deployment(&dep_id).await;
            let _ = state.store.update_status(&project_id, "failed", None).await;
            Err(internal_err(e))
        }
    }
}

pub async fn list_deployments(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> ApiResult<Json<Vec<Deployment>>> {
    state
        .store
        .list_deployments(&project_id)
        .await
        .map(Json)
        .map_err(internal_err)
}
