# mdbook-frontmatter

MDBook preprocessor to validate or fix YAML frontmatter against a JSON schema.

## Quick Start

1. Install the preprocessor:
   ```bash
   cargo install mdbook-frontmatter
   ```

2. Add to your `book.toml`:
   ```toml
   [preprocessor.frontmatter]
   schema = "https://schemas.arusty.dev/draft/2020-12/frontmatter.schema.json"
   mode = "validate"
   ```

3. Build your book:
   ```bash
   mdbook build
   ```

## Features

- **Validate Mode**: Check frontmatter and report errors
- **Fix Mode**: Automatically correct frontmatter issues
- **Flexible Schemas**: Load from HTTP(S) or local files
- **Build Integration**: Fail or warn on validation errors

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `schema` | string | required | Schema URL (http://, https://, file://) |
| `mode` | string | `"validate"` | Processing mode: `validate` or `fix` |
| `fail_on_error` | bool | `true` | Whether to fail build on errors |
