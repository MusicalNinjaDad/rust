#! /bin/bash

set -euxo pipefail

RUSTC_BOOTSTRAP=0 cargo +stable test
cargo +nightly test
cargo +nightly-2026-01-01 test
RUSTC_BOOTSTRAP=0 cargo +1.85.1 test
