# Read-the-Docs Cheat Sheet

Use this cheat sheet for quick access to common documentation queries. For full details, see the main SKILL.md file.

---

## I need to check what methods/fields are available on a type

This is the most common use case. The example from your issue shows checking `musicbrainz_rs::entity::discid::Discid`.

### Quick approach (using combined index):

```bash
# First, find which crate and file contains the type
jaq -r '.[] | select(.name == "Discid") | "\(.crate) - \(.file)"' ./docs/index/combined.json

# Then query that file for details
CRATE="musicbrainz_rs"
TYPE="Discid"
DOC_FILE="./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"

# Get basic info
jaq --arg TYPE "$TYPE" '\n  .index | to_entries[] | select(.value.name == $TYPE) | {\n    name: .value.name,\n    kind: (.value.inner | keys[0]),\n    visibility: .value.visibility,\n    docs: .value.docs,\n    fields: (.value.inner.struct?.fields // {} | to_entries | map({name: .key, type: .value.type}))\n  }\n' "$DOC_FILE"
```

### Full approach (all public API):

```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
DOC_FILE="./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"

jaq --arg TYPE "$TYPE" '\n  .index | to_entries[] | select(.value.name == $TYPE) | {\n    name: .value.name,\n    kind: (.value.inner | keys[0]),\n    fields: (.value.inner.struct?.fields // {} | to_entries | map({name: .key, type: .value.type})),\n    methods: (.value.inner.struct?.impls[]? | [.items[]? | select(.visibility == "public") | {name: .name, signature: (.signature // .decl | tostring)}] // [])\n  }\n' "$DOC_FILE"
```

---

## I need to check if a type has a specific method

```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
METHOD="id"
DOC_FILE="./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"

# Returns the method if it exists and is public
jaq --arg TYPE "$TYPE" --arg METHOD "$METHOD" '\n  .index | to_entries[] | select(.value.name == $TYPE) |\n  .value.inner.struct?.impls[]? |\n  .items[]? |\n  select(.name == $METHOD and .visibility == "public")\n' "$DOC_FILE"
```

---

## I need to find all trait implementations for a type

```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
DOC_FILE="./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"

jaq --arg TYPE "$TYPE" '\n  .index | to_entries[] | select(.value.name == $TYPE) |\n  [.value.inner.struct?.impls[]? | select(.trait != null) | .trait.name] // []\n' "$DOC_FILE"
```

---

## I need to find all types in a module

```bash
CRATE="musicbrainz_rs"
MODULE="entity::discid"
DOC_FILE="./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"

# First find the module
MODULE_ID=$(jaq -r --arg MODULE "$MODULE" '\n  .index | to_entries[] | select(.value.name == $MODULE) | .key\n' "$DOC_FILE")

# Then get its items
jaq --arg MOD_ID "$MODULE_ID" '\n  .index."$MOD_ID".inner.module?.items // [] | .[] | .index."\n' "$DOC_FILE"
```

---

## I need to search for a type across all crates

```bash
# Use the combined index to find where a type is defined
TYPE="Discid"
jaq --arg TYPE "$TYPE" '\n  .[] | select(.name == $TYPE) | {\n    name: .name,\n    type: .type,\n    crate: .crate,\n    file: .file,\n    docs: .docs\n  }\n' ./docs/index/combined.json
```

---

## I need to check the documentation string for an item

```bash
CRATE="musicbrainz_rs"
TYPE="Discid"
DOC_FILE="./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"

jaq --arg TYPE "$TYPE" '\n  .index | to_entries[] | select(.value.name == $TYPE) | .value.docs\n' "$DOC_FILE"
```

---

## I need to find all public items in a crate

```bash
CRATE="musicbrainz_rs"
DOC_FILE="./docs/x86_64-unknown-linux-gnu/doc/${CRATE}.json"

jaq '\n  .index | to_entries[] | select(.value.visibility == "public") | {\n    name: .value.name,\n    kind: (.value.inner | keys[0]),\n    type: .value.type\n  }\n' "$DOC_FILE"
```

---

## Common Crate Paths

| Crate | Doc File Path |
|-------|---------------|
| redbook | `./docs/x86_64-pc-windows-msvc/doc/redbook.json` or `./docs/x86_64-unknown-linux-gnu/doc/redbook.json` |
| musicbrainz_rs | `./docs/x86_64-unknown-linux-gnu/doc/musicbrainz_rs.json` |
| Other dependencies | `./docs/x86_64-unknown-linux-gnu/doc/<crate_name>.json` |

---

## Tips

1. **Start with the combined index** at `./docs/index/combined.json` to find where items are defined
2. **Use name-to-file index** at `./docs/index/name_to_file.json` for quick lookups
3. **Check target-specific docs** if you need cfg-gated items (Windows vs Linux)
4. **Use `-r` flag** with jaq to get raw output (without quotes) for scripting
5. **Pipe to less** for long output: `jaq '.' doc.json | less`
