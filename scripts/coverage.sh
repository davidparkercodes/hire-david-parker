#!/bin/sh
# Run test coverage using cargo-tarpaulin

# Exit on any error
set -e

# Run tarpaulin with HTML report and exclude runner.rs
cargo tarpaulin --workspace --skip-clean --exclude-files "src/tui/runner.rs" --out Html --output-dir coverage

echo "Coverage report generated in ./coverage/tarpaulin-report.html"