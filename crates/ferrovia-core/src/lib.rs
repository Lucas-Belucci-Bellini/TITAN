//! Peças independentes do transporte HTTP para aplicações Ferrovia.

pub mod config;
pub mod events;
pub mod plugins;

pub use config::{ConfigError, ServerConfig};
pub use events::{AppEvent, EventBus};
pub use plugins::{Plugin, PluginError, PluginRegistry};
