//! Error types for the frontmatter preprocessor.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to fetch schema from {url}: {source}")]
    SchemaFetch {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to read schema from file {path}: {source}")]
    SchemaRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid schema URL: {0}")]
    InvalidSchemaUrl(String),

    #[error("Failed to parse schema as JSON: {source}")]
    SchemaParseJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("Invalid JSON schema: {0}")]
    InvalidSchema(String),

    #[error("Failed to parse frontmatter as YAML in {chapter}: {source}")]
    FrontmatterParse {
        chapter: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("Frontmatter validation failed in {chapter}:\n{errors}")]
    ValidationFailed { chapter: String, errors: String },

    #[error("Failed to serialize fixed frontmatter: {source}")]
    FrontmatterSerialize {
        #[source]
        source: serde_yaml::Error,
    },

    #[error("MDBook error: {0}")]
    MdBook(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl From<mdbook::errors::Error> for Error {
    fn from(err: mdbook::errors::Error) -> Self {
        Error::MdBook(err.to_string())
    }
}
