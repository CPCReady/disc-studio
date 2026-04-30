#!/usr/bin/env bash
# MIT License — Copyright (c) Destroyer 2026
# =============================================================================
# build.sh — Compila todos los CLIs y el GUI de Disc Image Studio
# =============================================================================
#
# USO
#   ./build.sh              # Compila todo
#   ./build.sh --cli-only   # Solo CLIs (salta GUI)
#   ./build.sh --gui-only   # Solo GUI (salta CLIs)
#   ./build.sh --debug      # Modo debug (más rápido, binarios más grandes)
#   ./build.sh --target <TRIPLE>  # Fuerza un target Rust específico
#
# SALIDA
#   dist/xdsk               CLI — manipulación de imágenes DSK
#   dist/xcdt               CLI — imágenes de cassette CDT/TZX
#   dist/xcart              CLI — cartuchos GX-4000 CPR  (requiere ROMs)
#   dist/xDSK Desktop.app   Aplicación de escritorio macOS
#   dist/*.dmg              Instalador macOS (si Tauri lo genera)
#
# REQUISITOS
#   - rustup + toolchain stable  (para los CLIs)
#   - node + npm                 (para el GUI)
#   - cargo-tauri                (npm install lo instala automáticamente)
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

# ── Argumentos ────────────────────────────────────────────────────────────────
CLI_ONLY=false
GUI_ONLY=false
PROFILE=release
TARGET=""

while [[ $# -gt 0 ]]; do
  case $1 in
    --cli-only)  CLI_ONLY=true ;;
    --gui-only)  GUI_ONLY=true ;;
    --debug)     PROFILE=debug ;;
    --target)    TARGET="$2"; shift ;;
    -h|--help)
      echo "Uso: $0 [--cli-only] [--gui-only] [--debug] [--target TRIPLE]"
      echo ""
      echo "  --cli-only   Solo compilar CLIs (xdsk, xcdt, xcart)"
      echo "  --gui-only   Solo compilar GUI (xDSK Desktop)"
      echo "  --debug      Perfil debug (más rápido, sin optimizaciones)"
      echo "  --target T   Target Rust (ej. aarch64-apple-darwin)"
      exit 0 ;;
    *) echo "Argumento desconocido: $1" >&2; exit 1 ;;
  esac
  shift
done

# ── Detectar target ───────────────────────────────────────────────────────────
if [[ -z "$TARGET" ]]; then
  if ! command -v rustc &>/dev/null; then
    fail "rustc no encontrado. Instala Rust: https://rustup.rs"
    exit 1
  fi
  TARGET="$(rustc -vV 2>/dev/null | awk '/^host:/ { print $2 }')"
  if [[ -z "$TARGET" ]]; then
    fail "No se pudo detectar el target Rust."
    exit 1
  fi
fi

# ── Preparar dist ─────────────────────────────────────────────────────────────
mkdir -p "$DIST"

ERRORS=0

# ── Función: compilar un CLI ──────────────────────────────────────────────────
build_cli() {
  local name="$1"
  local dir="$2"
  local src_dir="$ROOT/$dir"

  header "Compilando $name"

  if [[ ! -d "$src_dir" ]]; then
    fail "Directorio no encontrado: $src_dir"
    ((ERRORS++))
    return
  fi

  local cargo_args=(--target "$TARGET")
  [[ "$PROFILE" == "release" ]] && cargo_args+=(--release)

  info "cargo build ${cargo_args[*]}"
  if ! (cd "$src_dir" && cargo build "${cargo_args[@]}"); then
    fail "$name: build fallido"
    ((ERRORS++))
    return
  fi

  local bin="$src_dir/target/$TARGET/$PROFILE/$name"
  if [[ -f "$bin" ]]; then
    cp "$bin" "$DIST/$name"
    success "$name → dist/$name  ($(du -sh "$DIST/$name" | cut -f1))"
  else
    fail "$name: binario no encontrado en $bin"
    ((ERRORS++))
  fi
}

# ── Compilar CLIs ─────────────────────────────────────────────────────────────
if [[ "$GUI_ONLY" == false ]]; then

  # xdsk — en carpeta disc/ (o xdsk/ si ya se renombró)
  if [[ -d "$ROOT/xdsk" ]]; then
    build_cli xdsk xdsk
  elif [[ -d "$ROOT/disc" ]]; then
    build_cli xdsk disc
  else
    fail "xdsk: no se encontró carpeta disc/ ni xdsk/"
    ((ERRORS++))
  fi

  # xcdt
  build_cli xcdt xcdt

  # xcart — requiere ROMs
  XCART_ROMS_OK=true
  for rom in os.rom basic.rom amsdos.rom; do
    if [[ ! -f "$ROOT/xcart/roms/$rom" ]]; then
      XCART_ROMS_OK=false
      break
    fi
  done

  if [[ "$XCART_ROMS_OK" == true ]]; then
    build_cli xcart xcart
  else
    warn "xcart: faltan ROMs en xcart/roms/ (os.rom, basic.rom, amsdos.rom) — omitido"
    warn "       Copia o enlaza las ROMs y vuelve a ejecutar este script."
  fi

fi

# ── Compilar GUI ──────────────────────────────────────────────────────────────
if [[ "$CLI_ONLY" == false ]]; then

  header "Compilando xDSK Desktop (Tauri)"

  GUI_DIR=""
  if [[ -d "$ROOT/xdsk-desktop" ]]; then
    GUI_DIR="$ROOT/xdsk-desktop"
  elif [[ -d "$ROOT/disc-desktop" ]]; then
    GUI_DIR="$ROOT/disc-desktop"
  else
    warn "GUI: no se encontró carpeta disc-desktop/ ni xdsk-desktop/ — omitido"
  fi

  if [[ -n "$GUI_DIR" ]]; then
    if ! command -v npm &>/dev/null; then
      warn "npm no encontrado — GUI omitido. Instala Node.js: https://nodejs.org"
    else
      info "npm install"
      (cd "$GUI_DIR" && npm install --silent)

      info "npm run tauri build"
      if ! (cd "$GUI_DIR" && npm run tauri build 2>&1); then
        fail "GUI: build fallido"
        ((ERRORS++))
      else
        # macOS: .app bundle
        APP_BUNDLE="$GUI_DIR/src-tauri/target/release/bundle/macos/xDSK Desktop.app"
        if [[ -d "$APP_BUNDLE" ]]; then
          rm -rf "$DIST/xDSK Desktop.app"
          cp -r "$APP_BUNDLE" "$DIST/xDSK Desktop.app"
          success "xDSK Desktop.app → dist/"
        fi

        # macOS: DMG
        DMG_DIR="$GUI_DIR/src-tauri/target/release/bundle/dmg"
        if [[ -d "$DMG_DIR" ]]; then
          for dmg in "$DMG_DIR"/*.dmg; do
            [[ -f "$dmg" ]] || continue
            cp "$dmg" "$DIST/"
            success "$(basename "$dmg") → dist/"
          done
        fi

        # Linux: AppImage / deb
        APPIMAGE_DIR="$GUI_DIR/src-tauri/target/release/bundle/appimage"
        if [[ -d "$APPIMAGE_DIR" ]]; then
          for ai in "$APPIMAGE_DIR"/*.AppImage; do
            [[ -f "$ai" ]] || continue
            cp "$ai" "$DIST/"
            success "$(basename "$ai") → dist/"
          done
        fi

        # Windows: NSIS / MSI
        for win_dir in nsis msi; do
          WIN_DIR="$GUI_DIR/src-tauri/target/release/bundle/$win_dir"
          if [[ -d "$WIN_DIR" ]]; then
            for pkg in "$WIN_DIR"/*; do
              [[ -f "$pkg" ]] || continue
              cp "$pkg" "$DIST/"
              success "$(basename "$pkg") → dist/"
            done
          fi
        done
      fi
    fi
  fi

fi

# ── Resumen ───────────────────────────────────────────────────────────────────
header "Resumen del Build"
echo ""
printf "  %-12s %s\n" "Target:"  "$TARGET"
printf "  %-12s %s\n" "Perfil:"  "$PROFILE"
printf "  %-12s %s\n" "Salida:"  "$DIST"
echo ""

if [[ -d "$DIST" ]]; then
  found=false
  for f in "$DIST"/*; do
    [[ -e "$f" ]] || continue
    found=true
    if [[ -d "$f" ]]; then
      printf "  %-30s %s\n" "$(basename "$f")/" "(bundle)"
    else
      printf "  %-30s %s\n" "$(basename "$f")" "($(du -sh "$f" | cut -f1))"
    fi
  done
  if [[ "$found" == false ]]; then
    echo "  (sin artefactos)"
  fi
fi

echo ""
if [[ "$ERRORS" -eq 0 ]]; then
  success "Build completado sin errores."
else
  fail "$ERRORS componente(s) fallaron."
  exit 1
fi
