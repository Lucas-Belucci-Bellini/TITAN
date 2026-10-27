//! Contract smoke tests for the versioned family API.

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

#[tokio::test]
async fn versioned_route_errors_use_the_family_v1_envelope() {
    let router = app(&ServerConfig::default()).expect("default test config is valid");
    let cases = [
        (
            Request::get("/api/v1/not-found")
                .body(Body::empty())
                .expect("request is valid"),
            StatusCode::NOT_FOUND,
            "not_found",
        ),
        (
            Request::post("/api/v1/hello")
                .body(Body::empty())
                .expect("request is valid"),
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed",
        ),
    ];

    for (request, expected_status, expected_code) in cases {
        let response: Response<Body> = router
            .clone()
            .oneshot(request)
            .await
            .expect("router is infallible");

        assert_eq!(response.status(), expected_status);
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

        assert_eq!(body["error"]["code"], expected_code, "{body}");
        assert!(body["error"]["message"].is_string(), "{body}");
        assert_eq!(body["meta"]["api_version"], "v1", "{body}");
    }
}
