---
name: compilation
description: Load this skill when you need to compile the project.
---

# Compilation Skill

## When to Load

Load this skill whenever you need to build/compile the Rust project.

## Target Discovery

To identify installed and available compilation targets:

```bash
rustup target list --installed
```

All relevant targets for this project are expected to be pre-installed in the environment.

## Compilation Commands

### Linux Targets

To compile for Linux targets, use:

```bash
cargo ninja build
```

See `cargo ninja build --help` for additional options and details.

### Windows Targets

To compile for Windows targets, use:

```bash
cargo xwin build --target <TARGET>
```

Replace `<TARGET>` with the appropriate Windows target identifier.

## Important Rule

**If you need a target that is NOT in the installed list from `rustup target list --installed`:**

- **STOP** immediately
- **DO NOT** attempt to install it yourself
- **ASK THE USER** for permission and guidance

Do not proceed with any target installation without explicit user approval.
