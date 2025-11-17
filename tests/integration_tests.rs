use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use rifs::app_state::AppState;
use rifs::routes::create_routes;
use rifs::utils::AppError;

/// 创建测试应用状态
async fn create_test_app() -> axum::Router {
    // 使用基于文件的SQLite数据库，存储在临时目录中
    // 这样每个测试运行都会创建一个新的数据库文件
    let db_path = format!("sqlite:test_db_{}.sqlite", std::process::id());
    std::env::set_var("RIFS_DATABASE_CONNECTION_STRING", &db_path);
    std::env::set_var("RIFS_SERVER_HOST", "127.0.0.1");
    std::env::set_var("RIFS_SERVER_PORT", "3000");
    
    // 初始化配置（如果配置已初始化，忽略错误）
    if let Err(err) = rifs::config::AppConfig::init(None) {
        // 只有在错误不是"配置已被初始化"时才panic
        if !matches!(err, AppError::Internal(ref msg) if msg == "配置已被初始化") {
            panic!("Failed to initialize config: {}", err);
        }
    }
    
    let app_state = AppState::new().await.expect("Failed to create app state");
    create_routes(app_state.clone(), app_state.config())
}

#[tokio::test]
async fn test_index_page() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证页面内容
    assert!(body_str.contains("RIFS"));
    assert!(body_str.contains("高性能 Rust 图床服务"));
    assert!(body_str.contains("图片上传"));
    assert!(body_str.contains("API 接口"));
}

#[tokio::test]
async fn test_gallery_page() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/gallery")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证页面内容
    assert!(body_str.contains("图片库"));
    assert!(body_str.contains("统计信息"));
    assert!(body_str.contains("图片列表"));
}

#[tokio::test]
async fn test_static_css_file() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/static/style.css")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "text/css; charset=utf-8"
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证 CSS 内容
    assert!(body_str.contains(".container"));
    assert!(body_str.contains(".header"));
    assert!(body_str.contains(".card"));
}

#[tokio::test]
async fn test_static_js_file() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/static/app.js")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "application/javascript; charset=utf-8"
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证 JavaScript 内容
    assert!(body_str.contains("document.addEventListener"));
    assert!(body_str.contains("uploadForm"));
    assert!(body_str.contains("fetch"));
}

#[tokio::test]
async fn test_static_gallery_js_file() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/static/gallery.js")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let headers = response.headers();
    assert_eq!(
        headers.get("content-type").unwrap(),
        "application/javascript; charset=utf-8"
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证 JavaScript 内容
    assert!(body_str.contains("loadImages"));
    assert!(body_str.contains("displayImages"));
    assert!(body_str.contains("formatSize"));
}

#[tokio::test]
async fn test_static_file_not_found() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/static/nonexistent.css")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_static_file_security() {
    let app = create_test_app().await;

    // 尝试访问 web 目录外的文件
    let request = Request::builder()
        .uri("/static/../Cargo.toml")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // 路径遍历攻击应该被拒绝 (Forbidden 或 Not Found)
    assert!(
        response.status() == StatusCode::FORBIDDEN || response.status() == StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证健康检查响应
    assert!(body_str.contains("status"));
    assert!(body_str.contains("timestamp"));
}
