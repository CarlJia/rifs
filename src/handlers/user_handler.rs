use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use tracing::{error, info};

use crate::{
    app_state::AppState,
    handlers::ApiResponse,
    middleware::{AdminGuard, AuthGuard},
    models::{CreateUserRequest, UserQuery, UserInfo, UserStats},
    utils::AppError,
};

/// 创建用户
pub async fn create_user(
    State(app_state): State<AppState>,
    admin: AdminGuard,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_connection = app_state.db_pool().get_connection();
        let user_service = crate::services::UserService::new(db_connection.clone()).map_err(|e| AppError::Internal(format!("创建用户服务失败: {}", e)))?;

        match user_service.create_user(request).await {
        Ok(user) => {
            info!("管理员 {} 创建用户: {}", admin.user.id, user.token);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "用户创建成功",
                "data": user
            })))
        }
        Err(e) => {
            error!("创建用户失败: {}", e);
            Err(e)
        }
    }
}

/// 删除用户
pub async fn delete_user(
    State(app_state): State<AppState>,
    admin: AdminGuard,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 防止删除自己
    if id == admin.user.id {
        return Err(AppError::BadRequest("不能删除自己的账户".to_string()));
    }

    let db_connection = app_state.db_pool().get_connection();
        let user_service = crate::services::UserService::new(db_connection.clone()).map_err(|e| AppError::Internal(format!("创建用户服务失败: {}", e)))?;

        match user_service.delete_user(id).await {
        Ok(_) => {
            info!("管理员 {} 删除用户: {}", admin.user.id, id);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "用户删除成功",
                "data": {"deleted_id": id}
            })))
        }
        Err(e) => {
            error!("删除用户失败: {}", e);
            Err(e)
        }
    }
}

/// 查询用户列表
pub async fn list_users(
    State(app_state): State<AppState>,
    admin: AdminGuard,
    Query(query): Query<UserQuery>,
) -> Result<impl IntoResponse, AppError> {
    let db_connection = app_state.db_pool().get_connection();
    let user_service = crate::services::UserService::new(db_connection)?;

    match user_service.list_users(query).await {
        Ok((users, total)) => {
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "查询用户列表成功",
                "data": {
                    "users": users,
                    "total": total,
                }
            })))
        }
        Err(e) => {
            error!("查询用户列表失败: {}", e);
            Err(e)
        }
    }
}

/// 获取用户统计信息
pub async fn get_user_stats(
    State(app_state): State<AppState>,
    admin: AdminGuard,
) -> Result<impl IntoResponse, AppError> {
    let db_connection = app_state.db_pool().get_connection();
    let user_service = crate::services::UserService::new(db_connection)?;

    match user_service.get_user_stats().await {
        Ok(stats) => Ok(Json(serde_json::json!({
            "success": true,
            "message": "获取用户统计成功",
            "data": stats
        }))),
        Err(e) => {
            error!("获取用户统计失败: {}", e);
            Err(e)
        }
    }
}

/// 获取当前用户信息
pub async fn get_current_user(
    auth: AuthGuard,
) -> Result<impl IntoResponse, AppError> {
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "获取当前用户信息成功",
        "data": {
            "id": auth.user.id,
            "token": auth.user.token,
            "role": auth.user.role,
            "expires_at": auth.user.expires_at,
            "upload_quota": auth.user.upload_quota,
            "used_quota": auth.user.used_quota,
            "remaining_quota": auth.user.remaining_quota,
        }
    })))
}

/// 更新用户配额
pub async fn update_user_quota(
    State(app_state): State<AppState>,
    admin: AdminGuard,
    Path(id): Path<i64>,
    Json(request): Json<Value>,
) -> Result<impl IntoResponse, AppError> {
    let upload_quota = request
        .get("upload_quota")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| AppError::BadRequest("缺少upload_quota字段".to_string()))?;

    let db_connection = app_state.db_pool().get_connection();
    let user_service = crate::services::UserService::new(db_connection)?;

    // 获取用户信息
    let user = user_service.get_user_by_id(id).await?;
    
    // 更新配额
    use sea_orm::{ActiveModelTrait, Set};
    use crate::entities::user;
    
    let active_model = user::ActiveModel {
        id: Set(id),
        upload_quota: Set(upload_quota as i64),
        ..Default::default()
    };
    
    active_model.update(&app_state.db_pool().get_connection()).await?;

    info!("管理员 {} 更新用户 {} 配额: {} bytes", admin.user.id, id, upload_quota);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "更新用户配额成功",
        "data": {
            "id": id,
            "upload_quota": upload_quota,
        }
    })))
}