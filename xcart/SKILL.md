---
name: xcart
description: Crear y gestionar imágenes .cpr de cartucho GX-4000 para Amstrad desde imágenes DSK con xcart. Usar cuando el usuario pida convertir un DSK a cartucho CPR, verificar la estructura de un CPR, inspeccionar metadatos de DSK o CPR, listar chunks de cartucho, extraer ROMs o datos de un cartucho, o hacer dump de sectores de un DSK. También para configurar autostart BASIC en cartuchos. Si el binario xcart no está disponible, indicar al usuario que compile el proyecto en xcart/ con `cargo build --release`.
---

# xcart

Herramienta Rust para convertir imágenes **DSK** de Amstrad CPC en cartuchos **CPR** para la consola GX-4000.

Embebe internamente las ROMs de OS, BASIC y AMSDOS, parcheando AMSDOS para el formato de disco y el autostart opcional.

**Binario:** `xcart` (compilar en `xcart/` con `cargo build --release --target aarch64-apple-darwin`)

---

## Verificar disponibilidad

```bash
xcart --version
# Si falla: cd xcart && cargo build --release --target aarch64-apple-darwin
```

---

## Comandos disponibles

### `xcart create` — Crear cartucho CPR desde DSK

Convierte un fichero DSK en un cartucho CPR para GX-4000. Embebe OS + BASIC + AMSDOS ROMs, parchea AMSDOS para el formato del disco y empaqueta los sectores como data chunks.

```bash
xcart create <INPUT.DSK> <OUTPUT.CPR> [OPCIONES]
```

| Opción | Descripción |
|--------|-------------|
| `-c, --command <CMD>` | Comando BASIC de autostart al arrancar (máx 16 chars) |

**Ejemplos:**
```bash
# Conversión básica
xcart create game.dsk game.cpr

# Con autostart BASIC: ejecuta DISC automáticamente
xcart create game.dsk game.cpr -c 'run"disc"'

# Con autostart CPM
xcart create game.dsk game.cpr -c '|cpm'

# Con autostart directo
xcart create game.dsk game.cpr -c 'run"game"'
```

El fichero CPR resultante tiene la estructura RIFF/AMS! con chunks indexados:
- Chunk 0: OS ROM (16 KB)
- Chunk 1: BASIC ROM (16 KB)  
- Chunk 2: AMSDOS ROM (16 KB, parcheado)
- Chunks 3+: datos del disco (sectores)

---

### `xcart check` — Verificar estructura de un CPR

Valida la cabecera RIFF/AMS!, los tags de cada chunk, los tamaños y la integridad de datos. Informa de errores si el fichero está corrupto o truncado.

```bash
xcart check <INPUT.CPR>
```

**Ejemplo:**
```bash
xcart check game.cpr
# Salida: OK o lista de errores encontrados
```

Devuelve código de salida 1 si hay errores.

---

### `xcart info` — Metadatos de DSK o CPR

Detecta automáticamente el tipo de fichero por extensión o magic bytes y muestra sus metadatos.

```bash
xcart info <INPUT>
```

**Ejemplos:**
```bash
# Metadatos de un DSK
xcart info game.dsk

# Metadatos de un CPR
xcart info game.cpr
```

Para un DSK muestra: formato de disco, número de pistas, sectores, tamaño.  
Para un CPR muestra: número de chunks, tamaño total, versión.

---

### `xcart list` — Listar chunks de un CPR

Muestra una tabla formateada con todos los chunks del cartucho: índice, tag, tamaño y descripción.

```bash
xcart list <INPUT.CPR> [-v]
```

| Opción | Descripción |
|--------|-------------|
| `-v, --verbose` | Muestra preview hex de los primeros 16 bytes de cada chunk |

**Ejemplo:**
```bash
xcart list game.cpr
xcart list game.cpr -v   # con preview hex
```

**Índices de chunks:**
| Índice | Contenido |
|--------|-----------|
| 0 | OS ROM (16 KB) |
| 1 | BASIC ROM (16 KB) |
| 2 | AMSDOS ROM parcheado (16 KB) |
| 3+ | Datos de sectores del disco |

---

### `xcart extract` — Extraer un chunk a fichero

Extrae un chunk individual del cartucho CPR a un fichero binario.

```bash
xcart extract <INPUT.CPR> <CHUNK_INDEX> <OUTPUT>
```

El índice de chunk es **0-based**.

**Ejemplos:**
```bash
# Extraer OS ROM
xcart extract game.cpr 0 os.rom

# Extraer BASIC ROM
xcart extract game.cpr 1 basic.rom

# Extraer AMSDOS ROM (parcheado)
xcart extract game.cpr 2 amsdos.rom

# Extraer primer chunk de datos del disco
xcart extract game.cpr 3 disk_data.bin
```

---

### `xcart dump` — Dump de sectores de un DSK

Vuelca los datos crudos de sectores de un DSK a un fichero binario, en orden pista/identificador, sin ninguna cabecera. Útil para parcheado manual antes de re-ensamblar un cartucho.

```bash
xcart dump <INPUT.DSK> <OUTPUT.BIN>
```

**Ejemplo:**
```bash
xcart dump game.dsk sectors.bin
# → sectors.bin contiene todos los sectores concatenados
```

---

## Formato CPR

Un fichero `.cpr` sigue el formato **RIFF/AMS!**:
- Cabecera RIFF 12 bytes: `"RIFF"` + tamaño + `"AMS!"`
- Cada chunk: tag 4 bytes + tamaño 4 bytes + datos

Los chunks de ROM siempre están presentes (OS, BASIC, AMSDOS). Los chunks de datos del disco siguen a continuación, empaquetando los sectores del DSK en bloques de 16 KB.

---

## Flujo típico

```bash
# 1. Inspeccionar el DSK original
xcart info game.dsk

# 2. Convertir a CPR con autostart
xcart create game.dsk game.cpr -c 'run"game"'

# 3. Verificar integridad
xcart check game.cpr

# 4. Listar chunks generados
xcart list game.cpr -v

# 5. Extraer AMSDOS parcheado para inspección
xcart extract game.cpr 2 amsdos_patched.rom
```

---

## Notas de compatibilidad

- El DSK debe estar en formato Extended DSK o Standard DSK de Amstrad.
- El comando de autostart (`-c`) admite hasta 16 caracteres; se envía como pulsaciones de teclado BASIC al arrancar.
- Los cartuchos CPR generados son compatibles con emuladores como **Retro Virtual Machine 2** y hardware GX-4000 real.
- Si el DSK tiene más sectores de los que caben en un cartucho estándar, xcart informa del desbordamiento.
