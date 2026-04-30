// MIT License — Copyright (c) Destroyer 2026.
//
// ROMs are embedded at compile-time.
// Place os.rom, basic.rom and amsdos.rom in xcart/roms/ before building.

pub static OS_ROM: &[u8] = include_bytes!("../roms/os.rom");
pub static BASIC_ROM: &[u8] = include_bytes!("../roms/basic.rom");
pub static AMSDOS_ROM: &[u8] = include_bytes!("../roms/amsdos.rom");
