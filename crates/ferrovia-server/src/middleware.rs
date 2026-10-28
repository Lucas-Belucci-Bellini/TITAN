//! Pilha de middleware padrão: request ID, tracing, timeout, limite de body e CORS opcional.

use std::time::Duration;

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, State},
    http::{header, HeaderName, HeaderValue, Method, Request},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    Router,
};
use ferrovia_core::{TitanConfigError, TitanServerConfig};
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
pub fn apply(router: Router, config: &TitanServerConfig) -> Result<Router, TitanConfigError> {
    let router = router.layer(DefaultBodyLimit::max(config.body_limit_bytes));

    // O timeout precisa ficar dentro de CORS, para respostas de timeout também
    // receberem os cabeçalhos CORS e poderem ser lidas pelo navegador.
    let router = router.layer(from_fn_with_state(config.request_timeout, timeout_request));

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
            .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER)),
    ))
}

/// Applies the configured timeout and preserves the versioned error contract.
async fn timeout_request(
    State(timeout): State<Duration>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path();
    let is_v1 = path == "/api/v1" || path.starts_with("/api/v1/");

    match tokio::time::timeout(timeout, next.run(request)).await {
        Ok(response) => response,
        Err(_) => {
            tracing::warn!(timeout_ms = timeout.as_millis(), "HTTP request timed out");
            let error = ApiError::timeout();
            if is_v1 {
                error.into_v1_response()
            } else {
                error.into_response()
            }
        }
    }
}

/// CORS só é ligado quando há origens explícitas. Sem configuração, navegadores bloqueiam chamadas de outras origens.
fn cors_layer(origins: &[String]) -> Result<Option<CorsLayer>, TitanConfigError> {
    if origins.is_empty() {
        return Ok(None);
    }
    let origins = origins
        .iter()
        .map(|origin| {
            HeaderValue::from_str(origin)
                .map_err(|_| TitanConfigError::InvalidCorsOrigin(origin.clone()))
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
