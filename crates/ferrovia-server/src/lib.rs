//! Ferrovia HTTP server: routes, middleware and shared state.
//! Ferrovia-prefixed aliases are recommended for new integrations; short
//! existing names remain available for compatibility.

pub mod api;
pub mod contract;
pub mod error;
pub mod middleware;

pub use contract::{ApiEnvelope, ApiErrorBody, ApiErrorEnvelope, ApiMeta};
pub use error::ApiError;

use axum::Router;
use ferrovia_core::{ConfigError, EventBus, ServerConfig};

/// State injected into handlers.
#[derive(Clone)]
pub struct AppState {
    pub environment: String,
    pub events: EventBus,
}

/// Framework-branded alias for AppState.
pub type FerroviaAppState = AppState;
/// Framework-branded alias for ApiEnvelope.
pub type FerroviaApiEnvelope<T> = ApiEnvelope<T>;
/// Framework-branded alias for ApiErrorEnvelope.
pub type FerroviaApiErrorEnvelope = ApiErrorEnvelope;
/// Framework-branded alias for ApiMeta.
pub type FerroviaApiMeta = ApiMeta;
/// Framework-branded alias for ApiError.
pub type FerroviaApiError = ApiError;

impl AppState {
    pub fn new(config: &ServerConfig) -> Self {
        Self {
            environment: config.environment.clone(),
            events: EventBus::new(64),
        }
    }
}

/// Builds the full application router and applies the default middleware stack.
pub fn app(config: &ServerConfig) -> Result<Router, ConfigError> {
    middleware::apply(api::router(AppState::new(config)), config)
}
