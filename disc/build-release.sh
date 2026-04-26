#!/usr/bin/env bash
# MIT License - Copyright (c) 2026 Destroyer
# build-release.sh — Genera binarios de release para todas las plataformas
#
# Uso:
#   ./build-release.sh              # todas las plataformas
#   ./build-release.sh --native     # solo la plataforma actual
#   ./build-release.sh --no-checks  # salta fmt/clippy/test
#
# Requisitos:
#   - rustup con toolchain stable instalado
#   - cross  (se instala automáticamente si falta)

set -euo pipefail

# ─── Colores ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'
YELLOW='\033[1;33m'; BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}→${RESET} $*"; }
success() { echo -e "${GREEN}✓${RESET} $*"; }
warn()    { echo -e "${YELLOW}!${RESET} $*"; }
error()   { echo -e "${RED}✗${RESET} $*" >&2; exit 1; }
header()  { echo -e "\n${BOLD}$*${RESET}"; }

# ─── Argumentos ───────────────────────────────────────────────────────────────
NATIVE_ONLY=false
SKIP_CHECKS=false
for arg in "$@"; do
  case $arg in
    --native)     NATIVE_ONLY=true ;;
    --no-checks)  SKIP_CHECKS=true ;;
    -h|--help)
      echo "Uso: $0 [--native] [--no-checks]"
      echo "  --native      Solo compilar para la plataforma actual"
      echo "  --no-checks   Saltar fmt / clippy / tests"
      exit 0 ;;
    *) error "Argumento desconocido: $arg" ;;
  esac
done

# ─── Directorios ──────────────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="$SCRIPT_DIR/dist"
mkdir -p "$DIST_DIR"

# ─── Calidad: fmt / clippy / tests ────────────────────────────────────────────
if [[ "$SKIP_CHECKS" == false ]]; then
  header "Comprobaciones de calidad"

  info "cargo fmt --check"
  cargo fmt --all -- --check || error "Fallo de formato. Ejecuta: cargo fmt"
  success "Formato OK"

  info "cargo clippy (0 warnings)"
  cargo clippy -- -D warnings
  success "Clippy OK"

  info "cargo test"
  cargo test --all
  success "Tests OK"
fi

# ─── Detectar plataforma actual ───────────────────────────────────────────────
detect_native_target() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"
  case "$os-$arch" in
    Linux-x86_64)   echo "x86_64-unknown-linux-gnu" ;;
    Linux-aarch64)  echo "aarch64-unknown-linux-gnu" ;;
    Darwin-x86_64)  echo "x86_64-apple-darwin" ;;
    Darwin-arm64)   echo "aarch64-apple-darwin" ;;
    MINGW*|MSYS*|CYGWIN*) echo "x86_64-pc-windows-msvc" ;;
    *) error "Plataforma no reconocida: $os-$arch" ;;
  esac
}

artifact_name() {
  local target="$1"
  case "$target" in
    x86_64-unknown-linux-gnu)   echo "disc-linux-x86_64" ;;
    aarch64-unknown-linux-gnu)  echo "disc-linux-aarch64" ;;
    x86_64-apple-darwin)        echo "disc-macos-x86_64" ;;
    aarch64-apple-darwin)       echo "disc-macos-aarch64" ;;
    x86_64-pc-windows-msvc)     echo "disc-windows-x86_64.exe" ;;
    *) echo "disc-$target" ;;
  esac
}

binary_path() {
  local target="$1"
  if [[ "$target" == *windows* ]]; then
    echo "target/$target/release/disc.exe"
  else
    echo "target/$target/release/disc"
  fi
}

# ─── Instalar cross si hace falta ─────────────────────────────────────────────
ensure_cross() {
  if ! command -v cross &>/dev/null; then
    warn "'cross' no encontrado. Instalando..."
    cargo install cross --git https://github.com/cross-rs/cross
    success "cross instalado"
  fi
}

# ─── Compilar un target ───────────────────────────────────────────────────────
build_target() {
  local target="$1"
  local use_cross="${2:-false}"
  local artifact
  artifact="$(artifact_name "$target")"
  local bin_path
  bin_path="$(binary_path "$target")"

  header "Compilando → $artifact"
  info "Target: $target"

  # Añadir target a rustup si no está instalado (solo si rustup está disponible)
  if command -v rustup &>/dev/null; then
    if ! rustup target list --installed | grep -q "$target"; then
      info "Añadiendo target $target a rustup..."
      rustup target add "$target"
    fi
  else
    info "rustup no disponible (instalación vía Homebrew) — asumiendo target nativo"
  fi

  if [[ "$use_cross" == true ]]; then
    ensure_cross
    info "Usando cross para compilación cruzada"
    cross build --release --target "$target"
  else
    cargo build --release --target "$target"
  fi

  local dest="$DIST_DIR/$artifact"
  cp "$bin_path" "$dest"

  local size sha256
  size="$(du -sh "$dest" | cut -f1)"
  if command -v sha256sum &>/dev/null; then
    sha256="$(sha256sum "$dest" | awk '{print $1}')"
  elif command -v shasum &>/dev/null; then
    sha256="$(shasum -a 256 "$dest" | awk '{print $1}')"
  else
    sha256="(sha256sum no disponible)"
  fi

  success "$artifact"
  echo "   Tamaño : $size"
  echo "   SHA256 : $sha256"
  # Guardar sha256 en fichero para facilitar actualización de packaging
  echo "$sha256  $artifact" >> "$DIST_DIR/SHA256SUMS"
}

# ─── Selección de targets ─────────────────────────────────────────────────────

# Limpiar SHA256SUMS anterior
rm -f "$DIST_DIR/SHA256SUMS"

if [[ "$NATIVE_ONLY" == true ]]; then
  NATIVE_TARGET="$(detect_native_target)"
  info "Modo nativo: $NATIVE_TARGET"
  build_target "$NATIVE_TARGET" false
else
  host_os="$(uname -s)"

  header "Build multi-plataforma"

  # macOS puede compilar nativamente x86_64 y arm64 sin cross
  if [[ "$host_os" == "Darwin" ]]; then
    build_target "aarch64-apple-darwin"  false
    build_target "x86_64-apple-darwin"   false
    # Linux y Windows requieren cross desde macOS
    build_target "x86_64-unknown-linux-gnu"  true
    build_target "aarch64-unknown-linux-gnu" true
    warn "Windows (x86_64-pc-windows-msvc) no está soportado por cross desde macOS."
    warn "Usa GitHub Actions (ci.yml) para generar el binario Windows."

  elif [[ "$host_os" == "Linux" ]]; then
    ARCH="$(uname -m)"
    if [[ "$ARCH" == "x86_64" ]]; then
      build_target "x86_64-unknown-linux-gnu"  false
      build_target "aarch64-unknown-linux-gnu" true
    else
      build_target "aarch64-unknown-linux-gnu" false
      build_target "x86_64-unknown-linux-gnu"  true
    fi
    warn "macOS y Windows requieren GitHub Actions (ci.yml)."

  else
    warn "Sistema no reconocido. Compilando solo nativo."
    NATIVE_TARGET="$(detect_native_target)"
    build_target "$NATIVE_TARGET" false
  fi
fi

# ─── Resumen final ────────────────────────────────────────────────────────────
header "Binarios generados en dist/"
ls -lh "$DIST_DIR"/
echo ""
if [[ -f "$DIST_DIR/SHA256SUMS" ]]; then
  echo -e "${BOLD}SHA256SUMS${RESET} (útil para actualizar packaging/):"
  cat "$DIST_DIR/SHA256SUMS"
fi
echo ""
success "¡Listo!"
