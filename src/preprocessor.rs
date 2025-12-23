//! MDBook preprocessor implementation.

use crate::{config::Mode, schema, Config, Error};
use jsonschema::JSONSchema;
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use serde_json::Value;

/// Frontmatter preprocessor for MDBook.
pub struct FrontmatterPreprocessor;

impl FrontmatterPreprocessor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FrontmatterPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for FrontmatterPreprocessor {
    fn name(&self) -> &str {
        "frontmatter"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        // Get preprocessor config
        let config = ctx
            .config
            .get_preprocessor(self.name())
            .ok_or_else(|| mdbook::errors::Error::msg("Missing preprocessor configuration"))?;

        let config = Config::from_table(config).map_err(mdbook::errors::Error::msg)?;

        // Load and compile schema
        let schema_value =
            schema::load_schema(&config.schema).map_err(mdbook::errors::Error::msg)?;
        let compiled_schema =
            schema::compile_schema(&schema_value).map_err(mdbook::errors::Error::msg)?;

        // Process each chapter
        let mut errors: Vec<String> = Vec::new();

        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut chapter) = item {
                if let Err(e) = process_chapter(chapter, &compiled_schema, &config) {
                    errors.push(e.to_string());
                }
            }
        });

        if !errors.is_empty() && config.fail_on_error {
            return Err(mdbook::errors::Error::msg(format!(
                "Frontmatter validation errors:\n{}",
                errors.join("\n")
            )));
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        // Support all renderers by default, or check config
        renderer != "not-supported"
    }
}

/// Process a single chapter's frontmatter.
fn process_chapter(
    chapter: &mut Chapter,
    schema: &JSONSchema,
    config: &Config,
) -> Result<(), Error> {
    let content = &chapter.content;

    // Check if content starts with frontmatter
    if !content.starts_with("---") {
        return Ok(());
    }

    // Find the end of frontmatter
    let end_marker = content[3..].find("---");
    let end_pos = match end_marker {
        Some(pos) => pos + 3,
        None => return Ok(()), // No closing marker, skip
    };

    let frontmatter_str = &content[3..end_pos].trim();
    let rest_of_content = &content[end_pos + 3..];

    // Parse frontmatter as YAML
    let frontmatter: Value =
        serde_yaml::from_str(frontmatter_str).map_err(|e| Error::FrontmatterParse {
            chapter: chapter.name.clone(),
            source: e,
        })?;

    // Validate against schema
    let validation_errors = schema::validate(schema, &frontmatter);

    if !validation_errors.is_empty() {
        match config.mode {
            Mode::Validate => {
                return Err(Error::ValidationFailed {
                    chapter: chapter.name.clone(),
                    errors: validation_errors.join("\n"),
                });
            }
            Mode::Fix => {
                // Attempt to fix frontmatter
                let fixed = fix_frontmatter(&frontmatter, schema)?;
                let fixed_yaml = serde_yaml::to_string(&fixed)
                    .map_err(|e| Error::FrontmatterSerialize { source: e })?;

                chapter.content = format!("---\n{}---{}", fixed_yaml, rest_of_content);
            }
        }
    }

    Ok(())
}

/// Attempt to fix frontmatter to match schema.
///
/// Currently only handles adding missing required fields with default values.
fn fix_frontmatter(frontmatter: &Value, _schema: &JSONSchema) -> Result<Value, Error> {
    // For now, just return the frontmatter as-is
    // TODO: Implement actual fixing logic based on schema
    // This would involve:
    // 1. Adding missing required fields with default values
    // 2. Coercing types where possible
    // 3. Removing extraneous fields if configured
    Ok(frontmatter.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessor_name() {
        let preprocessor = FrontmatterPreprocessor::new();
        assert_eq!(preprocessor.name(), "frontmatter");
    }

    #[test]
    fn test_supports_renderer() {
        let preprocessor = FrontmatterPreprocessor::new();
        assert!(preprocessor.supports_renderer("html"));
        assert!(preprocessor.supports_renderer("epub"));
    }
}
