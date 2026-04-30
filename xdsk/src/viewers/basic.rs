// MIT License - Copyright (c) 2026 Destroyer
// BASIC tokenized viewer for Amstrad CPC

use colored::*;

// CPC BASIC keywords (tokens 0x80-0xFF)
const KEYWORDS: [&str; 128] = [
    "AFTER",
    "AUTO",
    "BORDER",
    "CALL",
    "CAT",
    "CHAIN",
    "CLEAR",
    "CLG",
    "CLOSEIN",
    "CLOSEOUT",
    "CLS",
    "CONT",
    "DATA",
    "DEF",
    "DEFINT",
    "DEFREAL",
    "DEFSTR",
    "DEG",
    "DELETE",
    "DIM",
    "DRAW",
    "DRAWR",
    "EDIT",
    "ELSE",
    "END",
    "ENT",
    "ENV",
    "ERASE",
    "ERROR",
    "EVERY",
    "FOR",
    "GOSUB",
    "GOTO",
    "IF",
    "INK",
    "INPUT",
    "KEY",
    "LET",
    "LINE",
    "LIST",
    "LOAD",
    "LOCATE",
    "MEMORY",
    "MERGE",
    "MID$",
    "MODE",
    "MOVE",
    "MOVER",
    "NEXT",
    "NEW",
    "ON",
    "ON BREAK",
    "ON ERROR GOTO",
    "SQ",
    "OPENIN",
    "OPENOUT",
    "ORIGIN",
    "OUT",
    "PAPER",
    "PEN",
    "PLOT",
    "PLOTR",
    "POKE",
    "PRINT",
    "'",
    "RAD",
    "RANDOMIZE",
    "READ",
    "RELEASE",
    "REM",
    "RENUM",
    "RESTORE",
    "RESUME",
    "RETURN",
    "RUN",
    "SAVE",
    "SOUND",
    "SPEED",
    "STOP",
    "SYMBOL",
    "TAG",
    "TAGOFF",
    "TROFF",
    "TRON",
    "WAIT",
    "WEND",
    "WHILE",
    "WIDTH",
    "WINDOW",
    "WRITE",
    "ZONE",
    "DI",
    "EI",
    "FILL",
    "GRAPHICS",
    "MASK",
    "FRAME",
    "CURSOR",
    "#E2",
    "ERL",
    "FN",
    "SPC",
    "STEP",
    "SWAP",
    "#E8",
    "#E9",
    "TAB",
    "THEN",
    "TO",
    "USING",
    ">",
    "=",
    ">=",
    "<",
    "<>",
    "<=",
    "+",
    "-",
    "*",
    "/",
    "^",
    "\\ ",
    "AND",
    "MOD",
    "OR",
    "XOR",
    "NOT",
    "#FF",
];

// CPC BASIC functions (tokens 0xFF 0x00-0x7F)
const FUNCTIONS: [&str; 128] = [
    "ABS", "ASC", "ATN", "CHR$", "CINT", "COS", "CREAL", "EXP", "FIX", "FRE", "INKEY", "INP",
    "INT", "JOY", "LEN", "LOG", "LOG10", "LOWER$", "PEEK", "REMAIN", "SGN", "SIN", "SPACE$", "SQ",
    "SQR", "STR$", "TAN", "UNT", "UPPER$", "VAL", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "EOF",
    "ERR", "HIMEM", "INKEY$", "PI", "RND", "TIME", "XPOS", "YPOS", "DERR", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "BIN$", "DEC$", "HEX$", "INSTR", "LEFT$", "MAX", "MIN",
    "POS", "RIGHT$", "ROUND", "STRING$", "TEST", "TESTR", "COPYCHR$", "VPOS",
];

// Detect if data is tokenized BASIC (starts with line length word > 0)
pub fn is_tokenized(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }
    // First two bytes = line length (little endian), must be > 0 and < 256
    let line_len = u16::from_le_bytes([data[0], data[1]]) as usize;
    line_len > 0 && line_len < 256 && line_len <= data.len()
}

pub struct BasicViewer;

impl BasicViewer {
    pub fn view(data: &[u8], colorize: bool) -> String {
        let mut output = String::new();
        let mut pos = 0;

        loop {
            if pos + 2 > data.len() {
                break;
            }

            // Line length (2 bytes LE) - 0 means end of program
            let line_len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            if line_len == 0 {
                break;
            }
            pos += 2;

            if pos + 2 > data.len() {
                break;
            }

            // Line number (2 bytes LE)
            let line_num = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;

            let line_str = if colorize {
                format!("{} ", line_num.to_string().bright_yellow())
            } else {
                format!("{} ", line_num)
            };
            output.push_str(&line_str);

            // Parse tokens until 0x00 (end of line)
            let mut in_string = false;
            loop {
                if pos >= data.len() {
                    break;
                }

                let token = data[pos];
                pos += 1;

                if token == 0x00 {
                    break; // End of line
                }

                if in_string || token < 0x20 {
                    match token {
                        0x01 => output.push(':'),
                        0x02 => {
                            // Integer variable
                            let var = read_variable(data, &mut pos);
                            output.push_str(&var);
                            output.push('%');
                        }
                        0x03 => {
                            // String variable
                            let var = read_variable(data, &mut pos);
                            output.push_str(&var);
                            output.push('$');
                        }
                        0x04 => {
                            // Float variable
                            let var = read_variable(data, &mut pos);
                            output.push_str(&var);
                            output.push('!');
                        }
                        0x0B..=0x0D => {
                            // Standard variable
                            let var = read_variable(data, &mut pos);
                            output.push_str(&var);
                        }
                        0x0E..=0x18 => {
                            // Small integer constants 0-10
                            output.push_str(&(token - 0x0E).to_string());
                        }
                        0x19 => {
                            // 8-bit integer constant
                            if pos < data.len() {
                                output.push_str(&data[pos].to_string());
                                pos += 1;
                            }
                        }
                        0x1A => {
                            // 16-bit integer constant
                            if pos + 1 < data.len() {
                                let val = u16::from_le_bytes([data[pos], data[pos + 1]]);
                                output.push_str(&val.to_string());
                                pos += 2;
                            }
                        }
                        0x1D => {
                            // Line number reference (GOTO/GOSUB target, 3 bytes: 0x1D + LE word)
                            if pos + 1 < data.len() {
                                let line_ref = u16::from_le_bytes([data[pos], data[pos + 1]]);
                                pos += 2;
                                let s = line_ref.to_string();
                                if colorize {
                                    output.push_str(&s.bright_yellow().to_string());
                                } else {
                                    output.push_str(&s);
                                }
                            }
                        }
                        0x1E => {
                            // 16-bit integer constant (alternate encoding)
                            if pos + 1 < data.len() {
                                let val = u16::from_le_bytes([data[pos], data[pos + 1]]);
                                output.push_str(&val.to_string());
                                pos += 2;
                            }
                        }
                        0x1B => {
                            // Binary constant
                            if pos + 1 < data.len() {
                                let val = u16::from_le_bytes([data[pos], data[pos + 1]]);
                                output.push_str(&format!("&X{:X}", val));
                                pos += 2;
                            }
                        }
                        0x1C => {
                            // Hex constant
                            if pos + 1 < data.len() {
                                let val = u16::from_le_bytes([data[pos], data[pos + 1]]);
                                output.push_str(&format!("&{:X}", val));
                                pos += 2;
                            }
                        }
                        0x1F => {
                            // Float constant (5 bytes)
                            if pos + 4 < data.len() {
                                let f = decode_float(data, pos);
                                output.push_str(&format_float(f));
                                pos += 5;
                            }
                        }
                        0x7C => {
                            // RSX command |
                            output.push('|');
                            let var = read_variable(data, &mut pos);
                            output.push_str(&var);
                        }
                        _ => {
                            if token >= 0x20 {
                                let ch = token as char;
                                if ch == '"' {
                                    in_string = !in_string;
                                }
                                output.push(ch);
                            }
                        }
                    }
                } else if (0x80..0xFF).contains(&token) {
                    // Keyword
                    let keyword = KEYWORDS[(token & 0x7F) as usize];
                    if colorize {
                        output.push_str(&keyword.bright_cyan().to_string());
                    } else {
                        output.push_str(keyword);
                    }
                } else if token == 0xFF {
                    // Function
                    if pos < data.len() {
                        let func_token = data[pos];
                        pos += 1;
                        if (func_token as usize) < FUNCTIONS.len() {
                            let func = FUNCTIONS[func_token as usize];
                            if colorize {
                                output.push_str(&func.bright_green().to_string());
                            } else {
                                output.push_str(func);
                            }
                        }
                    }
                } else {
                    // Printable ASCII
                    let ch = token as char;
                    if ch == '"' {
                        in_string = !in_string;
                    }
                    output.push(ch);
                }
            }

            output.push('\n');
        }

        output
    }
}

fn read_variable(data: &[u8], pos: &mut usize) -> String {
    // Skip 2 bytes, then read until high bit set
    *pos += 2;
    let mut var = String::new();
    loop {
        if *pos >= data.len() {
            break;
        }
        let b = data[*pos];
        *pos += 1;
        var.push((b & 0x7F) as char);
        if b & 0x80 != 0 {
            break;
        }
    }
    var
}

fn decode_float(data: &[u8], pos: usize) -> f64 {
    if pos + 4 >= data.len() {
        return 0.0;
    }
    let mantissa = (data[pos] as u32)
        | ((data[pos + 1] as u32) << 8)
        | ((data[pos + 2] as u32) << 16)
        | (((data[pos + 3] & 0x7F) as u32) << 24);

    let mut f = 1.0 + (mantissa as f64 / 0x80000000u32 as f64);
    if data[pos + 3] & 0x80 != 0 {
        f = -f;
    }
    let exp = data[pos + 4] as i32 - 129;
    f * (2.0f64).powi(exp)
}

fn format_float(f: f64) -> String {
    let s = format!("{}", f);
    // Remove trailing zeros after decimal point
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal tokenized BASIC line:
    /// [line_len_lo, line_len_hi, line_num_lo, line_num_hi, ...tokens..., 0x00]
    fn make_line(line_num: u16, tokens: &[u8]) -> Vec<u8> {
        // total bytes in line = 4 (header) + tokens.len() + 1 (terminator)
        let line_len = (4 + tokens.len() + 1) as u16;
        let mut data = Vec::new();
        data.extend_from_slice(&line_len.to_le_bytes());
        data.extend_from_slice(&line_num.to_le_bytes());
        data.extend_from_slice(tokens);
        data.push(0x00); // end of line
        data
    }

    #[test]
    fn test_is_tokenized_valid() {
        let data = make_line(10, &[0x80]); // AFTER keyword
        assert!(is_tokenized(&data));
    }

    #[test]
    fn test_is_tokenized_empty() {
        assert!(!is_tokenized(&[]));
        assert!(!is_tokenized(&[0x00, 0x00]));
    }

    #[test]
    fn test_basic_keyword_print() {
        // 0xBF = PRINT keyword (index 63 = 0x3F in KEYWORDS)
        let data = make_line(10, &[0xBF, b'"', b'H', b'I', b'"']);
        let mut program = data.clone();
        program.extend_from_slice(&[0x00, 0x00]); // end of program
        let out = BasicViewer::view(&program, false);
        assert!(out.contains("10 "));
        assert!(out.contains("PRINT"));
        assert!(out.contains("HI"));
    }

    #[test]
    fn test_basic_small_integer() {
        // 0x0E = integer 0, 0x0F = integer 1
        let data = make_line(20, &[0x0F]);
        let mut program = data.clone();
        program.extend_from_slice(&[0x00, 0x00]);
        let out = BasicViewer::view(&program, false);
        assert!(out.contains("20 "));
        assert!(out.contains("1"));
    }

    #[test]
    fn test_basic_line_number_ref() {
        // 0x1D + LE u16 = line number reference
        let data = make_line(30, &[0x1D, 0x64, 0x00]); // ref to line 100
        let mut program = data.clone();
        program.extend_from_slice(&[0x00, 0x00]);
        let out = BasicViewer::view(&program, false);
        assert!(out.contains("100"));
    }

    #[test]
    fn test_basic_end_of_program() {
        // program with two lines
        let mut program = make_line(10, &[0x80]); // AFTER
        program.extend_from_slice(&make_line(20, &[0x9F])); // WHILE
        program.extend_from_slice(&[0x00, 0x00]); // end
        let out = BasicViewer::view(&program, false);
        assert!(out.contains("10 "));
        assert!(out.contains("20 "));
    }
}
