use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::AppState;

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

/// Extract user ID from Authorization header
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Auth("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Auth("Invalid authorization header format".to_string()))?;

    let claims = state.auth_service.validate_token(token)?;

    // Parse user ID from claims
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Auth("Invalid user ID in token".to_string()))?;

    // Insert AuthUser into request extensions
    req.extensions_mut().insert(AuthUser { user_id });

    Ok(next.run(req).await)
}

/// Helper function to extract user ID from request extensions
pub fn get_user_id(req: &Request) -> Result<Uuid> {
    req.extensions()
        .get::<AuthUser>()
        .map(|auth| auth.user_id)
        .ok_or_else(|| AppError::Auth("Unauthorized".to_string()))
}
