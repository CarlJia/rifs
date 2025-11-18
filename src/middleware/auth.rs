use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderName},
};
use tracing::{debug, warn};

use crate::{app_state::AppState, entities::user, models::CurrentUser, utils::AppError};

/// 请求认证守卫
///
/// 验证请求中的token并返回当前用户信息。
pub struct AuthGuard {
    pub user: CurrentUser,
}

impl<S> FromRequestParts<S> for AuthGuard
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 尝试从state中获取app_state
        let app_state = match state.downcast_ref::<AppState>() {
            Some(app_state) => app_state,
            None => {
                return Err(AppError::Internal(
                    "无法获取应用状态".to_string(),
                ));
            }
        };

        // 获取配置
        let config = app_state.config();
        let auth_config = &config.auth;

        // 如果未启用认证，返回默认管理员用户
        if !auth_config.enabled {
            return Ok(Self {
                user: CurrentUser {
                    id: 1,
                    token: "disabled".to_string(),
                    role: crate::models::UserRole::Admin,
                    expires_at: None,
                    upload_quota: u64::MAX,
                    used_quota: 0,
                    remaining_quota: u64::MAX,
                },
            });
        }

        // 从请求头中获取token
        let token = extract_token_from_headers(parts, auth_config)?;

        // 查询用户信息
        let db_connection = app_state.db_pool().get_connection();
        let user_service = crate::services::user_service::UserService::new(db_connection)
            .map_err(|e| AppError::Internal(format!("创建用户服务失败: {}", e)))?;

        let user_entity = user_service
            .get_user_by_token(&token)
            .await
            .map_err(|e| match e {
                crate::utils::AppError::NotFound(_) => {
                    AppError::Unauthorized("无效的认证令牌".to_string())
                }
                _ => AppError::Internal(format!("查询用户失败: {}", e)),
            })?;

        // 检查token是否过期
        if let Some(expires_at) = user_entity.expires_at {
            if expires_at < chrono::Utc::now() {
                return Err(AppError::Unauthorized("认证令牌已过期".to_string()));
            }
        }

        // 构建当前用户信息
        let current_user = CurrentUser {
            id: user_entity.id,
            token: user_entity.token.clone(),
            role: user_entity.role,
            expires_at: user_entity.expires_at,
            upload_quota: user_entity.upload_quota as u64,
            used_quota: user_entity.used_quota as u64,
            remaining_quota: (user_entity.upload_quota - user_entity.used_quota) as u64,
        };

        debug!("用户认证成功: ID={}, Role={}", current_user.id, current_user.role);

        Ok(Self { user: current_user })
    }
}

/// 从请求头中提取token
fn extract_token_from_headers(
    parts: &Parts,
    auth_config: &crate::config::AuthConfig,
) -> Result<String, AppError> {
    let header_name: HeaderName = auth_config
        .header_name
        .parse()
        .unwrap_or_else(|_| http::header::AUTHORIZATION.clone());
    let is_authorization_header = header_name == http::header::AUTHORIZATION;

    let header_value = parts
        .headers
        .get(&header_name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim);

    if let Some(value) = header_value {
        if is_authorization_header {
            if let Some(token) = value.strip_prefix("Bearer ") {
                return Ok(token.trim().to_string());
            }

            if !value.is_empty() {
                return Ok(value.to_string());
            }
        } else {
            // 对于其他 header 名称，直接比较值
            return Ok(value.to_string());
        }
    }

    warn!("认证失败: 缺少或提供了无效的凭证");
    Err(AppError::Unauthorized(
        "认证失败，缺少有效的凭证".to_string(),
    ))
}

/// 管理员权限守卫
pub struct AdminGuard {
    pub user: CurrentUser,
}

impl<S> FromRequestParts<S> for AdminGuard
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth = AuthGuard::from_request_parts(parts, state).await?;
        
        if !auth.user.is_admin() {
            return Err(AppError::Forbidden(
                "需要管理员权限".to_string(),
            ));
        }

        Ok(Self { user: auth.user })
    }
}