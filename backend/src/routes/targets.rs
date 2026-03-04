use axum::Json;

use crate::models::target::{available_targets, DeployTarget};

pub async fn list_targets() -> Json<Vec<DeployTarget>> {
    Json(available_targets())
}
