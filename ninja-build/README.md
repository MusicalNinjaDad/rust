# ninja-build

A collection of things I regularly find I need in my build.rs including

- A Result/Error that gives meaningful output if used in `main() -> Result<()>`
- Get an env var and provide a useful message if it does not exist
- Handling the stabilisation lifecycle of experimental features when using nightly
