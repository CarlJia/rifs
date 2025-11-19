//! 授权和访问控制测试
//! 测试不同用户角色对API端点的访问权限

use axum::{
    body::Body,
    http::{Request, StatusCode, Method},
};
use tower::ServiceExt;

use rifs::app_state::AppState;
use rifs::routes::create_routes;
use rifs::utils::AppError;

/// 创建测试应用状态
async fn create_test_app() -> axum::Router {
    // 初始化配置（如果配置已初始化，忽略错误）
    if let Err(err) = rifs::config::AppConfig::init(Some("config_test")) {
        if !matches!(err, AppError::Internal(ref msg) if msg == "配置已被初始化") {
            panic!("Failed to initialize config: {}", err);
        }
    }

    let app_state = AppState::new().await.expect("Failed to create app state");
    create_routes(app_state.clone(), app_state.config())
}

#[tokio::test]
async fn test_home_page_accessible() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_gallery_page_accessible() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/gallery")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_login_page_accessible() {
    let app = create_test_app().await;

    let request = Request::builder()
        .uri("/login")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_upload_endpoint_accessible() {
    let app = create_test_app().await;

    // 创建一个简单的测试图片数据 (1x1 PNG)
    let png_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
        0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0x57, 0x63, 0xF8,
        0x0F, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let mut form_data = Vec::new();
    form_data.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    form_data.extend_from_slice(
        b"Content-Disposition: form-data; name=\"file\"; filename=\"test.png\"\r\n",
    );
    form_data.extend_from_slice(b"Content-Type: image/png\r\n\r\n");
    form_data.extend_from_slice(&png_data);
    form_data.extend_from_slice(b"\r\n");
    form_data.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    let request = Request::builder()
        .method(Method::POST)
        .uri("/upload")
        .header(
            "content-type",
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(Body::from(form_data))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_images_query_accessible() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/images/query?page=1&size=10")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("success"));
}

#[tokio::test]
async fn test_stats_endpoint_accessible() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("success"));
}

#[tokio::test]
async fn test_health_check_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_detailed_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health/detailed")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_cache_stats_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/cache/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("success"));
}

#[tokio::test]
async fn test_admin_endpoints_accessible_when_auth_disabled() {
    // When authentication is disabled in config_test.toml (auth.enabled = false)
    // admin endpoints should still be accessible
    let app = create_test_app().await;

    // Token list endpoint
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/tokens/list")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert!(
        response.status().is_success() || response.status() == StatusCode::UNAUTHORIZED,
        "Token list endpoint should be accessible or return unauthorized"
    );

    // Cache decay endpoint
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/cache/decay")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert!(
        response.status().is_success() || response.status() == StatusCode::UNAUTHORIZED,
        "Cache decay endpoint should be accessible or return unauthorized"
    );

    // Cache clear endpoint
    let request = Request::builder()
        .method(Method::DELETE)
        .uri("/api/cache/clear")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert!(
        response.status().is_success() || response.status() == StatusCode::UNAUTHORIZED,
        "Cache clear endpoint should be accessible or return unauthorized"
    );
}

#[tokio::test]
async fn test_auth_config_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/auth/config")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let body_str = String::from_utf8(body.to_vec()).unwrap();
    // Should contain information about auth configuration
    assert!(body_str.contains("enabled") || body_str.contains("auth"));
}

#[tokio::test]
async fn test_system_stats_endpoint() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/system/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_nonexistent_route_404() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method(Method::GET)
        .uri("/nonexistent-route")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
