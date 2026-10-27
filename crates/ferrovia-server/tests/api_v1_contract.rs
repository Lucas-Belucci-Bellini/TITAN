//! Contract smoke test for the versioned family API.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use ferrovia_core::ServerConfig;
use ferrovia_server::app;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

#[tokio::test]
async fn versioned_hello_matches_http_v1_envelope() {
    let router = app(&ServerConfig {
        environment: "test".to_string(),
        ..ServerConfig::default()
    })
    .expect("default test config is valid");

    let response: Response<Body> = router
        .oneshot(
            Request::get("/api/v1/hello")
                .body(Body::empty())
                .expect("request is valid"),
        )
        .await
        .expect("router is infallible");

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers()["content-type"]
        .to_str()
        .expect("content type is text")
        .starts_with("application/json"));
    assert!(response.headers().contains_key("x-request-id"));

    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("response body is readable")
        .to_bytes();
    let body: Value = serde_json::from_slice(&bytes).expect("response is JSON");

    assert_eq!(
        body,
        json!({
            "data": {
                "message": "Olá do TITAN!",
                "environment": "test"
            },
            "meta": {
                "api_version": "v1"
            }
        })
    );
}
