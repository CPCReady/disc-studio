# xDSK Desktop — Gestor visual de imágenes DSK para Amstrad CPC

**xDSK Desktop** es una aplicación de escritorio para trabajar con imágenes de disco **DSK** del Amstrad CPC de forma visual e intuitiva. Construida con Tauri + React, está disponible para macOS, Windows y Linux.

---

## Índice

- [Instalación](#instalación)
- [Primeros pasos](#primeros-pasos)
- [Interfaz principal](#interfaz-principal)
- [Abrir imágenes de disco](#abrir-imágenes-de-disco)
- [Explorar el contenido del disco](#explorar-el-contenido-del-disco)
- [Importar ficheros](#importar-ficheros)
- [Exportar ficheros](#exportar-ficheros)
- [Exportar cartucho CPR](#exportar-cartucho-cpr)
- [Borrar ficheros](#borrar-ficheros)
- [Ver ficheros sin extraer](#ver-ficheros-sin-extraer)
- [Verificar integridad del disco](#verificar-integridad-del-disco)
- [Comparar dos discos](#comparar-dos-discos)
- [Información del disco](#información-del-disco)
- [Consola de comandos](#consola-de-comandos)
- [Lanzar en emulador](#lanzar-en-emulador)
- [Requisitos del sistema](#requisitos-del-sistema)
- [Compilar desde fuente](#compilar-desde-fuente)

---

## Instalación

### macOS

1. Descarga el fichero `.dmg` desde la carpeta `dist/` del proyecto.
2. Abre el DMG y arrastra **xDSK Desktop** a tu carpeta Aplicaciones.
3. En el primer lanzamiento, haz clic derecho → **Abrir** (necesario la primera vez en macOS por Gatekeeper).

### Desde el script de build del proyecto

```bash
# Desde la raíz del proyecto:
./build.sh --gui-only

# El .app y el .dmg quedan en dist/
```

---

## Primeros pasos

Al abrir xDSK Desktop verás una pantalla de bienvenida con la barra lateral vacía:

```
┌─────────────────────────────────────────────────────────┐
│  xDSK Desktop                              [+]          │
│ ─────────────────────────────────────────────────────── │
│  No hay discos abiertos.                                │
│                                                         │
│  Haz clic en [+] para abrir una imagen DSK.             │
└─────────────────────────────────────────────────────────┘
```

---

## Interfaz principal

La ventana se divide en tres zonas:

```
┌────────────────┬────────────────────────────────────────┐
│   Barra        │                                        │
│   lateral      │    Área principal de trabajo           │
│                │                                        │
│  [+] Añadir    │  Lista de ficheros / Vista / Diff...   │
│                │                                        │
│  JUEGO.DSK     │                                        │
│  BACKUP.DSK    │                                        │
├────────────────┴────────────────────────────────────────┤
│   Consola de comandos (barra inferior)                  │
└─────────────────────────────────────────────────────────┘
```

| Zona | Descripción |
|------|-------------|
| **Barra lateral** | Lista de imágenes DSK abiertas. Haz clic en [+] para añadir más. |
| **Área principal** | Muestra el contenido del disco seleccionado con barra de herramientas. |
| **Consola inferior** | Registro de todos los comandos ejecutados y su salida. |

---

## Abrir imágenes de disco

Haz clic en el botón **[+]** de la barra lateral. Se abre un diálogo de selección de fichero donde puedes elegir cualquier fichero `.DSK`.

Puedes tener **varios discos abiertos simultáneamente** — cada uno aparece como una pestaña en la barra lateral.

Para **cerrar** un disco, pasa el ratón sobre su nombre en la barra lateral y haz clic en la ✕.

---

## Explorar el contenido del disco

Al seleccionar un disco en la barra lateral, el área principal muestra una **tabla de ficheros** con:

| Columna | Descripción |
|---------|-------------|
| **Nombre** | Nombre del fichero en el disco |
| **Tipo** | BASIC, BINARY, ASCII... |
| **Tamaño** | Tamaño en bytes |
| **R** | Indicador de solo lectura |
| **S** | Indicador de fichero sistema |

Puedes hacer clic en los encabezados de columna para **ordenar** la lista.

El panel superior muestra un **resumen del disco**: formato, pistas, espacio usado y libre.

---

## Importar ficheros

1. En la barra de herramientas, haz clic en el botón **Importar** (icono de carpeta con flecha).
2. Se abre un diálogo — selecciona uno o más ficheros de tu sistema.
3. Si los ficheros son binarios, puedes especificar:
   - **Tipo**: ASCII, Binary o Raw
   - **Dirección de carga** (`--load`)
   - **Dirección de ejecución** (`--exec`)
4. Haz clic en **Confirmar** para importar.

La lista de ficheros se actualiza automáticamente tras importar.

---

## Exportar ficheros

1. En la tabla de ficheros, **selecciona** los ficheros que quieres exportar (clic para uno, Ctrl+clic o Shift+clic para varios).
2. Haz clic en el botón **Exportar** de la barra de herramientas (icono de descarga).
3. Elige el directorio de destino en el diálogo.
4. Opcionalmente, activa **Eliminar cabecera AMSDOS** para exportar solo el payload.

---

## Exportar cartucho CPR

Puedes convertir el DSK activo a un cartucho **.CPR** (GX-4000) desde el botón **Export CPR**.

1. Abre **Settings** y configura **XCART ROMs Path** con la carpeta que contiene:
   - `os.rom`
   - `basic.rom`
   - `amsdos.rom`
2. Vuelve al explorador del disco y pulsa **Export CPR**.
3. Elige la ruta de salida `.cpr`.
4. Opcionalmente, añade un comando BASIC de arranque automático (máx. 16 caracteres).

> Si no configuras `XCART ROMs Path`, la exportación CPR fallará con un error indicando que faltan ROMs.

---

## Borrar ficheros

1. Selecciona los ficheros en la tabla.
2. Haz clic en el botón **Borrar** (icono de papelera) o pulsa **Delete**.
3. Confirma la acción en el diálogo de confirmación.

> El fichero se marca como borrado en el directorio del disco. El espacio queda libre para nuevos ficheros.

---

## Ver ficheros sin extraer

1. Haz **doble clic** en un fichero de la tabla, o selecciónalo y haz clic en **Ver**.
2. El visualizador detecta automáticamente el tipo de contenido:

   | Tipo de fichero | Vista por defecto |
   |-----------------|-------------------|
   | BASIC (`.BAS`) | Listado BASIC en texto |
   | Binario | Hex dump con columna ASCII |
   | Texto ASCII | Texto plano |
   | Código máquina | Desensamblado Z80 |

3. Puedes cambiar el modo de visualización con el selector en la barra de herramientas del visor.

---

## Verificar integridad del disco

1. Con un disco seleccionado, haz clic en el botón **Verificar** (icono de escudo).
2. El panel muestra el resultado del análisis:

   ```
   ✓ Cabecera DSK válida (formato DATA)
   ✓ 40 pistas, 1 cara
   ✓ Directorio: 3/64 entradas usadas
   ✓ Cabecera AMSDOS de LOADER.BAS: OK
   ✓ Asignación de bloques: sin conflictos

   Resultado: OK — 0 problemas
   ```

3. Si hay errores, aparecen marcados en rojo con una descripción del problema.

---

## Comparar dos discos

1. Haz clic en el botón **Comparar** de la barra de herramientas.
2. Selecciona el **segundo disco** en el diálogo (el primero es el disco activo).
3. El resultado muestra una tabla con cuatro categorías:

   | Categoría | Color | Descripción |
   |-----------|-------|-------------|
   | **Solo en A** | Azul | Ficheros únicos en el primer disco |
   | **Solo en B** | Verde | Ficheros únicos en el segundo disco |
   | **Modificado** | Amarillo | Mismo nombre pero contenido diferente |
   | **Idéntico** | Gris | Exactamente iguales en ambos discos |

---

## Información del disco

Haz clic en el botón **Info** de la barra de herramientas para ver estadísticas detalladas:

```
Formato:           DATA (CPC estándar)
Pistas:            40
Sectores/pista:    9
Tamaño de sector:  512 bytes
Capacidad total:   180 KB

Directorio:        3/64 entradas usadas

Mapa de bloques:
  Total:  180 bloques
  Usados: 8 (4.4%)
  Libres: 172 (95.6%)
```

---

## Consola de comandos

En la parte inferior de la ventana hay una **consola** que registra todos los comandos xdsk ejecutados internamente, con su salida completa. Esto es útil para:

- Depurar problemas
- Ver el equivalente en línea de comandos de cada acción
- Copiar comandos para usarlos en scripts

---

## Lanzar en emulador

Si tienes **Retro Virtual Machine** instalado, puedes lanzar el disco directamente:

1. Selecciona un disco en la barra lateral.
2. Haz clic en el botón **Emulador** (icono de monitor) de la barra de herramientas.
3. Si el emulador no se detecta automáticamente, configura la ruta en **Ajustes**.

---

## Requisitos del sistema

| Sistema | Requisito mínimo |
|---------|-----------------|
| macOS | 11.0 Big Sur o superior |
| Windows | Windows 10 64-bit |
| Linux | Ubuntu 20.04 / Debian 11 o equivalente |

La aplicación incluye los binarios `xdsk` y `xcart` como sidecars. Para exportar CPR necesitas configurar en Settings la carpeta de ROMs de xcart.

---

## Compilar desde fuente

### Requisitos

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) stable
- En macOS: Xcode Command Line Tools (`xcode-select --install`)
- En Linux: `libwebkit2gtk-4.1-dev`, `libssl-dev`, `libgtk-3-dev`

### Pasos

```bash
# 1. Clonar el repositorio
git clone https://github.com/CPCReady/Disc-Image-Studio.git
cd Disc-Image-Studio

# 2. Compilar la CLI xdsk y copiarla como sidecar
cd disc && cargo build --release --target aarch64-apple-darwin
cd ..
cp disc/target/aarch64-apple-darwin/release/xdsk \
   disc-desktop/src-tauri/binaries/xdsk-aarch64-apple-darwin

# 3. Instalar dependencias de Node
cd disc-desktop && npm install

# 4a. Modo desarrollo (lanza ventana con hot-reload)
npm run tauri dev

# 4b. Build de producción
npm run tauri build
# → disc-desktop/src-tauri/target/release/bundle/
```

---

## Licencia

MIT License — Copyright (c) 2026 Destroyer
