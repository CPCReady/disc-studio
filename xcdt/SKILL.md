---
name: xcdt
description: Crear y gestionar imágenes .cdt de cassette tape para Amstrad CPC con xcdt. Usar cuando el usuario pida crear cintas CDT/TZX, añadir ficheros a una cinta, inspeccionar bloques TZX, listar ficheros AMSDOS en cinta, validar integridad CRC, extraer payloads de bloque, renombrar ficheros en cinta o convertir bloques entre tipos de encoding (turbo/standard/pure data). También para control de baud rates (1000-6000), métodos de codificación (blocks/headerless/spectrum) y salida JSON. Si el binario xcdt no está disponible, indicar al usuario que compile el proyecto en xcdt/ con `cargo build --release`.
---

# xcdt

Herramienta Rust para crear e inspeccionar imágenes de cassette **CDT/TZX** para Amstrad CPC.

**Binario:** `xcdt` (compilar en `xcdt/` con `cargo build --release --target aarch64-apple-darwin`)  
**Formato:** TZX v1.10 con extensión `.cdt` (compatible con emuladores Amstrad)

---

## Verificar disponibilidad

```bash
xcdt --version
# Si falla: cd xcdt && cargo build --release --target aarch64-apple-darwin
```

---

## Comandos disponibles

### `xcdt new` — Crear CDT desde un binario

Crea un **nuevo fichero CDT** (sobrescribe si existe) a partir de un fichero binario o AMSDOS.

```bash
xcdt new <INPUT> <OUTPUT> [OPCIONES]
```

| Opción | Por defecto | Descripción |
|--------|-------------|-------------|
| `-b, --baud <N>` | `2000` | Baud rate (rango: 1000–6000) |
| `-t, --block-type <0\|1\|2>` | `1` | Tipo de bloque TZX: 0=Pure Data, 1=Turbo, 2=Standard |
| `-m, --method <0\|1\|2>` | `0` | Método: 0=blocks CPC, 1=headerless, 2=Spectrum |
| `-r, --rename <NOMBRE>` | — | Nombre del fichero en cinta (máx 16 chars, se fuerza mayúsculas) |
| `-L, --load <ADDR>` | `0x1000` | Dirección de carga (`&XXXX`, `0xXXXX` o decimal) |
| `-X, --exec <ADDR>` | `0x1000` | Dirección de ejecución |
| `-F, --file-type <0\|1\|2>` | `2` | Tipo AMSDOS: 0=BASIC, 1=Protected, 2=Binary |
| `-p, --pause-ms <MS>` | `3000` | Pausa inicial antes del primer bloque (ms) |
| `-P, --buggy-emu` | — | Añade 1 ms de pre-pausa para emuladores con bugs |

**Ejemplos:**
```bash
# BASIC con nombre LOADER
xcdt new loader.bas loader.cdt -F 0 -r LOADER

# Binario en dirección &4000 con autostart en &4000
xcdt new game.bin game.cdt -L &4000 -X &4000

# Turbo a 3000 baud
xcdt new game.bin game.cdt -b 3000 -t 1

# Pure Data a 2000 baud
xcdt new game.bin game.cdt -t 0

# Bloque Standard Speed
xcdt new game.bin game.cdt -t 2

# Sin cabecera AMSDOS (headerless)
xcdt new game.bin game.cdt -m 1

# Formato Spectrum (bloque Standard Speed, sync=0xFF)
xcdt new game.bin game.cdt -m 2
```

---

### `xcdt save` — Añadir fichero a CDT existente

**Añade** (append) un binario a un CDT ya existente. Si el CDT no existe, lo crea.  
Las mismas opciones que `new` pero **sin pausa inicial** (la pausa entre ficheros es estándar).

```bash
xcdt save <INPUT> <OUTPUT> [OPCIONES]
```

**Ejemplo — cinta multi-fichero:**
```bash
xcdt new  loader.bin cinta.cdt -r LOADER -L &1000 -X &1000
xcdt save game.bin   cinta.cdt -r GAME   -L &4000 -X &4000
xcdt save music.bin  cinta.cdt -r MUSIC  -L &8000 -X &8000
```

---

### `xcdt cat` — Listar ficheros AMSDOS en cinta

Muestra los ficheros detectados en bloques de cabecera AMSDOS de la cinta.

```bash
xcdt cat <FILE.CDT> [--json]
```

**Salida texto:**
```
#    Name              Type      Load@    Exec@    Size     Block  First/Last
---------------------------------------------------------------------------
1    LOADER            BINARY    &1000   &1000      33B      1  FIRST LAST
```

**Salida JSON (`--json`):**
```json
{
  "file": "loader.cdt",
  "version": "1.10",
  "count": 1,
  "files": [
    {
      "index": 1,
      "name": "LOADER",
      "type": "BINARY",
      "load": "0x1000",
      "exec": "0x1000",
      "size": 33,
      "block": 1,
      "first": true,
      "last": true
    }
  ]
}
```

---

### `xcdt list` — Listar todos los bloques TZX

Muestra la estructura completa de bloques TZX/CDT (incluyendo pausas).

```bash
xcdt list <FILE.CDT> [--json]
```

**Salida texto:**
```
#     Type        ID      DataSize     Baud~  Pause(ms)
-------------------------------------------------------
1     PAUSE       0x20           0         -      3000
2     TURBO       0x11         263      2020        10
3     TURBO       0x11         263      2020      2500
```

**Salida JSON (`--json`):**
```json
{
  "file": "loader.cdt",
  "version": "1.10",
  "block_count": 3,
  "blocks": [
    { "index": 1, "type": "PAUSE",  "id": "0x20", "data_size": 0,   "baud": null, "pause_ms": 3000 },
    { "index": 2, "type": "TURBO",  "id": "0x11", "data_size": 263, "baud": 2020, "pause_ms": 10   },
    { "index": 3, "type": "TURBO",  "id": "0x11", "data_size": 263, "baud": 2020, "pause_ms": 2500 }
  ]
}
```

---

### `xcdt check` — Validar integridad CRC

Verifica el CRC-16 de cada chunk de 256B (bloques Turbo) o el checksum XOR (bloques Standard). Devuelve código de salida 1 si hay errores.

```bash
xcdt check <FILE.CDT> [--json]
```

**Salida texto:**
```
  Block   1 [PAUSE]: 3000 ms
  Block   2 [TURBO/HDR ]: OK (1 chunks, 256 bytes, pause 10ms)
  Block   3 [TURBO/DATA]: OK (1 chunks, 256 bytes, pause 2500ms)

Result: OK (3 blocks verified)
```

**Salida JSON (`--json`):**
```json
{
  "file": "loader.cdt",
  "version": "1.10",
  "ok": true,
  "errors": 0,
  "blocks_verified": 3,
  "blocks": [
    { "index": 1, "type": "PAUSE",      "status": "ok", "detail": "3000 ms" },
    { "index": 2, "type": "TURBO/HDR",  "status": "ok", "detail": "1 chunks, 256 bytes, pause 10ms" },
    { "index": 3, "type": "TURBO/DATA", "status": "ok", "detail": "1 chunks, 256 bytes, pause 2500ms" }
  ]
}
```

---

### `xcdt info` — Metadatos globales del fichero

Muestra estadísticas de bloques, bytes de datos, duración estimada.

```bash
xcdt info <FILE.CDT> [--json]
```

**Salida JSON (`--json`):**
```json
{
  "file": "loader.cdt",
  "size_bytes": 577,
  "version": "1.10",
  "block_count": 3,
  "pause_blocks": 1,
  "turbo_blocks": 2,
  "standard_blocks": 0,
  "pure_data_blocks": 0,
  "unknown_blocks": 0,
  "total_data_bytes": 526,
  "total_pause_ms": 5510,
  "estimated_duration_secs": 6
}
```

---

### `xcdt extract` — Extraer payload de un bloque

Extrae los bytes crudos del payload de un bloque específico a un fichero.  
El índice es **1-based** (como muestra `xcdt list`).

```bash
xcdt extract <FILE.CDT> <BLOCK_INDEX> <OUTPUT>
```

**Ejemplo:**
```bash
# Extraer el bloque 2 (cabecera AMSDOS)
xcdt extract loader.cdt 2 header.bin

# Extraer el bloque 3 (datos)
xcdt extract loader.cdt 3 data.bin
```

Los bloques PAUSE no tienen payload y dan error.

---

### `xcdt rename` — Renombrar fichero en cinta

Parchea el nombre AMSDOS **en el bloque de cabecera** directamente, sin re-encodificar el audio. Recalcula el CRC/checksum del bloque afectado.

```bash
xcdt rename <INPUT> <OLD_NAME> <NEW_NAME> [-o OUTPUT]
```

| Opción | Descripción |
|--------|-------------|
| `-o, --output <FILE>` | Escribe en un nuevo fichero (por defecto: modifica en lugar) |

**Ejemplos:**
```bash
# Renombrar en el mismo fichero
xcdt rename cinta.cdt LOADER MYPROG

# Renombrar y guardar en otro fichero
xcdt rename cinta.cdt LOADER MYPROG -o cinta_v2.cdt
```

El nombre se convierte siempre a mayúsculas. La búsqueda es case-insensitive.

---

### `xcdt convert` — Convertir tipo de bloque

Recodifica bloques entre tipos de encoding (Turbo ↔ Standard ↔ Pure Data).  
Decodifica el bloque original y lo re-encodifica como el tipo destino.

```bash
xcdt convert <INPUT> <OUTPUT> --to <0|1|2> [--block <IDX>] [-b <BAUD>]
```

| Opción | Descripción |
|--------|-------------|
| `--to <0\|1\|2>` | Tipo destino: 0=Pure Data, 1=Turbo, 2=Standard **(obligatorio)** |
| `--block <N>` | Convertir solo el bloque N (1-based); por defecto: todos |
| `-b, --baud <N>` | Baud rate para re-encodificación (1000-6000); por defecto: hereda del bloque origen o 2000 |

**Ejemplos:**
```bash
# Todos los bloques Turbo → Standard Speed
xcdt convert juego.cdt juego_std.cdt --to 2

# Solo el bloque 2 → Turbo a 3000 baud
xcdt convert juego.cdt juego_fast.cdt --to 1 --block 2 -b 3000

# Todos → Pure Data
xcdt convert juego.cdt juego_pd.cdt --to 0
```

Los bloques PAUSE, PureData y Unknown se dejan sin modificar (solo se convierten Turbo y Standard).

---

## Tipos de bloque TZX

| ID | Nombre | Descripción |
|----|--------|-------------|
| `0x10` | STANDARD | Standard Speed Data Block |
| `0x11` | TURBO | Turbo Loading Data Block (CPC nativo) |
| `0x14` | PURE DATA | Pure Data Block (bitstream) |
| `0x20` | PAUSE | Pausa en ms |

---

## Métodos de codificación (`-m`)

| Valor | Nombre | Descripción |
|-------|--------|-------------|
| `0` | blocks | CPC estándar: cabecera de 64B + bloques de datos de 2 KB (por defecto) |
| `1` | headerless | Un único bloque continuo sin cabecera AMSDOS |
| `2` | spectrum | Bloque Standard Speed compatible ZX Spectrum (sync=0xFF) |

---

## Detección automática de cabecera AMSDOS

Cuando el fichero de entrada tiene 128 bytes de cabecera AMSDOS válida (checksum correcto en bytes 67-68), `xcdt new`/`save` extrae automáticamente nombre, tipo, dirección de carga/ejecución y tamaño. Las opciones `-r/-L/-X/-F` sobreescriben los valores detectados.

---

## Flujo típico de automatización con JSON

```bash
# Crear cinta
xcdt new game.bin game.cdt -r GAME -L &4000 -X &4000

# Verificar
xcdt check --json game.cdt | jq '.ok'

# Listar ficheros
xcdt cat --json game.cdt | jq '.files[].name'

# Info completa
xcdt info --json game.cdt
```
