//! Testes de integração HTTP: exercitam o router completo sem abrir porta real.

use std::time::Duration;

use axum::{
    body::Body,
    http::{header, Request, Response, StatusCode},
    routing::get,
    Router,
};
use ferrovia_core::ServerConfig;
use ferrovia_server::{app, middleware};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn default_app() -> Router {
    app(&ServerConfig::default()).expect("configuração padrão é válida")
}

async fn send(router: Router, request: Request<Body>) -> Response<Body> {
    router.oneshot(request).await.expect("router é infalível")
}

async fn json_body(response: Response<Body>) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body legível")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("body deve ser JSON")
}

fn get_request(uri: &str) -> Request<Body> {
    Request::get(uri).body(Body::empty()).unwrap()
}

fn post_json(uri: &str, body: &str) -> Request<Body> {
    Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

fn assert_json_content_type(response: &Response<Body>) {
    let content_type = response.headers()[header::CONTENT_TYPE].to_str().unwrap();
    assert!(
        content_type.starts_with("application/json"),
        "{content_type}"
    );
}

async fn assert_error(response: Response<Body>, status: StatusCode, code: &str) {
    assert_eq!(response.status(), status);
    assert_json_content_type(&response);
    let body = json_body(response).await;
    assert_eq!(body["error"]["code"], code, "{body}");
    assert!(body["error"]["message"].is_string(), "{body}");
}

#[tokio::test]
async fn health_returns_ok() {
    let response = send(default_app(), get_request("/health")).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_json_content_type(&response);
    assert_eq!(
        json_body(response).await,
        json!({ "status": "ok", "service": "ferrovia" })
    );
}

#[tokio::test]
async fn hello_reports_environment() {
    let config = ServerConfig {
        environment: "test".to_string(),
        ..ServerConfig::default()
    };
    let response = send(app(&config).unwrap(), get_request("/api/hello")).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        json_body(response).await,
        json!({ "message": "Olá da Ferrovia!", "environment": "test" })
    );
}

#[tokio::test]
async fn generates_request_id_when_absent() {
    let response = send(default_app(), get_request("/health")).await;
    let request_id = response.headers()["x-request-id"].to_str().unwrap();
    assert_eq!(request_id.len(), 36, "esperado UUID, veio {request_id}");
}

#[tokio::test]
async fn preserves_client_request_id() {
    let request = Request::get("/health")
        .header("x-request-id", "abc-123")
        .body(Body::empty())
        .unwrap();
    let response = send(default_app(), request).await;
    assert_eq!(response.headers()["x-request-id"], "abc-123");
}

#[tokio::test]
async fn unknown_route_returns_json_404() {
    let response = send(default_app(), get_request("/nao-existe")).await;
    assert_error(response, StatusCode::NOT_FOUND, "not_found").await;
}

#[tokio::test]
async fn wrong_method_returns_json_405() {
    let request = Request::delete("/health").body(Body::empty()).unwrap();
    let response = send(default_app(), request).await;
    assert_error(
        response,
        StatusCode::METHOD_NOT_ALLOWED,
        "method_not_allowed",
    )
    .await;
}

#[tokio::test]
async fn greeting_accepts_valid_name() {
    let response = send(
        default_app(),
        post_json("/api/greetings", r#"{"name":"  Lucas  "}"#),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        json_body(response).await,
        json!({ "message": "Olá, Lucas!" })
    );
}

#[tokio::test]
async fn greeting_rejects_invalid_names() {
    let too_long = "a".repeat(65);
    for name in ["", "   ", too_long.as_str(), "linha\nquebrada"] {
        let body = json!({ "name": name }).to_string();
        let response = send(default_app(), post_json("/api/greetings", &body)).await;
        assert_error(
            response,
            StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
        )
        .await;
    }
}

#[tokio::test]
async fn greeting_rejects_malformed_json() {
    let response = send(default_app(), post_json("/api/greetings", "{\"name\":")).await;
    assert_error(response, StatusCode::BAD_REQUEST, "invalid_body").await;
}

#[tokio::test]
async fn greeting_rejects_unknown_fields() {
    let response = send(
        default_app(),
        post_json("/api/greetings", r#"{"name":"a","admin":true}"#),
    )
    .await;
    assert_error(response, StatusCode::UNPROCESSABLE_ENTITY, "invalid_body").await;
}

#[tokio::test]
async fn greeting_requires_json_content_type() {
    let request = Request::post("/api/greetings")
        .body(Body::from(r#"{"name":"a"}"#))
        .unwrap();
    let response = send(default_app(), request).await;
    assert_error(
        response,
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "unsupported_media_type",
    )
    .await;
}

#[tokio::test]
async fn body_over_limit_returns_413() {
    let config = ServerConfig {
        body_limit_bytes: 16,
        ..ServerConfig::default()
    };
    let body = json!({ "name": "nome comprido demais para o limite" }).to_string();
    let response = send(app(&config).unwrap(), post_json("/api/greetings", &body)).await;
    assert_error(response, StatusCode::PAYLOAD_TOO_LARGE, "payload_too_large").await;
}

#[tokio::test]
async fn slow_handler_times_out_with_json_408() {
    let config = ServerConfig {
        request_timeout: Duration::from_millis(50),
        ..ServerConfig::default()
    };
    let slow = Router::new().route(
        "/slow",
        get(|| async {
            tokio::time::sleep(Duration::from_secs(5)).await;
            "tarde demais"
        }),
    );
    let router = middleware::apply(slow, &config).unwrap();
    let response = send(router, get_request("/slow")).await;
    assert_error(response, StatusCode::REQUEST_TIMEOUT, "request_timeout").await;
}

#[tokio::test]
async fn cors_is_disabled_by_default() {
    let request = Request::get("/health")
        .header(header::ORIGIN, "https://qualquer.exemplo.com")
        .body(Body::empty())
        .unwrap();
    let response = send(default_app(), request).await;
    assert!(response
        .headers()
        .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
        .is_none());
}

#[tokio::test]
async fn cors_exposes_request_id_to_browser_clients() {
    let config = ServerConfig {
        cors_allowed_origins: vec!["https://app.exemplo.com".to_string()],
        ..ServerConfig::default()
    };
    let request = Request::get("/api/v1/hello")
        .header(header::ORIGIN, "https://app.exemplo.com")
        .body(Body::empty())
        .unwrap();

    let response = send(app(&config).unwrap(), request).await;
    assert_eq!(
        response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "https://app.exemplo.com"
    );
    let exposed = response.headers()[header::ACCESS_CONTROL_EXPOSE_HEADERS]
        .to_str()
        .unwrap()
        .to_ascii_lowercase();
    assert!(exposed.split(',').any(|value| value.trim() == "x-request-id"));
}

#[tokio::test]
async fn cors_allows_only_configured_origins() {
    let config = ServerConfig {
        cors_allowed_origins: vec!["https://app.exemplo.com".to_string()],
        ..ServerConfig::default()
    };
    let router = app(&config).unwrap();

    let allowed = Request::get("/health")
        .header(header::ORIGIN, "https://app.exemplo.com")
        .body(Body::empty())
        .unwrap();
    let response = send(router.clone(), allowed).await;
    assert_eq!(
        response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "https://app.exemplo.com"
    );

    let other = Request::get("/health")
        .header(header::ORIGIN, "https://malicioso.exemplo.com")
        .body(Body::empty())
        .unwrap();
    let response = send(router, other).await;
    assert!(response
        .headers()
        .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
        .is_none());
}
