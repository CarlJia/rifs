use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderName},
};
use tracing::warn;

use crate::{config::AppConfig, models::ApiTokenInfo, utils::AppError};
use crate::services::TokenService;

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
