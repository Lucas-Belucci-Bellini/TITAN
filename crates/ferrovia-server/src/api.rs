use axum::{
    extract::{FromRequest, State},
    routing::{get, post},
    Json, Router,
};
use ferrovia_core::AppEvent;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{ApiError, AppState};

/// Tamanho máximo de `name` em `POST /api/greetings`, em caracteres.
pub const MAX_NAME_CHARS: usize = 64;

/// Extrator JSON que devolve erros no formato padrão em vez do texto puro do Axum.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct ApiJson<T>(pub T);

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/hello", get(hello))
        .route("/api/v1/hello", get(hello_v1))
        .route("/api/greetings", post(create_greeting))
        .fallback(|| async { ApiError::not_found() })
        .method_not_allowed_fallback(|| async { ApiError::method_not_allowed() })
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "ferrovia" }))
}

async fn hello(State(state): State<AppState>) -> Json<Value> {
    // O bus é demonstrativo. Sem assinantes, a publicação pode falhar; isso não impede a resposta HTTP.
    let _ = state.events.publish(AppEvent {
        topic: "http.hello.requested".to_string(),
        payload: json!({ "environment": state.environment }),
    });

    Json(json!({
        "message": "Olá da Ferrovia!",
        "environment": state.environment,
    }))
}

/// Endpoint de referência para o contrato HTTP/JSON v1 da família AETHER/ATLAS/TITAN.
async fn hello_v1(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "data": {
            "message": "Olá do TITAN!",
            "environment": state.environment,
        },
        "meta": {
            "api_version": "v1",
        }
    }))
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GreetingRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct GreetingResponse {
    pub message: String,
}

impl GreetingRequest {
    /// Devolve o nome normalizado ou um erro de validação legível.
    fn validated_name(&self) -> Result<&str, ApiError> {
        let name = self.name.trim();
        if name.is_empty() {
            return Err(ApiError::validation("name não pode ser vazio"));
        }
        if name.chars().count() > MAX_NAME_CHARS {
            return Err(ApiError::validation(format!(
                "name deve ter no máximo {MAX_NAME_CHARS} caracteres"
            )));
        }
        if name.chars().any(char::is_control) {
            return Err(ApiError::validation(
                "name não pode conter caracteres de controle",
            ));
        }
        Ok(name)
    }
}

async fn create_greeting(
    ApiJson(request): ApiJson<GreetingRequest>,
) -> Result<Json<GreetingResponse>, ApiError> {
    let name = request.validated_name()?;
    Ok(Json(GreetingResponse {
        message: format!("Olá, {name}!"),
    }))
}
