//! MDBook preprocessor for validating or fixing frontmatter against a JSON schema.
//!
//! # Configuration
//!
//! Add to your `book.toml`:
//!
//! ```toml
//! [preprocessor.frontmatter]
//! schema = "https://example.com/schema.json"  # or file:///path/to/schema.json
//! mode = "validate"  # or "fix"
//! ```
//!
//! # Modes
//!
//! - `validate`: Report errors for invalid frontmatter, don't modify content
//! - `fix`: Attempt to fix frontmatter to match schema (add missing required fields, etc.)

pub mod config;
pub mod error;
pub mod preprocessor;
pub mod schema;

pub use config::Config;
pub use error::Error;
pub use preprocessor::FrontmatterPreprocessor;
