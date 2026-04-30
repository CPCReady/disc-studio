# xcdt — Herramienta de cintas CDT/TZX para Amstrad CPC

**xcdt** crea e inspecciona imágenes de cassette **CDT/TZX** para Amstrad CPC. Escrita en Rust, genera ficheros TZX v1.10 compatibles con todos los emuladores principales.

---

## Índice

- [Instalación](#instalación)
- [Inicio rápido](#inicio-rápido)
- [Comandos](#comandos)
  - [new — Crear CDT nuevo](#new--crear-cdt-nuevo)
  - [save — Añadir fichero a CDT](#save--añadir-fichero-a-cdt)
  - [cat — Listar ficheros AMSDOS](#cat--listar-ficheros-amsdos)
  - [list — Listar bloques TZX](#list--listar-bloques-tzx)
  - [check — Verificar integridad](#check--verificar-integridad-crc)
  - [info — Metadatos del CDT](#info--metadatos-del-cdt)
  - [extract — Extraer payload de bloque](#extract--extraer-payload-de-bloque)
  - [rename — Renombrar fichero en cinta](#rename--renombrar-fichero-en-cinta)
  - [convert — Convertir tipo de bloque](#convert--convertir-tipo-de-bloque)
- [Referencia de opciones de escritura](#referencia-de-opciones-de-escritura)
- [Tipos de bloque TZX](#tipos-de-bloque-tzx)
- [Flujo típico](#flujo-típico)
- [Compatibilidad](#compatibilidad)

---

## Instalación

### Compilar desde fuente

```bash
git clone https://github.com/CPCReady/Disc-Image-Studio.git
cd Disc-Image-Studio/xcdt
cargo build --release --target aarch64-apple-darwin

# Binario en:
./target/aarch64-apple-darwin/release/xcdt
```

### Usar el script de build del proyecto

```bash
# Desde la raíz del proyecto:
./build.sh --cli-only
# → dist/xcdt
```

### Instalar globalmente

```bash
cp dist/xcdt /usr/local/bin/xcdt
xcdt --version
```

---

## Inicio rápido

```bash
# Cinta de un solo fichero BASIC
xcdt new loader.bas loader.cdt -F 0 -r LOADER

# Cinta con binario en &4000
xcdt new juego.bin juego.cdt -L 0x4000 -X 0x4000

# Cinta multi-fichero
xcdt new  loader.bin cinta.cdt -r LOADER -L 0x1000 -X 0x1000
xcdt save juego.bin  cinta.cdt -r GAME   -L 0x4000 -X 0x4000
xcdt save musica.bin cinta.cdt -r MUSIC  -L 0x8000 -X 0x8000

# Inspeccionar
xcdt cat   cinta.cdt
xcdt list  cinta.cdt
xcdt check cinta.cdt
xcdt info  cinta.cdt
```

---

## Comandos

### `new` — Crear CDT nuevo

Crea un **nuevo fichero CDT** (sobreescribe si ya existe) a partir de un fichero binario o AMSDOS.

```bash
xcdt new <ENTRADA> <SALIDA.CDT> [OPCIONES]
```

**Opciones:**

| Opción | Por defecto | Descripción |
|--------|-------------|-------------|
| `-b, --baud <N>` | `2000` | Baud rate (rango: 1000–6000) |
| `-t, --block-type <0\|1\|2>` | `1` | Tipo de bloque: `0`=Pure Data, `1`=Turbo, `2`=Standard |
| `-m, --method <0\|1\|2>` | `0` | Método: `0`=bloques CPC, `1`=headerless, `2`=Spectrum |
| `-r, --rename <NOMBRE>` | — | Nombre del fichero en cinta (máx 16 chars) |
| `-L, --load <ADDR>` | `0x1000` | Dirección de carga (`&XXXX`, `0xXXXX` o decimal) |
| `-X, --exec <ADDR>` | `0x1000` | Dirección de ejecución |
| `-F, --file-type <0\|1\|2>` | `2` | Tipo AMSDOS: `0`=BASIC, `1`=Protected, `2`=Binary |
| `-p, --pause-ms <MS>` | `3000` | Pausa inicial antes del primer bloque (ms) |
| `-P, --buggy-emu` | — | Añade 1 ms de pre-pausa para emuladores con bugs |

**Ejemplos:**

```bash
# BASIC con nombre LOADER en cinta
xcdt new loader.bas loader.cdt -F 0 -r LOADER

# Binario en dirección &4000 con autostart
xcdt new juego.bin juego.cdt -L 0x4000 -X 0x4000

# Turbo a 3000 baud
xcdt new juego.bin juego.cdt -b 3000 -t 1

# Pure Data (bloques compactos sin cabecera de sincronización)
xcdt new juego.bin juego.cdt -t 0

# Standard Speed (compatible con hardware real más antiguo)
xcdt new juego.bin juego.cdt -t 2

# Sin cabecera AMSDOS (headerless)
xcdt new juego.bin juego.cdt -m 1

# Formato Spectrum (bloque Standard Speed, sync=0xFF)
xcdt new juego.bin juego.cdt -m 2
```

> Los nombres en cinta se convierten automáticamente a **mayúsculas**.

---

### `save` — Añadir fichero a CDT

**Añade** (append) un binario a un CDT existente. Si el CDT no existe, lo crea.  
Acepta las mismas opciones que `new` pero **sin pausa inicial**.

```bash
xcdt save <ENTRADA> <SALIDA.CDT> [OPCIONES]
```

**Cinta multi-fichero:**

```bash
xcdt new  loader.bin  cinta.cdt -r LOADER -L 0x1000 -X 0x1000
xcdt save juego.bin   cinta.cdt -r GAME   -L 0x4000 -X 0x4000
xcdt save musica.bin  cinta.cdt -r MUSIC  -L 0x8000 -X 0x8000
xcdt save efectos.bin cinta.cdt -r SFX    -L 0xA000 -X 0xA000
```

---

### `cat` — Listar ficheros AMSDOS

Muestra los ficheros detectados en bloques de cabecera AMSDOS de la cinta.

```bash
xcdt cat <FICHERO.CDT> [--json]
```

**Salida texto:**

```
#    Name              Type      Load@    Exec@    Size     Block  First/Last
---------------------------------------------------------------------------
1    LOADER            BINARY    &1000   &1000      33B      1  FIRST LAST
2    GAME              BINARY    &4000   &4000    8192B      3  FIRST LAST
```

**Salida JSON (`--json`):**

```json
{
  "file": "cinta.cdt",
  "version": "1.10",
  "count": 2,
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

### `list` — Listar bloques TZX

Muestra la estructura completa de bloques TZX/CDT (incluyendo pausas, tipos de bloque, tamaños).

```bash
xcdt list <FICHERO.CDT> [--json]
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

### `check` — Verificar integridad CRC

Verifica el CRC-16 de cada chunk de 256B (bloques Turbo) o el checksum XOR (bloques Standard). Sale con código `1` si hay errores.

```bash
xcdt check <FICHERO.CDT> [--json]
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

### `info` — Metadatos del CDT

Muestra estadísticas globales: número de bloques por tipo, bytes de datos totales, duración estimada.

```bash
xcdt info <FICHERO.CDT> [--json]
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

### `extract` — Extraer payload de bloque

Extrae los bytes crudos del payload de un bloque específico a un fichero. El índice es **1-based** (como muestra `xcdt list`).

```bash
xcdt extract <FICHERO.CDT> <INDICE_BLOQUE> <SALIDA>
```

**Ejemplos:**

```bash
# Extraer la cabecera AMSDOS del bloque 2
xcdt extract loader.cdt 2 cabecera.bin

# Extraer los datos del bloque 3
xcdt extract loader.cdt 3 datos.bin
```

> Los bloques PAUSE no tienen payload y producen error.

---

### `rename` — Renombrar fichero en cinta

Parchea el nombre AMSDOS **en el bloque de cabecera** directamente, sin re-encodificar el audio. Recalcula el CRC/checksum del bloque afectado.

```bash
xcdt rename <ENTRADA> <NOMBRE_VIEJO> <NOMBRE_NUEVO> [-o SALIDA]
```

| Opción | Descripción |
|--------|-------------|
| `-o, --output <FICHERO>` | Escribe en un nuevo fichero (por defecto: modifica en lugar) |

**Ejemplos:**

```bash
# Renombrar en el mismo fichero
xcdt rename cinta.cdt LOADER MYPROG

# Renombrar y guardar en otro fichero
xcdt rename cinta.cdt LOADER MYPROG -o cinta_v2.cdt
```

> El nombre se convierte siempre a **mayúsculas**. La búsqueda es insensible a mayúsculas.

---

### `convert` — Convertir tipo de bloque

Recodifica bloques entre tipos de encoding (Turbo ↔ Standard ↔ Pure Data). Decodifica el bloque original y lo re-encodifica como el tipo destino.

```bash
xcdt convert <ENTRADA> <SALIDA> --to <0|1|2> [--block <IDX>] [-b <BAUD>]
```

| Opción | Descripción |
|--------|-------------|
| `--to <0\|1\|2>` | Tipo destino: `0`=Pure Data, `1`=Turbo, `2`=Standard **(obligatorio)** |
| `--block <N>` | Convertir solo el bloque N (1-based); por defecto: todos |
| `-b, --baud <N>` | Baud rate para re-encodificación (1000-6000); hereda del origen si no se indica |

**Ejemplos:**

```bash
# Todos los bloques Turbo → Standard Speed
xcdt convert juego.cdt juego_std.cdt --to 2

# Solo el bloque 2 → Turbo a 3000 baud
xcdt convert juego.cdt juego_rapido.cdt --to 1 --block 2 -b 3000

# Todos → Pure Data (máxima compacidad)
xcdt convert juego.cdt juego_pd.cdt --to 0
```

> Los bloques PAUSE y Unknown se dejan sin modificar.

---

## Referencia de opciones de escritura

Estas opciones están disponibles en `new` y `save`:

| Tipo de bloque (`-t`) | ID TZX | Descripción |
|-----------------------|--------|-------------|
| `0` — Pure Data | `0x14` | Bitstream puro, máxima compacidad |
| `1` — Turbo (defecto) | `0x11` | Bloques turbo con sincronización |
| `2` — Standard | `0x10` | Velocidad estándar, máxima compatibilidad |

| Método (`-m`) | Descripción |
|---------------|-------------|
| `0` — blocks CPC (defecto) | 64B de cabecera de cinta + bloques de datos de 2 KB |
| `1` — headerless | Bloque continuo único, sin cabecera de cinta |
| `2` — Spectrum | Bloque Standard Speed con byte de sync `0xFF` |

---

## Tipos de bloque TZX

| ID | Nombre | Descripción |
|----|--------|-------------|
| `0x10` | Standard Speed | Compatible con hardware original |
| `0x11` | Turbo Loading | Velocidad configurable (hasta 6000 baud) |
| `0x14` | Pure Data | Bitstream puro sin cabecera de piloto |
| `0x20` | Pause | Silencio entre bloques |

---

## Flujo típico

```bash
# 1. Crear cinta multi-fichero
xcdt new  cargador.bin cinta.cdt -r LOADER -L 0x1000 -X 0x1000 -F 0
xcdt save juego.bin    cinta.cdt -r GAME   -L 0x4000 -X 0x4000

# 2. Verificar que todo está correcto
xcdt cat   cinta.cdt
xcdt check cinta.cdt

# 3. Ver estadísticas
xcdt info cinta.cdt

# 4. Renombrar un fichero si hace falta
xcdt rename cinta.cdt LOADER MENU

# 5. Convertir a Standard Speed para hardware antiguo
xcdt convert cinta.cdt cinta_std.cdt --to 2
```

---

## Compatibilidad

| Emulador / Hardware | Estado |
|---------------------|--------|
| Retro Virtual Machine 2 | ✅ Probado |
| WinAPE | ✅ CDT/TZX compatible |
| Arnold | ✅ TZX v1.10 compatible |
| Hardware CPC real (via 3rd party) | ✅ Probado |

---

## Licencia

GPL-2.0 — Copyright (c) Destroyer
