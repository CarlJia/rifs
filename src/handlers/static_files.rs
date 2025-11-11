use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
};
use std::path::PathBuf;
use tokio::fs;

/// 静态文件服务
pub async fn serve_static(Path(path): Path<String>) -> impl IntoResponse {
    let base_path = PathBuf::from("web");
    let file_path = base_path.join(&path);

    // 安全检查：确保文件在 web 目录内
    match file_path.canonicalize() {
        Ok(canonical_path) => {
            if !canonical_path.starts_with(&base_path.canonicalize().unwrap_or_else(|_| base_path.clone())) {
                return (StatusCode::FORBIDDEN, "Access denied").into_response();
            }
        }
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    }

    // 读取文件内容
    match fs::read(&file_path).await {
        Ok(contents) => {
            let content_type = get_content_type(&path);
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                contents,
            )
                .into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

/// 根据文件扩展名获取 MIME 类型
fn get_content_type(file_path: &str) -> &'static str {
    match std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        _ => "application/octet-stream",
    }
}

/// API文档根路径 - 重定向到主页
pub async fn api_docs() -> impl IntoResponse {
    let html_content = include_str!("../../web/index.html");
    Html(html_content)
}

/// 图片瀑布流页面
pub async fn gallery_page() -> impl IntoResponse {
    let html_content = include_str!("../../web/gallery.html");
    Html(html_content)
}

/// 缓存管理页面HTML
pub const CACHE_MANAGEMENT_HTML: &str = include_str!("../../web/cache_management.html");