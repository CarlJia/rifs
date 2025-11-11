use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use rifs::handlers::{api_docs, gallery_page, serve_static};

#[tokio::test]
async fn test_api_docs_handler() {
    let response = api_docs().await.into_response();
    
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
}

#[tokio::test]
async fn test_gallery_page_handler() {
    let response = gallery_page().await.into_response();
    
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
async fn test_serve_static_not_found() {
    let response = serve_static(axum::extract::Path("nonexistent.css".to_string())).await.into_response();
    
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
