use super::{Dsk, DskHeader};
use crate::error::{DskError, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct DskReader;

impl DskReader {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Dsk> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        if data.len() < DskHeader::SIZE {
            return Err(DskError::InvalidFormat(
                "File too small to be a valid DSK".to_string(),
            ));
        }

        let header = DskHeader::from_bytes(&data[..DskHeader::SIZE])?;
        header.validate()?;

        let dsk = Dsk { data, header };
        dsk.validate()?;

        Ok(dsk)
    }
}
