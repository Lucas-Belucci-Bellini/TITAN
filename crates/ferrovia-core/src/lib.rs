//! Transport-independent building blocks for Ferrovia applications.

pub mod config;
pub mod events;
pub mod plugins;

pub use config::{ConfigError, ServerConfig};
/// Framework-branded alias for ServerConfig.
pub type FerroviaServerConfig = ServerConfig;

pub use events::{AppEvent, EventBus};
/// Framework-branded alias for AppEvent.
pub type FerroviaEvent = AppEvent;
/// Framework-branded alias for EventBus.
pub type FerroviaEventBus = EventBus;

pub use plugins::{Plugin, PluginError, PluginRegistry};
/// Framework-branded alias for PluginRegistry.
pub type FerroviaPluginRegistry = PluginRegistry;
