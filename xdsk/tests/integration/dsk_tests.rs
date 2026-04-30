// MIT License - Copyright (c) 2026 Destroyer
// Unit tests for DSK image format: header parsing, creation, and directory entries

#[cfg(test)]
mod dsk_format_tests {
    use disc::dsk::{DirEntry, Dsk, DIR_ENTRY_SIZE, USER_DELETED};
    use tempfile::NamedTempFile;

    fn create_test_dsk() -> Dsk {
        Dsk::create(40, 9).expect("should create 40-track DSK")
    }

    // ── Header ──────────────────────────────────────────────────────────────

    #[test]
    fn test_create_dsk_header() {
        let dsk = create_test_dsk();
        let header = dsk.header();
        assert_eq!(header.tracks, 40);
        assert_eq!(header.heads, 1);
        assert!(header.magic.starts_with("MV -"));
    }

    #[test]
    fn test_validate_new_dsk() {
        let dsk = create_test_dsk();
        assert!(dsk.validate().is_ok());
    }

    #[test]
    fn test_dsk_data_non_empty() {
        let dsk = create_test_dsk();
        assert!(!dsk.data().is_empty());
    }

    // ── Save / reload ────────────────────────────────────────────────────────

    #[test]
    fn test_save_and_reload() {
        let dsk = create_test_dsk();
        let tmp = NamedTempFile::new().unwrap();
        dsk.save(tmp.path()).expect("save should succeed");

        let loaded = Dsk::open(tmp.path()).expect("reload should succeed");
        assert_eq!(loaded.header().tracks, 40);
        assert!(loaded.validate().is_ok());
    }

    #[test]
    fn test_reload_same_size() {
        let dsk = create_test_dsk();
        let tmp = NamedTempFile::new().unwrap();
        dsk.save(tmp.path()).unwrap();
        let loaded = Dsk::open(tmp.path()).unwrap();
        assert_eq!(dsk.data().len(), loaded.data().len());
    }

    // ── DirEntry ─────────────────────────────────────────────────────────────

    #[test]
    fn test_dir_entry_new_is_deleted() {
        let entry = DirEntry::new();
        assert!(entry.is_deleted());
        assert_eq!(entry.user, USER_DELETED);
    }

    #[test]
    fn test_dir_entry_round_trip() {
        let mut entry = DirEntry::new();
        entry.user = 0;
        entry.name.copy_from_slice(b"TEST    ");
        entry.ext.copy_from_slice(b"BAS");
        entry.num_pages = 4;
        entry.blocks[0] = 2;
        entry.blocks[1] = 3;

        let bytes = entry.to_bytes();
        assert_eq!(bytes.len(), DIR_ENTRY_SIZE);

        let parsed = DirEntry::from_bytes(&bytes).expect("round-trip");
        assert_eq!(parsed.user, 0);
        assert_eq!(&parsed.name, b"TEST    ");
        assert_eq!(&parsed.ext, b"BAS");
        assert_eq!(parsed.num_pages, 4);
        assert_eq!(parsed.blocks[0], 2);
        assert_eq!(parsed.blocks[1], 3);
    }

    #[test]
    fn test_dir_entry_is_first_extent() {
        let mut entry = DirEntry::new();
        entry.page_num = 0;
        assert!(entry.is_first_extent());
        entry.page_num = 1;
        assert!(!entry.is_first_extent());
    }

    // ── Catalog ──────────────────────────────────────────────────────────────

    #[test]
    fn test_empty_catalog() {
        let dsk = create_test_dsk();
        let catalog = dsk.catalog().expect("catalog from empty DSK");
        assert!(catalog.entries.is_empty());
        assert_eq!(catalog.total_size, 0);
    }

    // ── Block bitmap ─────────────────────────────────────────────────────────

    #[test]
    fn test_block_bitmap_reserved() {
        let dsk = create_test_dsk();
        let bitmap = dsk.get_block_bitmap().expect("bitmap");
        // Blocks 0 and 1 are always reserved for the directory
        assert!(bitmap[0]);
        assert!(bitmap[1]);
    }

    #[test]
    fn test_free_dir_entry_in_new_dsk() {
        let dsk = create_test_dsk();
        let idx = dsk.find_free_dir_entry().expect("should find free entry");
        assert!(idx < 64);
    }
}
