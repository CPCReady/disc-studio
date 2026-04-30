---
name: xdsk
description: Gestionar imágenes .dsk de Amstrad CPC con xdsk. Usar cuando el usuario pida listar ficheros en un DSK, importar o exportar ficheros a/desde un DSK, borrar ficheros de un DSK, crear una imagen DSK nueva, ver contenido de ficheros (BASIC, hex, disasm, ASCII), verificar integridad de un DSK, comparar dos imágenes DSK o copiar ficheros entre DSKs. También para salida en JSON/CSV y control de atributos AMSDOS (tipo, dirección de carga/ejecución, usuario, read-only, system). El binario se llama `xdsk`. Si no está disponible, indicar al usuario que compile en xdsk/ con `cargo build --release`.
---

# xdsk

Herramienta Rust moderna para trabajar con imágenes **DSK** de Amstrad CPC.

**Binario:** `xdsk` (compilar en `xdsk/` con `cargo build --release --target aarch64-apple-darwin`)

---

## Verificar disponibilidad

```bash
xdsk --version
# Si falla: cd xdsk && cargo build --release --target aarch64-apple-darwin
```

---

## Opciones globales

Disponibles en todos los comandos:

| Opción | Descripción |
|--------|-------------|
| `--no-color` | Desactiva colores en la salida |
| `-v, --verbose` | Salida detallada |

---

## Comandos disponibles

### `disc list` — Listar ficheros en un DSK

Muestra el directorio de una imagen DSK con nombre, tipo, tamaño y atributos.

```bash
disc list <IMAGE.DSK> [-f <FORMAT>] [-v]
```

| Opción | Valores | Por defecto | Descripción |
|--------|---------|-------------|-------------|
| `-f, --format` | `table`, `json`, `csv`, `simple` | `table` | Formato de salida |

**Ejemplos:**
```bash
# Tabla formateada (por defecto)
disc list game.dsk

# Salida JSON (ideal para automatización)
disc list game.dsk -f json

# CSV
disc list game.dsk -f csv

# Lista simple de nombres
disc list game.dsk -f simple

# Con detalles extra
disc list game.dsk -v
```

**Salida JSON:**
```json
[
  { "name": "GAME.BIN", "size": 4096, "type": "binary", "user": 0, "read_only": false }
]
```

---

### `disc import` — Importar ficheros al DSK

Añade ficheros locales a una imagen DSK, con opciones AMSDOS completas.

```bash
disc import <IMAGE.DSK> [FICHEROS...] [OPCIONES]
```

| Opción | Descripción |
|--------|-------------|
| `-t, --file-type <TYPE>` | Tipo: `ascii`, `binary`, `raw` (sin cabecera AMSDOS) |
| `-c, --load <ADDR>` | Dirección de carga hex (ej: `0x4000`) |
| `-e, --exec <ADDR>` | Dirección de ejecución hex (ej: `0xC000`) |
| `-u, --user <N>` | Número de usuario AMSDOS (0-15, por defecto: 0) |
| `-o, --read-only` | Marcar como solo lectura |
| `-s, --system` | Marcar como fichero de sistema |
| `-f, --force` | Sobrescribir si ya existe |

**Ejemplos:**
```bash
# Importar binario con direcciones
disc import game.dsk loader.bin -t binary -c 0x4000 -e 0x4000

# Importar múltiples ficheros
disc import game.dsk *.bin *.bas

# Importar BASIC ASCII
disc import game.dsk game.bas -t ascii

# Importar binario crudo sin cabecera AMSDOS
disc import game.dsk raw.bin -t raw

# Forzar sobrescritura
disc import game.dsk loader.bin -f

# Usuario 1, read-only
disc import game.dsk loader.bin -u 1 -o
```

---

### `disc export` — Exportar ficheros desde el DSK

Extrae ficheros de una imagen DSK al sistema de ficheros local.

```bash
disc export <IMAGE.DSK> [FICHEROS...] [-o DIR] [--strip-header]
```

| Opción | Descripción |
|--------|-------------|
| `-o, --output <DIR>` | Directorio de salida (por defecto: directorio actual) |
| `--strip-header` | Elimina la cabecera AMSDOS del fichero extraído |

**Ejemplos:**
```bash
# Exportar todos los ficheros
disc export game.dsk

# Exportar a directorio específico
disc export game.dsk -o /tmp/output/

# Exportar fichero concreto
disc export game.dsk GAME.BIN

# Exportar con wildcard
disc export game.dsk *.BIN

# Exportar sin cabecera AMSDOS (payload bruto)
disc export game.dsk GAME.BIN --strip-header -o /tmp/
```

---

### `disc remove` — Eliminar ficheros del DSK

Borra ficheros de una imagen DSK.

```bash
disc remove <IMAGE.DSK> [FICHEROS...] [-f]
```

| Opción | Descripción |
|--------|-------------|
| `-f, --force` | Eliminar sin pedir confirmación |

**Ejemplos:**
```bash
# Eliminar un fichero (pedirá confirmación)
disc remove game.dsk GAME.BIN

# Eliminar varios ficheros sin confirmación
disc remove game.dsk GAME.BIN MUSIC.BIN -f

# Eliminar con wildcard
disc remove game.dsk *.BIN -f
```

---

### `disc create` — Crear nueva imagen DSK

Crea una imagen DSK vacía con los parámetros de formato especificados.

```bash
disc create <IMAGE.DSK> [--tracks N] [--sectors N] [-f]
```

| Opción | Por defecto | Descripción |
|--------|-------------|-------------|
| `--tracks <N>` | `40` | Número de pistas |
| `--sectors <N>` | `9` | Sectores por pista |
| `-f, --force` | — | Sobrescribir si ya existe |

**Ejemplos:**
```bash
# DSK estándar CPC (40 pistas, 9 sectores)
disc create nuevo.dsk

# DSK de doble cara (80 pistas)
disc create doble.dsk --tracks 80

# DSK personalizado
disc create custom.dsk --tracks 40 --sectors 10

# Sobrescribir uno existente
disc create nuevo.dsk -f
```

---

### `disc view` — Ver contenido de un fichero

Muestra el contenido de un fichero dentro del DSK en diferentes formatos.

```bash
disc view <IMAGE.DSK> <FICHERO> [-f <FORMAT>]
```

| Formato | Descripción |
|---------|-------------|
| `auto` | Detección automática por tipo (por defecto) |
| `basic` | Listado BASIC desensamblado |
| `hex` | Volcado hexadecimal |
| `ascii` | Texto ASCII |
| `disasm` | Desensamblado Z80 |

**Ejemplos:**
```bash
# Auto-detección (BASIC si es .BAS, hex si es binario)
disc view game.dsk LOADER.BAS

# Forzar listado BASIC
disc view game.dsk LOADER.BAS -f basic

# Hex dump
disc view game.dsk GAME.BIN -f hex

# Desensamblado Z80
disc view game.dsk GAME.BIN -f disasm

# Forzar ASCII
disc view game.dsk README.TXT -f ascii
```

---

### `disc check` — Verificar integridad del DSK

Valida la estructura interna de la imagen DSK, cabeceras de pista y sectores.

```bash
disc check <IMAGE.DSK> [-v]
```

**Ejemplo:**
```bash
disc check game.dsk
disc check game.dsk -v   # con detalles de cada pista/sector
```

Devuelve código de salida 1 si el DSK está corrupto.

---

### `disc info` — Información detallada del DSK

Muestra metadatos completos: formato, número de pistas, sectores, espacio libre/usado.

```bash
disc info <IMAGE.DSK> [-v]
```

**Ejemplo:**
```bash
disc info game.dsk
disc info game.dsk -v
```

Muestra: tipo de DSK (Standard/Extended), pistas, sectores por pista, tamaño total, espacio libre, número de ficheros.

---

### `disc diff` — Comparar dos imágenes DSK

Compara el contenido de dos imágenes DSK e informa de las diferencias en ficheros.

```bash
disc diff <IMAGE1.DSK> <IMAGE2.DSK> [-v]
```

**Ejemplo:**
```bash
disc diff original.dsk modificado.dsk
disc diff v1.dsk v2.dsk -v
```

Muestra qué ficheros se añadieron, eliminaron o modificaron entre los dos DSKs.

---

### `disc copy` — Copiar ficheros entre dos DSKs

Copia ficheros de un DSK origen a un DSK destino. Soporta wildcards.

```bash
disc copy <SRC.DSK> <DST.DSK> [FICHEROS...] [-f]
```

| Opción | Descripción |
|--------|-------------|
| `-f, --force` | Sobrescribir si ya existe en destino |

Sin especificar ficheros, copia **todos** los ficheros del origen.

**Ejemplos:**
```bash
# Copiar todos los ficheros
disc copy origen.dsk destino.dsk

# Copiar ficheros específicos
disc copy origen.dsk destino.dsk GAME.BIN MUSIC.BIN

# Copiar con wildcard
disc copy origen.dsk destino.dsk *.BIN

# Sobrescribir si existen en destino
disc copy origen.dsk destino.dsk -f
```

---

## Flujos típicos

### Crear disco con ficheros importados

```bash
# Crear DSK vacío
disc create game.dsk

# Importar ficheros
disc import game.dsk loader.bin -t binary -c 0x1000 -e 0x1000
disc import game.dsk game.bin   -t binary -c 0x4000 -e 0x4000
disc import game.dsk music.bin  -t binary -c 0x8000 -e 0x8000

# Verificar resultado
disc list game.dsk
disc check game.dsk
```

### Inspección y extracción

```bash
# Ver directorio completo
disc list game.dsk -f json | jq '.[].name'

# Inspeccionar un fichero BASIC
disc view game.dsk LOADER.BAS -f basic

# Exportar todo a directorio
disc export game.dsk -o extracted/

# Comparar con versión anterior
disc diff v1.dsk v2.dsk
```

### Pipeline de automatización con JSON

```bash
# Listar en JSON para procesar con jq
disc list game.dsk -f json | jq '.[] | select(.type == "binary") | .name'

# Info del disco
disc info game.dsk -v

# Verificar integridad antes de distribuir
disc check game.dsk && echo "DSK válido"
```

---

## Tipos de fichero AMSDOS

| Tipo | Descripción |
|------|-------------|
| `ascii` | Fichero de texto ASCII |
| `binary` | Fichero binario con cabecera AMSDOS (load/exec address) |
| `raw` | Fichero sin cabecera AMSDOS |

---

## Notas

- Los wildcards en nombres de ficheros (`*.BIN`, `*.BAS`) se aplican a los nombres **dentro del DSK**, no en el sistema de ficheros local.
- El rango de usuario (`-u`) va de 0 a 15 siguiendo el estándar CP/M del CPC.
- `disc` es el nombre real del binario. La herramienta está en el directorio `disc/` del workspace.
