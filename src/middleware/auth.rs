use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderName},
};
use tracing::warn;

use crate::{app_state::AppState, config::AppConfig, models::{ApiTokenInfo, TokenRole}, utils::AppError};
use crate::services::TokenService;

/// 从请求头中验证token并返回用户信息（公共方法）
pub async fn verify_token_from_headers(
    headers: &axum::http::HeaderMap,
    app_state: &AppState,
) -> Result<ApiTokenInfo, AppError> {
    use axum::http::{header, HeaderName};
    use chrono::Utc;
    use crate::models::TokenRole;

    let config = AppConfig::get();
    let auth_config = &config.auth;

    // 如果认证未启用，返回一个默认的管理员token信息
    if !auth_config.enabled {
        return Ok(ApiTokenInfo {
            id: 0,
            name: "default".to_string(),
            role: TokenRole::Admin,
            max_upload_size: None,
            used_upload_size: 0,
            expires_at: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_used_at: None,
        });
    }

    let header_name: HeaderName = auth_config
        .header_name
        .parse()
        .unwrap_or_else(|_| header::AUTHORIZATION.clone());
    let is_authorization_header = header_name == header::AUTHORIZATION;

    let header_value = headers
        .get(&header_name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim);

    if let Some(value) = header_value {
        let token = if is_authorization_header {
            if let Some(token) = value.strip_prefix("Bearer ") {
                token.trim()
            } else {
                value.trim()
            }
        } else {
            value.trim()
        };

        // 从数据库验证token
        let connection = app_state.db_pool().get_connection();
        let token_service = TokenService::new(connection);
        
        // 通过token hash查找用户信息
        if let Some(token_info) = token_service.find_by_token_hash(token).await? {
            if !token_info.is_active {
                return Err(AppError::Unauthorized("Token已被禁用".to_string()));
            }
            
            // 检查token是否过期
            if let Some(expires_at) = token_info.expires_at {
                if expires_at < Utc::now() {
                    return Err(AppError::Unauthorized("Token已过期".to_string()));
                }
            }
            
            Ok(token_info)
        } else {
            Err(AppError::Unauthorized("Token不存在".to_string()))
        }
    } else {
        Err(AppError::Unauthorized("缺少认证token".to_string()))
    }
}

/// 请求认证守卫
///
/// 当配置启用令牌认证时，检查请求头中是否携带有效的令牌。
pub struct AuthGuard;

impl<S> FromRequestParts<S> for AuthGuard
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        {
            let config = AppConfig::get();
            let auth_config = &config.auth;

            if !auth_config.enabled {
                return Ok(Self);
            }

            let expected_token = auth_config
                .token
                .as_deref()
                .filter(|token| !token.trim().is_empty())
                .ok_or_else(|| AppError::Unauthorized("认证配置缺少有效令牌".to_string()))?;

            let header_name: HeaderName = auth_config
                .header_name
                .parse()
                .unwrap_or_else(|_| header::AUTHORIZATION.clone());
            let is_authorization_header = header_name == header::AUTHORIZATION;

            let header_value = parts
                .headers
                .get(&header_name)
                .and_then(|value| value.to_str().ok())
                .map(str::trim);

            if let Some(value) = header_value {
                if is_authorization_header {
                    if let Some(token) = value.strip_prefix("Bearer ") {
                        if token.trim() == expected_token {
                            return Ok(Self);
                        }
                    }

                    if value == expected_token {
                        return Ok(Self);
                    }
                } else {
                    // 对于其他 header 名称，直接比较值
                    if value.trim() == expected_token {
                        return Ok(Self);
                    }
                }
            }

            warn!("认证失败: 缺少或提供了无效的凭证");
            Err(AppError::Unauthorized(
                "认证失败，缺少有效的凭证".to_string(),
            ))
        }
    }
}

/// 带Token信息的认证守卫
pub struct AuthenticatedUser(pub ApiTokenInfo);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = AppConfig::get();
        let auth_config = &config.auth;

        if !auth_config.enabled {
            return Err(AppError::Unauthorized("认证未启用".to_string()));
        }

        let header_name: HeaderName = auth_config
            .header_name
            .parse()
            .unwrap_or_else(|_| header::AUTHORIZATION.clone());
        let is_authorization_header = header_name == header::AUTHORIZATION;

        let header_value = parts
            .headers
            .get(&header_name)
            .and_then(|value| value.to_str().ok())
            .map(str::trim);

        if let Some(value) = header_value {
            let token = if is_authorization_header {
                if let Some(token) = value.strip_prefix("Bearer ") {
                    token.trim()
                } else {
                    value.trim()
                }
            } else {
                value.trim()
            };

            // 首先检查是否是配置文件中的管理员token
            if let Some(expected_token) = &auth_config.token {
                if token.trim() == expected_token.trim() {
                    // 创建一个管理员token信息
                    use crate::models::TokenRole;
                    use chrono::Utc;
                    let token_info = ApiTokenInfo {
                        id: 0, // 使用0表示管理员token
                        name: "管理员".to_string(),
                        role: TokenRole::Admin,
                        max_upload_size: None,
                        used_upload_size: 0,
                        expires_at: None,
                        is_active: true,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        last_used_at: Some(Utc::now()),
                    };
                    return Ok(AuthenticatedUser(token_info));
                }
            }

            // 如果不是配置文件中的token，则返回错误（暂时不支持数据库token查询）
            // TODO: 实现从数据库查询token的逻辑
            Err(AppError::Unauthorized("Token不存在".to_string()))
        } else {
            warn!("认证失败: 缺少或提供了无效的凭证");
            Err(AppError::Unauthorized(
                "认证失败，缺少有效的凭证".to_string(),
            ))
        }
    }
}

/// Admin角色守卫 - 确保只有管理员可以访问受保护的端点
pub struct AdminGuard(pub ApiTokenInfo);

impl<S> FromRequestParts<S> for AdminGuard
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = AppConfig::get();
        let auth_config = &config.auth;

        if !auth_config.enabled {
            return Ok(Self(ApiTokenInfo {
                id: 0,
                name: "default_admin".to_string(),
                role: TokenRole::Admin,
                max_upload_size: None,
                used_upload_size: 0,
                expires_at: None,
                is_active: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                last_used_at: None,
            }));
        }

        let header_name: HeaderName = auth_config
            .header_name
            .parse()
            .unwrap_or_else(|_| header::AUTHORIZATION.clone());
        let is_authorization_header = header_name == header::AUTHORIZATION;

        let header_value = parts
            .headers
            .get(&header_name)
            .and_then(|value| value.to_str().ok())
            .map(str::trim);

        if let Some(value) = header_value {
            let token = if is_authorization_header {
                if let Some(token) = value.strip_prefix("Bearer ") {
                    token.trim()
                } else {
                    value.trim()
                }
            } else {
                value.trim()
            };

            // 首先检查是否是配置文件中的管理员token
            if let Some(expected_token) = &auth_config.token {
                if token.trim() == expected_token.trim() {
                    // 创建一个管理员token信息
                    let token_info = ApiTokenInfo {
                        id: 0,
                        name: "管理员".to_string(),
                        role: TokenRole::Admin,
                        max_upload_size: None,
                        used_upload_size: 0,
                        expires_at: None,
                        is_active: true,
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                        last_used_at: Some(chrono::Utc::now()),
                    };
                    return Ok(AdminGuard(token_info));
                }
            }

            warn!("Admin访问被拒绝: 提供的令牌无效或权限不足");
            Err(AppError::Unauthorized(
                "需要管理员权限访问此资源".to_string(),
            ))
        } else {
            warn!("认证失败: 缺少或提供了无效的凭证");
            Err(AppError::Unauthorized(
                "缺少有效的认证凭证".to_string(),
            ))
        }
    }
}
