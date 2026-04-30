// MIT License - Copyright (c) 2026 Destroyer
// Unit tests for AMSDOS header parsing and round-trip serialization

#[cfg(test)]
mod amsdos_tests {
    use disc::amsdos::{has_header, AmsdosHeader, FileType};

    /// Build a valid raw AMSDOS header with a correct checksum.
    fn make_raw_header(file_type: u8, load_addr: u16, entry_addr: u16, length: u16) -> Vec<u8> {
        let mut h = AmsdosHeader::new("TEST    BAS", FileType::from_u8(file_type));
        h.load_address = load_addr;
        h.entry_address = entry_addr;
        h.logical_length = length;
        h.real_length = length;
        h.length = length;
        h.to_bytes()
    }

    #[test]
    fn test_header_round_trip_basic() {
        let raw = make_raw_header(0, 0x0000, 0x0000, 256);
        let parsed = AmsdosHeader::from_bytes(&raw).expect("valid BASIC header");
        assert_eq!(parsed.file_type, 0);
        assert_eq!(parsed.logical_length, 256);
    }

    #[test]
    fn test_header_round_trip_binary() {
        let raw = make_raw_header(2, 0xC000, 0xC000, 0x4000);
        let parsed = AmsdosHeader::from_bytes(&raw).expect("valid BINARY header");
        assert_eq!(parsed.file_type, 2);
        assert_eq!(parsed.load_address, 0xC000);
        assert_eq!(parsed.entry_address, 0xC000);
    }

    #[test]
    fn test_has_header_valid() {
        let mut raw = make_raw_header(2, 0x4000, 0x4000, 100);
        raw.resize(1024, 0xE5); // pad to block size
        assert!(has_header(&raw));
    }

    #[test]
    fn test_has_header_invalid_checksum() {
        let mut raw = make_raw_header(2, 0x4000, 0x4000, 100);
        raw[0x43] ^= 0xFF; // corrupt checksum byte
        assert!(!has_header(&raw));
    }

    #[test]
    fn test_has_header_too_short() {
        let data = vec![0u8; 10];
        assert!(!has_header(&data));
    }

    #[test]
    fn test_checksum_changes_on_content() {
        let raw1 = make_raw_header(0, 0x0000, 0x0000, 100);
        let raw2 = make_raw_header(0, 0x0000, 0x0000, 200);
        // Different lengths → different checksums
        let cs1 = u16::from_le_bytes([raw1[0x43], raw1[0x44]]);
        let cs2 = u16::from_le_bytes([raw2[0x43], raw2[0x44]]);
        assert_ne!(cs1, cs2);
    }

    #[test]
    fn test_file_type_enum() {
        assert_eq!(FileType::from_u8(0).as_str(), "BASIC");
        assert_eq!(FileType::from_u8(1).as_str(), "BASIC(P)");
        assert_eq!(FileType::from_u8(2).as_str(), "BINARY");
        assert_eq!(FileType::from_u8(3).as_str(), "BINARY(P)");
        assert_eq!(FileType::from_u8(99).as_str(), "ASCII");
    }
}
