use super::FileType;
use crate::error::{DskError, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

/// AMSDOS file header (128 bytes)
#[derive(Debug, Clone)]
pub struct AmsdosHeader {
    pub user: u8,
    pub filename: [u8; 15],
    pub block_num: u8,
    pub last_block: u8,
    pub file_type: u8,
    pub length: u16,
    pub load_address: u16,
    pub first_block: u8,
    pub logical_length: u16,
    pub entry_address: u16,
    pub real_length: u16,
    #[allow(dead_code)]
    pub checksum: u16,
}

impl AmsdosHeader {
    pub const SIZE: usize = 128;

    pub fn new(filename: &str, file_type: FileType) -> Self {
        let mut header = Self {
            user: 0,
            filename: [0x20; 15],
            block_num: 0,
            last_block: 0,
            file_type: file_type as u8,
            length: 0,
            load_address: 0,
            first_block: 0,
            logical_length: 0,
            entry_address: 0,
            real_length: 0,
            checksum: 0,
        };

        // Copy filename (8.3 format)
        let name_bytes = filename.as_bytes();
        let copy_len = name_bytes.len().min(11);
        header.filename[..copy_len].copy_from_slice(&name_bytes[..copy_len]);

        header
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(DskError::InvalidAmsdosHeader);
        }

        let mut cursor = Cursor::new(data);

        let user = cursor.read_u8()?;
        let mut filename = [0u8; 15];
        cursor.read_exact(&mut filename)?;
        let block_num = cursor.read_u8()?;
        let last_block = cursor.read_u8()?;
        let file_type = cursor.read_u8()?;
        let length = cursor.read_u16::<LittleEndian>()?;
        let load_address = cursor.read_u16::<LittleEndian>()?;
        let first_block = cursor.read_u8()?;
        let logical_length = cursor.read_u16::<LittleEndian>()?;
        let entry_address = cursor.read_u16::<LittleEndian>()?;

        // Skip unused bytes
        cursor.set_position(0x40);
        let real_length = cursor.read_u16::<LittleEndian>()?;
        cursor.set_position(0x43);
        let checksum = cursor.read_u16::<LittleEndian>()?;

        let header = Self {
            user,
            filename,
            block_num,
            last_block,
            file_type,
            length,
            load_address,
            first_block,
            logical_length,
            entry_address,
            real_length,
            checksum,
        };

        // Validate checksum
        let calculated = super::calculate_checksum(data);
        if calculated != checksum {
            return Err(DskError::InvalidAmsdosHeader);
        }

        Ok(header)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = vec![0u8; Self::SIZE];

        {
            let mut cursor = Cursor::new(data.as_mut_slice());
            cursor.write_u8(self.user).unwrap();
            cursor.write_all(&self.filename).unwrap();
            cursor.write_u8(self.block_num).unwrap();
            cursor.write_u8(self.last_block).unwrap();
            cursor.write_u8(self.file_type).unwrap();
            cursor.write_u16::<LittleEndian>(self.length).unwrap();
            cursor.write_u16::<LittleEndian>(self.load_address).unwrap();
            cursor.write_u8(self.first_block).unwrap();
            cursor
                .write_u16::<LittleEndian>(self.logical_length)
                .unwrap();
            cursor
                .write_u16::<LittleEndian>(self.entry_address)
                .unwrap();

            cursor.set_position(0x40);
            cursor.write_u16::<LittleEndian>(self.real_length).unwrap();
        }

        // Calculate and write checksum
        let checksum = super::calculate_checksum(&data);
        let mut cursor = Cursor::new(data.as_mut_slice());
        cursor.set_position(0x43);
        cursor.write_u16::<LittleEndian>(checksum).unwrap();

        data
    }

    #[allow(dead_code)]
    pub fn get_file_type(&self) -> FileType {
        FileType::from_u8(self.file_type)
    }
}
