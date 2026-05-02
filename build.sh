#!/usr/bin/env bash
# MIT License — Copyright (c) Destroyer 2026
# =============================================================================
# build.sh — Compila xDSK Desktop + CLIs para múltiples plataformas
# =============================================================================
#
# USO
#   ./build.sh                        # Todos los targets disponibles
#   ./build.sh --target macos         # Solo macOS (host, sin Docker)
#   ./build.sh --target linux         # Solo Linux x86_64 (Docker)
#   ./build.sh --target windows       # Solo Windows x86_64 (Docker)
#   ./build.sh --cli-only             # Solo CLIs, sin GUI
#   ./build.sh --gui-only             # Solo GUI, sin CLIs
#   ./build.sh --no-docker            # Deshabilita Docker (solo host)
#
# TARGETS SOPORTADOS
#   macos     → aarch64-apple-darwin  (compilado en host macOS)
#   linux     → x86_64-unknown-linux-gnu  (Docker: ghcr.io/cross-rs/...)
#   windows   → x86_64-pc-windows-gnu    (Docker: cross)
#
# SALIDA  (dist/ se borra y recrea en cada ejecución)
#   dist/
#     macos/
#       xdsk                          CLI para macOS ARM64
#       xcdt                          CLI para macOS ARM64
#       xcart                         CLI para macOS ARM64
#       xDSK Desktop.app/             Bundle .app
#       *.dmg                         Instalador DMG
#     linux/
#       xdsk                          CLI Linux x86_64
#       xcdt
#       xcart
#       *.AppImage                    Instalador AppImage
#       *.deb                         Paquete Debian
#     windows/
#       xdsk.exe                      CLI Windows x86_64
#       xcdt.exe
#       xcart.exe
#       *-setup.exe                   Instalador NSIS
#       *.msi                         Instalador MSI
#
# REQUISITOS
#   macOS  : rustup, node/npm, @tauri-apps/cli
#   Linux  : Docker con imagen cross-rs o ghcr.io/cross-rs
#   Windows: Docker con imagen cross-rs
# =============================================================================

set -euo pipefail

# ── Colores ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'
YELLOW='\033[1;33m'; BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}→${RESET} $*"; }
success() { echo -e "${GREEN}✓${RESET} $*"; }
warn()    { echo -e "${YELLOW}!${RESET} $*"; }
fail()    { echo -e "${RED}✗${RESET} $*" >&2; }
header()  { echo -e "\n${BOLD}━━━ $* ━━━${RESET}"; }

# ── Rutas ─────────────────────────────────────────────────────────────────────
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST="$ROOT/dist"
GUI_DIR="$ROOT/xdsk-desktop"

# ── Argumentos ────────────────────────────────────────────────────────────────
CLI_ONLY=false
GUI_ONLY=false
USE_DOCKER=true
TARGETS=()

while [[ $# -gt 0 ]]; do
  case $1 in
    --cli-only)  CLI_ONLY=true ;;
    --gui-only)  GUI_ONLY=true ;;
    --no-docker) USE_DOCKER=false ;;
    --target)
      TARGETS+=("$2"); shift ;;
    -h|--help)
      sed -n '3,50p' "$0" | grep '^#' | sed 's/^# \?//'
      exit 0 ;;
    *) fail "Argumento desconocido: $1"; exit 1 ;;
  esac
  shift
done

# Si no se especificaron targets, construir todos los posibles
if [[ ${#TARGETS[@]} -eq 0 ]]; then
  TARGETS=(macos)
  if [[ "$USE_DOCKER" == true ]] && command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
    TARGETS+=(linux windows)
  else
    warn "Docker no disponible — solo se compilará para macOS (host)"
    warn "Para compilar Linux/Windows instala Docker o usa --no-docker"
  fi
fi

ERRORS=0

# ── Preparar dist ─────────────────────────────────────────────────────────────
mkdir -p "$DIST"/{macos,linux,windows}

# =============================================================================
# HELPERS
# =============================================================================

# Comprueba si cross está instalado, si no lo instala
ensure_cross() {
  # Asegurar que ~/.cargo/bin está en PATH (puede no estarlo en subshells)
  export PATH="$HOME/.cargo/bin:$PATH"
  if ! command -v cross &>/dev/null; then
    info "Instalando cross (cross-compilation)..."
    cargo install cross --git https://github.com/cross-rs/cross
    export PATH="$HOME/.cargo/bin:$PATH"
  fi
}

# Copia un binario a dist/<plataforma>/
copy_bin() {
  local src="$1" dst="$2"
  if [[ -f "$src" ]]; then
    cp "$src" "$dst"
    success "$(basename "$dst")  ($(du -sh "$dst" | cut -f1))"
  else
    fail "Binario no encontrado: $src"
    ((ERRORS++))
  fi
}

# Copia todos los archivos de un directorio (no recursivo) al destino
copy_bundles() {
  local src_dir="$1" dst_dir="$2" ext_pat="${3:-*}"
  if [[ -d "$src_dir" ]]; then
    for f in "$src_dir"/$ext_pat; do
      [[ -f "$f" ]] || continue
      cp "$f" "$dst_dir/"
      success "$(basename "$f") → $(basename "$dst_dir")/"
    done
  fi
}

# =============================================================================
# BUILDS POR TARGET
# =============================================================================

# ─────────────────────────────────────────────────────────────────────────────
# macOS (host nativo)
# ─────────────────────────────────────────────────────────────────────────────
build_macos() {
  local rust_target="aarch64-apple-darwin"
  local out="$DIST/macos"

  header "macOS · $rust_target (host)"

  # Instalar target si no existe
  rustup target add "$rust_target" 2>/dev/null || true

  # ── CLIs ──
  if [[ "$GUI_ONLY" == false ]]; then
    for cli in xdsk xcdt xcart; do
      local src_dir="$ROOT/$cli"
      [[ -d "$src_dir" ]] || { warn "$cli: directorio no encontrado — omitido"; continue; }
      # xcart requiere ROMs embebidas en compilación
      if [[ "$cli" == xcart ]]; then
        local missing_roms=false
        for rom in os.rom basic.rom amsdos.rom; do
          [[ -f "$src_dir/roms/$rom" ]] || { missing_roms=true; break; }
        done
        if [[ "$missing_roms" == true ]]; then
          warn "xcart: faltan ROMs en xcart/roms/ (os.rom, basic.rom, amsdos.rom) — omitido"
          continue
        fi
      fi
      info "cargo build $cli (release)"
      (cd "$src_dir" && cargo build --release --target "$rust_target") || { fail "$cli build fallido"; ((ERRORS++)); continue; }
      copy_bin "$src_dir/target/$rust_target/release/$cli" "$out/$cli"
    done

    # Copiar sidecar xdsk al bundle de Tauri
    if [[ -f "$out/xdsk" ]]; then
      cp "$out/xdsk" "$GUI_DIR/src-tauri/binaries/xdsk-$rust_target"
      info "Sidecar xdsk-$rust_target actualizado"
    fi
  fi

  # ── GUI ──
  if [[ "$CLI_ONLY" == false ]]; then
    if ! command -v npm &>/dev/null; then
      warn "npm no encontrado — GUI omitido"
    else
      info "npm ci"
      (cd "$GUI_DIR" && npm ci --silent)
      info "tauri build --target $rust_target"
      if (cd "$GUI_DIR" && npm run tauri build -- --target "$rust_target"); then
        local bundle_base="$GUI_DIR/src-tauri/target/$rust_target/release/bundle"
        # .app
        local app="$bundle_base/macos/xDSK Desktop.app"
        [[ -d "$app" ]] && { rm -rf "$out/xDSK Desktop.app"; cp -r "$app" "$out/xDSK Desktop.app"; success "xDSK Desktop.app → dist/macos/"; }
        # .dmg
        copy_bundles "$bundle_base/dmg" "$out" "*.dmg"
      else
        fail "GUI macOS build fallido"; ((ERRORS++))
      fi
    fi
  fi
}

# ─────────────────────────────────────────────────────────────────────────────
# Linux x86_64 (Docker linux/amd64 — imagen oficial rust:slim)
# ─────────────────────────────────────────────────────────────────────────────
build_linux() {
  local rust_target="x86_64-unknown-linux-gnu"
  local out="$DIST/linux"

  header "Linux · $rust_target (Docker linux/amd64)"

  if ! command -v docker &>/dev/null || ! docker info &>/dev/null 2>&1; then
    warn "Docker no disponible — target linux omitido"
    return
  fi

  # ── CLIs ──
  if [[ "$GUI_ONLY" == false ]]; then
    for cli in xdsk xcdt xcart; do
      local src_dir="$ROOT/$cli"
      [[ -d "$src_dir" ]] || { warn "$cli: directorio no encontrado — omitido"; continue; }
      if [[ "$cli" == xcart ]]; then
        local missing_roms=false
        for rom in os.rom basic.rom amsdos.rom; do
          [[ -f "$src_dir/roms/$rom" ]] || { missing_roms=true; break; }
        done
        if [[ "$missing_roms" == true ]]; then
          warn "xcart: faltan ROMs en xcart/roms/ (os.rom, basic.rom, amsdos.rom) — omitido"
          continue
        fi
      fi
      info "docker build $cli → linux/amd64"
      docker run --rm \
        --platform linux/amd64 \
        -v "$src_dir:/project" \
        -v "xdsk-cargo-cache:/usr/local/cargo/registry" \
        -w /project \
        rust:slim \
        cargo build --release || { fail "$cli Linux build fallido"; ((ERRORS++)); continue; }
      copy_bin "$src_dir/target/release/$cli" "$out/$cli"
    done

    # Sidecar para Tauri
    if [[ -f "$out/xdsk" ]]; then
      cp "$out/xdsk" "$GUI_DIR/src-tauri/binaries/xdsk-$rust_target"
      info "Sidecar xdsk-$rust_target actualizado"
    fi
  fi

  # ── GUI Linux ──
  if [[ "$CLI_ONLY" == false ]]; then
    warn "GUI Linux: build local no soportado (requiere WebKit2GTK)."
    warn "Usa 'git tag vX.Y.Z && git push origin vX.Y.Z' para el bundle Linux via GitHub Actions."
  fi
}

# ─────────────────────────────────────────────────────────────────────────────
# Windows x86_64 — xdsk CLI + xdsk-desktop NSIS .exe
# Prioridad: OrbStack Ubuntu VM > Docker (linux/amd64)
# ─────────────────────────────────────────────────────────────────────────────
build_windows() {
  local rust_target="x86_64-pc-windows-gnu"
  local out="$DIST/windows"

  # ── Elegir método de ejecución ──────────────────────────────────────────────
  # OrbStack Ubuntu VM: nativo, rápido, herramientas persistentes
  # Docker linux/amd64: fallback via QEMU (más lento, reinstala cada vez)
  local RUN_CMD=""
  local WORKSPACE_IN_HOST="$ROOT"

  if command -v orb &>/dev/null && orb list 2>/dev/null | grep -q "^ubuntu"; then
    header "Windows · $rust_target (OrbStack ubuntu VM)"
    RUN_CMD="orb run ubuntu --"
    # En OrbStack el home de macOS es accesible en la VM con la misma ruta
  elif command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
    header "Windows · $rust_target (Docker linux/amd64)"
    # Usamos una función auxiliar para docker run
    RUN_CMD="__docker_run"
  else
    warn "Ni OrbStack Ubuntu ni Docker disponibles — target windows omitido"
    return
  fi

  # Función auxiliar para Docker
  __docker_run() {
    docker run --rm \
      --platform linux/amd64 \
      -v "$ROOT:$ROOT" \
      -v "xdsk-cargo-cache-win:/root/.cargo/registry" \
      -w "$ROOT" \
      ubuntu:22.04 \
      bash -c "$*"
  }

  # ── Script de build (mismo para OrbStack y Docker) ─────────────────────────
  local BUILD_SCRIPT
  BUILD_SCRIPT="
set -e

# ── Dependencias del sistema (idempotente: solo instala si falta) ──
if ! command -v x86_64-w64-mingw32-gcc &>/dev/null || ! command -v makensis &>/dev/null || ! command -v node &>/dev/null; then
  apt-get update -qq
  apt-get install -y -qq build-essential curl gcc-mingw-w64-x86-64 nsis nodejs npm
fi

# ── Rust + target Windows ──
if ! command -v cargo &>/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal --no-modify-path
fi
export PATH=\"\$HOME/.cargo/bin:\$PATH\"
rustup target add x86_64-pc-windows-gnu 2>/dev/null || true

# ── CLI xdsk ──
cd $ROOT/xdsk
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/xdsk.exe \
   $ROOT/xdsk-desktop/src-tauri/binaries/xdsk-x86_64-pc-windows-gnu.exe

# ── GUI xdsk-desktop (NSIS) ──
cd $ROOT/xdsk-desktop
npm ci --silent
npm run tauri build -- --target x86_64-pc-windows-gnu --bundles nsis
"

  if [[ "$RUN_CMD" == "__docker_run" ]]; then
    __docker_run "$BUILD_SCRIPT" || { fail "Windows build fallido"; ((ERRORS++)); return; }
  else
    $RUN_CMD bash -c "$BUILD_SCRIPT" || { fail "Windows build fallido"; ((ERRORS++)); return; }
  fi

  local nsis_dir="$GUI_DIR/src-tauri/target/$rust_target/release/bundle/nsis"
  copy_bin "$ROOT/xdsk/target/$rust_target/release/xdsk.exe" "$out/xdsk.exe"
  copy_bundles "$nsis_dir" "$out" "*.exe"
  success "Instalador Windows NSIS generado en dist/windows/"
}

# =============================================================================
# EJECUTAR TARGETS SELECCIONADOS
# =============================================================================

for target in "${TARGETS[@]}"; do
  case "$target" in
    macos)   build_macos   ;;
    linux)   build_linux   ;;
    windows) build_windows ;;
    *)       fail "Target desconocido: $target (usa: macos, linux, windows)"; ((ERRORS++)) ;;
  esac
done

# =============================================================================
# RESUMEN
# =============================================================================
header "Resumen del Build"
echo ""
printf "  %-14s %s\n" "Targets:"  "${TARGETS[*]}"
printf "  %-14s %s\n" "Perfil:"   "release"
printf "  %-14s %s\n" "Salida:"   "$DIST"
echo ""

for platform_dir in "$DIST"/*/; do
  [[ -d "$platform_dir" ]] || continue
  platform="$(basename "$platform_dir")"
  echo -e "  ${BOLD}$platform/${RESET}"
  found=false
  for f in "$platform_dir"*; do
    [[ -e "$f" ]] || continue
    found=true
    if [[ -d "$f" ]]; then
      printf "    %-32s %s\n" "$(basename "$f")/" "(bundle)"
    else
      printf "    %-32s %s\n" "$(basename "$f")" "($(du -sh "$f" | cut -f1))"
    fi
  done
  [[ "$found" == false ]] && echo "    (sin artefactos)"
  echo ""
done

if [[ "$ERRORS" -eq 0 ]]; then
  success "Build completado sin errores."
else
  fail "$ERRORS componente(s) fallaron."
  exit 1
fi
