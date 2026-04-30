#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

TARGET="aarch64-apple-darwin"

echo "==> Building xcdt (release)..."
cargo build --release --target "$TARGET"

BINARY="target/$TARGET/release/xcdt"
echo "==> Binary: $BINARY"
echo "==> Size  : $(du -sh "$BINARY" | cut -f1)"

# Quick smoke test
echo
echo "==> xcdt --help"
"$BINARY" --help
echo
echo "Build OK."
