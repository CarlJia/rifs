use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use crate::app_state::AppState;
use crate::config::AppConfig;
use crate::utils::AppError;

/// 认证配置响应
#[derive(Debug, Serialize)]
pub struct AuthConfigResponse {
    pub enabled: bool,
}

/// 认证请求
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub token: String,
}

/// 认证响应
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
}

impl AuthResponse {
    pub fn success(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            header_name: Some(AppConfig::get().auth.header_name.clone()),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            header_name: None,
        }
    }
}

/// 获取认证配置
pub async fn get_auth_config() -> Result<impl IntoResponse, AppError> {
    let config = AppConfig::get();
    Ok(Json(AuthConfigResponse {
        enabled: config.auth.enabled,
    }))
}

/// 验证认证令牌
pub async fn verify_token(
    State(_app_state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let config = AppConfig::get();
    let auth_config = &config.auth;

    // 如果认证未启用，直接允许访问
    if !auth_config.enabled {
        return Ok(Json(AuthResponse::success("认证已禁用，无需验证")));
    }

    // 检查 token 是否匹配
    if let Some(expected_token) = &auth_config.token {
        if payload.token.trim() == expected_token.trim() {
            Ok(Json(AuthResponse::success("认证成功")))
        } else {
            Ok(Json(AuthResponse::error("认证令牌无效")))
        }
    } else {
        Ok(Json(AuthResponse::error("未配置认证令牌")))
    }
}
