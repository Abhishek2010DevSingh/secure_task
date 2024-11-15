use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use secure_task::router;
use tower::ServiceExt;

#[tokio::test]
async fn health() {
    let router = router::router();

    let response = router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn not_found() {
    let router = router::router();

    let response = router
        .oneshot(
            Request::builder()
                .uri("/error")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
