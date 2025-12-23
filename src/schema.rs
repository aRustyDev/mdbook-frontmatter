//! Schema loading and validation.

use crate::Error;
use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs;

/// Load a JSON schema from a URL or file path.
pub fn load_schema(uri: &str) -> Result<Value, Error> {
    if uri.starts_with("http://") || uri.starts_with("https://") {
        load_schema_http(uri)
    } else if uri.starts_with("file://") {
        load_schema_file(uri)
    } else {
        Err(Error::InvalidSchemaUrl(uri.to_string()))
    }
}

fn load_schema_http(url: &str) -> Result<Value, Error> {
    let response = reqwest::blocking::get(url).map_err(|e| Error::SchemaFetch {
        url: url.to_string(),
        source: e,
    })?;

    let schema: Value = response.json().map_err(|e| Error::SchemaFetch {
        url: url.to_string(),
        source: e,
    })?;

    Ok(schema)
}

fn load_schema_file(uri: &str) -> Result<Value, Error> {
    let path = uri
        .strip_prefix("file://")
        .ok_or_else(|| Error::InvalidSchemaUrl(uri.to_string()))?;

    let content = fs::read_to_string(path).map_err(|e| Error::SchemaRead {
        path: path.to_string(),
        source: e,
    })?;

    let schema: Value =
        serde_json::from_str(&content).map_err(|e| Error::SchemaParseJson { source: e })?;

    Ok(schema)
}

/// Compile a JSON schema for validation.
pub fn compile_schema(schema: &Value) -> Result<JSONSchema, Error> {
    JSONSchema::compile(schema).map_err(|e| Error::InvalidSchema(e.to_string()))
}

/// Validate a YAML value against a compiled JSON schema.
pub fn validate(schema: &JSONSchema, value: &Value) -> Vec<String> {
    schema
        .validate(value)
        .err()
        .map(|errors| errors.map(|e| format!("  - {}", e)).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compile_valid_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" }
            },
            "required": ["title"]
        });

        let result = compile_schema(&schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_valid_data() {
        let schema = json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" }
            },
            "required": ["title"]
        });

        let compiled = compile_schema(&schema).unwrap();
        let data = json!({ "title": "Hello" });

        let errors = validate(&compiled, &data);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_data() {
        let schema = json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" }
            },
            "required": ["title"]
        });

        let compiled = compile_schema(&schema).unwrap();
        let data = json!({ "description": "Missing title" });

        let errors = validate(&compiled, &data);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_load_schema_invalid_url() {
        let result = load_schema("/path/without/scheme");
        assert!(result.is_err());
    }
}
