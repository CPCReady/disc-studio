use super::{DirEntry, Dsk, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES};
use crate::error::Result;
use crate::utils;

#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub name: String,
    pub size: usize,
    #[allow(dead_code)]
    pub user: u8,
    pub read_only: bool,
    pub system: bool,
    pub file_type: String,
}

pub struct Catalog {
    pub entries: Vec<CatalogEntry>,
    pub total_size: usize,
    pub free_space: usize,
}

impl Catalog {
    pub fn from_dsk(dsk: &Dsk) -> Result<Self> {
        let mut entries = Vec::new();
        let mut total_size = 0;

        // Read directory using block reads to correctly handle sector interleaving.
        // The directory occupies blocks 0 and 1 (4 sectors: 0xC1, 0xC2, 0xC3, 0xC4).
        // Reading via read_block() searches sectors by ID, not by physical position.
        let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
        dir_data.extend_from_slice(&dsk.read_block(0)?);
        dir_data.extend_from_slice(&dsk.read_block(1)?);

        for i in 0..MAX_DIR_ENTRIES {
            let offset = i * DIR_ENTRY_SIZE;
            if offset + DIR_ENTRY_SIZE > dir_data.len() {
                break;
            }

            let dir_entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;

            if !dir_entry.is_deleted() && dir_entry.is_first_extent() {
                let name =
                    utils::from_amsdos_name(&[&dir_entry.name[..], &dir_entry.ext[..]].concat());

                // Calculate file size by counting pages across all extents
                let mut page_count = 0;

                for j in i..MAX_DIR_ENTRIES {
                    let check_offset = j * DIR_ENTRY_SIZE;
                    if check_offset + DIR_ENTRY_SIZE > dir_data.len() {
                        break;
                    }

                    let check_entry = DirEntry::from_bytes(
                        &dir_data[check_offset..check_offset + DIR_ENTRY_SIZE],
                    )?;

                    if check_entry.user == dir_entry.user
                        && check_entry.name == dir_entry.name
                        && check_entry.ext == dir_entry.ext
                    {
                        page_count += check_entry.num_pages as usize;
                    }
                }

                let size = (page_count * 128).div_ceil(1024) * 1024; // Round up to KB
                total_size += size;

                let file_type = detect_file_type(dsk, &dir_entry);

                entries.push(CatalogEntry {
                    name,
                    size,
                    user: dir_entry.user,
                    read_only: (dir_entry.ext[0] & 0x80) != 0,
                    system: (dir_entry.ext[1] & 0x80) != 0,
                    file_type,
                });
            }
        }

        let disk_capacity: usize = 178 * 1024; // Standard 178KB disk
        let free_space = disk_capacity.saturating_sub(total_size);

        Ok(Self {
            entries,
            total_size,
            free_space,
        })
    }
}

fn detect_file_type(dsk: &Dsk, dir_entry: &DirEntry) -> String {
    // Read first block to check for AMSDOS header
    if dir_entry.blocks[0] == 0 {
        return "ASCII".to_string();
    }

    if let Ok(block) = dsk.read_block(dir_entry.blocks[0]) {
        // Check AMSDOS header
        if crate::amsdos::has_header(&block) {
            let file_type = block[18]; // FileType field in AMSDOS header
            return match file_type {
                0 => "BASIC".to_string(),
                1 => "BASIC(P)".to_string(),
                2 => "BINARY".to_string(),
                3 => "BINARY(P)".to_string(),
                _ => "ASCII".to_string(),
            };
        }

        // No AMSDOS header - check if tokenized BASIC
        if crate::viewers::basic::is_tokenized(&block) {
            return "BASIC".to_string();
        }

        // Check if ASCII (mostly printable)
        let printable = block
            .iter()
            .take(128)
            .filter(|&&b| (0x20..0x7F).contains(&b) || b == b'\r' || b == b'\n' || b == 0x00)
            .count();
        if printable > 100 {
            return "ASCII".to_string();
        }
    }

    "BINARY".to_string()
}
