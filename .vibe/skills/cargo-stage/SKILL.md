---
name: cargo-stage
description: Load this skill when you need to verify Rust code quality or run tests. Use cargo-stage instead of cargo fmt, cargo check, cargo clippy, or cargo test directly.
---

# Cargo Stage Verification Skill

**Always use `cargo stage --strict --json` for ALL verification tasks.** This single command replaces `cargo fmt`, `cargo check`, `cargo clippy`, and `cargo test` — do not invoke those commands directly.

## When to Use

Use this skill whenever you need to:
- Format code (replaces `cargo fmt`)
- Check code compiles (replaces `cargo check`)
- Lint code (replaces `cargo clippy`)
- Run tests (replaces `cargo test`)

## The Command

```bash
cargo stage --strict --json
```

This command runs all verification stages in sequence and outputs JSON.

## Output Format

The tool outputs a JSON array where each element represents a task:

```json
[
  {
    "payload": [...],
    "status": "exit status: 0",
    "task": "fmt"
  },
  {
    "payload": [...],
    "status": "exit status: 0", 
    "task": "clippy"
  },
  {
    "payload": [...],
    "status": "exit status: 0",
    "task": "clippy the tests"
  },
  {
    "payload": [...],
    "status": "exit status: 0",
    "task": "tests"
  },
  {
    "payload": [...],
    "status": "exit status: 0",
    "task": "test examples"
  }
]
```

## Task Breakdown

### fmt task
- **Success**: `"payload": []` with `"status": "exit status: 0"`
- **Failure**: payload contains formatting error details

### clippy and clippy the tests tasks
- **Success**: `"payload": [{"reason": "build-finished", "success": true}]` with `"status": "exit status: 0"`
- **Failure**: payload contains compiler messages. Each message has:
  - `reason`: "compiler-message"
  - `package_id`: package identifier
  - `manifest_path`: path to Cargo.toml
  - `target`: object with kind, crate_types, name, src_path, edition, etc.
  - `message`: the diagnostic message (see compiler_message_format below)

The `message` field in clippy payloads follows the compiler message format with:
- `$message_type`: "diagnostic"
- `message`: the primary diagnostic text
- `code`: object with `code` (lint name) and optional `explanation`
- `level`: "error", "warning", "note", "help", "failure-note", or "error: internal compiler error"
- `spans`: array of source locations with byte/line/column offsets, file names, suggested replacements, etc.
- `children`: array of related diagnostic messages
- `rendered`: the full formatted diagnostic string

### tests and test examples tasks
- **Success**: payload contains test events including `{"event": "ok", ...}` for each passing test, and a summary with `"failed": 0`
- **Failure**: payload contains test events with `"event": "failed"` entries, and a summary with `"failed": N` where N > 0

Test events include:
- `started` (suite or test)
- `ok` (passing test)
- `failed` (failing test with details)
- Summary with: `exec_time`, `failed`, `filtered_out`, `ignored`, `measured`, `passed`, `type: "suite"`

## How to Check Results

1. **Overall success**: All tasks have `"status": "exit status: 0"` AND
   - For fmt: empty payload
   - For clippy/clippy the tests: payload contains `{"reason": "build-finished", "success": true}`
   - For tests/test examples: payload summary has `"failed": 0`

2. **Overall failure**: Any task has non-zero exit status, OR
   - clippy payload contains compiler-message entries with `level: "error"` or `level: "warning"` (in --strict mode, warnings are treated as errors)
   - tests payload contains `"event": "failed"` entries or summary has `"failed": N > 0`

## What NOT to Do

- **Do NOT** run `cargo fmt` directly
- **Do NOT** run `cargo check` directly
- **Do NOT** run `cargo clippy` directly
- **Do NOT** run `cargo test` directly
- **DO** always use `cargo stage --strict --json`

## Environment

The tool is pre-configured to work correctly in this environment with the proper environment variables and flags.

## Reference Files

The following files in the skill directory provide detailed payload format documentation:
- `clippy_payload_format` - format for clippy task compiler messages
- `compiler_message_format` - format for individual diagnostic messages

These match the Rust compiler's JSON output format. See https://doc.rust-lang.org/rustc/json.html for full details.
