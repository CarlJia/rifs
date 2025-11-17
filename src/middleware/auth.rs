use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderName},
};
use tracing::warn;

use crate::{config::AppConfig, utils::AppError};

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
