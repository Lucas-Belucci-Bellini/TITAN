//! Typed success envelope for the family HTTP/JSON contract v1.

use serde::Serialize;

/// Metadata shared by versioned successful API responses.
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
