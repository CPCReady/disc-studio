# TODO - Disc Image Studio

## Sprint 1: Fundamentos ✅ COMPLETADO

- [x] Setup proyecto Rust + Cargo
- [x] Estructuras DSK format (header, track, sector)
- [x] Parser DSK básico (lectura)
- [x] Comando `list` funcional con salida bonita
- [x] Comando `create` funcional
- [x] Sistema de errores con thiserror
- [x] CLI moderna con clap

## Sprint 2: Operaciones Core ✅ COMPLETADO

### Comando Import ✅
- [x] Archivos ASCII con conversión automática LF→CRLF
- [x] Archivos binarios con header AMSDOS automático
- [x] Soporte load/exec address (`--load`, `--exec`)
- [x] Selector de tipo `--file-type ascii|binary|raw`
- [x] Atributos read-only y system
- [x] Detección de disco lleno
- [x] NbPages calculado correctamente
- [x] Padding 0x00 para ASCII, 0xE5 para binario
- [x] Múltiples archivos
- [x] **PROBADO EN EMULADOR REAL (RetroVirtualMachine)** ✅

### Comando Export ✅
- [x] Buscar archivo en catálogo con wildcards
- [x] Leer bloques y reconstruir archivo
- [x] Truncar al tamaño real (NbPages * 128)
- [x] Opción `--strip-header`
- [x] Múltiples archivos y wildcards

### Comando Remove ✅
- [x] Wildcards
- [x] Confirmación interactiva
- [x] Modo `--force`
- [x] Actualiza DSK correctamente vía block I/O

### AMSDOS Headers ✅
- [x] Validación completa de checksum
- [x] Creación automática para binarios
- [x] Detección de tipo de archivo
- [x] Manejo de direcciones load/exec

## Sprint 3: Viewers ✅ COMPLETADO

### BASIC Viewer ✅
- [x] Parser de tokens BASIC del CPC
- [x] Tabla de tokens completa (256 + prefijos CB/DD/ED/FD)
- [x] Formateo de líneas con número de línea
- [x] Detección de BASIC tokenizado

### Z80 Disassembler ✅
- [x] Tabla de instrucciones Z80 (256 opcodes base)
- [x] Prefijos CB, DD, ED, FD, DDCB, FDCB
- [x] Mostrar direcciones
- [x] Formato legible

### Hex Viewer ✅
- [x] Dump hexadecimal con dirección y ASCII
- [x] Offset configurable

### ASCII Viewer ✅
- [x] Visualización básica
- [x] Manejo de caracteres especiales CPC (CR como newline, Ctrl+Z como EOF)

## Sprint 3b: Bug Crítico Interleaving ✅ RESUELTO (2026-04-25)

- [x] `Catalog::from_dsk` — usar `read_block(0/1)` en vez de offsets físicos
- [x] `Dsk::get_block_bitmap` — usar `read_block(0/1)`
- [x] `Dsk::find_free_dir_entry` — usar `read_block(0/1)`
- [x] `Dsk::write_dir_entry` — usar `read_block` → modificar → `write_block`
- [x] `commands/export.rs` — `export_file` usa offsets físicos
- [x] `commands/remove.rs` — `find_matching_files` y `remove_file` usan offsets físicos

## Sprint 4: Polish & Release ✅ COMPLETADO

### Testing ✅
- [x] Tests unitarios para DSK format (11 tests)
- [x] Tests unitarios para AMSDOS (7 tests)
- [x] Tests de integración para módulos (28 tests lib + 28 tests bin)
- [x] Tests de integración end-to-end para comandos CLI — 25 tests con assert_cmd
- [ ] Fixtures con DSK reales de ejemplo en tests/

### Documentación ✅
- [x] README.md completo y actualizado
- [x] Ejemplos de uso detallados
- [x] Tabla de compatibilidad DSK
- [x] Man pages — subcomando oculto `disc mangen [DIR]` genera disc.1 y disc-*.1
- [ ] Comparación con iDSK

### CI/CD ✅
- [x] GitHub Actions: cargo test + clippy en push (`.github/workflows/ci.yml`)
- [x] GitHub Actions: build release binaries (Linux / macOS / Windows)
- [x] GitHub Actions: publicar release automático en tag

### Distribución ✅
- [x] Binarios para Linux (x86_64, ARM64) — CI
- [x] Binarios para macOS (Intel, Apple Silicon) — CI
- [x] Binarios para Windows (x86_64) — CI
- [x] Homebrew formula — `packaging/homebrew/disc.rb`
- [x] AUR package (Arch Linux) — `packaging/aur/PKGBUILD`
- [x] Chocolatey package (Windows) — `packaging/chocolatey/`
- [x] Script build local — `build-release.sh` (nativo + cross)

### Features Adicionales 📋
- [x] Shell completions (bash, zsh, fish) — `disc completions <shell>`
- [x] Verbose detallado con `--verbose` + `env_logger`
- [ ] Dry-run mode con `--dry-run`

## Sprint 5: Nuevos Comandos ✅ COMPLETADO (2026-04-26)

### `disc check` ✅
- [x] Validar magic del header DSK
- [x] Verificar geometría (tracks, heads)
- [x] Comprobar entradas de directorio
- [x] Verificar cabeceras AMSDOS por archivo
- [x] Detectar conflictos en asignación de bloques
- [x] Exit code 1 si hay errores (apto para CI)
- [x] Log de depuración con `--verbose`

### `disc info` ✅
- [x] Formato y geometría del disco
- [x] Occupación del directorio (entradas usadas / 64)
- [x] Mapa de bloques (total, usados %, libres %)
- [x] Tabla de archivos con tipo, tamaño y atributos

### `disc diff` ✅
- [x] Clasificar archivos: solo en DSK1, solo en DSK2, modificados, idénticos
- [x] Comparación byte a byte del contenido
- [x] Resumen: N added, N removed, N modified, N identical
- [x] Exit code 0 siempre (diferencias no son errores)

### Documentación ✅
- [x] `README.md` reescrito completamente (inglés)
- [x] `README.es.md` creado (español)

### Tests ✅
- [x] 11 tests nuevos para check / info / diff (110 tests totales, 0 fallos)

## Sprint 6: disc copy ✅ COMPLETADO (2026-04-26)

### `disc copy` / `cp` ✅
- [x] Copiar todos los archivos entre dos imágenes DSK (`disc copy src.dsk dst.dsk`)
- [x] Copiar un archivo concreto por nombre exacto
- [x] Comodín por extensión `*.EXT` (p. ej. `*.BAS`)
- [x] Comodín por prefijo `NAME*` (p. ej. `LEVEL*`)
- [x] Comodín total `*`
- [x] Opción `--force` para sobreescribir archivos existentes
- [x] Preservar cabeceras AMSDOS, direcciones load/exec y bits de atributo
- [x] Alias `cp`
- [x] 8 tests de integración nuevos (119 tests totales, 0 fallos)

### Documentación ✅
- [x] `AGENT.md` creado con contexto completo del proyecto
- [x] `README.md` actualizado — sección `copy` añadida
- [x] `README.es.md` actualizado — sección `copy` añadida
- [x] `TODO.md` actualizado

## Backlog: Features Futuras 💡

### Extended DSK Support
- [ ] Leer formato Extended DSK
- [ ] Escribir formato Extended DSK
- [ ] Conversión DATA ↔ Extended

### Advanced Commands
- [ ] `disc defrag` — desfragmentar bloques libres

### Misc
- [ ] Soporte imágenes comprimidas (.dsk.gz)
- [ ] Modo interactivo (REPL)
- [ ] Progress bar para archivos grandes (indicatif ya incluido como dependencia)
- [ ] Soporte para archivos protegidos (AMSDOS type 1/3)

## Bugs Conocidos 🐛

Ninguno conocido. Los 119 tests pasan con 0 warnings.

## Notas de Desarrollo

### Convenciones de Código
- Todos los archivos deben tener header MIT con copyright: `// MIT License - Copyright (c) 2026 Destroyer`
- Usar `cargo fmt` antes de commit
- Usar `cargo clippy` para linting (0 warnings exigido)
- Tests en `tests/integration/`
- Documentación con `///` para APIs públicas

### Arquitectura
- `anyhow` para errores en comandos (user-facing)
- `thiserror` para errores en librería (internal)
- Toda lectura/escritura del directorio CP/M **debe** usar `read_block` / `write_block` (nunca offsets físicos directos) para manejar correctamente el sector interleaving del CPC

### Testing
```bash
cargo test    # 119 tests
cargo clippy  # 0 warnings
```

---

**Última actualización**: 2026-04-26
