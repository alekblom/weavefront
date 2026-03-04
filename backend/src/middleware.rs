use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct AuthError {
    error: String,
}

pub async fn require_auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthError>)> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t.to_string(),
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    error: "Missing or invalid Authorization header".to_string(),
                }),
            ))
        }
    };

    let valid = state.store.validate_session(&token).await.unwrap_or(false);
    if !valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "Invalid or expired session".to_string(),
            }),
        ));
    }

    Ok(next.run(req).await)
}
