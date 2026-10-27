//! Servidor HTTP da Ferrovia: monta rotas, middlewares e o estado compartilhado.
//!
//! `app` devolve um `Router` pronto, o que permite testar as rotas sem abrir uma porta real.

pub mod api;
pub mod error;
pub mod middleware;

pub use error::ApiError;

use axum::Router;
use ferrovia_core::{ConfigError, EventBus, ServerConfig};

/// Estado injetado nos handlers. Não guarde segredos aqui sem necessidade.
#[derive(Clone)]
pub struct AppState {
    pub environment: String,
    pub events: EventBus,
}

impl AppState {
    pub fn new(config: &ServerConfig) -> Self {
        Self {
            environment: config.environment.clone(),
            events: EventBus::new(64),
        }
    }
}

/// Monta a aplicação completa: rotas de exemplo mais a pilha de middleware padrão.
pub fn app(config: &ServerConfig) -> Result<Router, ConfigError> {
    middleware::apply(api::router(AppState::new(config)), config)
}
