use std::{env, error::Error, fmt, time::Duration};

/// Configuração de rede, ambiente e limites HTTP da aplicação.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
    /// Tempo máximo para responder a uma requisição.
    pub request_timeout: Duration,
    /// Tamanho máximo aceito para o corpo de uma requisição, em bytes.
    pub body_limit_bytes: usize,
    /// Origens liberadas para CORS. Vazio desativa CORS; `*` não é aceito.
    pub cors_allowed_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            environment: "development".to_string(),
            request_timeout: Duration::from_secs(30),
            body_limit_bytes: 1024 * 1024,
            cors_allowed_origins: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    InvalidPort(String),
    InvalidTimeout(String),
    InvalidBodyLimit(String),
    InvalidCorsOrigin(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort(raw) => write!(
                f,
                "FERROVIA_PORT inválida: {raw:?}; informe um número entre 0 e 65535"
            ),
            Self::InvalidTimeout(raw) => write!(
                f,
                "FERROVIA_REQUEST_TIMEOUT_SECS inválido: {raw:?}; informe um número inteiro maior que zero"
            ),
            Self::InvalidBodyLimit(raw) => write!(
                f,
                "FERROVIA_BODY_LIMIT_BYTES inválido: {raw:?}; informe um número inteiro maior que zero"
            ),
            Self::InvalidCorsOrigin(raw) => write!(
                f,
                "origem CORS inválida: {raw:?}; use origens explícitas como https://app.exemplo.com (\"*\" não é aceito)"
            ),
        }
    }
}

impl Error for ConfigError {}

impl ServerConfig {
    /// Carrega configuração de ambiente; valores ausentes usam os padrões seguros para desenvolvimento local.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| env::var(key).ok())
    }

    /// Carrega configuração a partir de uma função de consulta, o que permite testar sem alterar o ambiente do processo.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, ConfigError> {
        let defaults = Self::default();
        let read = |key: &str| lookup(key).filter(|value| !value.trim().is_empty());

        let host = read("FERROVIA_HOST").unwrap_or(defaults.host);
        let environment = read("FERROVIA_ENV").unwrap_or(defaults.environment);

        let port = match read("FERROVIA_PORT") {
            Some(raw) => raw
                .trim()
                .parse::<u16>()
                .map_err(|_| ConfigError::InvalidPort(raw))?,
            None => defaults.port,
        };

        let request_timeout = match read("FERROVIA_REQUEST_TIMEOUT_SECS") {
            Some(raw) => match raw.trim().parse::<u64>() {
                Ok(secs) if secs > 0 => Duration::from_secs(secs),
                _ => return Err(ConfigError::InvalidTimeout(raw)),
            },
            None => defaults.request_timeout,
        };

        let body_limit_bytes = match read("FERROVIA_BODY_LIMIT_BYTES") {
            Some(raw) => match raw.trim().parse::<usize>() {
                Ok(bytes) if bytes > 0 => bytes,
                _ => return Err(ConfigError::InvalidBodyLimit(raw)),
            },
            None => defaults.body_limit_bytes,
        };

        let cors_allowed_origins = match read("FERROVIA_CORS_ORIGINS") {
            Some(raw) => parse_origins(&raw)?,
            None => defaults.cors_allowed_origins,
        };

        Ok(Self {
            host,
            port,
            environment,
            request_timeout,
            body_limit_bytes,
            cors_allowed_origins,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn parse_origins(raw: &str) -> Result<Vec<String>, ConfigError> {
    raw.split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            let has_scheme = origin.starts_with("http://") || origin.starts_with("https://");
            let has_path = origin
                .split_once("://")
                .is_some_and(|(_, rest)| rest.is_empty() || rest.contains('/'));
            if !has_scheme || has_path || origin.contains('*') {
                Err(ConfigError::InvalidCorsOrigin(origin.to_string()))
            } else {
                Ok(origin.to_string())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{ConfigError, ServerConfig};
    use std::{collections::HashMap, time::Duration};

    fn load(pairs: &[(&str, &str)]) -> Result<ServerConfig, ConfigError> {
        let vars: HashMap<String, String> = pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        ServerConfig::from_lookup(|key| vars.get(key).cloned())
    }

    #[test]
    fn default_address_is_localhost() {
        let config = ServerConfig::default();
        assert_eq!(config.bind_address(), "127.0.0.1:3000");
        assert_eq!(config.environment, "development");
        assert!(config.cors_allowed_origins.is_empty());
    }

    #[test]
    fn empty_environment_uses_defaults() {
        assert_eq!(load(&[]).unwrap(), ServerConfig::default());
        assert_eq!(
            load(&[("FERROVIA_HOST", "  ")]).unwrap(),
            ServerConfig::default()
        );
    }

    #[test]
    fn reads_all_values() {
        let config = load(&[
            ("FERROVIA_HOST", "0.0.0.0"),
            ("FERROVIA_PORT", "8080"),
            ("FERROVIA_ENV", "production"),
            ("FERROVIA_REQUEST_TIMEOUT_SECS", "5"),
            ("FERROVIA_BODY_LIMIT_BYTES", "2048"),
            (
                "FERROVIA_CORS_ORIGINS",
                "https://app.exemplo.com, http://localhost:5173",
            ),
        ])
        .unwrap();

        assert_eq!(config.bind_address(), "0.0.0.0:8080");
        assert_eq!(config.environment, "production");
        assert_eq!(config.request_timeout, Duration::from_secs(5));
        assert_eq!(config.body_limit_bytes, 2048);
        assert_eq!(
            config.cors_allowed_origins,
            ["https://app.exemplo.com", "http://localhost:5173"]
        );
    }

    #[test]
    fn rejects_invalid_numbers() {
        assert!(matches!(
            load(&[("FERROVIA_PORT", "70000")]),
            Err(ConfigError::InvalidPort(_))
        ));
        assert!(matches!(
            load(&[("FERROVIA_REQUEST_TIMEOUT_SECS", "0")]),
            Err(ConfigError::InvalidTimeout(_))
        ));
        assert!(matches!(
            load(&[("FERROVIA_BODY_LIMIT_BYTES", "-1")]),
            Err(ConfigError::InvalidBodyLimit(_))
        ));
    }

    #[test]
    fn rejects_wildcard_and_malformed_origins() {
        for origin in [
            "*",
            "app.exemplo.com",
            "https://*.exemplo.com",
            "https://a.com/path",
        ] {
            assert!(
                matches!(
                    load(&[("FERROVIA_CORS_ORIGINS", origin)]),
                    Err(ConfigError::InvalidCorsOrigin(_))
                ),
                "{origin} deveria ser rejeitada"
            );
        }
    }
}
