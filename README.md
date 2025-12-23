# mdbook-frontmatter

MDBook preprocessor to validate or fix frontmatter against a JSON schema.

## Features

- **Schema Validation**: Validate YAML frontmatter against a JSON schema
- **Auto-Fix Mode**: Optionally fix frontmatter to match schema requirements
- **Flexible Schema Sources**: Load schemas from HTTP(S) URLs or local files
- **Fail-Fast or Continue**: Configure whether validation errors should fail the build

## Installation

```bash
cargo install mdbook-frontmatter
```

Or build from source:

```bash
git clone https://github.com/aRustyDev/mdbook-frontmatter.git
cd mdbook-frontmatter
cargo install --path .
```

## Configuration

Add the preprocessor to your `book.toml`:

```toml
[preprocessor.frontmatter]
# Required: Schema URL (http://, https://, or file://)
schema = "https://schemas.arusty.dev/draft/2020-12/frontmatter.schema.json"

# Optional: Processing mode (default: "validate")
# - "validate": Report errors but don't modify content
# - "fix": Attempt to fix frontmatter to match schema
mode = "validate"

# Optional: Whether to fail the build on validation errors (default: true)
fail_on_error = true
```

### Schema Sources

The schema can be loaded from:

- **HTTPS**: `https://example.com/schema.json`
- **HTTP**: `http://example.com/schema.json`
- **Local file**: `file:///path/to/schema.json`

## Usage

### Frontmatter Format

Your markdown files should have YAML frontmatter at the top:

```markdown
---
title: My Chapter
description: A description of the chapter
tags: [rust, mdbook]
---

# Chapter Content

...
```

### Validation Mode

In validation mode, the preprocessor checks each chapter's frontmatter against the schema and reports errors:

```
Error: Frontmatter validation failed in "Chapter 1":
  - "title" is a required property
  - "tags" should be an array
```

### Fix Mode

In fix mode, the preprocessor attempts to:
- Add missing required fields with default values
- Coerce types where possible

## Example Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "title": { "type": "string" },
    "description": { "type": "string" },
    "tags": {
      "type": "array",
      "items": { "type": "string" }
    },
    "status": {
      "type": "string",
      "enum": ["draft", "review", "published"]
    }
  },
  "required": ["title"]
}
```

## Development

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy

# Build release
cargo build --release
```

## License

GPL-3.0-or-later
