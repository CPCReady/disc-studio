# xcart — Conversor DSK → Cartucho GX-4000 para Amstrad

**xcart** convierte imágenes de disco **DSK** del Amstrad CPC en cartuchos **CPR** para la consola **Amstrad GX-4000**. Embebe internamente las ROMs de OS, BASIC y AMSDOS, parcheando AMSDOS para el formato de disco y el autostart opcional.

---

## Índice

- [Instalación](#instalación)
- [Inicio rápido](#inicio-rápido)
- [Requisito: ROMs](#requisito-roms)
- [Comandos](#comandos)
  - [create — Crear cartucho CPR](#create--crear-cartucho-cpr)
  - [check — Verificar integridad](#check--verificar-integridad)
  - [info — Metadatos de DSK o CPR](#info--metadatos-de-dsk-o-cpr)
  - [list — Listar chunks del cartucho](#list--listar-chunks-del-cartucho)
  - [extract — Extraer un chunk](#extract--extraer-un-chunk)
  - [dump — Dump de sectores de un DSK](#dump--dump-de-sectores-de-un-dsk)
- [Formato CPR](#formato-cpr)
- [Flujo típico](#flujo-típico)
- [Compatibilidad](#compatibilidad)

---

## Instalación

### Requisito previo: ROMs

xcart necesita las ROMs del firmware de Amstrad CPC para generar el cartucho. Deben estar en `xcart/roms/`:

```
xcart/roms/
  os.rom       (16 KB — ROM del sistema operativo)
  basic.rom    (16 KB — ROM de Amstrad BASIC)
  amsdos.rom   (16 KB — ROM de AMSDOS, será parcheada)
```

> Estas ROMs son propiedad de Amstrad/Sky. Obtenlas de una fuente legal
> (dump de hardware real o distribuidores autorizados).

### Compilar desde fuente

```bash
git clone https://github.com/CPCReady/Disc-Image-Studio.git
cd Disc-Image-Studio/xcart

# Verificar que las ROMs están en su sitio
ls roms/

# Compilar
cargo build --release --target aarch64-apple-darwin

# Binario en:
./target/aarch64-apple-darwin/release/xcart
```

### Usar el script de build del proyecto

```bash
# Desde la raíz del proyecto:
./build.sh --cli-only
# → dist/xcart  (solo si las ROMs están disponibles)
```

### Instalar globalmente

```bash
cp dist/xcart /usr/local/bin/xcart
xcart --version
```

---

## Inicio rápido

```bash
# Inspeccionar el DSK
xcart info juego.dsk

# Convertir a cartucho con autostart
xcart create juego.dsk juego.cpr -c 'run"game"'

# Verificar el cartucho generado
xcart check juego.cpr

# Ver los chunks del cartucho
xcart list juego.cpr
```

---

## Comandos

### `create` — Crear cartucho CPR

Convierte un fichero DSK en un cartucho CPR para GX-4000. Embebe OS + BASIC + AMSDOS ROMs, parchea AMSDOS para el formato del disco y empaqueta los sectores como data chunks.

```bash
xcart create <ENTRADA.DSK> <SALIDA.CPR> [OPCIONES]
```

| Opción | Descripción |
|--------|-------------|
| `-c, --command <CMD>` | Comando BASIC de autostart al arrancar (máx 16 chars) |

**Ejemplos:**

```bash
# Conversión básica sin autostart
xcart create juego.dsk juego.cpr

# Con autostart: ejecuta el disco automáticamente
xcart create juego.dsk juego.cpr -c 'run"disc"'

# Con autostart CPM
xcart create juego.dsk juego.cpr -c '|cpm'

# Con autostart directo
xcart create juego.dsk juego.cpr -c 'run"game"'
```

**Estructura del CPR generado:**

| Chunk | Contenido |
|-------|-----------|
| 0 | OS ROM (16 KB) |
| 1 | BASIC ROM (16 KB) |
| 2 | AMSDOS ROM parcheado (16 KB) |
| 3+ | Datos del disco (sectores en bloques de 16 KB) |

---

### `check` — Verificar integridad

Valida la cabecera RIFF/AMS!, los tags de cada chunk, los tamaños y la integridad de datos. Informa de errores si el fichero está corrupto o truncado.

```bash
xcart check <ENTRADA.CPR>
```

**Ejemplo:**

```bash
xcart check juego.cpr
# → OK  o  lista de errores encontrados
```

Sale con código `1` si hay errores. Útil para verificar antes de grabar en hardware real:

```bash
xcart check juego.cpr && echo "Cartucho OK" || echo "Cartucho con errores"
```

---

### `info` — Metadatos de DSK o CPR

Detecta automáticamente el tipo de fichero y muestra sus metadatos.

```bash
xcart info <ENTRADA>
```

**Ejemplos:**

```bash
# Información de un DSK
xcart info juego.dsk
# → formato, pistas, sectores, tamaño

# Información de un CPR
xcart info juego.cpr
# → número de chunks, tamaño total, versión RIFF
```

---

### `list` — Listar chunks del cartucho

Muestra una tabla con todos los chunks del cartucho: índice, tag, tamaño y descripción.

```bash
xcart list <ENTRADA.CPR> [-v]
```

| Opción | Descripción |
|--------|-------------|
| `-v, --verbose` | Muestra preview hex de los primeros 16 bytes de cada chunk |

**Ejemplo:**

```bash
xcart list juego.cpr
xcart list juego.cpr -v   # con preview hex
```

**Índices estándar:**

| Índice | Contenido |
|--------|-----------|
| 0 | OS ROM (16 KB) |
| 1 | BASIC ROM (16 KB) |
| 2 | AMSDOS ROM parcheado (16 KB) |
| 3+ | Datos de sectores del disco |

---

### `extract` — Extraer un chunk

Extrae un chunk individual del cartucho CPR a un fichero binario. El índice es **0-based**.

```bash
xcart extract <ENTRADA.CPR> <INDICE_CHUNK> <SALIDA>
```

**Ejemplos:**

```bash
# Extraer OS ROM
xcart extract juego.cpr 0 os.rom

# Extraer BASIC ROM
xcart extract juego.cpr 1 basic.rom

# Extraer AMSDOS ROM (parcheado para este disco)
xcart extract juego.cpr 2 amsdos_parcheado.rom

# Extraer primer bloque de datos del disco
xcart extract juego.cpr 3 datos_disco.bin
```

---

### `dump` — Dump de sectores de un DSK

Vuelca los datos crudos de sectores de un DSK a un fichero binario, en orden pista/identificador, sin ninguna cabecera. Útil para parcheado manual antes de re-ensamblar un cartucho.

```bash
xcart dump <ENTRADA.DSK> <SALIDA.BIN>
```

**Ejemplo:**

```bash
xcart dump juego.dsk sectores.bin
# → sectores.bin contiene todos los sectores concatenados
```

---

## Formato CPR

Un fichero `.cpr` sigue el formato **RIFF/AMS!**:

```
Cabecera RIFF (12 bytes):
  "RIFF" + tamaño_total (4 bytes LE) + "AMS!"

Cada chunk:
  tag (4 bytes) + tamaño (4 bytes LE) + datos
```

Los chunks de ROM siempre están presentes. Los chunks de datos del disco siguen a continuación, empaquetando los sectores del DSK en bloques de **16 KB**.

---

## Flujo típico

```bash
# 1. Inspeccionar el DSK original
xcart info juego.dsk

# 2. Convertir a CPR con autostart
xcart create juego.dsk juego.cpr -c 'run"game"'

# 3. Verificar integridad
xcart check juego.cpr

# 4. Listar chunks generados
xcart list juego.cpr -v

# 5. Extraer AMSDOS parcheado para inspección
xcart extract juego.cpr 2 amsdos_parcheado.rom

# 6. Cargar en emulador (con retrovirtualmachine)
rvm juego.cpr
```

---

## Compatibilidad

- El DSK debe estar en formato **Extended DSK** o **Standard DSK** de Amstrad.
- El comando de autostart (`-c`) admite hasta **16 caracteres**; se envía como pulsaciones de teclado BASIC al arrancar.
- Los cartuchos CPR generados son compatibles con emuladores como **Retro Virtual Machine 2** y con hardware **GX-4000 real**.
- Si el DSK tiene más sectores de los que caben en un cartucho estándar, xcart informa del desbordamiento.

| Emulador / Hardware | Estado |
|---------------------|--------|
| Retro Virtual Machine 2 | ✅ Probado |
| Hardware GX-4000 real | ✅ Compatible |
| WinAPE | ✅ CPR compatible |

---

## Licencia

GPL-2.0 — Copyright (c) Destroyer
