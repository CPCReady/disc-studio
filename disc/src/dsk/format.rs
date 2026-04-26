use crate::error::{DskError, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

pub const SECTOR_SIZE: usize = 512;
pub const BLOCK_SIZE: usize = 1024; // 2 sectors
pub const DIR_ENTRY_SIZE: usize = 32;
pub const MAX_DIR_ENTRIES: usize = 64;
pub const USER_DELETED: u8 = 0xE5;

/// DSK image header (256 bytes)
#[derive(Debug, Clone)]
pub struct DskHeader {
    pub magic: String,
    pub tracks: u8,
    pub heads: u8,
    pub track_size: u16,
}

impl DskHeader {
    pub const SIZE: usize = 256;

    pub fn new(tracks: u8, sectors: u8) -> Self {
        Self {
            magic: "MV - CPCEMU Disk-File\r\nDisk-Info\r\n".to_string(),
            tracks,
            heads: 1,
            track_size: (TrackInfo::SIZE + SECTOR_SIZE * sectors as usize) as u16,
        }
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(DskError::InvalidFormat("Header too short".to_string()));
        }

        let mut cursor = Cursor::new(data);
        let mut magic_bytes = vec![0u8; 48];
        cursor.read_exact(&mut magic_bytes)?;
        let magic = String::from_utf8_lossy(&magic_bytes).to_string();

        let tracks = cursor.read_u8()?;
        let heads = cursor.read_u8()?;
        let track_size = cursor.read_u16::<LittleEndian>()?;

        Ok(Self {
            magic,
            tracks,
            heads,
            track_size,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut data = vec![0u8; Self::SIZE];
        let mut cursor = Cursor::new(&mut data);

        cursor.write_all(self.magic.as_bytes())?;
        cursor.set_position(48);
        cursor.write_u8(self.tracks)?;
        cursor.write_u8(self.heads)?;
        cursor.write_u16::<LittleEndian>(self.track_size)?;

        Ok(data)
    }

    pub fn validate(&self) -> Result<()> {
        if !self.magic.starts_with("MV -") && !self.magic.starts_with("EXTENDED CPC DSK") {
            return Err(DskError::InvalidFormat(
                "Invalid DSK magic string".to_string(),
            ));
        }
        Ok(())
    }
}

/// Track information block
#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub track: u8,
    pub head: u8,
    pub sector_size: u8,
    pub sector_count: u8,
    pub gap3: u8,
    pub filler: u8,
    pub sectors: Vec<SectorInfo>,
}

impl TrackInfo {
    pub const SIZE: usize = 256;

    #[allow(dead_code)]
    pub fn new(track: u8, sector_count: u8, first_sector: u8) -> Self {
        let mut sectors = Vec::new();

        // Standard interleaving pattern
        for i in 0..sector_count {
            let sector_id = if i % 2 == 0 {
                first_sector + i / 2
            } else {
                first_sector + sector_count.div_ceil(2) + i / 2
            };

            sectors.push(SectorInfo {
                track,
                head: 0,
                sector_id,
                size_code: 2, // 512 bytes
                status1: 0,
                status2: 0,
                actual_size: SECTOR_SIZE as u16,
            });
        }

        Self {
            track,
            head: 0,
            sector_size: 2,
            sector_count,
            gap3: 0x4E,
            filler: 0xE5,
            sectors,
        }
    }

    #[allow(dead_code)]
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(DskError::InvalidFormat("Track info too short".to_string()));
        }

        let mut cursor = Cursor::new(data);

        // Skip "Track-Info\r\n" magic
        cursor.set_position(16);

        let track = cursor.read_u8()?;
        let head = cursor.read_u8()?;
        cursor.set_position(20);
        let sector_size = cursor.read_u8()?;
        let sector_count = cursor.read_u8()?;
        let gap3 = cursor.read_u8()?;
        let filler = cursor.read_u8()?;

        let mut sectors = Vec::new();
        cursor.set_position(24);

        for _ in 0..sector_count {
            sectors.push(SectorInfo::from_cursor(&mut cursor)?);
        }

        Ok(Self {
            track,
            head,
            sector_size,
            sector_count,
            gap3,
            filler,
            sectors,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut data = vec![0u8; Self::SIZE];

        {
            let mut cursor = Cursor::new(data.as_mut_slice());
            cursor.write_all(b"Track-Info\r\n")?;
            cursor.set_position(16);
            cursor.write_u8(self.track)?;
            cursor.write_u8(self.head)?;
            cursor.set_position(20);
            cursor.write_u8(self.sector_size)?;
            cursor.write_u8(self.sector_count)?;
            cursor.write_u8(self.gap3)?;
            cursor.write_u8(self.filler)?;

            cursor.set_position(24);
            for sector in &self.sectors {
                sector.write_to_cursor(&mut cursor)?;
            }
        }

        Ok(data)
    }
}

/// Sector information
#[derive(Debug, Clone)]
pub struct SectorInfo {
    pub track: u8,
    pub head: u8,
    pub sector_id: u8,
    pub size_code: u8,
    pub status1: u8,
    pub status2: u8,
    pub actual_size: u16,
}

impl SectorInfo {
    #[allow(dead_code)]
    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(Self {
            track: cursor.read_u8()?,
            head: cursor.read_u8()?,
            sector_id: cursor.read_u8()?,
            size_code: cursor.read_u8()?,
            status1: cursor.read_u8()?,
            status2: cursor.read_u8()?,
            actual_size: cursor.read_u16::<LittleEndian>()?,
        })
    }

    pub fn write_to_cursor(&self, cursor: &mut Cursor<&mut [u8]>) -> Result<()> {
        cursor.write_u8(self.track)?;
        cursor.write_u8(self.head)?;
        cursor.write_u8(self.sector_id)?;
        cursor.write_u8(self.size_code)?;
        cursor.write_u8(self.status1)?;
        cursor.write_u8(self.status2)?;
        cursor.write_u16::<LittleEndian>(self.actual_size)?;
        Ok(())
    }
}

/// Directory entry (32 bytes)
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub user: u8,
    pub name: [u8; 8],
    pub ext: [u8; 3],
    pub page_num: u8,
    pub num_pages: u8,
    pub blocks: [u8; 16],
}

impl Default for DirEntry {
    fn default() -> Self {
        Self::new()
    }
}

impl DirEntry {
    pub fn new() -> Self {
        Self {
            user: USER_DELETED,
            name: [0x20; 8],
            ext: [0x20; 3],
            page_num: 0,
            num_pages: 0,
            blocks: [0; 16],
        }
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < DIR_ENTRY_SIZE {
            return Err(DskError::InvalidFormat("Dir entry too short".to_string()));
        }

        let mut entry = Self::new();
        entry.user = data[0];
        entry.name.copy_from_slice(&data[1..9]);
        entry.ext.copy_from_slice(&data[9..12]);
        entry.page_num = data[12];
        entry.num_pages = data[15];
        entry.blocks.copy_from_slice(&data[16..32]);

        Ok(entry)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![0u8; DIR_ENTRY_SIZE];
        data[0] = self.user;
        data[1..9].copy_from_slice(&self.name);
        data[9..12].copy_from_slice(&self.ext);
        data[12] = self.page_num;
        data[15] = self.num_pages;
        data[16..32].copy_from_slice(&self.blocks);
        data
    }

    pub fn is_deleted(&self) -> bool {
        self.user == USER_DELETED
    }

    pub fn is_first_extent(&self) -> bool {
        self.page_num == 0
    }
}
