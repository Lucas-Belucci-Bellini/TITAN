//! Pilha de middleware padrão: request ID, tracing, timeout, limite de body e CORS opcional.

use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    http::{header, HeaderName, HeaderValue, Method, Request},
    BoxError, Router,
};
use ferrovia_core::{ConfigError, ServerConfig};
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::ApiError;

/// Header usado para correlacionar logs e respostas. Um valor enviado pelo cliente é preservado.
pub const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

/// Aplica a pilha padrão a um router. Use com rotas próprias para herdar os mesmos limites.
pub fn apply(router: Router, config: &ServerConfig) -> Result<Router, ConfigError> {
    let router = router.layer(DefaultBodyLimit::max(config.body_limit_bytes));
    let router = match cors_layer(&config.cors_allowed_origins)? {
        Some(cors) => router.layer(cors),
        None => router,
    };

    // Só o path entra no span: query strings podem carregar tokens.
    let trace = TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let request_id = request
            .headers()
            .get(&REQUEST_ID_HEADER)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("-");
        tracing::info_span!(
            "http",
            method = %request.method(),
            path = %request.uri().path(),
            request_id,
        )
    });

    Ok(router.layer(
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(REQUEST_ID_HEADER, MakeRequestUuid))
            .layer(trace)
            .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER))
            .layer(HandleErrorLayer::new(handle_middleware_error))
            .timeout(config.request_timeout),
    ))
}

async fn handle_middleware_error(error: BoxError) -> ApiError {
    if error.is::<tower::timeout::error::Elapsed>() {
        ApiError::timeout()
    } else {
        tracing::error!(%error, "falha no middleware");
        ApiError::internal()
    }
}

/// CORS só é ligado quando há origens explícitas. Sem configuração, navegadores bloqueiam chamadas de outras origens.
fn cors_layer(origins: &[String]) -> Result<Option<CorsLayer>, ConfigError> {
    if origins.is_empty() {
        return Ok(None);
    }
    let origins = origins
        .iter()
        .map(|origin| {
            HeaderValue::from_str(origin)
                .map_err(|_| ConfigError::InvalidCorsOrigin(origin.clone()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Some(
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE, REQUEST_ID_HEADER])
            .expose_headers([REQUEST_ID_HEADER]),
    ))
}
