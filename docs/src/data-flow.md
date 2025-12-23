# Data Flow

## Overview

```mermaid
flowchart TD
    subgraph MDBook["MDBook Build"]
        A[mdbook build] --> B[Invoke Preprocessor]
    end

    subgraph Preprocessor["mdbook-frontmatter"]
        B --> C[Parse stdin JSON]
        C --> D[Load Config from book.toml]
        D --> E{Schema Source?}

        E -->|HTTP/HTTPS| F[Fetch Remote Schema]
        E -->|file://| G[Read Local Schema]

        F --> H[Compile JSON Schema]
        G --> H

        H --> I[Process Chapters]

        subgraph ChapterLoop["For Each Chapter"]
            I --> J{Has Frontmatter?}
            J -->|No| K[Skip]
            J -->|Yes| L[Extract YAML]
            L --> M[Parse to JSON]
            M --> N[Validate vs Schema]
            N --> O{Valid?}
            O -->|Yes| P[Keep Original]
            O -->|No| Q{Mode?}
            Q -->|validate| R[Collect Error]
            Q -->|fix| S[Apply Fixes]
            S --> T[Serialize YAML]
            T --> U[Replace Frontmatter]
        end

        I --> V{Errors?}
        V -->|No| W[Output Modified Book]
        V -->|Yes & fail_on_error| X[Exit with Error]
        V -->|Yes & !fail_on_error| Y[Log Warnings]
        Y --> W
    end

    W --> Z[Continue MDBook Build]
```

## Input Format

MDBook sends a JSON object via stdin:

```json
{
  "root": "/path/to/book",
  "config": {
    "book": { ... },
    "preprocessor": {
      "frontmatter": {
        "schema": "https://...",
        "mode": "validate"
      }
    }
  },
  "renderer": "html",
  "mdbook_version": "0.4.52"
}
```

Followed by the book content:

```json
{
  "sections": [
    {
      "Chapter": {
        "name": "Chapter 1",
        "content": "---\ntitle: ...\n---\n\n# Content",
        "path": "chapter-1.md",
        ...
      }
    }
  ]
}
```

## Output Format

The preprocessor outputs the modified book to stdout:

```json
{
  "sections": [
    {
      "Chapter": {
        "name": "Chapter 1",
        "content": "---\ntitle: Fixed Title\n---\n\n# Content",
        ...
      }
    }
  ]
}
```

## Frontmatter Processing

### Extraction

1. Check if content starts with `---`
2. Find closing `---` marker
3. Extract YAML between markers
4. Parse YAML to serde_json::Value

### Validation

1. Apply JSON Schema validation
2. Collect all validation errors
3. Format errors with chapter context

### Fixing (fix mode)

1. Check for missing required fields
2. Add defaults from schema if available
3. Coerce types where unambiguous
4. Re-serialize to YAML
5. Replace original frontmatter
