use axum::{extract::State, http::StatusCode, Json};

use crate::error::Result;
use crate::AppState;
use shared::protocol::{AuthResponse, LoginRequest, RegisterRequest};

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    let (user, token) = state
        .auth_service
        .register(&state.db, req.username, req.email, req.password)
        .await?;

    let response = AuthResponse {
        token,
        user: shared::types::User {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        },
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Login a user
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let (user, token) = state
        .auth_service
        .login(&state.db, req.username, req.password)
        .await?;

    let response = AuthResponse {
        token,
        user: shared::types::User {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        },
    };

    Ok(Json(response))
}
