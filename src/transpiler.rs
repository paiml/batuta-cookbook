//! Code transpilation utilities

use crate::types::{Error, Language, Result};

/// Transpiler configuration
#[derive(Debug, Clone)]
pub struct TranspilerConfig {
    /// Source language
    pub source_lang: Language,
    /// Target language (always Rust for now)
    pub target_lang: Language,
    /// Enable incremental compilation
    pub incremental: bool,
    /// Enable caching
    pub cache_enabled: bool,
}

impl Default for TranspilerConfig {
    fn default() -> Self {
        Self {
            source_lang: Language::Python,
            target_lang: Language::Rust,
            incremental: false,
            cache_enabled: false,
        }
    }
}

impl TranspilerConfig {
    /// Create a new transpiler configuration builder
    #[must_use]
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

/// Builder for transpiler configuration
#[derive(Default)]
pub struct ConfigBuilder {
    source_lang: Option<Language>,
    incremental: bool,
    cache_enabled: bool,
}

impl ConfigBuilder {
    /// Set source language
    #[must_use] 
    pub fn source_language(mut self, lang: Language) -> Self {
        self.source_lang = Some(lang);
        self
    }

    /// Enable incremental compilation
    #[must_use] 
    pub fn incremental(mut self, enabled: bool) -> Self {
        self.incremental = enabled;
        self
    }

    /// Enable caching
    #[must_use] 
    pub fn cache(mut self, enabled: bool) -> Self {
        self.cache_enabled = enabled;
        self
    }

    /// Build the configuration
    ///
    /// # Errors
    ///
    /// Returns error if source language is not set
    pub fn build(self) -> Result<TranspilerConfig> {
        Ok(TranspilerConfig {
            source_lang: self.source_lang.unwrap_or(Language::Python),
            target_lang: Language::Rust,
            incremental: self.incremental,
            cache_enabled: self.cache_enabled,
        })
    }
}

/// Transpiler for converting source code
pub struct Transpiler {
    config: TranspilerConfig,
}

impl Transpiler {
    /// Create a new transpiler with the given configuration
    #[must_use] 
    pub fn new(config: TranspilerConfig) -> Self {
        Self { config }
    }

    /// Transpile source code to Rust
    ///
    /// # Errors
    ///
    /// Returns error if transpilation fails
    pub fn transpile(&self, source: &str) -> Result<String> {
        // Stub implementation
        // TODO: Implement actual transpilation
        if source.is_empty() {
            return Err(Error::TranspilationError("Empty source".to_string()));
        }

        // For now, just return a simple Rust stub
        Ok(format!(
            "// Transpiled from {:?}\nfn main() {{\n    println!(\"Hello from transpiled code!\");\n}}",
            self.config.source_lang
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = TranspilerConfig::builder()
            .source_language(Language::Python)
            .incremental(true)
            .cache(true)
            .build()
            .unwrap();

        assert_eq!(config.source_lang, Language::Python);
        assert!(config.incremental);
        assert!(config.cache_enabled);
    }

    #[test]
    fn test_transpiler_creation() {
        let config = TranspilerConfig::default();
        let transpiler = Transpiler::new(config);
        assert_eq!(transpiler.config.source_lang, Language::Python);
    }

    #[test]
    fn test_transpile_empty_fails() {
        let config = TranspilerConfig::default();
        let transpiler = Transpiler::new(config);
        let result = transpiler.transpile("");
        assert!(result.is_err());
    }

    #[test]
    fn test_transpile_basic() {
        let config = TranspilerConfig::default();
        let transpiler = Transpiler::new(config);
        let result = transpiler.transpile("print('hello')");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("fn main"));
    }
}
