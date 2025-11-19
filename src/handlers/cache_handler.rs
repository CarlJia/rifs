use axum::{
    extract::State,
    response::{IntoResponse, Json},
    http::HeaderMap,
};
use serde::Serialize;

use crate::app_state::AppState;
use crate::handlers::static_files::CACHE_MANAGEMENT_HTML;
use crate::middleware::verify_token_from_headers;
use crate::models::{CacheCleanupResult, TokenRole};
use crate::services::CacheService;
use crate::utils::AppError;

/// 通用API响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: Option<T>) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data,
        }
    }
}

/// 获取缓存统计信息 - 仅管理员
pub async fn get_cache_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // 验证管理员权限
    let auth_user = verify_token_from_headers(&headers, &state).await?;
    if auth_user.role != TokenRole::Admin {
        return Err(AppError::Unauthorized("需要管理员权限访问此资源".to_string()));
    }

    let db_connection = state.db_pool().get_connection();
    let cache_service = CacheService::new(db_connection)?;

    let stats = cache_service.get_stats().await?;

    Ok(Json(ApiResponse::success("获取缓存统计成功", Some(stats))))
}

/// 执行热度衰减 - 仅管理员
pub async fn decay_heat_scores(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // 验证管理员权限
    let auth_user = verify_token_from_headers(&headers, &state).await?;
    if auth_user.role != TokenRole::Admin {
        return Err(AppError::Unauthorized("需要管理员权限访问此资源".to_string()));
    }

    let db_connection = state.db_pool().get_connection();
    let cache_service = CacheService::new(db_connection)?;

    let updated_count = cache_service.decay_heat_scores().await?;

    Ok(Json(ApiResponse::success(
        &format!("热度衰减完成，更新了 {} 个缓存项", updated_count),
        Some(updated_count),
    )))
}

/// 自动清理缓存（主要清理接口） - 仅管理员
/// 只在空间使用率达到阈值时执行清理
pub async fn auto_cleanup_cache(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<CacheCleanupResult>>, AppError> {
    // 验证管理员权限
    let auth_user = verify_token_from_headers(&headers, &app_state).await?;
    if auth_user.role != TokenRole::Admin {
        return Err(AppError::Unauthorized("需要管理员权限访问此资源".to_string()));
    }

    let connection = app_state.db_pool().get_connection();
    let cache_service = CacheService::new(connection)?;

    let result = cache_service.auto_cleanup().await?;

    Ok(Json(ApiResponse::success("自动清理完成", Some(result))))
}

/// 通用清理缓存 - 仅管理员
pub async fn clean_cache(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<CacheCleanupResult>>, AppError> {
    // 验证管理员权限
    let auth_user = verify_token_from_headers(&headers, &app_state).await?;
    if auth_user.role != TokenRole::Admin {
        return Err(AppError::Unauthorized("需要管理员权限访问此资源".to_string()));
    }

    let connection = app_state.db_pool().get_connection();
    let cache_service = CacheService::new(connection)?;

    // 解析参数
    // max_age: 前端传递秒数，转换为天数
    let max_age = payload.get("max_age")
        .and_then(|v| v.as_u64())
        .map(|seconds| (seconds / 86400) as u32); // 转换为天数
    
    let max_size = payload.get("max_size")
        .and_then(|v| v.as_u64());

    let result = cache_service.cleanup_cache(max_age, max_size).await?;

    Ok(Json(ApiResponse::success("清理完成", Some(result))))
}

/// 清空所有缓存 - 仅管理员
pub async fn clear_all_cache(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<CacheCleanupResult>>, AppError> {
    // 验证管理员权限
    let auth_user = verify_token_from_headers(&headers, &app_state).await?;
    if auth_user.role != TokenRole::Admin {
        return Err(AppError::Unauthorized("需要管理员权限访问此资源".to_string()));
    }

    let connection = app_state.db_pool().get_connection();
    let cache_service = CacheService::new(connection)?;

    let result = cache_service.clear_all().await?;

    Ok(Json(ApiResponse::success("清理完成", Some(result))))
}

/// 缓存管理面板（返回HTML页面）
pub async fn cache_management_dashboard() -> impl IntoResponse {
    axum::response::Html(CACHE_MANAGEMENT_HTML)
}
