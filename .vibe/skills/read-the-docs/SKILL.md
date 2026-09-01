---
name: read-the-docs
description: Load this skill when you need to access Rust crate documentation (JSON format) or standard library documentation (HTML format) for this project.
user-invocable: true
---

# Read the Docs

**Decide first: Docs or Source?**
- ✅ **Use docs FIRST** for: API surfaces, public fields, methods, trait impls, doc comments, cross-references
- 🔍 **Use source second** only if: docs are missing, unclear, or you need private implementation details

## Quick Start: Common Tasks

| What you need | Docs approach | Manual approach (don't do this) |
|---------------|---------------|---------------------------------|
| Methods on a type | Query docs JSON with jaq | `grep -r "impl Discid"` in cargo registry |
| Fields of a struct | Check `inner.fields` in docs | `cat /opt/cargo/registry/src/*/discid.rs` |
| Implemented traits | Check `inner.impls` in docs | Manual source reading |
| Doc comments | `index[].docs` field | Reading source comments |

## Copy-Paste Queries

**Find a type and its basic info:**
```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
jaq --arg TYPE "$TYPE" '\n  .index | to_entries[] | select(.value.name == $TYPE) | {\n    name: .value.name,\n    kind: (.value.inner | keys[0]),\n    visibility: .value.visibility,\n    docs: .value.docs,\n    file: .value.span.filename\n  }\n' "./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"
```

**List all public fields and methods for a struct:**
```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
jaq --arg TYPE "$TYPE" '\n  .index | to_entries[] | select(.value.name == $TYPE) | {\n    name: .value.name,\n    kind: (.value.inner | keys[0]),\n    fields: (.value.inner.struct?.fields // {} | to_entries | map({name: .key, type: .value.type})),\n    methods: (.value.inner.struct?.impls[]? | [.items[]? | select(.visibility == "public") | {name: .name, signature: (.signature // .decl | tostring)}] // [])\n  }\n' "./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"
```

**Check if a type has a specific method:**
```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
METHOD="id"
jaq --arg TYPE "$TYPE" --arg METHOD "$METHOD" '\n  .index | to_entries[] | select(.value.name == $TYPE) |\n  .value.inner.struct?.impls[]? |\n  .items[]? |\n  select(.name == $METHOD and .visibility == "public")\n' "./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"
```

**Find all trait implementations for a type:**
```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
jaq --arg TYPE "$TYPE" '\n  .index | to_entries[] | select(.value.name == $TYPE) |\n  [.value.inner.struct?.impls[]? | select(.trait != null) | .trait.name] // []\n' "./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"
```

This skill provides access to comprehensive Rust documentation for the redbook crate and all its dependencies, as well as the nightly Rust standard library.

## Documentation Sources

### 1. Crate Documentation (JSON Format)

Full documentation for this crate and all dependencies is available in JSON format.

**Target-independent libraries:** `./docs/doc/*.json`

**Target-specific libraries (with target specific cfgs):** `./docs/<target>/doc/*.json`
- Windows MSVC: `./docs/x86_64-pc-windows-msvc/doc/*.json`
- Linux GNU: `./docs/x86_64-unknown-linux-gnu/doc/*.json`

Each JSON file contains:
- Complete API documentation including private items for this project's crate(s)
- Item descriptions, signatures, and documentation strings
- Cross-references and links between items
- Span information (source file locations)
- Attribute information

**Main crate:** `./docs/x86_64-pc-windows-msvc/doc/redbook.json`

**Dependencies:** Individual JSON files for each dependency in the same directories.

### 2. Standard Library Documentation (HTML Format)

Full documentation for the installed nightly version of the Rust standard library is available in HTML format.

**Location:** `/opt/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/`

**Structure:**
- `alloc/` - Allocator and collection types
- `core/` - Core library (no_std compatible)
- `std/` - Standard library
- `index.html` - Main entry point with search
- Various guides in the root (e.g., `guide-ownership.html`, `guide-macros.html`)

## Indexes

Pre-built indexes are available at `./docs/index/` to speed up documentation queries:

### 1. Name-to-File Index: `./docs/index/name_to_file.json`

Maps item names to their containing file and crate. Structure:
```json
{
  "<item_name>": [
    {"crate": "<crate_name>", "file": "<filename.json>"},
    ...
  ]
}
```

```bash
#!/bin/bash
set -euo pipefail

INDEX_DIR=./docs/index
DOC_DIRS=("./docs/doc" "./docs/x86_64-pc-windows-msvc/doc" "./docs/x86_64-unknown-linux-gnu/doc")

mkdir -p "$INDEX_DIR"

# Function to extract crate name from filename
get_crate_name() {
  local file="$1"
  local basename
  basename=$(basename "$file" .json)
  
  # For redbook, use the basename directly
  if [ "$basename" = "redbook" ]; then
    echo "redbook"
    return
  fi
  
  # Try to get crate name from the first entry in the index
  local crate_name
  crate_name=$(jaq -r '.index | to_entries[0] | .value.span.filename | split("/") | .[6] | split("-")[0]' "$file" 2>/dev/null || echo "")
  
  # Fallback to basename if extraction fails
  if [ -z "$crate_name" ]; then
    echo "$basename"
  else
    echo "$crate_name"
  fi
}

# =============================================================================
# 1. Name-to-File Index
# =============================================================================
echo "Generating name-to-file index..."
rm -f "$INDEX_DIR/name_to_file.tmp"

for doc_dir in "${DOC_DIRS[@]}"; do
  for file in "$doc_dir"/*.json; do
    [ -f "$file" ] || continue
    
    crate_name=$(get_crate_name "$file")
    
    jaq -c '.index | to_entries[] | select(.value.name != null) | {name: .value.name, crate: $CRATE, file: $FILE} | select(.name != null)' \
      --arg CRATE "$crate_name" \
      --arg FILE "$file" \
      "$file" >> "$INDEX_DIR/name_to_file.tmp"
  done
done

jaq -s 'reduce .[] as $item ({}; .[$item.name] += [{crate: $item.crate, file: $item.file}])' "$INDEX_DIR/name_to_file.tmp" > "$INDEX_DIR/name_to_file.json"
rm -f "$INDEX_DIR/name_to_file.tmp"
echo "Name-to-file index generated."

# =============================================================================
# 2. Name-to-ID Index (Per Crate)
# =============================================================================
echo "Generating name-to-ID indexes..."
rm -f "$INDEX_DIR"/*_name_to_id.json

for doc_dir in "${DOC_DIRS[@]}"; do
  for file in "$doc_dir"/*.json; do
    [ -f "$file" ] || continue
    basename=$(basename "$file" .json)
    
    jaq '{
      name_to_id: (.index | to_entries | map({(.value.name // ""): .key}) | reduce .[] as $item ({}; . + $item) | with_entries(select(.key != ""))),
      crate: $CRATE
    } | .name_to_id' \
      --arg CRATE "$basename" \
      "$file" > "$INDEX_DIR/${basename}_name_to_id.json"
    
    echo "  Generated: $INDEX_DIR/${basename}_name_to_id.json"
  done
done
echo "Name-to-ID indexes generated."

# =============================================================================
# 3. Combined Index with Type Information
# =============================================================================
echo "Generating combined index..."
rm -f "$INDEX_DIR/combined.tmp"

for doc_dir in "${DOC_DIRS[@]}"; do
  for file in "$doc_dir"/*.json; do
    [ -f "$file" ] || continue
    
    crate_name=$(get_crate_name "$file")
    
    jaq --arg CRATE "$crate_name" --arg FILE "$file" '
      .index | to_entries[] | 
      select(.value.name != null) | {
        name: .value.name,
        type: (.value.inner | keys[0] // "unknown"),
        crate: $CRATE,
        file: $FILE,
        id: .key,
        docs: (.value.docs // null)
      }
    ' "$file" >> "$INDEX_DIR/combined.tmp"
  done
done

jaq -s '.' "$INDEX_DIR/combined.tmp" > "$INDEX_DIR/combined.json"
rm -f "$INDEX_DIR/combined.tmp"
echo "Combined index generated."

echo "All indexes updated successfully."
```

Make it executable:
```bash
chmod +x ./docs/update-indexes.sh
```

## JSON Structure Reference

Each JSON documentation file has the following structure:

```json
{
  "root": <root_item_id>,
  "crate_version": "<version>",
  "includes_private": true/false,
  "index": {
    "<item_id>": {
      "id": <item_id>,
      "crate_id": <crate_id>,
      "name": "<item_name>",
      "span": {
        "filename": "<source_file>",
        "begin": [<line>, <column>],
        "end": [<line>, <column>]
      },
      "visibility": "public"|"crate"|"private",
      "docs": "<documentation_string>",
      "links": {<link_name>: <target_id>},
      "attrs": [<attributes>],
      "deprecation": null|<deprecation_info>,
      "stability": null|<stability_info>,
      "const_stability": null|<const_stability_info>,
      "inner": {<type_specific_data>}
    }
  },
  "paths": {
    "<path_id>": {
      "crate_id": <crate_id>,
      "path": [<path>, <components>],
      "kind": "<item_kind>",
      "..."
    }
  }
}
```

The `inner` field contains type-specific data:
- `module`: { "is_crate": bool, "items": [<item_ids>], "is_stripped": bool }
- `struct`: { "fields": { ... }, "impls": [...] }
- `enum`: { "variants": [...] }
- `function`: { "signature": ..., "decl": { ... } }
- `trait`: { "items": [...], "impls": [...] }
- `proc_macro`: { "kind": "attr"|"derive"|"bang", "helpers": [...] }

## Best Practices

1. **Prefer indexes for known lookups**: Use the combined index or name-to-file index when you know what you're looking for.

2. **Use ripgrep for discovery**: Use `rg` to find candidate items across all files, then use jaq to extract precise information.

3. **Chain with jaq**: Pipe ripgrep output to jaq for structured extraction.

4. **Always use jaq**: Load the `jaq` skill and use it for all JSON parsing tasks.

5. **Document your queries**: Keep a log of useful jaq queries in `docs/useful-queries` for reuse.

6. **jaq Compatibility**: The indexes and scripts are designed for jaq 3.x. Key differences from jq:
   - Use `to_entries | map(...)` instead of `to_entries[] | map(...)` for proper iteration
   - Use `reduce .[] as $item ({}; . + $item)` instead of `group_by()` for aggregation
   - Use `split()` instead of `capture()` for regex extraction
   - Arrays are indexed with `[n]`, not `.[] | .[n]`

## When to Use This Skill

**Load this skill IMMEDIATELY when you need to:**

- ✅ Investigate a type from a dependency (struct, enum, trait, function)
- ✅ Check what methods/fields are available on a type
- ✅ Find which traits are implemented for a type
- ✅ Read documentation comments for an item
- ✅ Navigate cross-references between items
- ✅ Understand the public API of any crate

**BEFORE you:**
- [ ] Search dependency source with `grep`/`find`
- [ ] Read raw source files from `/opt/cargo/registry/src/`
- [ ] Manually trace type definitions

**Rule of thumb:** If it's about *what* the API provides (not *how* it's implemented), use docs first.

---

### Original Use Cases

Load this skill when you need to:
- Look up API documentation for this crate or its dependencies
- Search for specific types, functions, or modules
- Understand the structure of a dependency's API
- Find documentation strings or examples
- Navigate cross-references between items
- Access standard library documentation
- Update or regenerate documentation and indexes
