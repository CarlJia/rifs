use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use chrono::DateTime;
use serde::Deserialize;
use tracing::info;

use crate::app_state::AppState;
use crate::middleware::{verify_token_from_headers, AdminGuard};
use crate::models::{CreateTokenPayload, TokenQuery, TokenRole};
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

/// 列出Token - 仅管理员
pub async fn list_tokens(
    State(app_state): State<AppState>,
    _admin: AdminGuard,
    Query(query): Query<TokenQuery>,
) -> Result<impl IntoResponse, AppError> {
    info!("收到Token列表请求: {:?}", query);

    let connection = app_state.db_pool().get_connection();
    let token_service = TokenService::new(connection);
    let page_result = token_service.query_tokens(&query).await?;
    let tokens = page_result.items;
    let total = page_result.total;

    info!("返回Token列表: {} 条记录，总计 {} 条", tokens.len(), total);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "查询Token列表成功",
        "data": {
            "items": tokens,
            "total": total,
            "limit": query.limit.unwrap_or(20),
            "offset": query.offset.unwrap_or(0)
        }
    })))
}

/// 创建Token - 仅管理员
pub async fn create_token(
    State(app_state): State<AppState>,
    _admin: AdminGuard,
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

/// 删除Token - 仅管理员
pub async fn delete_token(
    State(app_state): State<AppState>,
    Path(token_id): Path<i32>,
    _admin: AdminGuard,
) -> Result<impl IntoResponse, AppError> {
    let token_service = TokenService::new(app_state.db_pool().get_connection());
    token_service.delete_token_with_data(app_state.db_pool(), token_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// 获取单个Token - 仅管理员
pub async fn get_token(
    State(app_state): State<AppState>,
    Path(token_id): Path<i32>,
    _admin: AdminGuard,
) -> Result<impl IntoResponse, AppError> {
    let connection = app_state.db_pool().get_connection();
    let token_service = TokenService::new(connection);
    let token = token_service.get_token(token_id).await?;
    Ok(Json(token))
}
