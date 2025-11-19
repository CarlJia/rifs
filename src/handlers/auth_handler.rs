use crate::app_state::AppState;
use crate::config::AppConfig;
use crate::utils::AppError;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl AuthResponse {
    pub fn success(message: &str, role: Option<String>) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            header_name: Some(AppConfig::get().auth.header_name.clone()),
            role,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            header_name: None,
            role: None,
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
    State(app_state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    use crate::middleware::verify_token_from_headers;
    use axum::http::{HeaderMap, HeaderName, header};
    
    let config = AppConfig::get();
    let auth_config = &config.auth;

    // 如果认证未启用，直接允许访问（无权限限制）
    if !auth_config.enabled {
        return Ok(Json(AuthResponse::success("认证已禁用，无需验证", None)));
    }

    // 构造请求头来模拟token验证
    let mut headers = HeaderMap::new();
    let header_name: HeaderName = auth_config
        .header_name
        .parse()
        .unwrap_or_else(|_| header::AUTHORIZATION.clone());
    
    // 根据header类型设置token格式
    if header_name == header::AUTHORIZATION {
        headers.insert(header_name, format!("Bearer {}", payload.token.trim()).parse().unwrap());
    } else {
        headers.insert(header_name, payload.token.trim().parse().unwrap());
    }

    // 使用verify_token_from_headers验证token
    match verify_token_from_headers(&headers, &app_state).await {
        Ok(token_info) => {
            Ok(Json(AuthResponse::success(
                "认证成功", 
                Some(token_info.role.as_str().to_string())
            )))
        }
        Err(err) => {
            Ok(Json(AuthResponse::error(&format!("认证失败: {}", err))))
        }
    }
}
