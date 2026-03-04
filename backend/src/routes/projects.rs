use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;

use crate::models::project::{CreateProjectRequest, Project};
use crate::AppState;

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

type ApiResult<T> = Result<T, (StatusCode, Json<ApiError>)>;

fn internal_err(e: anyhow::Error) -> (StatusCode, Json<ApiError>) {
    tracing::error!("store error: {e:#}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: "Internal server error".to_string(),
        }),
    )
}

pub async fn list_projects(State(state): State<AppState>) -> ApiResult<Json<Vec<Project>>> {
    state.store.list().await.map(Json).map_err(internal_err)
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<Project>> {
    match state.store.get(&id).await {
        Ok(Some(p)) => Ok(Json(p)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "Project not found".to_string(),
            }),
        )),
        Err(e) => Err(internal_err(e)),
    }
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> ApiResult<(StatusCode, Json<Project>)> {
    state
        .store
        .create(req)
        .await
        .map(|p| (StatusCode::CREATED, Json(p)))
        .map_err(internal_err)
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    match state.store.delete(&id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: "Project not found".to_string(),
            }),
        )),
        Err(e) => Err(internal_err(e)),
    }
}
