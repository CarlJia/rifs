use axum::{
    extract::State,
    response::{IntoResponse, Json},
};

use crate::app_state::AppState;
use crate::handlers::{ApiResponse, static_files::CACHE_MANAGEMENT_HTML};
use crate::middleware::AuthGuard;
use crate::models::CacheCleanupResult;
use crate::services::CacheService;
use crate::utils::AppError;

/// 获取缓存统计信息
pub async fn get_cache_stats(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let db_connection = state.db_pool().get_connection();
    let cache_service = CacheService::new(db_connection)?;

    let stats = cache_service.get_stats().await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "获取缓存统计成功",
        "data": stats
    })))
}

/// 执行热度衰减
pub async fn decay_heat_scores(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let db_connection = state.db_pool().get_connection();
    let cache_service = CacheService::new(db_connection)?;

    let updated_count = cache_service.decay_heat_scores().await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("热度衰减完成，更新了 {} 个缓存项", updated_count),
        "data": updated_count
    })))
}

/// 自动清理缓存（主要清理接口）
/// 只在空间使用率达到阈值时执行清理
pub async fn auto_cleanup_cache(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let connection = app_state.db_pool().get_connection();
    let cache_service = CacheService::new(connection)?;

    let result = cache_service.auto_cleanup().await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "自动清理完成",
        "data": result
    })))
}

/// 清空所有缓存
pub async fn clear_all_cache(
    _: AuthGuard,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let connection = app_state.db_pool().get_connection();
    let cache_service = CacheService::new(connection)?;

    let result = cache_service.clear_all().await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "清理完成",
        "data": result
    })))
}

/// 缓存管理面板（返回HTML页面）
pub async fn cache_management_dashboard() -> impl IntoResponse {
    axum::response::Html(CACHE_MANAGEMENT_HTML)
}
