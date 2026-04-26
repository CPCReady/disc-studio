# disc — Modern Amstrad CPC DSK Image Tool

A fast, modern command-line tool for working with Amstrad CPC DSK disk images, written in Rust.

Replaces the legacy `iDSK` C++ tool with a clean, cross-platform CLI that handles AMSDOS headers, sector interleaving, and CP/M directory format correctly.

---

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Global Options](#global-options)
- [Commands](#commands)
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
- [File Types and AMSDOS Headers](#file-types-and-amsdos-headers)
- [DSK Format Reference](#dsk-format-reference)
- [Shell Completions](#shell-completions)
- [Environment Variables](#environment-variables)
- [Exit Codes](#exit-codes)
- [Compatibility](#compatibility)
- [License](#license)

---

## Installation

### From source

```bash
git clone https://github.com/CPCReady/Disc-Image-Studio.git
cd Disc-Image-Studio/disc
cargo build --release
# Binary at: target/release/disc
```

### Install globally via Cargo

```bash
cargo install --path disc/
```

### Pre-built binaries

Run the release script to get a native binary:

```bash
./build-release.sh --native
# Output: dist/disc-macos-aarch64  (or linux-x86_64, etc.)
```

---

## Quick Start

```bash
# 1. Create a blank 40-track DATA disk
disc create game.dsk

# 2. Import files
disc import game.dsk loader.bas
disc import game.dsk sprites.bin --file-type binary --load 0x4000 --exec 0x4000

# 3. List contents
disc list game.dsk

# 4. View a file directly
disc view game.dsk LOADER.BAS --format basic

# 5. Export files
disc export game.dsk "*.BAS" --output backup/

# 6. Check disk integrity
disc check game.dsk

# 7. Detailed statistics
disc info game.dsk

# 8. Compare two disk images
disc diff game.dsk game_backup.dsk

# 9. Copy files between disk images
disc copy src.dsk dst.dsk "*.BAS"

# 10. Remove a file
disc remove game.dsk OLDFILE.BIN --force
```

---

## Global Options

These options can be used with **any** subcommand:

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Enable debug logging (shows internal operations) |
| `--no-color` | — | Disable colored output (useful for scripts/pipes) |
| `--help` | `-h` | Show help for the command |
| `--version` | `-V` | Print version number |

```bash
disc --verbose list game.dsk
disc --no-color list game.dsk | grep .BAS
disc --version
```

---

## Commands

### `create` / `new`

Create a new blank DSK image formatted as a standard Amstrad CPC DATA disk.

```
disc create <IMAGE> [OPTIONS]
disc new    <IMAGE> [OPTIONS]          # alias
```

| Option | Description | Default |
|--------|-------------|---------|
| `--tracks <N>` | Number of tracks | `40` |
| `--sectors <N>` | Sectors per track | `9` |
| `-f, --force` | Overwrite if the file already exists | off |

**Examples:**

```bash
# Standard 40-track DATA disk (178 KB)
disc create game.dsk

# 80-track disk (for extended images)
disc create extended.dsk --tracks 80

# Overwrite an existing disk
disc create game.dsk --force
```

> **Note:** The created disk uses standard CPC sector interleaving
> (`0xC1, 0xC6, 0xC2, 0xC7, 0xC3, 0xC8, 0xC4, 0xC9, 0xC5`) and is
> compatible with all major Amstrad CPC emulators.

---

### `list` / `ls`

List the files stored in a DSK image.

```
disc list <IMAGE> [OPTIONS]
disc ls   <IMAGE> [OPTIONS]            # alias
```

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--format <FORMAT>` | `-f` | Output format: `table`, `json`, `csv`, `simple` | `table` |

**Examples:**

```bash
# Human-readable table (default)
disc list game.dsk

# JSON — useful for scripting
disc list game.dsk --format json

# CSV — for spreadsheets or grep
disc list game.dsk --format csv

# Simple list — just filenames, one per line
disc list game.dsk --format simple

# Filter using shell tools
disc list game.dsk --format simple | grep "\.BAS"
```

**Table output:**

```
DSK Image: game.dsk
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

**Attrs column:**

| Symbol | Meaning |
|--------|---------|
| `R` | Read-only |
| `S` | System file (hidden) |

**JSON output structure:**

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

Import one or more files from your host filesystem into a DSK image.

```
disc import <IMAGE> <FILES>... [OPTIONS]
disc add    <IMAGE> <FILES>... [OPTIONS]   # alias
```

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--file-type <TYPE>` | `-t` | File type: `ascii`, `binary`, `raw` | auto-detect |
| `--load <ADDR>` | `-c` | Load address in hex (e.g. `0x4000`) | — |
| `--exec <ADDR>` | `-e` | Execution address in hex (e.g. `0xC000`) | — |
| `--user <N>` | `-u` | CP/M user number 0–15 | `0` |
| `--read-only` | `-o` | Mark as read-only | off |
| `--system` | `-s` | Mark as system file (hidden) | off |
| `--force` | `-f` | Overwrite if the file already exists in the disk | off |

**File type detection** (when `--file-type` is omitted):

| Condition | Detected type |
|-----------|---------------|
| Source file has a valid AMSDOS header | `binary` |
| Otherwise | `ascii` |

**File type behaviour:**

| Type | What `disc` does |
|------|-----------------|
| `ascii` | Strips any existing AMSDOS header; converts `LF` → `CR+LF` |
| `binary` | Adds an AMSDOS binary header with load/exec addresses if not already present |
| `raw` | Stores the file exactly as-is — no header added or removed |

**Examples:**

```bash
# ASCII BASIC source — auto-detected
disc import game.dsk loader.bas

# Explicit ASCII
disc import game.dsk notes.txt --file-type ascii

# Binary with load and execution addresses
disc import game.dsk code.bin --file-type binary --load 0x4000 --exec 0x4000

# Binary — load only (no autostart)
disc import game.dsk data.bin --file-type binary --load 0x8000

# Raw file — stored exactly as-is
disc import game.dsk palette.dat --file-type raw

# Multiple files in one command
disc import game.dsk src/*.bas src/*.bin

# Import to CP/M user area 2
disc import game.dsk private.bin --user 2

# Read-only system file
disc import game.dsk boot.bin --file-type binary --read-only --system

# Overwrite an existing file
disc import game.dsk loader.bas --force
```

> If a file already exists in the disk and `--force` is not given, the error
> is printed to stderr and `disc` continues with the remaining files (exit code 0).

---

### `export` / `get`

Extract one or more files from a DSK image to your host filesystem.

```
disc export <IMAGE> <FILES>... [OPTIONS]
disc get    <IMAGE> <FILES>... [OPTIONS]   # alias
```

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--output <DIR>` | `-o` | Destination directory (created if it doesn't exist) | current directory |
| `--strip-header` | — | Remove the AMSDOS header from the exported file | off |

Filenames are **case-insensitive**. A trailing `*` wildcard is supported.

**Examples:**

```bash
# Export a single file to the current directory
disc export game.dsk LOADER.BAS

# Export to a specific directory
disc export game.dsk LOADER.BAS --output backup/

# Wildcard — export all BASIC files
disc export game.dsk "*.BAS" --output backup/

# Export without AMSDOS header (raw payload only)
disc export game.dsk SPRITES.BIN --strip-header --output raw/

# Export all files
disc export game.dsk "*" --output full_backup/
```

> The output directory is created automatically if it does not exist.

---

### `remove` / `rm`

Delete one or more files from a DSK image (marks directory entries as deleted; the disk is not reformatted).

```
disc remove <IMAGE> <FILES>... [OPTIONS]
disc rm     <IMAGE> <FILES>... [OPTIONS]   # alias
```

| Option | Short | Description |
|--------|-------|-------------|
| `--force` | `-f` | Remove without confirmation prompt |

**Examples:**

```bash
# Remove with confirmation
disc remove game.dsk OLDFILE.BIN

# Remove without confirmation
disc remove game.dsk OLDFILE.BIN --force

# Remove multiple files at once
disc remove game.dsk OLD1.BIN OLD2.BIN TEMP.DAT --force
```

> Removing a file that does not exist is a no-op (exit code 0).

---

### `copy` / `cp`

Copy one or more files from one DSK image to another, preserving AMSDOS headers and file attributes.

```
disc copy <SRC> <DST> [FILES]... [OPTIONS]
disc cp   <SRC> <DST> [FILES]... [OPTIONS]   # alias
```

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--force` | `-f` | Overwrite existing files in the destination | off |

**Wildcard patterns supported:**

| Pattern | Matches |
|---------|---------|
| `*` | All files |
| `*.EXT` | All files with that extension (e.g. `*.BAS`) |
| `NAME*` | All files whose name starts with `NAME` (e.g. `LEVEL*`) |
| `EXACT` | Exact filename (case-insensitive) |

**Examples:**

```bash
# Copy all files from src to dst
disc copy src.dsk dst.dsk

# Copy a single file
disc copy src.dsk dst.dsk LOADER.BAS

# Copy all BASIC files (wildcard by extension)
disc copy src.dsk dst.dsk "*.BAS"

# Copy all files starting with LEVEL
disc copy src.dsk dst.dsk "LEVEL*"

# Copy multiple patterns
disc copy src.dsk dst.dsk "*.BAS" SPRITES.BIN

# Overwrite existing files in the destination
disc copy src.dsk dst.dsk "*.BAS" --force
```

> AMSDOS headers, load/exec addresses and read-only/system attribute bits are preserved exactly as they are in the source image.

> If a file already exists in the destination and `--force` is not given, the error is printed to stderr and `disc` continues with the remaining files.

---

### `view`

Display the contents of a file stored in a DSK image without extracting it.

```
disc view <IMAGE> <FILE> [OPTIONS]
```

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--format <FORMAT>` | `-f` | Viewer: `auto`, `basic`, `hex`, `ascii`, `disasm` | `auto` |

**Viewer modes:**

| Mode | Description |
|------|-------------|
| `auto` | Auto-detects: tries BASIC listing first, falls back to hex dump |
| `basic` | Amstrad CPC tokenized BASIC listing |
| `hex` | Hex + ASCII side-by-side dump |
| `ascii` | Plain text with CPC extended character handling |
| `disasm` | Z80 disassembly |

**Examples:**

```bash
# Auto-detect viewer
disc view game.dsk LOADER.BAS

# Force BASIC listing
disc view game.dsk LOADER.BAS --format basic

# Hex dump of a binary
disc view game.dsk SPRITES.BIN --format hex

# Z80 disassembly
disc view game.dsk CODE.BIN --format disasm

# Raw ASCII text
disc view game.dsk README.TXT --format ascii
```

**Hex output example:**

```
0000: 00 01 FF 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
0010: 80 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
```

**BASIC listing example:**

```
10 MODE 1
20 BORDER 0
30 PRINT "HELLO AMSTRAD CPC"
40 END
```

---

### `check`

Validate the integrity of a DSK image: header magic, track geometry, directory entries, AMSDOS headers and block allocation conflicts.

```
disc check <IMAGE>
```

**Exit code:** `0` if the disk is healthy, `1` if any issue is found.

**Example:**

```bash
disc check game.dsk
```

**Output (healthy disk):**

```
Checking game.dsk...

Header
  ✓ Magic valid (DATA format)
  ✓ 40 tracks, 1 head(s)

Directory
  ✓ 3 / 64 entries used
  ✓ LOADER.BAS — no AMSDOS header (ASCII/raw)
  ✓ SPRITES.BIN — AMSDOS header valid
  ✓ LEVEL1.DAT — AMSDOS header valid
  ✓ Block allocation OK (no conflicts)

Result: OK — 0 issues
```

**Output (corrupt disk):**

```
Checking bad.dsk...

Header
  ✗ Header invalid: Invalid DSK magic string

Result: 1 issue(s) found
Error: Check failed: 1 error(s) found
```

Use `--verbose` to see every check in detail:

```bash
disc --verbose check game.dsk
```

---

### `info`

Show detailed statistics about a DSK image: format, geometry, directory occupancy, block-map usage and a file listing with sizes and attributes.

```
disc info <IMAGE>
```

**Example:**

```bash
disc info game.dsk
```

**Output:**

```
DSK Image: game.dsk
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

Compare the contents of two DSK images and report which files are unique to each image, which have been modified, and which are identical.

```
disc diff <IMAGE1> <IMAGE2>
```

**Exit code:** Always `0` — differences are reported on stdout, not treated as errors.

**Example:**

```bash
disc diff game.dsk game_v2.dsk
```

**Output:**

```
Comparing game.dsk ↔ game_v2.dsk

Only in game.dsk:
  OLD_LEVEL.DAT  2 KB

Only in game_v2.dsk:
  LEVEL2.DAT     4 KB
  HISCORE.DAT    1 KB

Modified (same name, different content):
  LOADER.BAS     1 KB → 2 KB

Identical:
  SPRITES.BIN    8 KB
  MUSIC.BIN      4 KB

Summary: 2 added, 1 removed, 1 modified, 2 identical
```

**Common use cases:**

```bash
# Verify a backup is identical
disc diff original.dsk backup.dsk

# Check what changed between two versions
disc diff game_v1.dsk game_v2.dsk

# Diff with no-color for scripts
disc --no-color diff a.dsk b.dsk
```

---

## File Types and AMSDOS Headers

Every file stored on an Amstrad CPC DATA disk can optionally carry an **AMSDOS header** — a 128-byte prefix that tells the firmware the file type, load address, execution address and logical length.

| AMSDOS file type | Header byte | Meaning |
|-----------------|-------------|---------|
| `0x00` | BASIC | Tokenized Amstrad BASIC |
| `0x01` | BASIC(P) | Protected tokenized BASIC |
| `0x02` | BINARY | Machine code / binary data |
| `0x03` | BINARY(P) | Protected binary |
| *(none)* | — | ASCII text, raw data |

When you import with `--file-type binary`, `disc` generates a valid AMSDOS header automatically including the checksum. When you export, the header is preserved by default; use `--strip-header` to get the raw payload.

---

## DSK Format Reference

`disc` creates and reads standard Amstrad CPC **DATA format** disks:

| Property | Value |
|----------|-------|
| Tracks | 40 (default) |
| Sectors / track | 9 (default) |
| Sector size | 512 bytes |
| Block size | 1024 bytes (2 sectors) |
| Total blocks | 180 |
| Directory blocks | 2 (blocks 0–1, 64 entries) |
| Usable blocks | 178 (178 KB) |
| Sector IDs | `0xC1`–`0xC9` |

**Physical interleaving order** (2:1) per track:

```
Slot:    0     1     2     3     4     5     6     7     8
ID:    0xC1  0xC6  0xC2  0xC7  0xC3  0xC8  0xC4  0xC9  0xC5
```

`disc` correctly resolves this interleaving when reading and writing blocks — it never uses raw physical offsets for directory or file data.

---

## Shell Completions

Generate and install tab-completion scripts for your shell:

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

After installing, restart your shell (or source the file) to activate completions.

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Override the log level directly (e.g. `RUST_LOG=debug disc list game.dsk`) |
| `NO_COLOR` | Set to any value to disable colored output (same as `--no-color`) |

The `--verbose` flag sets the log level to **debug**. Without it, only warnings and errors are printed.

```bash
# Maximum verbosity via env var
RUST_LOG=trace disc check game.dsk

# Quiet — suppress all output except errors
RUST_LOG=error disc import game.dsk file.bas
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | An error occurred (file not found, corrupt DSK, check failed, etc.) |

Commands that process multiple files (e.g. `import`, `export`) continue on per-file errors and exit `0`; the individual errors are printed to **stderr**.

`disc check` exits `1` if any integrity issue is found, making it suitable for use in CI pipelines:

```bash
disc check release.dsk && echo "Disk OK" || echo "Disk has errors!"
```

---

## Compatibility

| Tool / Emulator | Status |
|----------------|--------|
| RetroVirtualMachine 2 | ✅ Tested |
| WinAPE | ✅ DSK format compatible |
| CPCDiskXP | ✅ DSK format compatible |
| JavaCPC | ✅ DSK format compatible |
| iDSK | ✅ Drop-in replacement |

---

## License

MIT License — Copyright (c) 2026 Destroyer

---

## Related Projects

- [iDSK](https://github.com/cpcsdk/idsk) — original C++ tool this replaces
- [RetroVirtualMachine](https://www.retrovm.com/) — Amstrad CPC emulator
- [2cdt](https://github.com/cpcsdk/2cdt) — CDT tape image tool
