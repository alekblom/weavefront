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

pub async fn list_projects(State(state): State<AppState>) -> Json<Vec<Project>> {
    Json(state.store.list().await)
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Project>, (StatusCode, Json<ApiError>)> {
    state.store.get(&id).await.map(Json).ok_or((
        StatusCode::NOT_FOUND,
        Json(ApiError {
            error: "Project not found".to_string(),
        }),
    ))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> (StatusCode, Json<Project>) {
    let project = state.store.create(req).await;
    (StatusCode::CREATED, Json(project))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> StatusCode {
    if state.store.delete(&id).await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
