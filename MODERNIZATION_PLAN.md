# Plan de Modernización: iDSK → Disc

## Resumen Ejecutivo

Migración de iDSK (C++ legacy) a una herramienta CLI moderna llamada `disc`, con arquitectura limpia, mejor UX y preparada para futura GUI desktop.

---

## Fase 1: CLI Moderna (`disc`)

### Objetivos
- ✅ Binario compilado multiplataforma
- ✅ Experiencia de usuario moderna
- ✅ Código mantenible y testeable
- ✅ 100% compatibilidad funcional con iDSK
- ✅ Mejor manejo de errores y mensajes
- ✅ Documentación integrada

---

## Opciones de Lenguaje

### 🏆 Opción 1: Rust (RECOMENDADO)

**Por qué Rust:**
- ✅ Seguridad de memoria sin garbage collector
- ✅ Rendimiento equivalente a C++
- ✅ Excelente manejo de errores con `Result<T, E>`
- ✅ Ecosistema CLI maduro (clap, anyhow, thiserror)
- ✅ Cross-compilation sencilla
- ✅ Testing integrado
- ✅ Documentación como ciudadano de primera clase
- ✅ Preparado para GUI (Tauri, egui, iced)

**Stack Propuesto:**
```toml
[dependencies]
clap = { version = "4.5", features = ["derive", "cargo"] }
anyhow = "1.0"      # Error handling
thiserror = "1.0"   # Custom errors
serde = { version = "1.0", features = ["derive"] }
byteorder = "1.5"   # Endianness
colored = "2.1"     # Terminal colors
indicatif = "0.17"  # Progress bars
```

**Estructura del Proyecto:**
```
disc/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # Command definitions (clap)
│   ├── dsk/
│   │   ├── mod.rs        # DSK module
│   │   ├── format.rs     # DSK format structures
│   │   ├── reader.rs     # Read operations
│   │   ├── writer.rs     # Write operations
│   │   └── catalog.rs    # Directory management
│   ├── amsdos/
│   │   ├── mod.rs
│   │   ├── header.rs     # AMSDOS header
│   │   └── types.rs      # File types
│   ├── viewers/
│   │   ├── mod.rs
│   │   ├── basic.rs      # BASIC viewer
│   │   ├── hex.rs        # Hex viewer
│   │   ├── disasm.rs     # Z80 disassembler
│   │   └── ascii.rs      # ASCII viewer
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── list.rs       # List command
│   │   ├── import.rs     # Import command
│   │   ├── export.rs     # Export command
│   │   ├── remove.rs     # Remove command
│   │   └── create.rs     # Create command
│   ├── error.rs          # Error types
│   └── utils.rs          # Utilities
├── tests/
│   ├── integration/
│   └── fixtures/
└── README.md
```

**Ventajas:**
- Mejor manejo de errores que C++
- Sin undefined behavior
- Concurrencia segura (futuro)
- Comunidad activa y creciente
- Tooling excelente (cargo, rustfmt, clippy)

**Desventajas:**
- Curva de aprendizaje inicial (borrow checker)
- Tiempos de compilación más largos

**Ejemplo de Código:**
```rust
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "disc")]
#[command(about = "Modern DSK image manipulation tool", long_about = None)]
struct Cli {
    /// DSK image file
    image: PathBuf,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List files in the DSK image
    List,
    /// Import files into the DSK image
    Import {
        /// Files to import
        files: Vec<PathBuf>,
        #[arg(short, long)]
        force: bool,
    },
    // ...
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut dsk = Dsk::open(&cli.image)
        .context("Failed to open DSK image")?;
    
    match cli.command {
        Commands::List => dsk.list_catalog(),
        Commands::Import { files, force } => {
            for file in files {
                dsk.import_file(&file, force)?;
            }
            Ok(())
        }
    }
}
```

---

### Opción 2: Go

**Por qué Go:**
- ✅ Sintaxis simple y legible
- ✅ Compilación rápida
- ✅ Cross-compilation trivial
- ✅ Binarios estáticos sin dependencias
- ✅ Garbage collector eficiente
- ✅ Excelente para CLI (cobra, viper)

**Stack Propuesto:**
```
github.com/spf13/cobra      # CLI framework
github.com/fatih/color      # Terminal colors
github.com/schollz/progressbar/v3
```

**Ventajas:**
- Curva de aprendizaje suave
- Compilación muy rápida
- Deployment simple
- Buen rendimiento

**Desventajas:**
- Garbage collector (pausas impredecibles)
- Menos control sobre memoria que Rust/C++
- Manejo de errores verboso
- No ideal para GUI nativa (Fase 2)

---

### Opción 3: Zig

**Por qué Zig:**
- ✅ Sintaxis moderna, más simple que Rust
- ✅ Control total de memoria sin GC
- ✅ Interop C excelente (podría reusar código)
- ✅ Compilación cruzada integrada
- ✅ Rendimiento C++

**Ventajas:**
- Más simple que Rust
- Sin runtime ni GC
- Excelente para sistemas

**Desventajas:**
- ⚠️ Lenguaje aún no estable (pre-1.0)
- Ecosistema inmaduro
- Menos librerías disponibles
- Documentación limitada
- Riesgo de breaking changes

---

### Opción 4: C++ Moderno (C++20/23)

**Por qué C++ Moderno:**
- ✅ Evolución natural del código existente
- ✅ Rendimiento máximo
- ✅ Reutilización parcial de código
- ✅ Ecosistema maduro

**Stack Propuesto:**
```
- CLI11 (parsing CLI)
- fmt (formatting)
- spdlog (logging)
- Catch2 (testing)
- CMake + vcpkg
```

**Ventajas:**
- Menor reescritura inicial
- Conocimiento existente del dominio
- Rendimiento óptimo

**Desventajas:**
- Sigue siendo C++ (complejidad, UB posible)
- Tooling fragmentado
- Manejo de errores manual
- No resuelve problemas fundamentales del código legacy

---

## Comparativa Rápida

| Criterio | Rust | Go | Zig | C++20 |
|----------|------|----|----|-------|
| **Seguridad** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |
| **Rendimiento** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Curva Aprendizaje** | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Ecosistema CLI** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **Cross-compile** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Madurez** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **GUI Futuro** | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Mantenibilidad** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |

---

## Recomendación Final: **Rust** 🦀

### Justificación
1. **Mejor balance** entre seguridad, rendimiento y experiencia de desarrollo
2. **Ecosistema CLI maduro** con herramientas excelentes
3. **Preparado para Fase 2** (GUI con Tauri o iced)
4. **Mantenibilidad a largo plazo** superior
5. **Comunidad activa** y en crecimiento
6. **Tooling de primera clase** (cargo, clippy, rustfmt)

### Alternativa Pragmática: **Go**
Si la curva de aprendizaje de Rust es un blocker, Go es una excelente segunda opción para la CLI, aunque requerirá cambio de stack para la GUI.

---

## Mejoras de UX Propuestas

### CLI Moderna con Subcomandos
```bash
# Antes (iDSK)
iDSK image.dsk -l
iDSK image.dsk -i file.bin -t 1 -e C000 -c 4000

# Después (disc)
disc list image.dsk
disc import image.dsk file.bin --type binary --exec 0xC000 --load 0x4000
disc import image.dsk *.bas --force
disc export image.dsk file.bin -o output/
disc create image.dsk --tracks 40 --sectors 9
disc view image.dsk file.bas --format basic
```

### Características Nuevas
- ✅ **Colores en terminal** para mejor legibilidad
- ✅ **Progress bars** para operaciones largas
- ✅ **Validación proactiva** con mensajes claros
- ✅ **Autocompletado** (shell completions)
- ✅ **Formato de salida flexible** (table, json, csv)
- ✅ **Modo interactivo** opcional
- ✅ **Wildcards** para operaciones batch
- ✅ **Dry-run mode** para preview

### Ejemplo de Salida Mejorada
```
$ disc list retro.dsk

📀 DSK Image: retro.dsk
   Tracks: 40 | Sectors: 9 | Format: DATA
   Used: 89 KB / 178 KB (50%)

┌──────────────┬──────┬──────────┬────────┐
│ Name         │ Type │ Size     │ Attrs  │
├──────────────┼──────┼──────────┼────────┤
│ GAME.BAS     │ BASIC│ 12 KB    │        │
│ LOADER.BIN   │ BIN  │ 8 KB     │ RO SYS │
│ MUSIC.BIN    │ BIN  │ 16 KB    │        │
│ SPRITES.BIN  │ BIN  │ 32 KB    │        │
└──────────────┴──────┴──────────┴────────┘

4 files, 68 KB total
```

---

## Roadmap de Implementación

### Sprint 1: Fundamentos (2 semanas)
- [ ] Setup proyecto Rust + CI/CD
- [ ] Estructuras DSK format (header, track, sector)
- [ ] Parser DSK básico (lectura)
- [ ] Tests con imágenes de ejemplo
- [ ] Comando `list` funcional

### Sprint 2: Operaciones Core (2 semanas)
- [ ] Comando `import` (ASCII y binario)
- [ ] Comando `export`
- [ ] Comando `remove`
- [ ] Comando `create`
- [ ] Manejo AMSDOS headers

### Sprint 3: Viewers (1 semana)
- [ ] Viewer BASIC
- [ ] Viewer hexadecimal
- [ ] Viewer ASCII
- [ ] Desensamblador Z80 básico

### Sprint 4: Polish & Release (1 semana)
- [ ] Documentación completa
- [ ] Ejemplos y tutoriales
- [ ] Binarios para Linux/macOS/Windows
- [ ] Homebrew formula / apt package
- [ ] Release 1.0.0

---

## Fase 2: GUI Desktop (Futuro)

### Opciones de Framework

#### Con Rust (si elegimos Rust en Fase 1)
1. **Tauri** (recomendado)
   - Web tech (HTML/CSS/JS) + Rust backend
   - Binarios pequeños (~3MB)
   - Multiplataforma nativo
   
2. **iced**
   - GUI nativa en Rust puro
   - Inspirado en Elm
   - Más control, más código

3. **egui**
   - Immediate mode GUI
   - Rápido de prototipar
   - Bueno para herramientas

#### Con C++
- **Qt** (más maduro, más pesado)
- **wxWidgets** (nativo, más complejo)

### Features GUI Propuestas
- Drag & drop de archivos
- Preview visual de archivos BASIC/binarios
- Editor hexadecimal integrado
- Búsqueda y filtrado
- Operaciones batch
- Comparación de DSK
- Conversión de formatos

---

## Migración de Datos

### Compatibilidad
- ✅ 100% compatible con formato DSK existente
- ✅ Lee imágenes creadas por iDSK
- ✅ Imágenes creadas por `disc` funcionan en emuladores
- ✅ Sin cambios en formato de archivo

### Testing de Compatibilidad
- Suite de imágenes DSK de prueba
- Comparación byte-a-byte con iDSK
- Tests con emuladores (RetroVirtualMachine, WinAPE)

---

## Recursos y Referencias

### Documentación Técnica
- [DSK Format Specification](http://www.cpcwiki.eu/index.php/Disk_structure)
- [AMSDOS File System](http://www.cpm8680.com/cpmtools/cpm.htm)
- [Z80 Instruction Set](http://www.z80.info/z80code.htm)

### Herramientas Relacionadas
- **CPCDiskXP** (Windows GUI)
- **DSKTools** (Python)
- **ManageDSK** (predecesor de iDSK)

### Comunidad
- [CPC Wiki](http://www.cpcwiki.eu/)
- [CPCRulez](http://www.cpcrulez.fr/)
- [Amstrad CPC Discord](https://discord.gg/amstrad)

---

## Decisión Requerida

**¿Qué lenguaje elegimos para Fase 1?**

1. **Rust** (recomendado) - Mejor opción a largo plazo
2. **Go** - Más rápido de implementar, menos ideal para GUI
3. **Zig** - Interesante pero arriesgado
4. **C++20** - Seguro pero no resuelve problemas fundamentales

**Siguiente paso**: Confirmar elección y crear proyecto base.
