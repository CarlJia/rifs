use serde::Serialize;

/// 通用API响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn success_with_option(message: &str, data: Option<T>) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data,
        }
    }
}

impl ApiResponse<()> {
    pub fn success_no_data(message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: None,
        }
    }
}