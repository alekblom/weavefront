use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct AuthError {
    pub error: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<AuthError>)> {
    if req.password != state.config.admin_password {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "Invalid password".to_string(),
            }),
        ));
    }

    let token = generate_token();
    state.store.create_session(&token).await.map_err(|e| {
        tracing::error!("session create error: {e:#}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthError {
                error: "Internal server error".to_string(),
            }),
        )
    })?;

    Ok(Json(LoginResponse { token }))
}

pub async fn logout(
    State(state): State<AppState>,
    token: String,
) -> StatusCode {
    let _ = state.store.delete_session(&token).await;
    StatusCode::NO_CONTENT
}

fn generate_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    // Simple token: hex timestamp + random-ish suffix from address
    let random_part: u64 = (ts as u64).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    format!("{:x}{:x}", ts, random_part)
}
