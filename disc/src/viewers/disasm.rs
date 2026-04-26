// MIT License - Copyright (c) 2026 Destroyer
// Z80 disassembler for Amstrad CPC

pub struct DisasmViewer;

impl DisasmViewer {
    pub fn view(data: &[u8], load_address: u16) -> String {
        let mut output = String::new();
        let mut pos = 0;

        while pos < data.len() {
            let addr = load_address.wrapping_add(pos as u16);
            let old_pos = pos;

            let (instr, size) = decode_instruction(data, pos, load_address);
            pos += size;

            // Format: ADDR  BYTES  INSTRUCTION
            let bytes: Vec<String> = data[old_pos..pos.min(data.len())]
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect();

            output.push_str(&format!(
                "{:04X}  {:<14}  {}\n",
                addr,
                bytes.join(" "),
                instr
            ));
        }

        output
    }
}

fn decode_instruction(data: &[u8], pos: usize, base: u16) -> (String, usize) {
    if pos >= data.len() {
        return ("???".to_string(), 1);
    }

    let op = data[pos];

    match op {
        0xCB => {
            if pos + 1 >= data.len() {
                return ("???".to_string(), 1);
            }
            let op2 = data[pos + 1];
            (CB_OPS[op2 as usize].to_string(), 2)
        }
        0xED => {
            if pos + 1 >= data.len() {
                return ("???".to_string(), 1);
            }
            let op2 = data[pos + 1];
            let instr = ED_OPS[op2 as usize];
            if instr.is_empty() {
                (format!("DB {:02X}h,{:02X}h", op, op2), 2)
            } else {
                (instr.to_string(), 2)
            }
        }
        0xDD => decode_indexed(data, pos, "IX", base),
        0xFD => decode_indexed(data, pos, "IY", base),
        _ => decode_main(data, pos, base),
    }
}

fn decode_main(data: &[u8], pos: usize, base: u16) -> (String, usize) {
    let op = data[pos];
    let instr = MAIN_OPS[op as usize];

    if instr.contains("nnnn") {
        if pos + 2 >= data.len() {
            return (instr.to_string(), 1);
        }
        let addr = u16::from_le_bytes([data[pos + 1], data[pos + 2]]);
        (instr.replace("nnnn", &format!("{:04X}h", addr)), 3)
    } else if instr.contains("eeee") {
        if pos + 1 >= data.len() {
            return (instr.to_string(), 1);
        }
        let offset = data[pos + 1] as i8;
        // Target is relative to instruction AFTER this one (pos+2)
        let target = base
            .wrapping_add(pos as u16)
            .wrapping_add(2)
            .wrapping_add(offset as u16);
        (instr.replace("eeee", &format!("{:04X}h", target)), 2)
    } else if instr.contains("nn") {
        if pos + 1 >= data.len() {
            return (instr.to_string(), 1);
        }
        let val = data[pos + 1];
        (instr.replace("nn", &format!("{:02X}h", val)), 2)
    } else if instr.is_empty() {
        (format!("DB {:02X}h", op), 1)
    } else {
        (instr.to_string(), 1)
    }
}

/// Format an (IX+d) / (IY+d) displacement operand
fn fmt_disp(reg: &str, d: i8) -> String {
    if d >= 0 {
        format!("({}+{:02X}h)", reg, d as u8)
    } else {
        format!("({}-{:02X}h)", reg, (-(d as i16)) as u8)
    }
}

fn decode_indexed(data: &[u8], pos: usize, reg: &str, _base: u16) -> (String, usize) {
    if pos + 1 >= data.len() {
        return ("???".to_string(), 1);
    }
    let op2 = data[pos + 1];

    // Helper: read displacement byte at pos+2
    let d = |p: usize| -> i8 {
        if p + 2 < data.len() {
            data[p + 2] as i8
        } else {
            0
        }
    };

    match op2 {
        // ----- 16-bit load / arithmetic -----
        0x09 => (format!("ADD {},BC", reg), 2),
        0x19 => (format!("ADD {},DE", reg), 2),
        0x21 => {
            if pos + 3 >= data.len() {
                return ("???".to_string(), 2);
            }
            let val = u16::from_le_bytes([data[pos + 2], data[pos + 3]]);
            (format!("LD {},{:04X}h", reg, val), 4)
        }
        0x22 => {
            if pos + 3 >= data.len() {
                return ("???".to_string(), 2);
            }
            let addr = u16::from_le_bytes([data[pos + 2], data[pos + 3]]);
            (format!("LD ({:04X}h),{}", addr, reg), 4)
        }
        0x23 => (format!("INC {}", reg), 2),
        0x24 => (format!("INC {}H", reg), 2),
        0x25 => (format!("DEC {}H", reg), 2),
        0x26 => {
            if pos + 2 >= data.len() {
                return ("???".to_string(), 2);
            }
            (format!("LD {}H,{:02X}h", reg, data[pos + 2]), 3)
        }
        0x29 => (format!("ADD {},{}", reg, reg), 2),
        0x2A => {
            if pos + 3 >= data.len() {
                return ("???".to_string(), 2);
            }
            let addr = u16::from_le_bytes([data[pos + 2], data[pos + 3]]);
            (format!("LD {},({:04X}h)", reg, addr), 4)
        }
        0x2B => (format!("DEC {}", reg), 2),
        0x2C => (format!("INC {}L", reg), 2),
        0x2D => (format!("DEC {}L", reg), 2),
        0x2E => {
            if pos + 2 >= data.len() {
                return ("???".to_string(), 2);
            }
            (format!("LD {}L,{:02X}h", reg, data[pos + 2]), 3)
        }
        0x39 => (format!("ADD {},SP", reg), 2),

        // ----- (IX/IY+d) memory operations -----
        0x34 => (format!("INC {}", fmt_disp(reg, d(pos))), 3),
        0x35 => (format!("DEC {}", fmt_disp(reg, d(pos))), 3),
        0x36 => {
            if pos + 3 >= data.len() {
                return ("???".to_string(), 2);
            }
            let disp = data[pos + 2] as i8;
            let val = data[pos + 3];
            (format!("LD {},{:02X}h", fmt_disp(reg, disp), val), 4)
        }

        // ----- LD r,(IX+d) -----
        0x46 => (format!("LD B,{}", fmt_disp(reg, d(pos))), 3),
        0x4E => (format!("LD C,{}", fmt_disp(reg, d(pos))), 3),
        0x56 => (format!("LD D,{}", fmt_disp(reg, d(pos))), 3),
        0x5E => (format!("LD E,{}", fmt_disp(reg, d(pos))), 3),
        0x66 => (format!("LD H,{}", fmt_disp(reg, d(pos))), 3),
        0x6E => (format!("LD L,{}", fmt_disp(reg, d(pos))), 3),
        0x7E => (format!("LD A,{}", fmt_disp(reg, d(pos))), 3),

        // ----- LD (IX+d),r -----
        0x70 => (format!("LD {},B", fmt_disp(reg, d(pos))), 3),
        0x71 => (format!("LD {},C", fmt_disp(reg, d(pos))), 3),
        0x72 => (format!("LD {},D", fmt_disp(reg, d(pos))), 3),
        0x73 => (format!("LD {},E", fmt_disp(reg, d(pos))), 3),
        0x74 => (format!("LD {},H", fmt_disp(reg, d(pos))), 3),
        0x75 => (format!("LD {},L", fmt_disp(reg, d(pos))), 3),
        0x77 => (format!("LD {},A", fmt_disp(reg, d(pos))), 3),

        // ----- Arithmetic/logic with IXH/IXL -----
        0x44 => (format!("LD B,{}H", reg), 2),
        0x45 => (format!("LD B,{}L", reg), 2),
        0x4C => (format!("LD C,{}H", reg), 2),
        0x4D => (format!("LD C,{}L", reg), 2),
        0x54 => (format!("LD D,{}H", reg), 2),
        0x55 => (format!("LD D,{}L", reg), 2),
        0x5C => (format!("LD E,{}H", reg), 2),
        0x5D => (format!("LD E,{}L", reg), 2),
        0x60 => (format!("LD {}H,B", reg), 2),
        0x61 => (format!("LD {}H,C", reg), 2),
        0x62 => (format!("LD {}H,D", reg), 2),
        0x63 => (format!("LD {}H,E", reg), 2),
        0x64 => (format!("LD {}H,{}H", reg, reg), 2),
        0x65 => (format!("LD {}H,{}L", reg, reg), 2),
        0x67 => (format!("LD {}H,A", reg), 2),
        0x68 => (format!("LD {}L,B", reg), 2),
        0x69 => (format!("LD {}L,C", reg), 2),
        0x6A => (format!("LD {}L,D", reg), 2),
        0x6B => (format!("LD {}L,E", reg), 2),
        0x6C => (format!("LD {}L,{}H", reg, reg), 2),
        0x6D => (format!("LD {}L,{}L", reg, reg), 2),
        0x6F => (format!("LD {}L,A", reg), 2),
        0x7C => (format!("LD A,{}H", reg), 2),
        0x7D => (format!("LD A,{}L", reg), 2),

        0x84 => (format!("ADD A,{}H", reg), 2),
        0x85 => (format!("ADD A,{}L", reg), 2),
        0x86 => (format!("ADD A,{}", fmt_disp(reg, d(pos))), 3),
        0x8C => (format!("ADC A,{}H", reg), 2),
        0x8D => (format!("ADC A,{}L", reg), 2),
        0x8E => (format!("ADC A,{}", fmt_disp(reg, d(pos))), 3),
        0x94 => (format!("SUB {}H", reg), 2),
        0x95 => (format!("SUB {}L", reg), 2),
        0x96 => (format!("SUB {}", fmt_disp(reg, d(pos))), 3),
        0x9C => (format!("SBC A,{}H", reg), 2),
        0x9D => (format!("SBC A,{}L", reg), 2),
        0x9E => (format!("SBC A,{}", fmt_disp(reg, d(pos))), 3),
        0xA4 => (format!("AND {}H", reg), 2),
        0xA5 => (format!("AND {}L", reg), 2),
        0xA6 => (format!("AND {}", fmt_disp(reg, d(pos))), 3),
        0xAC => (format!("XOR {}H", reg), 2),
        0xAD => (format!("XOR {}L", reg), 2),
        0xAE => (format!("XOR {}", fmt_disp(reg, d(pos))), 3),
        0xB4 => (format!("OR {}H", reg), 2),
        0xB5 => (format!("OR {}L", reg), 2),
        0xB6 => (format!("OR {}", fmt_disp(reg, d(pos))), 3),
        0xBC => (format!("CP {}H", reg), 2),
        0xBD => (format!("CP {}L", reg), 2),
        0xBE => (format!("CP {}", fmt_disp(reg, d(pos))), 3),

        // ----- DDCB/FDCB prefix — bit operations on (IX/IY+d) -----
        0xCB => {
            if pos + 3 >= data.len() {
                return ("???".to_string(), 2);
            }
            let disp = data[pos + 2] as i8;
            let sub = data[pos + 3];
            let mem = fmt_disp(reg, disp);
            let instr = match sub {
                0x06 => format!("RLC {}", mem),
                0x0E => format!("RRC {}", mem),
                0x16 => format!("RL {}", mem),
                0x1E => format!("RR {}", mem),
                0x26 => format!("SLA {}", mem),
                0x2E => format!("SRA {}", mem),
                0x3E => format!("SRL {}", mem),
                0x46 => format!("BIT 0,{}", mem),
                0x4E => format!("BIT 1,{}", mem),
                0x56 => format!("BIT 2,{}", mem),
                0x5E => format!("BIT 3,{}", mem),
                0x66 => format!("BIT 4,{}", mem),
                0x6E => format!("BIT 5,{}", mem),
                0x76 => format!("BIT 6,{}", mem),
                0x7E => format!("BIT 7,{}", mem),
                0x86 => format!("RES 0,{}", mem),
                0x8E => format!("RES 1,{}", mem),
                0x96 => format!("RES 2,{}", mem),
                0x9E => format!("RES 3,{}", mem),
                0xA6 => format!("RES 4,{}", mem),
                0xAE => format!("RES 5,{}", mem),
                0xB6 => format!("RES 6,{}", mem),
                0xBE => format!("RES 7,{}", mem),
                0xC6 => format!("SET 0,{}", mem),
                0xCE => format!("SET 1,{}", mem),
                0xD6 => format!("SET 2,{}", mem),
                0xDE => format!("SET 3,{}", mem),
                0xE6 => format!("SET 4,{}", mem),
                0xEE => format!("SET 5,{}", mem),
                0xF6 => format!("SET 6,{}", mem),
                0xFE => format!("SET 7,{}", mem),
                _ => format!(
                    "DB {:02X}h,{:02X}h,{:02X}h,{:02X}h",
                    data[pos],
                    op2,
                    data[pos + 2],
                    sub
                ),
            };
            (instr, 4)
        }

        // ----- Stack / jump -----
        0xE1 => (format!("POP {}", reg), 2),
        0xE3 => (format!("EX (SP),{}", reg), 2),
        0xE5 => (format!("PUSH {}", reg), 2),
        0xE9 => (format!("JP ({})", reg), 2),
        0xF9 => (format!("LD SP,{}", reg), 2),

        _ => (format!("DB {:02X}h,{:02X}h", data[pos], op2), 2),
    }
}

// Main Z80 opcode table (nnnn = 16-bit addr, nn = 8-bit imm, eeee = relative offset)
const MAIN_OPS: [&str; 256] = [
    "NOP",
    "LD BC,nnnn",
    "LD (BC),A",
    "INC BC",
    "INC B",
    "DEC B",
    "LD B,nn",
    "RLCA",
    "EX AF,AF'",
    "ADD HL,BC",
    "LD A,(BC)",
    "DEC BC",
    "INC C",
    "DEC C",
    "LD C,nn",
    "RRCA",
    "DJNZ eeee",
    "LD DE,nnnn",
    "LD (DE),A",
    "INC DE",
    "INC D",
    "DEC D",
    "LD D,nn",
    "RLA",
    "JR eeee",
    "ADD HL,DE",
    "LD A,(DE)",
    "DEC DE",
    "INC E",
    "DEC E",
    "LD E,nn",
    "RRA",
    "JR NZ,eeee",
    "LD HL,nnnn",
    "LD (nnnn),HL",
    "INC HL",
    "INC H",
    "DEC H",
    "LD H,nn",
    "DAA",
    "JR Z,eeee",
    "ADD HL,HL",
    "LD HL,(nnnn)",
    "DEC HL",
    "INC L",
    "DEC L",
    "LD L,nn",
    "CPL",
    "JR NC,eeee",
    "LD SP,nnnn",
    "LD (nnnn),A",
    "INC SP",
    "INC (HL)",
    "DEC (HL)",
    "LD (HL),nn",
    "SCF",
    "JR C,eeee",
    "ADD HL,SP",
    "LD A,(nnnn)",
    "DEC SP",
    "INC A",
    "DEC A",
    "LD A,nn",
    "CCF",
    "LD B,B",
    "LD B,C",
    "LD B,D",
    "LD B,E",
    "LD B,H",
    "LD B,L",
    "LD B,(HL)",
    "LD B,A",
    "LD C,B",
    "LD C,C",
    "LD C,D",
    "LD C,E",
    "LD C,H",
    "LD C,L",
    "LD C,(HL)",
    "LD C,A",
    "LD D,B",
    "LD D,C",
    "LD D,D",
    "LD D,E",
    "LD D,H",
    "LD D,L",
    "LD D,(HL)",
    "LD D,A",
    "LD E,B",
    "LD E,C",
    "LD E,D",
    "LD E,E",
    "LD E,H",
    "LD E,L",
    "LD E,(HL)",
    "LD E,A",
    "LD H,B",
    "LD H,C",
    "LD H,D",
    "LD H,E",
    "LD H,H",
    "LD H,L",
    "LD H,(HL)",
    "LD H,A",
    "LD L,B",
    "LD L,C",
    "LD L,D",
    "LD L,E",
    "LD L,H",
    "LD L,L",
    "LD L,(HL)",
    "LD L,A",
    "LD (HL),B",
    "LD (HL),C",
    "LD (HL),D",
    "LD (HL),E",
    "LD (HL),H",
    "LD (HL),L",
    "HALT",
    "LD (HL),A",
    "LD A,B",
    "LD A,C",
    "LD A,D",
    "LD A,E",
    "LD A,H",
    "LD A,L",
    "LD A,(HL)",
    "LD A,A",
    "ADD A,B",
    "ADD A,C",
    "ADD A,D",
    "ADD A,E",
    "ADD A,H",
    "ADD A,L",
    "ADD A,(HL)",
    "ADD A,A",
    "ADC A,B",
    "ADC A,C",
    "ADC A,D",
    "ADC A,E",
    "ADC A,H",
    "ADC A,L",
    "ADC A,(HL)",
    "ADC A,A",
    "SUB B",
    "SUB C",
    "SUB D",
    "SUB E",
    "SUB H",
    "SUB L",
    "SUB (HL)",
    "SUB A",
    "SBC A,B",
    "SBC A,C",
    "SBC A,D",
    "SBC A,E",
    "SBC A,H",
    "SBC A,L",
    "SBC A,(HL)",
    "SBC A,A",
    "AND B",
    "AND C",
    "AND D",
    "AND E",
    "AND H",
    "AND L",
    "AND (HL)",
    "AND A",
    "XOR B",
    "XOR C",
    "XOR D",
    "XOR E",
    "XOR H",
    "XOR L",
    "XOR (HL)",
    "XOR A",
    "OR B",
    "OR C",
    "OR D",
    "OR E",
    "OR H",
    "OR L",
    "OR (HL)",
    "OR A",
    "CP B",
    "CP C",
    "CP D",
    "CP E",
    "CP H",
    "CP L",
    "CP (HL)",
    "CP A",
    "RET NZ",
    "POP BC",
    "JP NZ,nnnn",
    "JP nnnn",
    "CALL NZ,nnnn",
    "PUSH BC",
    "ADD A,nn",
    "RST 00h",
    "RET Z",
    "RET",
    "JP Z,nnnn",
    "",
    "CALL Z,nnnn",
    "CALL nnnn",
    "ADC A,nn",
    "RST 08h",
    "RET NC",
    "POP DE",
    "JP NC,nnnn",
    "OUT (nn),A",
    "CALL NC,nnnn",
    "PUSH DE",
    "SUB nn",
    "RST 10h",
    "RET C",
    "EXX",
    "JP C,nnnn",
    "IN A,(nn)",
    "CALL C,nnnn",
    "",
    "SBC A,nn",
    "RST 18h",
    "RET PE",
    "POP HL",
    "JP PE,nnnn",
    "EX (SP),HL",
    "CALL PE,nnnn",
    "PUSH HL",
    "AND nn",
    "RST 20h",
    "RET PO",
    "JP (HL)",
    "JP PO,nnnn",
    "EX DE,HL",
    "CALL PO,nnnn",
    "",
    "XOR nn",
    "RST 28h",
    "RET P",
    "POP AF",
    "JP P,nnnn",
    "DI",
    "CALL P,nnnn",
    "PUSH AF",
    "OR nn",
    "RST 30h",
    "RET M",
    "LD SP,HL",
    "JP M,nnnn",
    "EI",
    "CALL M,nnnn",
    "",
    "CP nn",
    "RST 38h",
];

// CB prefix opcodes
const CB_OPS: [&str; 256] = [
    "RLC B",
    "RLC C",
    "RLC D",
    "RLC E",
    "RLC H",
    "RLC L",
    "RLC (HL)",
    "RLC A",
    "RRC B",
    "RRC C",
    "RRC D",
    "RRC E",
    "RRC H",
    "RRC L",
    "RRC (HL)",
    "RRC A",
    "RL B",
    "RL C",
    "RL D",
    "RL E",
    "RL H",
    "RL L",
    "RL (HL)",
    "RL A",
    "RR B",
    "RR C",
    "RR D",
    "RR E",
    "RR H",
    "RR L",
    "RR (HL)",
    "RR A",
    "SLA B",
    "SLA C",
    "SLA D",
    "SLA E",
    "SLA H",
    "SLA L",
    "SLA (HL)",
    "SLA A",
    "SRA B",
    "SRA C",
    "SRA D",
    "SRA E",
    "SRA H",
    "SRA L",
    "SRA (HL)",
    "SRA A",
    "SLL B",
    "SLL C",
    "SLL D",
    "SLL E",
    "SLL H",
    "SLL L",
    "SLL (HL)",
    "SLL A",
    "SRL B",
    "SRL C",
    "SRL D",
    "SRL E",
    "SRL H",
    "SRL L",
    "SRL (HL)",
    "SRL A",
    "BIT 0,B",
    "BIT 0,C",
    "BIT 0,D",
    "BIT 0,E",
    "BIT 0,H",
    "BIT 0,L",
    "BIT 0,(HL)",
    "BIT 0,A",
    "BIT 1,B",
    "BIT 1,C",
    "BIT 1,D",
    "BIT 1,E",
    "BIT 1,H",
    "BIT 1,L",
    "BIT 1,(HL)",
    "BIT 1,A",
    "BIT 2,B",
    "BIT 2,C",
    "BIT 2,D",
    "BIT 2,E",
    "BIT 2,H",
    "BIT 2,L",
    "BIT 2,(HL)",
    "BIT 2,A",
    "BIT 3,B",
    "BIT 3,C",
    "BIT 3,D",
    "BIT 3,E",
    "BIT 3,H",
    "BIT 3,L",
    "BIT 3,(HL)",
    "BIT 3,A",
    "BIT 4,B",
    "BIT 4,C",
    "BIT 4,D",
    "BIT 4,E",
    "BIT 4,H",
    "BIT 4,L",
    "BIT 4,(HL)",
    "BIT 4,A",
    "BIT 5,B",
    "BIT 5,C",
    "BIT 5,D",
    "BIT 5,E",
    "BIT 5,H",
    "BIT 5,L",
    "BIT 5,(HL)",
    "BIT 5,A",
    "BIT 6,B",
    "BIT 6,C",
    "BIT 6,D",
    "BIT 6,E",
    "BIT 6,H",
    "BIT 6,L",
    "BIT 6,(HL)",
    "BIT 6,A",
    "BIT 7,B",
    "BIT 7,C",
    "BIT 7,D",
    "BIT 7,E",
    "BIT 7,H",
    "BIT 7,L",
    "BIT 7,(HL)",
    "BIT 7,A",
    "RES 0,B",
    "RES 0,C",
    "RES 0,D",
    "RES 0,E",
    "RES 0,H",
    "RES 0,L",
    "RES 0,(HL)",
    "RES 0,A",
    "RES 1,B",
    "RES 1,C",
    "RES 1,D",
    "RES 1,E",
    "RES 1,H",
    "RES 1,L",
    "RES 1,(HL)",
    "RES 1,A",
    "RES 2,B",
    "RES 2,C",
    "RES 2,D",
    "RES 2,E",
    "RES 2,H",
    "RES 2,L",
    "RES 2,(HL)",
    "RES 2,A",
    "RES 3,B",
    "RES 3,C",
    "RES 3,D",
    "RES 3,E",
    "RES 3,H",
    "RES 3,L",
    "RES 3,(HL)",
    "RES 3,A",
    "RES 4,B",
    "RES 4,C",
    "RES 4,D",
    "RES 4,E",
    "RES 4,H",
    "RES 4,L",
    "RES 4,(HL)",
    "RES 4,A",
    "RES 5,B",
    "RES 5,C",
    "RES 5,D",
    "RES 5,E",
    "RES 5,H",
    "RES 5,L",
    "RES 5,(HL)",
    "RES 5,A",
    "RES 6,B",
    "RES 6,C",
    "RES 6,D",
    "RES 6,E",
    "RES 6,H",
    "RES 6,L",
    "RES 6,(HL)",
    "RES 6,A",
    "RES 7,B",
    "RES 7,C",
    "RES 7,D",
    "RES 7,E",
    "RES 7,H",
    "RES 7,L",
    "RES 7,(HL)",
    "RES 7,A",
    "SET 0,B",
    "SET 0,C",
    "SET 0,D",
    "SET 0,E",
    "SET 0,H",
    "SET 0,L",
    "SET 0,(HL)",
    "SET 0,A",
    "SET 1,B",
    "SET 1,C",
    "SET 1,D",
    "SET 1,E",
    "SET 1,H",
    "SET 1,L",
    "SET 1,(HL)",
    "SET 1,A",
    "SET 2,B",
    "SET 2,C",
    "SET 2,D",
    "SET 2,E",
    "SET 2,H",
    "SET 2,L",
    "SET 2,(HL)",
    "SET 2,A",
    "SET 3,B",
    "SET 3,C",
    "SET 3,D",
    "SET 3,E",
    "SET 3,H",
    "SET 3,L",
    "SET 3,(HL)",
    "SET 3,A",
    "SET 4,B",
    "SET 4,C",
    "SET 4,D",
    "SET 4,E",
    "SET 4,H",
    "SET 4,L",
    "SET 4,(HL)",
    "SET 4,A",
    "SET 5,B",
    "SET 5,C",
    "SET 5,D",
    "SET 5,E",
    "SET 5,H",
    "SET 5,L",
    "SET 5,(HL)",
    "SET 5,A",
    "SET 6,B",
    "SET 6,C",
    "SET 6,D",
    "SET 6,E",
    "SET 6,H",
    "SET 6,L",
    "SET 6,(HL)",
    "SET 6,A",
    "SET 7,B",
    "SET 7,C",
    "SET 7,D",
    "SET 7,E",
    "SET 7,H",
    "SET 7,L",
    "SET 7,(HL)",
    "SET 7,A",
];

// ED prefix opcodes (empty string = invalid/undocumented)
const ED_OPS: [&str; 256] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "IN B,(C)",
    "OUT (C),B",
    "SBC HL,BC",
    "LD (nnnn),BC",
    "NEG",
    "RETN",
    "IM 0",
    "LD I,A",
    "IN C,(C)",
    "OUT (C),C",
    "ADC HL,BC",
    "LD BC,(nnnn)",
    "",
    "RETI",
    "",
    "LD R,A",
    "IN D,(C)",
    "OUT (C),D",
    "SBC HL,DE",
    "LD (nnnn),DE",
    "",
    "",
    "IM 1",
    "LD A,I",
    "IN E,(C)",
    "OUT (C),E",
    "ADC HL,DE",
    "LD DE,(nnnn)",
    "",
    "",
    "IM 2",
    "LD A,R",
    "IN H,(C)",
    "OUT (C),H",
    "SBC HL,HL",
    "",
    "",
    "",
    "",
    "RRD",
    "IN L,(C)",
    "OUT (C),L",
    "ADC HL,HL",
    "",
    "",
    "",
    "",
    "RLD",
    "",
    "OUT (C),0",
    "SBC HL,SP",
    "LD (nnnn),SP",
    "",
    "",
    "",
    "",
    "IN A,(C)",
    "OUT (C),A",
    "ADC HL,SP",
    "LD SP,(nnnn)",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "LDI",
    "CPI",
    "INI",
    "OUTI",
    "",
    "",
    "",
    "",
    "LDD",
    "CPD",
    "IND",
    "OUTD",
    "",
    "",
    "",
    "",
    "LDIR",
    "CPIR",
    "INIR",
    "OTIR",
    "",
    "",
    "",
    "",
    "LDDR",
    "CPDR",
    "INDR",
    "OTDR",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let data = &[0x00u8];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("NOP"));
        assert!(out.contains("0000"));
    }

    #[test]
    fn test_ld_bc_imm() {
        // LD BC,1234h
        let data = &[0x01u8, 0x34, 0x12];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("LD BC,1234h"));
    }

    #[test]
    fn test_jr_relative() {
        // JR +2 from address 0x0000 → target = 0x0000 + 2 + 2 = 0x0004
        let data = &[0x18u8, 0x02];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("JR "));
        assert!(out.contains("0004h"));
    }

    #[test]
    fn test_ix_load() {
        // DD 21 00 40 = LD IX,4000h
        let data = &[0xDDu8, 0x21, 0x00, 0x40];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("LD IX,4000h"));
    }

    #[test]
    fn test_ix_disp_load() {
        // DD 7E 05 = LD A,(IX+05h)
        let data = &[0xDDu8, 0x7E, 0x05];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("LD A,(IX+05h)"));
    }

    #[test]
    fn test_iy_inc() {
        // FD 23 = INC IY
        let data = &[0xFDu8, 0x23];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("INC IY"));
    }

    #[test]
    fn test_ddcb_bit() {
        // DD CB 00 46 = BIT 0,(IX+00h)
        let data = &[0xDDu8, 0xCB, 0x00, 0x46];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("BIT 0,(IX+00h)"));
    }

    #[test]
    fn test_ed_ldir() {
        // ED B0 = LDIR
        let data = &[0xEDu8, 0xB0];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("LDIR"));
    }

    #[test]
    fn test_cb_rl() {
        // CB 16 = RL (HL)
        let data = &[0xCBu8, 0x16];
        let out = DisasmViewer::view(data, 0x0000);
        assert!(out.contains("RL (HL)"));
    }

    #[test]
    fn test_load_address_offset() {
        // NOP at address 0xC000
        let data = &[0x00u8];
        let out = DisasmViewer::view(data, 0xC000);
        assert!(out.contains("C000"));
    }
}
