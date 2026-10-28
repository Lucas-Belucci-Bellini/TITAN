//! Titan HTTP server: routes, middleware and shared state.
//! Titan-prefixed names are the official API; Ferrovia-prefixed aliases remain
//! available for compatibility with earlier releases.

pub mod api;
pub mod contract;
pub mod error;
pub mod middleware;

pub use contract::{ApiEnvelope, ApiErrorBody, ApiErrorEnvelope, ApiMeta};
pub use error::ApiError;

use axum::Router;
use ferrovia_core::{TitanConfigError, TitanEventBus, TitanServerConfig};

/// State injected into handlers.
#[derive(Clone)]
pub struct AppState {
    pub environment: String,
    pub events: TitanEventBus,
}

/// Official Titan name for AppState.
pub type TitanAppState = AppState;
/// Official Titan name for ApiEnvelope.
pub type TitanApiEnvelope<T> = ApiEnvelope<T>;
/// Official Titan name for ApiErrorBody.
pub type TitanApiErrorBody = ApiErrorBody;
/// Official Titan name for ApiErrorEnvelope.
pub type TitanApiErrorEnvelope = ApiErrorEnvelope;
/// Official Titan name for ApiMeta.
pub type TitanApiMeta = ApiMeta;
/// Official Titan name for ApiError.
pub type TitanApiError = ApiError;

/// Compatibility aliases for the previous public branding.
pub type FerroviaAppState = TitanAppState;
pub type FerroviaApiEnvelope<T> = TitanApiEnvelope<T>;
pub type FerroviaApiErrorBody = TitanApiErrorBody;
pub type FerroviaApiErrorEnvelope = TitanApiErrorEnvelope;
pub type FerroviaApiMeta = TitanApiMeta;
pub type FerroviaApiError = TitanApiError;

impl AppState {
    pub fn new(config: &TitanServerConfig) -> Self {
        Self {
            environment: config.environment.clone(),
            events: TitanEventBus::new(64),
        }
    }
}

/// Builds the full application router and applies the default middleware stack.
pub fn app(config: &TitanServerConfig) -> Result<Router, TitanConfigError> {
    middleware::apply(api::router(TitanAppState::new(config)), config)
}
