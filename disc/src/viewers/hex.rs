// MIT License - Copyright (c) 2026 Destroyer
// Hex dump viewer for Amstrad CPC DSK files

use colored::*;

pub struct HexViewer;

impl HexViewer {
    pub fn view(data: &[u8], load_address: Option<u16>) -> String {
        let mut output = String::new();
        let base_addr = load_address.unwrap_or(0);

        for (i, chunk) in data.chunks(16).enumerate() {
            let addr = base_addr.wrapping_add((i * 16) as u16);

            // Address column
            output.push_str(&format!("{:04X}", addr).bright_cyan().to_string());
            output.push_str(&":  ".bright_black().to_string());

            // Hex bytes (two groups of 8)
            for (j, byte) in chunk.iter().enumerate() {
                let hex = format!("{:02X}", byte);
                let colored_hex = if *byte == 0x00 {
                    hex.bright_black()
                } else if *byte == 0xFF {
                    hex.bright_white()
                } else if *byte >= 0x20 && *byte < 0x7F {
                    hex.white()
                } else {
                    hex.yellow()
                };
                output.push_str(&colored_hex.to_string());
                output.push(' ');
                if j == 7 {
                    output.push(' ');
                }
            }

            // Padding for short last line
            let missing = 16 - chunk.len();
            for j in 0..missing {
                output.push_str("   ");
                if chunk.len() + j == 7 {
                    output.push(' ');
                }
            }

            output.push_str(&" | ".bright_black().to_string());

            // ASCII sidebar
            for byte in chunk {
                let ch = if *byte >= 0x20 && *byte < 0x7F {
                    (*byte as char).to_string().green().to_string()
                } else if *byte == 0x00 {
                    ".".bright_black().to_string()
                } else {
                    ".".yellow().to_string()
                };
                output.push_str(&ch);
            }

            output.push('\n');
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_viewer_basic() {
        let data = b"Hello, World!";
        let result = HexViewer::view(data, None);
        assert!(result.contains("48")); // 'H'
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_hex_viewer_with_address() {
        let data = &[0x3E, 0x00, 0xDF];
        let result = HexViewer::view(data, Some(0xC000));
        assert!(result.contains("C000"));
    }

    #[test]
    fn test_hex_viewer_empty() {
        let result = HexViewer::view(&[], None);
        assert!(result.is_empty());
    }
}
