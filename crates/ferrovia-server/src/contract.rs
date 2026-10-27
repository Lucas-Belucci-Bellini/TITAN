//! Typed success and error envelopes for the family HTTP contract v1.

use serde::Serialize;

/// Metadata shared by versioned HTTP responses.
#[derive(Debug, Clone, Serialize)]
pub struct ApiMeta {
    /// Literal contract version, for example "v1".
    pub api_version: &'static str,
}

/// Successful response envelope for versioned endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct ApiEnvelope<T> {
    /// Route-specific response payload.
    pub data: T,
    /// Contract metadata.
    pub meta: ApiMeta,
}

impl<T> ApiEnvelope<T> {
    /// Wraps a payload using the current family contract version.
    pub fn v1(data: T) -> Self {
        Self {
            data,
            meta: ApiMeta { api_version: "v1" },
        }
    }
}

/// Minimal machine-readable and human-readable fields in a versioned API error.
#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorBody {
    /// Stable code intended for programmatic handling.
    pub code: String,
    /// Safe display text; never include internal diagnostics.
    pub message: String,
}

/// Error envelope used by errors explicitly handled inside a v1 route namespace.
#[derive(Debug, Clone, Serialize)]
pub struct ApiErrorEnvelope {
    /// Structured error fields.
    pub error: ApiErrorBody,
    /// Contract metadata.
    pub meta: ApiMeta,
}

impl ApiErrorEnvelope {
    /// Wraps an error using the current family contract version.
    pub fn v1(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: ApiErrorBody {
                code: code.into(),
                message: message.into(),
            },
            meta: ApiMeta { api_version: "v1" },
        }
    }
}
