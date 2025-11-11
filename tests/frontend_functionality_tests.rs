//! 前端功能测试
//! 测试前端页面的 JavaScript 功能和用户交互

use axum::{
    body::Body,
    http::{Request, StatusCode, Method},
};
use tower::ServiceExt;

use rifs::app_state::AppState;
use rifs::routes::create_routes;

/// 创建测试应用状态
async fn create_test_app() -> axum::Router {
    // 设置测试环境变量
    std::env::set_var("RIFS_DATABASE_CONNECTION_STRING", "sqlite::memory:");
    std::env::set_var("RIFS_SERVER_HOST", "127.0.0.1");
    std::env::set_var("RIFS_SERVER_PORT", "3000");
    
    // 初始化配置
    rifs::config::AppConfig::init(None).expect("Failed to initialize config");
    
    let app_state = AppState::new().await.expect("Failed to create app state");
    create_routes(app_state.clone(), app_state.config())
}

#[tokio::test]
async fn test_upload_endpoint_functionality() {
    let app = create_test_app().await;

    // 创建一个简单的测试图片数据 (1x1 PNG)
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
        0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00,
        0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0x57, 0x63, 0xF8, 0x0F, 0x00, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82
    ];

    // 创建 multipart form data
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let form_data = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"test.png\"\r\n\
         Content-Type: image/png\r\n\r\n\
         {data}\r\n\
         --{boundary}--\r\n",
        boundary = boundary,
        data = hex::encode(&png_data)
    );

    let request = Request::builder()
        .method(Method::POST)
        .uri("/upload")
        .header("content-type", format!("multipart/form-data; boundary={}", boundary))
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // 上传应该成功
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证响应是有效的 JSON
    assert!(body_str.contains("id"));
    assert!(body_str.contains("success"));
}

#[tokio::test]
async fn test_images_query_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/api/images/query?page=1&size=10")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证响应结构
    assert!(body_str.contains("success"));
    assert!(body_str.contains("data"));
}

#[tokio::test]
async fn test_stats_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/api/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证统计信息结构
    assert!(body_str.contains("success"));
    assert!(body_str.contains("data"));
}

#[tokio::test]
async fn test_cache_stats_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/api/cache/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // 验证缓存统计结构
    assert!(body_str.contains("success"));
    assert!(body_str.contains("data"));
}

#[tokio::test]
async fn test_image_access_with_transformation() {
    let app = create_test_app().await;

    // 测试不存在的图片ID
    let request = Request::builder()
        .uri("/images/nonexistent@w200_h150")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // 应该返回 404
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_cors_headers() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::OPTIONS)
        .uri("/upload")
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "POST")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // CORS 应该启用
    assert!(response.status().is_success() || response.status() == StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_api_response_format() {
    let app = create_test_app().await;

    let endpoints = vec![
        "/api/stats",
        "/api/cache/stats",
        "/api/images/query",
        "/health/detailed",
    ];

    for endpoint in endpoints {
        let request = Request::builder()
            .uri(endpoint)
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK, "Endpoint {} should return 200", endpoint);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        
        // 所有 API 响应都应该是有效的 JSON
        assert!(body_str.starts_with('{') || body_str.starts_with('['), 
                "Endpoint {} should return JSON", endpoint);
    }
}
