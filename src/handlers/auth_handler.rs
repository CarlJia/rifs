use crate::app_state::AppState;
use crate::config::AppConfig;
use crate::utils::AppError;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use crate::middleware::verify_token_from_headers;

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

/// 用户信息响应
#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UserInfo>,
}

/// 用户信息
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub role: String,
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

/// 获取当前用户信息
pub async fn get_user_info(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let config = AppConfig::get();
    let auth_config = &config.auth;

    // 如果认证未启用，返回默认管理员信息
    if !auth_config.enabled {
        return Ok(Json(UserInfoResponse {
            success: true,
            data: Some(UserInfo {
                name: "admin".to_string(),
                role: "admin".to_string(),
            }),
        }));
    }

    // 验证token并获取用户信息
    let user_info = verify_token_from_headers(&headers, &app_state).await?;
    
    Ok(Json(UserInfoResponse {
        success: true,
        data: Some(UserInfo {
            name: user_info.name,
            role: user_info.role.as_str().to_string(),
        }),
    }))
}
