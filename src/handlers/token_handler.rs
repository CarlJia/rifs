use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use chrono::DateTime;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::middleware::auth::AuthGuard;
use crate::models::{CreateTokenPayload, TokenRole};
use crate::services::TokenService;
use crate::utils::AppError;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    pub role: String,
    #[serde(default)]
    pub max_upload_size: Option<u64>,
    pub expires_at: Option<String>,
}

pub async fn list_tokens(
    State(app_state): State<AppState>,
    _: AuthGuard,
) -> Result<impl IntoResponse, AppError> {
    let connection = app_state.db_pool().get_connection();
    let token_service = TokenService::new(connection);
    let tokens = token_service.list_tokens().await?;
    Ok(Json(tokens))
}

pub async fn create_token(
    State(app_state): State<AppState>,
    _: AuthGuard,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let connection = app_state.db_pool().get_connection();
    let token_service = TokenService::new(connection);
    
    let expires_at = if let Some(s) = payload.expires_at {
        DateTime::parse_from_rfc3339(&s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc))
    } else {
        None
    };
    
    let create_payload = CreateTokenPayload {
        name: payload.name,
        role: TokenRole::from(payload.role.as_str()),
        max_upload_size: payload.max_upload_size,
        expires_at,
    };
    
    let response = token_service.create_token(create_payload).await?;
    Ok(Json(response))
}

pub async fn delete_token(
    State(app_state): State<AppState>,
    Path(token_id): Path<i32>,
    _: AuthGuard,
) -> Result<impl IntoResponse, AppError> {
    let token_service = TokenService::new(app_state.db_pool().get_connection());
    token_service.delete_token_with_data(app_state.db_pool(), token_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn get_token(
    State(app_state): State<AppState>,
    Path(token_id): Path<i32>,
    _: AuthGuard,
) -> Result<impl IntoResponse, AppError> {
    let connection = app_state.db_pool().get_connection();
    let token_service = TokenService::new(connection);
    let token = token_service.get_token(token_id).await?;
    Ok(Json(token))
}
