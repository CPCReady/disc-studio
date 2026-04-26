mod catalog;
mod format;
mod reader;
mod writer;

pub use catalog::Catalog;
pub use format::{
    DirEntry, DskHeader, TrackInfo, BLOCK_SIZE, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES, SECTOR_SIZE,
    USER_DELETED,
};
pub use reader::DskReader;
pub use writer::DskWriter;

use crate::error::{DskError, Result};
use std::path::Path;

/// Main DSK image structure
pub struct Dsk {
    data: Vec<u8>,
    header: DskHeader,
}

impl Dsk {
    /// Open an existing DSK image
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        DskReader::read(path)
    }

    /// Create a new DSK image
    pub fn create(tracks: u8, sectors: u8) -> Result<Self> {
        DskWriter::create(tracks, sectors)
    }

    /// Save DSK image to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        DskWriter::write(self, path)
    }

    /// Get the catalog (directory listing)
    pub fn catalog(&self) -> Result<Catalog> {
        Catalog::from_dsk(self)
    }

    /// Check if DSK format is valid
    pub fn validate(&self) -> Result<()> {
        self.header.validate()?;

        // Check track count
        if self.header.tracks == 0 || self.header.tracks > 42 {
            return Err(DskError::InvalidFormat(format!(
                "Invalid track count: {}",
                self.header.tracks
            )));
        }

        // Check head count (should be 1 for standard CPC disks)
        if self.header.heads != 1 {
            return Err(DskError::UnsupportedFormat(format!(
                "Multi-head disks not supported (heads: {})",
                self.header.heads
            )));
        }

        Ok(())
    }

    /// Get raw data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get header
    pub fn header(&self) -> &DskHeader {
        &self.header
    }

    /// Read a block (1024 bytes = 2 sectors)
    pub fn read_block(&self, block_num: u8) -> Result<Vec<u8>> {
        let track = (block_num as usize * 2) / 9;
        let sector = (block_num as usize * 2) % 9;
        let min_sect = self.get_min_sector()?;

        let mut data = Vec::with_capacity(BLOCK_SIZE);

        // Read first sector
        let sector1_data = self.read_sector(track, sector, min_sect)?;
        data.extend_from_slice(sector1_data);

        // Read second sector
        let sector2 = if sector + 1 > 8 { 0 } else { sector + 1 };
        let track2 = if sector + 1 > 8 { track + 1 } else { track };

        let sector2_data = self.read_sector(track2, sector2, min_sect)?;
        data.extend_from_slice(sector2_data);

        Ok(data)
    }

    /// Read a sector
    fn read_sector(&self, track: usize, sector: usize, min_sect: u8) -> Result<&[u8]> {
        let track_adjusted = match min_sect {
            0x41 => track + 2,
            0x01 => track + 1,
            _ => track,
        };

        let pos = self.get_sector_position(track_adjusted, sector + min_sect as usize)?;

        if pos + SECTOR_SIZE > self.data.len() {
            return Err(DskError::InvalidFormat("Sector out of bounds".to_string()));
        }

        Ok(&self.data[pos..pos + SECTOR_SIZE])
    }

    /// Get minimum sector number
    fn get_min_sector(&self) -> Result<u8> {
        let track_info_pos = DskHeader::SIZE;
        if track_info_pos + 24 + 8 > self.data.len() {
            return Err(DskError::InvalidFormat("Track info too short".to_string()));
        }

        // Read first sector ID from track 0
        let sector_id = self.data[track_info_pos + 24 + 2]; // Offset to first sector's R field
        Ok(sector_id)
    }

    /// Get position of a sector in the data
    fn get_sector_position(&self, track: usize, sector_id: usize) -> Result<usize> {
        let mut pos = DskHeader::SIZE;

        // Skip to the correct track
        for _ in 0..track {
            pos += self.header.track_size as usize;
        }

        // Skip track info
        pos += TrackInfo::SIZE;

        // Find the sector with matching ID
        let track_info_pos = DskHeader::SIZE + (track * self.header.track_size as usize);
        let num_sectors = self.data[track_info_pos + 21]; // NbSect field

        for s in 0..num_sectors as usize {
            let sector_info_pos = track_info_pos + 24 + (s * 8);
            if sector_info_pos + 2 < self.data.len() {
                let sid = self.data[sector_info_pos + 2]; // R field
                if sid as usize == sector_id {
                    return Ok(pos + (s * SECTOR_SIZE));
                }
            }
        }

        // If not found, use physical position
        Ok(pos + (sector_id * SECTOR_SIZE))
    }

    /// Write a block (1024 bytes = 2 sectors)
    pub fn write_block(&mut self, block_num: u8, data: &[u8]) -> Result<()> {
        if data.len() != BLOCK_SIZE {
            return Err(DskError::InvalidFormat(format!(
                "Block must be {} bytes",
                BLOCK_SIZE
            )));
        }

        let track = (block_num as usize * 2) / 9;
        let sector = (block_num as usize * 2) % 9;
        let min_sect = self.get_min_sector()?;

        // Write first sector
        self.write_sector(track, sector, min_sect, &data[0..SECTOR_SIZE])?;

        // Write second sector
        let sector2 = if sector + 1 > 8 { 0 } else { sector + 1 };
        let track2 = if sector + 1 > 8 { track + 1 } else { track };

        self.write_sector(track2, sector2, min_sect, &data[SECTOR_SIZE..BLOCK_SIZE])?;

        Ok(())
    }

    /// Write a sector
    fn write_sector(
        &mut self,
        track: usize,
        sector: usize,
        min_sect: u8,
        data: &[u8],
    ) -> Result<()> {
        if data.len() != SECTOR_SIZE {
            return Err(DskError::InvalidFormat(format!(
                "Sector must be {} bytes",
                SECTOR_SIZE
            )));
        }

        let track_adjusted = match min_sect {
            0x41 => track + 2,
            0x01 => track + 1,
            _ => track,
        };

        // Check if we need to expand the DSK
        if track_adjusted >= self.header.tracks as usize {
            return Err(DskError::DiskFull);
        }

        let pos = self.get_sector_position(track_adjusted, sector + min_sect as usize)?;

        if pos + SECTOR_SIZE > self.data.len() {
            return Err(DskError::DiskFull);
        }

        self.data[pos..pos + SECTOR_SIZE].copy_from_slice(data);

        Ok(())
    }

    /// Get bitmap of used blocks
    pub fn get_block_bitmap(&self) -> Result<Vec<bool>> {
        let mut bitmap = vec![false; 256];

        // Blocks 0 and 1 are reserved for directory
        bitmap[0] = true;
        bitmap[1] = true;

        // Read directory via block reads to correctly handle sector interleaving
        let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
        dir_data.extend_from_slice(&self.read_block(0)?);
        dir_data.extend_from_slice(&self.read_block(1)?);

        for i in 0..MAX_DIR_ENTRIES {
            let offset = i * DIR_ENTRY_SIZE;
            if offset + DIR_ENTRY_SIZE > dir_data.len() {
                break;
            }

            let dir_entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;

            if !dir_entry.is_deleted() {
                for j in 0..16 {
                    let block = dir_entry.blocks[j];
                    if block > 1 && (block as usize) < bitmap.len() {
                        bitmap[block as usize] = true;
                    }
                }
            }
        }

        Ok(bitmap)
    }

    /// Find a free directory entry
    pub fn find_free_dir_entry(&self) -> Result<usize> {
        // Read directory via block reads to correctly handle sector interleaving
        let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
        dir_data.extend_from_slice(&self.read_block(0)?);
        dir_data.extend_from_slice(&self.read_block(1)?);

        for i in 0..MAX_DIR_ENTRIES {
            let offset = i * DIR_ENTRY_SIZE;
            if offset + DIR_ENTRY_SIZE > dir_data.len() {
                break;
            }

            let dir_entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;

            if dir_entry.is_deleted() {
                return Ok(i);
            }
        }

        Err(DskError::NoFreeDirEntry)
    }

    /// Write directory entry
    pub fn write_dir_entry(&mut self, index: usize, entry: &DirEntry) -> Result<()> {
        if index >= MAX_DIR_ENTRIES {
            return Err(DskError::InvalidFormat(
                "Directory index out of bounds".to_string(),
            ));
        }

        // Each block holds BLOCK_SIZE / DIR_ENTRY_SIZE = 1024 / 32 = 32 entries.
        // Calculate which directory block and the byte offset within it.
        let block_num = (index / 32) as u8; // 0 or 1
        let entry_in_block = index % 32;
        let block_offset = entry_in_block * DIR_ENTRY_SIZE;

        // Read → modify → write back, using block I/O to handle sector interleaving
        let mut block_data = self.read_block(block_num)?;
        let entry_bytes = entry.to_bytes();
        block_data[block_offset..block_offset + DIR_ENTRY_SIZE].copy_from_slice(&entry_bytes);
        self.write_block(block_num, &block_data)?;

        Ok(())
    }

    /// Returns the number of sectors per track, derived from the track size in the header.
    pub fn sectors_per_track(&self) -> u8 {
        if self.header.track_size as usize <= TrackInfo::SIZE {
            return 0;
        }
        ((self.header.track_size as usize - TrackInfo::SIZE) / SECTOR_SIZE) as u8
    }

    /// Returns the total number of blocks on the disk (including the 2 directory blocks).
    pub fn total_blocks(&self) -> usize {
        (self.header.tracks as usize * self.sectors_per_track() as usize) / 2
    }

    /// Read the raw file data for a given filename from the DSK image.
    ///
    /// The filename must match exactly the form returned by the catalog
    /// (uppercase, e.g. `"HELLO.BAS"`).  Returns `None` if the file is not
    /// found; returns `Ok(Some(data))` with the file bytes (including the
    /// AMSDOS header when present) truncated to the logical file length.
    pub fn read_file_data(&self, filename: &str) -> Result<Option<Vec<u8>>> {
        // Quick existence check via catalog
        let catalog = self.catalog()?;
        if !catalog.entries.iter().any(|e| e.name == filename) {
            return Ok(None);
        }

        // Read directory blocks
        let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
        dir_data.extend_from_slice(&self.read_block(0)?);
        dir_data.extend_from_slice(&self.read_block(1)?);

        let mut file_data: Vec<u8> = Vec::new();
        let mut first_block = true;
        let mut amsdos_logical_len: Option<usize> = None;
        let mut total_pages: usize = 0;

        for i in 0..MAX_DIR_ENTRIES {
            let offset = i * DIR_ENTRY_SIZE;
            if offset + DIR_ENTRY_SIZE > dir_data.len() {
                break;
            }

            let dir_entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;

            if dir_entry.is_deleted() {
                continue;
            }

            let entry_name =
                crate::utils::from_amsdos_name(&[&dir_entry.name[..], &dir_entry.ext[..]].concat());
            if entry_name != filename {
                continue;
            }

            total_pages += dir_entry.num_pages as usize;

            let num_blocks = dir_entry.num_pages.div_ceil(8) as usize;
            for j in 0..num_blocks {
                if j < 16 && dir_entry.blocks[j] != 0 {
                    let block_data = self.read_block(dir_entry.blocks[j])?;

                    if first_block {
                        if crate::amsdos::has_header(&block_data) {
                            if let Ok(hdr) =
                                crate::amsdos::AmsdosHeader::from_bytes(&block_data[..128])
                            {
                                amsdos_logical_len = Some(hdr.logical_length as usize + 128);
                            }
                        }
                        first_block = false;
                    }

                    file_data.extend_from_slice(&block_data);
                }
            }
        }

        // Truncate to real size
        let actual_size = amsdos_logical_len.unwrap_or(total_pages * 128);
        if file_data.len() > actual_size {
            file_data.truncate(actual_size);
        }

        Ok(Some(file_data))
    }
}
