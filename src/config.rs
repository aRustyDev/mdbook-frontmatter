//! Configuration for the frontmatter preprocessor.

use serde::Deserialize;
use toml::value::Table;

/// Processing mode for frontmatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Validate frontmatter and report errors, but don't modify.
    #[default]
    Validate,
    /// Attempt to fix frontmatter to match schema.
    Fix,
}

/// Configuration for the frontmatter preprocessor.
///
/// Configured in `book.toml` under `[preprocessor.frontmatter]`.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Schema URI (http://, https://, or file://).
    pub schema: String,

    /// Processing mode: "validate" or "fix".
    #[serde(default)]
    pub mode: Mode,

    /// Whether to fail the build on validation errors.
    #[serde(default = "default_fail_on_error")]
    pub fail_on_error: bool,

    /// Renderers to run this preprocessor for.
    #[serde(default)]
    pub renderers: Option<Vec<String>>,
}

fn default_fail_on_error() -> bool {
    true
}

impl Config {
    /// Parse configuration from mdbook's preprocessor config table.
    pub fn from_table(table: &Table) -> Result<Self, crate::Error> {
        let value = toml::Value::Table(table.clone());
        let config: Config = value
            .try_into()
            .map_err(|e| crate::Error::Config(format!("Invalid configuration: {}", e)))?;

        // Validate schema URL format
        if !config.schema.starts_with("http://")
            && !config.schema.starts_with("https://")
            && !config.schema.starts_with("file://")
        {
            return Err(crate::Error::InvalidSchemaUrl(format!(
                "Schema must be a URL (http://, https://, or file://): {}",
                config.schema
            )));
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_mode_default() {
        assert_eq!(Mode::default(), Mode::Validate);
    }

    #[test]
    fn test_config_from_table_valid_https() {
        let mut table = Table::new();
        table.insert(
            "schema".to_string(),
            Value::String("https://example.com/schema.json".to_string()),
        );

        let config = Config::from_table(&table).unwrap();
        assert_eq!(config.schema, "https://example.com/schema.json");
        assert_eq!(config.mode, Mode::Validate);
        assert!(config.fail_on_error);
    }

    #[test]
    fn test_config_from_table_valid_file() {
        let mut table = Table::new();
        table.insert(
            "schema".to_string(),
            Value::String("file:///path/to/schema.json".to_string()),
        );
        table.insert("mode".to_string(), Value::String("fix".to_string()));

        let config = Config::from_table(&table).unwrap();
        assert_eq!(config.schema, "file:///path/to/schema.json");
        assert_eq!(config.mode, Mode::Fix);
    }

    #[test]
    fn test_config_from_table_invalid_schema() {
        let mut table = Table::new();
        table.insert(
            "schema".to_string(),
            Value::String("/path/to/schema.json".to_string()),
        );

        let result = Config::from_table(&table);
        assert!(result.is_err());
    }
}
