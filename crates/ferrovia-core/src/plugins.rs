use std::{collections::BTreeSet, error::Error, fmt};

/// Extensão nomeada. O sistema inicial registra plugins; carregamento dinâmico não está incluído.
pub trait Plugin {
    fn name(&self) -> &'static str;

    fn register(&self, _registry: &mut PluginRegistry) -> Result<(), PluginError> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginError(pub String);

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for PluginError {}

#[derive(Debug, Default, Clone)]
pub struct PluginRegistry {
    installed: Vec<String>,
    capabilities: BTreeSet<String>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn install<P: Plugin>(&mut self, plugin: P) -> Result<(), PluginError> {
        let name = plugin.name();
        if self.installed.iter().any(|registered| registered == name) {
            return Err(PluginError(format!("plugin duplicado: {name}")));
        }
        plugin.register(self)?;
        self.installed.push(name.to_string());
        Ok(())
    }

    pub fn add_capability(&mut self, capability: impl Into<String>) {
        self.capabilities.insert(capability.into());
    }

    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn installed_plugins(&self) -> &[String] {
        &self.installed
    }
}

#[cfg(test)]
mod tests {
    use super::{Plugin, PluginError, PluginRegistry};

    struct JsonPlugin;
    impl Plugin for JsonPlugin {
        fn name(&self) -> &'static str {
            "json"
        }
        fn register(&self, registry: &mut PluginRegistry) -> Result<(), PluginError> {
            registry.add_capability("content-type:application/json");
            Ok(())
        }
    }

    #[test]
    fn registers_capabilities_and_rejects_duplicate_names() {
        let mut registry = PluginRegistry::new();
        registry
            .install(JsonPlugin)
            .expect("first install succeeds");
        assert!(registry.has_capability("content-type:application/json"));
        assert!(registry.install(JsonPlugin).is_err());
        assert_eq!(registry.installed_plugins(), &["json"]);
    }
}
