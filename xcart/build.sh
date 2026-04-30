#!/usr/bin/env bash
# build.sh — Build xcart for the current platform or cross-compile for aarch64-apple-darwin
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── Config ────────────────────────────────────────────────────────────────────
TARGET="${1:-}"          # optional: e.g. "aarch64-apple-darwin"
PROFILE="${2:-release}"  # "release" or "debug"

# ── ROMs check ────────────────────────────────────────────────────────────────
echo "Checking ROMs..."
for rom in roms/os.rom roms/basic.rom roms/amsdos.rom; do
    if [[ ! -f "$rom" ]]; then
        echo "ERROR: Missing ROM: $rom"
        echo "       Run: ln -s ../../nocart/roms/<name>.rom $rom"
        exit 1
    fi
    echo "  OK  $rom ($(wc -c < "$rom") bytes)"
done

# ── Build ─────────────────────────────────────────────────────────────────────
CARGO_ARGS=()
[[ "$PROFILE" == "release" ]] && CARGO_ARGS+=(--release)
[[ -n "$TARGET" ]] && CARGO_ARGS+=(--target "$TARGET")

echo ""
echo "Building xcart [profile=$PROFILE${TARGET:+, target=$TARGET}]..."
cargo build "${CARGO_ARGS[@]}"

# ── Locate binary ─────────────────────────────────────────────────────────────
if [[ -n "$TARGET" ]]; then
    BIN="target/$TARGET/$PROFILE/xcart"
else
    BIN="target/$PROFILE/xcart"
fi

echo ""
echo "Binary : $BIN"
echo "Size   : $(du -sh "$BIN" | cut -f1)"
echo ""
echo "Done."
