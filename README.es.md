# disc — Herramienta Moderna para Imágenes DSK del Amstrad CPC

Una herramienta de línea de comandos rápida y moderna para trabajar con imágenes de disco DSK del Amstrad CPC, escrita en Rust.

Sustituye a la herramienta legacy `iDSK` en C++ con una CLI limpia y multiplataforma que gestiona correctamente las cabeceras AMSDOS, el entrelazado de sectores y el formato de directorio CP/M.

---

## Tabla de Contenidos

- [Instalación](#instalación)
- [Inicio Rápido](#inicio-rápido)
- [Opciones Globales](#opciones-globales)
- [Comandos](#comandos)
  - [create](#create--new)
  - [list](#list--ls)
  - [import](#import--add)
  - [export](#export--get)
  - [remove](#remove--rm)
  - [copy](#copy--cp)
  - [view](#view)
  - [check](#check)
  - [info](#info)
  - [diff](#diff)
- [Tipos de Archivo y Cabeceras AMSDOS](#tipos-de-archivo-y-cabeceras-amsdos)
- [Referencia del Formato DSK](#referencia-del-formato-dsk)
- [Completado de Shell](#completado-de-shell)
- [Variables de Entorno](#variables-de-entorno)
- [Códigos de Salida](#códigos-de-salida)
- [Compatibilidad](#compatibilidad)
- [Licencia](#licencia)

---

## Instalación

### Desde el código fuente

```bash
git clone https://github.com/CPCReady/Disc-Image-Studio.git
cd Disc-Image-Studio/disc
cargo build --release
# Binario en: target/release/disc
```

### Instalación global con Cargo

```bash
cargo install --path disc/
```

### Binarios precompilados

Ejecuta el script de release para obtener un binario nativo:

```bash
./build-release.sh --native
# Salida: dist/disc-macos-aarch64  (o linux-x86_64, etc.)
```

---

## Inicio Rápido

```bash
# 1. Crear un disco DATA vacío de 40 pistas
disc create juego.dsk

# 2. Importar archivos
disc import juego.dsk cargador.bas
disc import juego.dsk sprites.bin --file-type binary --load 0x4000 --exec 0x4000

# 3. Listar el contenido
disc list juego.dsk

# 4. Ver un archivo directamente
disc view juego.dsk CARGADOR.BAS --format basic

# 5. Exportar archivos
disc export juego.dsk "*.BAS" --output backup/

# 6. Comprobar la integridad del disco
disc check juego.dsk

# 7. Estadísticas detalladas
disc info juego.dsk

# 8. Comparar dos imágenes de disco
disc diff juego.dsk juego_backup.dsk

# 9. Copiar archivos entre imágenes de disco
disc copy src.dsk dst.dsk "*.BAS"

# 10. Eliminar un archivo
disc remove juego.dsk VIEJO.BIN --force
```

---

## Opciones Globales

Estas opciones pueden usarse con **cualquier** subcomando:

| Opción | Abrev. | Descripción |
|--------|--------|-------------|
| `--verbose` | `-v` | Activa el log de depuración (muestra operaciones internas) |
| `--no-color` | — | Desactiva la salida coloreada (útil para scripts o tuberías) |
| `--help` | `-h` | Muestra la ayuda del comando |
| `--version` | `-V` | Imprime el número de versión |

```bash
disc --verbose list juego.dsk
disc --no-color list juego.dsk | grep .BAS
disc --version
```

---

## Comandos

### `create` / `new`

Crea una nueva imagen DSK vacía formateada como disco DATA estándar del Amstrad CPC.

```
disc create <IMAGEN> [OPCIONES]
disc new    <IMAGEN> [OPCIONES]          # alias
```

| Opción | Descripción | Por defecto |
|--------|-------------|-------------|
| `--tracks <N>` | Número de pistas | `40` |
| `--sectors <N>` | Sectores por pista | `9` |
| `-f, --force` | Sobreescribir si el archivo ya existe | desactivado |

**Ejemplos:**

```bash
# Disco DATA estándar de 40 pistas (178 KB)
disc create juego.dsk

# Disco de 80 pistas (para imágenes extendidas)
disc create extendido.dsk --tracks 80

# Sobreescribir un disco existente
disc create juego.dsk --force
```

> **Nota:** El disco creado utiliza el entrelazado estándar de sectores del CPC
> (`0xC1, 0xC6, 0xC2, 0xC7, 0xC3, 0xC8, 0xC4, 0xC9, 0xC5`) y es
> compatible con los principales emuladores del Amstrad CPC.

---

### `list` / `ls`

Lista los archivos almacenados en una imagen DSK.

```
disc list <IMAGEN> [OPCIONES]
disc ls   <IMAGEN> [OPCIONES]            # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--format <FORMATO>` | `-f` | Formato de salida: `table`, `json`, `csv`, `simple` | `table` |

**Ejemplos:**

```bash
# Tabla legible (por defecto)
disc list juego.dsk

# JSON — útil para scripting
disc list juego.dsk --format json

# CSV — para hojas de cálculo o grep
disc list juego.dsk --format csv

# Lista simple — solo nombres de archivo, uno por línea
disc list juego.dsk --format simple

# Filtrar con herramientas del shell
disc list juego.dsk --format simple | grep "\.BAS"
```

**Salida en formato tabla:**

```
DSK Image: juego.dsk
Tracks: 40 | Sectors/Track: 9 | Format: DATA
Used: 12 KB / 178 KB (6.7%)

┌──────────────┬──────────┬──────┬────────┐
│ Name         │ Type     │ Size │ Attrs  │
├──────────────┼──────────┼──────┼────────┤
│ CARGADOR.BAS │ BASIC    │ 2 KB │        │
│ SPRITES.BIN  │ BINARY   │ 8 KB │ R      │
│ NIVEL1.DAT   │ BINARY   │ 2 KB │        │
└──────────────┴──────────┴──────┴────────┘
3 files, 12 KB total, 166 KB free
```

**Columna Attrs:**

| Símbolo | Significado |
|---------|-------------|
| `R` | Solo lectura |
| `S` | Archivo de sistema (oculto) |

**Estructura de salida JSON:**

```json
{
  "files": [
    { "name": "CARGADOR.BAS", "type": "BASIC", "size": 2048, "read_only": false, "system": false }
  ],
  "total_size": 2048,
  "free_space": 180224
}
```

---

### `import` / `add`

Importa uno o varios archivos desde el sistema de archivos del host a una imagen DSK.

```
disc import <IMAGEN> <ARCHIVOS>... [OPCIONES]
disc add    <IMAGEN> <ARCHIVOS>... [OPCIONES]   # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--file-type <TIPO>` | `-t` | Tipo de archivo: `ascii`, `binary`, `raw` | detección automática |
| `--load <ADDR>` | `-c` | Dirección de carga en hex (p. ej. `0x4000`) | — |
| `--exec <ADDR>` | `-e` | Dirección de ejecución en hex (p. ej. `0xC000`) | — |
| `--user <N>` | `-u` | Número de usuario CP/M 0–15 | `0` |
| `--read-only` | `-o` | Marcar como solo lectura | desactivado |
| `--system` | `-s` | Marcar como archivo de sistema (oculto) | desactivado |
| `--force` | `-f` | Sobreescribir si el archivo ya existe en el disco | desactivado |

**Detección automática del tipo de archivo** (cuando se omite `--file-type`):

| Condición | Tipo detectado |
|-----------|---------------|
| El archivo fuente tiene una cabecera AMSDOS válida | `binary` |
| En cualquier otro caso | `ascii` |

**Comportamiento por tipo de archivo:**

| Tipo | Qué hace `disc` |
|------|-----------------|
| `ascii` | Elimina cualquier cabecera AMSDOS existente; convierte `LF` → `CR+LF` |
| `binary` | Añade una cabecera AMSDOS binaria con las direcciones de carga/ejecución si no está presente |
| `raw` | Almacena el archivo exactamente como está — no añade ni elimina cabecera |

**Ejemplos:**

```bash
# Fuente BASIC ASCII — detección automática
disc import juego.dsk cargador.bas

# ASCII explícito
disc import juego.dsk notas.txt --file-type ascii

# Binario con direcciones de carga y ejecución
disc import juego.dsk codigo.bin --file-type binary --load 0x4000 --exec 0x4000

# Binario — solo carga (sin arranque automático)
disc import juego.dsk datos.bin --file-type binary --load 0x8000

# Archivo raw — almacenado exactamente como está
disc import juego.dsk paleta.dat --file-type raw

# Múltiples archivos en un solo comando
disc import juego.dsk src/*.bas src/*.bin

# Importar al área de usuario CP/M 2
disc import juego.dsk privado.bin --user 2

# Archivo de sistema solo lectura
disc import juego.dsk arranque.bin --file-type binary --read-only --system

# Sobreescribir un archivo existente
disc import juego.dsk cargador.bas --force
```

> Si un archivo ya existe en el disco y no se indica `--force`, el error
> se imprime en stderr y `disc` continúa con los archivos restantes (código de salida 0).

---

### `export` / `get`

Extrae uno o varios archivos de una imagen DSK al sistema de archivos del host.

```
disc export <IMAGEN> <ARCHIVOS>... [OPCIONES]
disc get    <IMAGEN> <ARCHIVOS>... [OPCIONES]   # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--output <DIR>` | `-o` | Directorio de destino (se crea si no existe) | directorio actual |
| `--strip-header` | — | Elimina la cabecera AMSDOS del archivo exportado | desactivado |

Los nombres de archivo son **insensibles a mayúsculas**. Se admite el comodín `*` al final.

**Ejemplos:**

```bash
# Exportar un único archivo al directorio actual
disc export juego.dsk CARGADOR.BAS

# Exportar a un directorio específico
disc export juego.dsk CARGADOR.BAS --output backup/

# Comodín — exportar todos los archivos BASIC
disc export juego.dsk "*.BAS" --output backup/

# Exportar sin cabecera AMSDOS (solo datos)
disc export juego.dsk SPRITES.BIN --strip-header --output raw/

# Exportar todos los archivos
disc export juego.dsk "*" --output copia_completa/
```

> El directorio de salida se crea automáticamente si no existe.

---

### `remove` / `rm`

Elimina uno o varios archivos de una imagen DSK (marca las entradas de directorio como borradas; el disco no se reformatea).

```
disc remove <IMAGEN> <ARCHIVOS>... [OPCIONES]
disc rm     <IMAGEN> <ARCHIVOS>... [OPCIONES]   # alias
```

| Opción | Abrev. | Descripción |
|--------|--------|-------------|
| `--force` | `-f` | Eliminar sin pedir confirmación |

**Ejemplos:**

```bash
# Eliminar con confirmación
disc remove juego.dsk VIEJO.BIN

# Eliminar sin confirmación
disc remove juego.dsk VIEJO.BIN --force

# Eliminar varios archivos a la vez
disc remove juego.dsk OLD1.BIN OLD2.BIN TEMP.DAT --force
```

> Eliminar un archivo que no existe no hace nada (código de salida 0).

---

### `copy` / `cp`

Copia uno o varios archivos de una imagen DSK a otra, preservando las cabeceras AMSDOS y los atributos de archivo.

```
disc copy <SRC> <DST> [ARCHIVOS]... [OPCIONES]
disc cp   <SRC> <DST> [ARCHIVOS]... [OPCIONES]   # alias
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--force` | `-f` | Sobreescribir archivos existentes en el destino | desactivado |

**Patrones de comodín admitidos:**

| Patrón | Coincide con |
|--------|-------------|
| `*` | Todos los archivos |
| `*.EXT` | Todos los archivos con esa extensión (p. ej. `*.BAS`) |
| `NAME*` | Todos cuyo nombre empiece por `NAME` (p. ej. `LEVEL*`) |
| `EXACTO` | Nombre de archivo exacto (insensible a mayúsculas) |

**Ejemplos:**

```bash
# Copiar todos los archivos de src a dst
disc copy src.dsk dst.dsk

# Copiar un único archivo
disc copy src.dsk dst.dsk CARGADOR.BAS

# Copiar todos los archivos BASIC (comodín por extensión)
disc copy src.dsk dst.dsk "*.BAS"

# Copiar todos los archivos que empiecen por LEVEL
disc copy src.dsk dst.dsk "LEVEL*"

# Copiar varios patrones
disc copy src.dsk dst.dsk "*.BAS" SPRITES.BIN

# Sobreescribir archivos existentes en el destino
disc copy src.dsk dst.dsk "*.BAS" --force
```

> Las cabeceras AMSDOS, las direcciones de carga/ejecución y los bits de atributo solo-lectura/sistema se preservan exactamente tal como están en la imagen de origen.

> Si un archivo ya existe en el destino y no se indica `--force`, el error se imprime en stderr y `disc` continúa con los archivos restantes.

---

### `view`

Muestra el contenido de un archivo almacenado en una imagen DSK sin extraerlo.

```
disc view <IMAGEN> <ARCHIVO> [OPCIONES]
```

| Opción | Abrev. | Descripción | Por defecto |
|--------|--------|-------------|-------------|
| `--format <FORMATO>` | `-f` | Visor: `auto`, `basic`, `hex`, `ascii`, `disasm` | `auto` |

**Modos de visualización:**

| Modo | Descripción |
|------|-------------|
| `auto` | Detección automática: intenta listado BASIC primero, cae a volcado hex si falla |
| `basic` | Listado de BASIC tokenizado del Amstrad CPC |
| `hex` | Volcado hex + ASCII lado a lado |
| `ascii` | Texto plano con manejo de caracteres extendidos del CPC |
| `disasm` | Desensamblado Z80 |

**Ejemplos:**

```bash
# Visor con detección automática
disc view juego.dsk CARGADOR.BAS

# Forzar listado BASIC
disc view juego.dsk CARGADOR.BAS --format basic

# Volcado hex de un binario
disc view juego.dsk SPRITES.BIN --format hex

# Desensamblado Z80
disc view juego.dsk CODIGO.BIN --format disasm

# Texto ASCII plano
disc view juego.dsk LEEME.TXT --format ascii
```

**Ejemplo de salida hex:**

```
0000: 00 01 FF 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
0010: 80 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
```

**Ejemplo de listado BASIC:**

```
10 MODE 1
20 BORDER 0
30 PRINT "HOLA AMSTRAD CPC"
40 END
```

---

### `check`

Valida la integridad de una imagen DSK: magic del header, geometría de pistas, entradas de directorio, cabeceras AMSDOS y conflictos de asignación de bloques.

```
disc check <IMAGEN>
```

**Código de salida:** `0` si el disco está sano, `1` si se encuentra algún problema.

**Ejemplo:**

```bash
disc check juego.dsk
```

**Salida (disco sano):**

```
Checking juego.dsk...

Header
  ✓ Magic valid (DATA format)
  ✓ 40 tracks, 1 head(s)

Directory
  ✓ 3 / 64 entries used
  ✓ CARGADOR.BAS — no AMSDOS header (ASCII/raw)
  ✓ SPRITES.BIN — AMSDOS header valid
  ✓ NIVEL1.DAT — AMSDOS header valid
  ✓ Block allocation OK (no conflicts)

Result: OK — 0 issues
```

**Salida (disco corrupto):**

```
Checking malo.dsk...

Header
  ✗ Header invalid: Invalid DSK magic string

Result: 1 issue(s) found
Error: Check failed: 1 error(s) found
```

Usa `--verbose` para ver cada comprobación en detalle:

```bash
disc --verbose check juego.dsk
```

---

### `info`

Muestra estadísticas detalladas de una imagen DSK: formato, geometría, ocupación del directorio, uso del mapa de bloques y una tabla de archivos con tamaños y atributos.

```
disc info <IMAGEN>
```

**Ejemplo:**

```bash
disc info juego.dsk
```

**Salida:**

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
  CARGADOR.BAS   ASCII      1 KB       -
  SPRITES.BIN    BINARY     4 KB       -
  NIVEL1.DAT     BINARY     2 KB       R
```

---

### `diff`

Compara el contenido de dos imágenes DSK e informa qué archivos son exclusivos de cada imagen, cuáles han sido modificados y cuáles son idénticos.

```
disc diff <IMAGEN1> <IMAGEN2>
```

**Código de salida:** Siempre `0` — las diferencias se muestran en stdout, no se tratan como errores.

**Ejemplo:**

```bash
disc diff juego.dsk juego_v2.dsk
```

**Salida:**

```
Comparing juego.dsk ↔ juego_v2.dsk

Only in juego.dsk:
  NIVEL_VIEJO.DAT  2 KB

Only in juego_v2.dsk:
  NIVEL2.DAT       4 KB
  RECORD.DAT       1 KB

Modified (same name, different content):
  CARGADOR.BAS     1 KB → 2 KB

Identical:
  SPRITES.BIN      8 KB
  MUSICA.BIN       4 KB

Summary: 2 added, 1 removed, 1 modified, 2 identical
```

**Casos de uso habituales:**

```bash
# Verificar que un backup es idéntico
disc diff original.dsk backup.dsk

# Ver qué cambió entre dos versiones
disc diff juego_v1.dsk juego_v2.dsk

# Diff sin color para scripts
disc --no-color diff a.dsk b.dsk
```

---

## Tipos de Archivo y Cabeceras AMSDOS

Cada archivo almacenado en un disco DATA del Amstrad CPC puede llevar opcionalmente una **cabecera AMSDOS** — un prefijo de 128 bytes que indica al firmware el tipo de archivo, la dirección de carga, la dirección de ejecución y la longitud lógica.

| Tipo de archivo AMSDOS | Byte de cabecera | Significado |
|------------------------|-----------------|-------------|
| `0x00` | BASIC | BASIC tokenizado del Amstrad |
| `0x01` | BASIC(P) | BASIC tokenizado protegido |
| `0x02` | BINARY | Código máquina / datos binarios |
| `0x03` | BINARY(P) | Binario protegido |
| *(ninguna)* | — | Texto ASCII, datos sin cabecera |

Al importar con `--file-type binary`, `disc` genera automáticamente una cabecera AMSDOS válida incluyendo el checksum. Al exportar, la cabecera se conserva por defecto; usa `--strip-header` para obtener solo el contenido útil.

---

## Referencia del Formato DSK

`disc` crea y lee discos estándar del Amstrad CPC en **formato DATA**:

| Propiedad | Valor |
|-----------|-------|
| Pistas | 40 (por defecto) |
| Sectores / pista | 9 (por defecto) |
| Tamaño de sector | 512 bytes |
| Tamaño de bloque | 1024 bytes (2 sectores) |
| Bloques totales | 180 |
| Bloques de directorio | 2 (bloques 0–1, 64 entradas) |
| Bloques utilizables | 178 (178 KB) |
| IDs de sector | `0xC1`–`0xC9` |

**Orden de entrelazado físico** (2:1) por pista:

```
Ranura:  0     1     2     3     4     5     6     7     8
ID:    0xC1  0xC6  0xC2  0xC7  0xC3  0xC8  0xC4  0xC9  0xC5
```

`disc` resuelve correctamente este entrelazado al leer y escribir bloques — nunca usa offsets físicos directos para el directorio ni los datos de archivo.

---

## Completado de Shell

Genera e instala scripts de completado con tabulador para tu shell:

```bash
# Bash
disc completions bash > ~/.local/share/bash-completion/completions/disc

# Zsh
disc completions zsh > "${fpath[1]}/_disc"

# Fish
disc completions fish > ~/.config/fish/completions/disc.fish

# PowerShell
disc completions powershell > disc.ps1
```

Después de instalar, reinicia tu shell (o carga el archivo) para activar los completados.

---

## Variables de Entorno

| Variable | Descripción |
|----------|-------------|
| `RUST_LOG` | Sobreescribe el nivel de log directamente (p. ej. `RUST_LOG=debug disc list juego.dsk`) |
| `NO_COLOR` | Establece cualquier valor para desactivar la salida coloreada (igual que `--no-color`) |

La opción `--verbose` establece el nivel de log a **debug**. Sin ella, solo se imprimen advertencias y errores.

```bash
# Máxima verbosidad mediante variable de entorno
RUST_LOG=trace disc check juego.dsk

# Silencioso — suprimir toda la salida excepto errores
RUST_LOG=error disc import juego.dsk archivo.bas
```

---

## Códigos de Salida

| Código | Significado |
|--------|-------------|
| `0` | Éxito |
| `1` | Se produjo un error (archivo no encontrado, DSK corrupto, fallo en check, etc.) |

Los comandos que procesan varios archivos (p. ej. `import`, `export`) continúan ante errores por archivo y salen con `0`; los errores individuales se imprimen en **stderr**.

`disc check` sale con `1` si se encuentra algún problema de integridad, lo que lo hace apto para pipelines de CI:

```bash
disc check release.dsk && echo "Disco OK" || echo "El disco tiene errores!"
```

---

## Compatibilidad

| Herramienta / Emulador | Estado |
|------------------------|--------|
| RetroVirtualMachine 2 | ✅ Probado |
| WinAPE | ✅ Compatible con formato DSK |
| CPCDiskXP | ✅ Compatible con formato DSK |
| JavaCPC | ✅ Compatible con formato DSK |
| iDSK | ✅ Sustituto directo |

---

## Licencia

MIT License — Copyright (c) 2026 Destroyer

---

## Proyectos Relacionados

- [iDSK](https://github.com/cpcsdk/idsk) — herramienta C++ original que este proyecto sustituye
- [RetroVirtualMachine](https://www.retrovm.com/) — emulador del Amstrad CPC
- [2cdt](https://github.com/cpcsdk/2cdt) — herramienta para imágenes de cinta CDT
