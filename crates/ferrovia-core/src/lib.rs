//! Transport-independent building blocks for Titan applications.

pub mod config;
pub mod events;
pub mod plugins;

pub use config::{ConfigError, ServerConfig};
/// Official Titan name for ServerConfig.
pub type TitanServerConfig = ServerConfig;
/// Official Titan name for configuration errors.
pub type TitanConfigError = ConfigError;
/// Compatibility alias for the previous public branding.
pub type FerroviaServerConfig = TitanServerConfig;

pub use events::{AppEvent, EventBus};
/// Official Titan name for AppEvent.
pub type TitanEvent = AppEvent;
/// Official Titan name for EventBus.
pub type TitanEventBus = EventBus;
/// Compatibility aliases for the previous public branding.
pub type FerroviaEvent = TitanEvent;
pub type FerroviaEventBus = TitanEventBus;

/// Official Titan name for the plugin trait.
pub use plugins::Plugin as TitanPlugin;
pub use plugins::{Plugin, PluginError, PluginRegistry};
/// Official Titan name for plugin registration errors.
pub type TitanPluginError = PluginError;
/// Official Titan name for PluginRegistry.
pub type TitanPluginRegistry = PluginRegistry;
/// Compatibility alias for the previous public branding.
pub type FerroviaPluginRegistry = TitanPluginRegistry;
