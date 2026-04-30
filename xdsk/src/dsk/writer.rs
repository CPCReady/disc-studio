use super::{Dsk, DskHeader, TrackInfo};
use crate::error::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct DskWriter;

impl DskWriter {
    pub fn create(tracks: u8, sectors: u8) -> Result<Dsk> {
        let header = DskHeader::new(tracks, sectors);
        let mut data = header.to_bytes()?;

        // Create tracks
        for track in 0..tracks {
            let track_info = TrackInfo::new(track, sectors, 0xC1);
            data.extend_from_slice(&track_info.to_bytes()?);

            // Add sector data (filled with 0xE5)
            data.extend(vec![0xE5; 512 * sectors as usize]);
        }

        Ok(Dsk { data, header })
    }

    pub fn write<P: AsRef<Path>>(dsk: &Dsk, path: P) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&dsk.data)?;
        Ok(())
    }
}
