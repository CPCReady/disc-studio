/// Convert a filename to AMSDOS format (8.3, uppercase, padded)
pub fn to_amsdos_name(filename: &str) -> String {
    let mut name = String::with_capacity(11);

    let parts: Vec<&str> = filename.split('.').collect();
    let basename = parts[0];
    let extension = if parts.len() > 1 { parts[1] } else { "" };

    // Add basename (max 8 chars, padded with spaces)
    for c in basename.chars().take(8) {
        name.push(c.to_ascii_uppercase());
    }
    for _ in basename.len()..8 {
        name.push(' ');
    }

    // Add extension (max 3 chars, padded with spaces)
    for c in extension.chars().take(3) {
        name.push(c.to_ascii_uppercase());
    }
    for _ in extension.len()..3 {
        name.push(' ');
    }

    name
}

/// Convert AMSDOS format name back to regular filename.
/// In CP/M directory entries the high bit (bit 7) of each character byte encodes
/// file attributes and must be masked off before treating the byte as ASCII.
/// Non-printable characters are discarded so corrupted entries don't leak
/// control codes into the output.
pub fn from_amsdos_name(amsdos_name: &[u8]) -> String {
    if amsdos_name.len() < 11 {
        return String::new();
    }

    // Strip attribute bit (bit 7) and keep only printable ASCII (0x20–0x7E)
    let clean: Vec<u8> = amsdos_name[0..11]
        .iter()
        .map(|&b| b & 0x7F)
        .collect();

    let name: String = clean[0..8]
        .iter()
        .filter(|&&b| b > 0x20 && b < 0x7F)
        .map(|&b| b as char)
        .collect::<String>()
        .trim_end()
        .to_string();

    let ext: String = clean[8..11]
        .iter()
        .filter(|&&b| b > 0x20 && b < 0x7F)
        .map(|&b| b as char)
        .collect::<String>()
        .trim_end()
        .to_string();

    if ext.is_empty() {
        name
    } else {
        format!("{}.{}", name, ext)
    }
}

/// Format size in human-readable format
pub fn format_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else {
        format!("{} KB", bytes.div_ceil(1024))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_amsdos_name() {
        assert_eq!(to_amsdos_name("test.bas"), "TEST    BAS");
        assert_eq!(to_amsdos_name("file"), "FILE       ");
        assert_eq!(to_amsdos_name("verylongname.txt"), "VERYLONGTXT");
    }

    #[test]
    fn test_from_amsdos_name() {
        assert_eq!(from_amsdos_name(b"TEST    BAS"), "TEST.BAS");
        assert_eq!(from_amsdos_name(b"FILE       "), "FILE");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1 KB");
        assert_eq!(format_size(2048), "2 KB");
    }
}
