# xdsk — Herramienta de imágenes DSK para Amstrad CPC

**xdsk** es una herramienta de línea de comandos para crear, inspeccionar y manipular imágenes de disco **DSK** del Amstrad CPC. Escrita en Rust, es rápida, multiplataforma y reemplaza a la legacy `iDSK`.

---

## Índice

- [Instalación](#instalación)
- [Inicio rápido](#inicio-rápido)
- [Opciones globales](#opciones-globales)
- [Comandos](#comandos)
  - [create — Crear disco](#create--new)
  - [list — Listar ficheros](#list--ls)
  - [import — Importar ficheros](#import--add)
  - [export — Exportar ficheros](#export--get)
  - [remove — Borrar ficheros](#remove--rm)
  - [copy — Copiar entre discos](#copy--cp)
  - [view — Ver contenido](#view)
  - [check — Verificar integridad](#check)
  - [info — Información del disco](#info)
  - [diff — Comparar dos discos](#diff)
- [Tipos de fichero y cabeceras AMSDOS](#tipos-de-fichero-y-cabeceras-amsdos)
- [Formato DSK](#formato-dsk)
- [Completado de shell](#completado-de-shell)
- [Variables de entorno](#variables-de-entorno)
- [Códigos de salida](#códigos-de-salida)
- [Compatibilidad](#compatibilidad)

---

## Instalación

### Compilar desde fuente

```bash
git clone https://github.com/CPCReady/Disc-Image-Studio.git
cd Disc-Image-Studio

# Compilar para macOS ARM (M1/M2/M3)
cd xdsk
cargo build --release --target aarch64-apple-darwin

# El binario queda en:
./target/aarch64-apple-darwin/release/xdsk
```

### Usar el script de build del proyecto

Desde la raíz del proyecto:

```bash
./build.sh --cli-only
# → El binario xdsk queda en dist/xdsk
```

### Instalar globalmente

```bash
# Tras compilar, copia el binario a cualquier carpeta en tu $PATH:
cp dist/xdsk /usr/local/bin/xdsk
xdsk --version
```

---

## Inicio rápido

```bash
# 1. Crear un disco DATA vacío de 40 pistas
xdsk create juego.dsk

# 2. Importar ficheros al disco
xdsk import juego.dsk cargador.bas
xdsk import juego.dsk sprites.bin --file-type binary --load 0x4000 --exec 0x4000

# 3. Listar el contenido
xdsk list juego.dsk

# 4. Ver un fichero sin extraerlo
xdsk view juego.dsk CARGADOR.BAS --format basic

# 5. Exportar ficheros
xdsk export juego.dsk "*.BAS" --output backup/

# 6. Verificar integridad
xdsk check juego.dsk

# 7. Estadísticas detalladas
xdsk info juego.dsk

# 8. Comparar dos imágenes de disco
xdsk diff juego.dsk juego_backup.dsk

# 9. Copiar ficheros entre imágenes
xdsk copy origen.dsk destino.dsk "*.BAS"

# 10. Borrar un fichero del disco
xdsk remove juego.dsk VIEJO.BIN --force
```

---

## Opciones globales

Disponibles en **todos** los subcomandos:

| Opción | Abreviatura | Descripción |
|--------|-------------|-------------|
| `--verbose` | `-v` | Activar logging de depuración |
| `--no-color` | — | Desactivar colores (útil en scripts/pipes) |
| `--help` | `-h` | Mostrar ayuda del comando |
| `--version` | `-V` | Mostrar número de versión |

```bash
xdsk --verbose list juego.dsk
xdsk --no-color list juego.dsk | grep .BAS
xdsk --version
```

---

## Comandos

### `create` / `new`

Crea una nueva imagen DSK vacía formateada como disco DATA estándar del Amstrad CPC.

```
xdsk create <IMAGEN> [OPCIONES]
xdsk new    <IMAGEN> [OPCIONES]    # alias
```

| Opción | Descripción | Por defecto |
|--------|-------------|-------------|
| `--tracks <N>` | Número de pistas | `40` |
| `--sectors <N>` | Sectores por pista | `9` |
| `-f, --force` | Sobreescribir si ya existe | off |

**Ejemplos:**

```bash
# Disco DATA estándar de 40 pistas (178 KB)
xdsk create juego.dsk

# Disco de 80 pistas (para imágenes extendidas)
xdsk create extendido.dsk --tracks 80

# Sobreescribir un disco existente
xdsk create juego.dsk --force
```

> El disco usa el interleaving estándar del CPC (`0xC1–0xC9`) y es compatible
> con todos los emuladores principales.

---

### `list` / `ls`

Lista los ficheros almacenados en una imagen DSK.

```
xdsk list <IMAGEN> [OPCIONES]
xdsk ls   <IMAGEN> [OPCIONES]    # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--format <FORMATO>` | `-f` | `table`, `json`, `csv`, `simple` | `table` |

**Ejemplos:**

```bash
# Tabla legible (por defecto)
xdsk list juego.dsk

# JSON — ideal para scripts
xdsk list juego.dsk --format json

# CSV — para filtrar con herramientas de texto
xdsk list juego.dsk --format csv

# Lista simple — un nombre por línea
xdsk list juego.dsk --format simple

# Filtrar con herramientas del sistema
xdsk list juego.dsk --format simple | grep "\.BAS"
```

**Salida en tabla:**

```
DSK Image: juego.dsk
Tracks: 40 | Sectors/Track: 9 | Format: DATA
Used: 12 KB / 178 KB (6.7%)

┌──────────────┬──────────┬──────┬────────┐
│ Name         │ Type     │ Size │ Attrs  │
├──────────────┼──────────┼──────┼────────┤
│ LOADER.BAS   │ BASIC    │ 2 KB │        │
│ SPRITES.BIN  │ BINARY   │ 8 KB │ R      │
│ LEVEL1.DAT   │ BINARY   │ 2 KB │        │
└──────────────┴──────────┴──────┴────────┘
3 files, 12 KB total, 166 KB free
```

**Columna Attrs:**

| Símbolo | Significado |
|---------|-------------|
| `R` | Solo lectura |
| `S` | Fichero sistema (oculto) |

**Estructura JSON:**

```json
{
  "files": [
    { "name": "LOADER.BAS", "type": "BASIC", "size": 2048, "read_only": false, "system": false }
  ],
  "total_size": 2048,
  "free_space": 180224
}
```

---

### `import` / `add`

Importa uno o más ficheros desde el sistema de archivos host a una imagen DSK.

```
xdsk import <IMAGEN> <FICHEROS>... [OPCIONES]
xdsk add    <IMAGEN> <FICHEROS>... [OPCIONES]    # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--file-type <TIPO>` | `-t` | `ascii`, `binary`, `raw` | auto-detect |
| `--load <ADDR>` | `-c` | Dirección de carga en hex (`0x4000`) | — |
| `--exec <ADDR>` | `-e` | Dirección de ejecución en hex | — |
| `--user <N>` | `-u` | Número de usuario CP/M (0–15) | `0` |
| `--read-only` | `-o` | Marcar como solo lectura | off |
| `--system` | `-s` | Marcar como fichero sistema (oculto) | off |
| `--force` | `-f` | Sobreescribir si ya existe en el disco | off |

**Detección automática de tipo** (cuando no se especifica `--file-type`):

| Condición | Tipo detectado |
|-----------|----------------|
| El fichero tiene cabecera AMSDOS válida | `binary` |
| En caso contrario | `ascii` |

**Comportamiento por tipo:**

| Tipo | Qué hace xdsk |
|------|---------------|
| `ascii` | Elimina cualquier cabecera AMSDOS; convierte `LF` → `CR+LF` |
| `binary` | Añade cabecera AMSDOS con dirección de carga/ejecución si no la tiene |
| `raw` | Almacena el fichero exactamente como está, sin cabecera |

**Ejemplos:**

```bash
# Fuente BASIC ASCII — auto-detectado
xdsk import juego.dsk cargador.bas

# Binario con dirección de carga y ejecución
xdsk import juego.dsk codigo.bin --file-type binary --load 0x4000 --exec 0x4000

# Fichero raw — almacenado tal cual
xdsk import juego.dsk paleta.dat --file-type raw

# Múltiples ficheros de una vez
xdsk import juego.dsk src/*.bas src/*.bin

# Importar en área de usuario CP/M 2
xdsk import juego.dsk privado.bin --user 2

# Sobreescribir un fichero existente
xdsk import juego.dsk cargador.bas --force
```

> Si un fichero ya existe en el disco y no se usa `--force`, el error se muestra
> en stderr y xdsk continúa con los demás ficheros (código de salida 0).

---

### `export` / `get`

Extrae uno o más ficheros de una imagen DSK al sistema de archivos host.

```
xdsk export <IMAGEN> <FICHEROS>... [OPCIONES]
xdsk get    <IMAGEN> <FICHEROS>... [OPCIONES]    # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--output <DIR>` | `-o` | Directorio de destino (se crea si no existe) | directorio actual |
| `--strip-header` | — | Eliminar cabecera AMSDOS del fichero exportado | off |

Los nombres de fichero son **insensibles a mayúsculas**. Se soporta el comodín `*` al final.

**Ejemplos:**

```bash
# Exportar un fichero al directorio actual
xdsk export juego.dsk LOADER.BAS

# Exportar a un directorio específico
xdsk export juego.dsk LOADER.BAS --output backup/

# Comodín — exportar todos los ficheros BASIC
xdsk export juego.dsk "*.BAS" --output backup/

# Exportar sin cabecera AMSDOS (payload puro)
xdsk export juego.dsk SPRITES.BIN --strip-header --output raw/

# Exportar todos los ficheros
xdsk export juego.dsk "*" --output backup_completo/
```

---

### `remove` / `rm`

Elimina uno o más ficheros de una imagen DSK (marca las entradas del directorio como borradas).

```
xdsk remove <IMAGEN> <FICHEROS>... [OPCIONES]
xdsk rm     <IMAGEN> <FICHEROS>... [OPCIONES]    # alias
```

| Opción | Abrev. | Descripción |
|--------|--------|-------------|
| `--force` | `-f` | Eliminar sin pedir confirmación |

**Ejemplos:**

```bash
# Eliminar con confirmación
xdsk remove juego.dsk ANTIGUO.BIN

# Eliminar sin confirmación
xdsk remove juego.dsk ANTIGUO.BIN --force

# Eliminar múltiples ficheros
xdsk remove juego.dsk ANT1.BIN ANT2.BIN TEMP.DAT --force
```

---

### `copy` / `cp`

Copia uno o más ficheros de una imagen DSK a otra, preservando cabeceras AMSDOS y atributos.

```
xdsk copy <ORIGEN> <DESTINO> [FICHEROS]... [OPCIONES]
xdsk cp   <ORIGEN> <DESTINO> [FICHEROS]... [OPCIONES]    # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--force` | `-f` | Sobreescribir ficheros existentes en destino | off |

**Patrones comodín soportados:**

| Patrón | Qué coincide |
|--------|--------------|
| `*` | Todos los ficheros |
| `*.EXT` | Todos con esa extensión (ej. `*.BAS`) |
| `NOMBRE*` | Los que empiezan por ese nombre |
| `EXACTO` | Nombre exacto (insensible a mayúsculas) |

**Ejemplos:**

```bash
# Copiar todos los ficheros
xdsk copy origen.dsk destino.dsk

# Copiar un fichero concreto
xdsk copy origen.dsk destino.dsk LOADER.BAS

# Copiar todos los ficheros BASIC
xdsk copy origen.dsk destino.dsk "*.BAS"

# Sobreescribir ficheros existentes en destino
xdsk copy origen.dsk destino.dsk "*.BAS" --force
```

---

### `view`

Muestra el contenido de un fichero almacenado en una imagen DSK sin extraerlo.

```
xdsk view <IMAGEN> <FICHERO> [OPCIONES]
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--format <FORMATO>` | `-f` | `auto`, `basic`, `hex`, `ascii`, `disasm` | `auto` |

**Modos de visualización:**

| Modo | Descripción |
|------|-------------|
| `auto` | Auto-detecta: prueba BASIC primero, si no hex dump |
| `basic` | Listado BASIC tokenizado del Amstrad CPC |
| `hex` | Volcado hex + ASCII lado a lado |
| `ascii` | Texto plano con manejo de caracteres extendidos CPC |
| `disasm` | Desensamblado Z80 |

**Ejemplos:**

```bash
# Auto-detectar
xdsk view juego.dsk LOADER.BAS

# Forzar listado BASIC
xdsk view juego.dsk LOADER.BAS --format basic

# Hex dump de un binario
xdsk view juego.dsk SPRITES.BIN --format hex

# Desensamblado Z80
xdsk view juego.dsk CODE.BIN --format disasm
```

**Ejemplo de salida hex:**

```
0000: 00 01 FF 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
0010: 80 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
```

---

### `check`

Valida la integridad de una imagen DSK: magic de cabecera, geometría de pistas, entradas de directorio, cabeceras AMSDOS y conflictos de asignación de bloques.

```
xdsk check <IMAGEN>
```

**Código de salida:** `0` si el disco está sano, `1` si hay algún problema.

**Ejemplo:**

```bash
xdsk check juego.dsk
```

**Salida (disco sano):**

```
Checking juego.dsk...

Header
  ✓ Magic valid (DATA format)
  ✓ 40 tracks, 1 head(s)

Directory
  ✓ 3 / 64 entries used
  ✓ LOADER.BAS — no AMSDOS header (ASCII/raw)
  ✓ SPRITES.BIN — AMSDOS header valid
  ✓ Block allocation OK (no conflicts)

Result: OK — 0 issues
```

Ideal para usar en CI/CD:

```bash
xdsk check release.dsk && echo "Disco OK" || echo "Disco con errores"
```

---

### `info`

Muestra estadísticas detalladas de una imagen DSK: formato, geometría, ocupación del directorio, mapa de bloques y listado de ficheros.

```
xdsk info <IMAGEN>
```

**Ejemplo de salida:**

```
DSK Image: juego.dsk
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Format           DATA (standard CPC)
  Tracks           40
  Sectors/Track    9
  Sector size      512 bytes
  Capacity         180 KB

Directory
  Entries used     3 / 64

Block Map
  Total blocks     180
  Used             8  (4.4%)
  Free             172 (95.6%)

Files
  Name           Type       Size       Attr
  ──────────────────────────────────────────────
  LOADER.BAS     ASCII      1 KB       -
  SPRITES.BIN    BINARY     4 KB       -
  LEVEL1.DAT     BINARY     2 KB       R
```

---

### `diff`

Compara el contenido de dos imágenes DSK e informa de qué ficheros son únicos en cada una, cuáles han cambiado y cuáles son idénticos.

```
xdsk diff <IMAGEN1> <IMAGEN2>
```

**Ejemplo:**

```bash
xdsk diff juego.dsk juego_v2.dsk
```

**Salida:**

```
Comparing juego.dsk ↔ juego_v2.dsk

Only in juego.dsk:
  NIVEL_VIEJO.DAT  2 KB

Only in juego_v2.dsk:
  NIVEL2.DAT       4 KB
  HISCORE.DAT      1 KB

Modified (same name, different content):
  LOADER.BAS       1 KB → 2 KB

Identical:
  SPRITES.BIN      8 KB

Summary: 2 added, 1 removed, 1 modified, 1 identical
```

---

## Tipos de fichero y cabeceras AMSDOS

Cada fichero en un disco DATA puede llevar opcionalmente una **cabecera AMSDOS** — 128 bytes al inicio que indican el tipo, dirección de carga, dirección de ejecución y longitud lógica.

| Tipo AMSDOS | Byte de cabecera | Significado |
|-------------|------------------|-------------|
| `0x00` | BASIC | BASIC tokenizado del Amstrad |
| `0x01` | BASIC(P) | BASIC tokenizado protegido |
| `0x02` | BINARY | Código máquina / datos binarios |
| `0x03` | BINARY(P) | Binario protegido |
| *(ninguno)* | — | Texto ASCII, datos raw |

Al importar con `--file-type binary`, xdsk genera automáticamente una cabecera AMSDOS válida con checksum. Al exportar, la cabecera se preserva por defecto; usa `--strip-header` para obtener solo el payload.

---

## Formato DSK

xdsk crea y lee discos en formato DATA estándar del Amstrad CPC:

| Propiedad | Valor |
|-----------|-------|
| Pistas | 40 (por defecto) |
| Sectores/pista | 9 (por defecto) |
| Tamaño de sector | 512 bytes |
| Tamaño de bloque | 1024 bytes (2 sectores) |
| Total bloques | 180 |
| Bloques de directorio | 2 (bloques 0–1, 64 entradas) |
| Bloques utilizables | 178 (178 KB) |
| IDs de sector | `0xC1`–`0xC9` |

**Orden de interleaving físico (2:1) por pista:**

```
Slot:    0     1     2     3     4     5     6     7     8
ID:    0xC1  0xC6  0xC2  0xC7  0xC3  0xC8  0xC4  0xC9  0xC5
```

---

## Completado de shell

Genera e instala scripts de autocompletado para tu shell:

```bash
# Bash
xdsk completions bash > ~/.local/share/bash-completion/completions/xdsk

# Zsh
xdsk completions zsh > "${fpath[1]}/_xdsk"

# Fish
xdsk completions fish > ~/.config/fish/completions/xdsk.fish

# PowerShell
xdsk completions powershell > xdsk.ps1
```

---

## Variables de entorno

| Variable | Descripción |
|----------|-------------|
| `RUST_LOG` | Nivel de log (ej. `RUST_LOG=debug xdsk list juego.dsk`) |
| `NO_COLOR` | Desactiva colores (equivalente a `--no-color`) |

---

## Códigos de salida

| Código | Significado |
|--------|-------------|
| `0` | Éxito |
| `1` | Error (fichero no encontrado, DSK corrupto, check fallido, etc.) |

Los comandos que procesan múltiples ficheros (`import`, `export`) continúan ante errores individuales y salen con `0`; los errores se imprimen en stderr.

---

## Compatibilidad

| Herramienta / Emulador | Estado |
|------------------------|--------|
| RetroVirtualMachine 2 | ✅ Probado |
| WinAPE | ✅ Formato DSK compatible |
| CPCDiskXP | ✅ Formato DSK compatible |
| JavaCPC | ✅ Formato DSK compatible |
| iDSK | ✅ Reemplazo directo |

---

## Licencia

MIT License — Copyright (c) 2026 Destroyer
