// MIT License - Copyright (c) 2026 Destroyer
// ASCII text viewer for Amstrad CPC DSK files — handles CPC character set

/// CPC extended characters 0x80-0xFF mapped to Unicode equivalents.
/// The Amstrad CPC uses its own extended charset with box-drawing, accented
/// letters and special symbols in the 0x80-0xFF range.
const CPC_CHARSET: [char; 128] = [
    // 0x80-0x8F
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
    // 0x90-0x9F
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
    // 0xA0-0xAF
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
    // 0xB0-0xBF  (box drawing)
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
    // 0xC0-0xCF  (box drawing)
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
    // 0xD0-0xDF  (box drawing + misc)
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
    // 0xE0-0xEF  (Greek / math)
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
    // 0xF0-0xFF  (math symbols)
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ',
];

pub struct AsciiViewer;

impl AsciiViewer {
    /// Display file content as text, handling CPC character conventions:
    /// - 0x0D (CR) and 0x0A (LF) are both treated as newlines
    /// - 0x00 and 0x1A act as end-of-file markers
    /// - 0x80-0xFF are mapped via CPC extended charset
    /// - Other control chars are shown as '·' (middle dot)
    pub fn view(data: &[u8]) -> String {
        let mut output = String::with_capacity(data.len());

        for &byte in data {
            match byte {
                0x00 | 0x1A => break,                                            // EOF markers
                0x0A => output.push('\n'),                                       // LF
                0x0D => output.push('\n'),                                       // CR → newline
                0x09 => output.push('\t'),                                       // TAB
                0x20..=0x7E => output.push(byte as char),                        // Standard ASCII
                0x7F => output.push('␡'),                                        // DEL
                0x80..=0xFF => output.push(CPC_CHARSET[(byte - 0x80) as usize]), // CPC extended
                _ => output.push('·'), // Other control chars
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_ascii() {
        let data = b"Hello, World!";
        assert_eq!(AsciiViewer::view(data), "Hello, World!");
    }

    #[test]
    fn test_cr_as_newline() {
        let data = b"line1\x0Dline2";
        assert_eq!(AsciiViewer::view(data), "line1\nline2");
    }

    #[test]
    fn test_eof_marker_zero() {
        let data = b"Hello\x00ignored";
        assert_eq!(AsciiViewer::view(data), "Hello");
    }

    #[test]
    fn test_eof_marker_ctrl_z() {
        let data = b"Hello\x1Aignored";
        assert_eq!(AsciiViewer::view(data), "Hello");
    }

    #[test]
    fn test_cpc_extended_char() {
        // 0x80 = 'Ç'
        let data = &[0x80u8];
        assert_eq!(AsciiViewer::view(data), "Ç");
    }

    #[test]
    fn test_control_char_shown_as_dot() {
        let data = &[0x01u8]; // SOH
        assert_eq!(AsciiViewer::view(data), "·");
    }
}
