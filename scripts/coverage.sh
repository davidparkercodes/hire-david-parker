#!/bin/sh
# Run test coverage using cargo-tarpaulin

# Exit on any error
set -e

# Run tarpaulin with HTML report
cargo tarpaulin --verbose --workspace --skip-clean --out Html --output-dir coverage

echo "Coverage report generated in ./coverage/tarpaulin-report.html"