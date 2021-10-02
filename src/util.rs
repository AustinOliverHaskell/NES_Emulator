use crate::addressing_modes::AddressingMode;

pub fn map_instruction_to_addressing_mode(instruction: u8) -> AddressingMode {
    match instruction {
        // ADC
        0x69 => AddressingMode::Immediate,
        0x65 => AddressingMode::ZeroPage,
        0x75 => AddressingMode::ZeroPage_X,
        0x6D => AddressingMode::Absolute,
        0x7D => AddressingMode::Absolute_X,
        0x79 => AddressingMode::Absolute_Y,
        0x61 => AddressingMode::Indirect_X,
        0x71 => AddressingMode::Indirect_Y,
        // AND
        0x29 => AddressingMode::Immediate,
        0x25 => AddressingMode::ZeroPage,
        0x35 => AddressingMode::ZeroPage_X,
        0x2D => AddressingMode::Absolute,
        0x3D => AddressingMode::Absolute_X,
        0x39 => AddressingMode::Absolute_Y,
        0x21 => AddressingMode::Indirect_X,
        0x31 => AddressingMode::Indirect_Y,
        // ASL
        0x0A => AddressingMode::Accumulator,
        0x06 => AddressingMode::ZeroPage,
        0x16 => AddressingMode::ZeroPage_X,
        0x0E => AddressingMode::Absolute,
        0x1E => AddressingMode::Absolute_X,
        // BCC
        0x90 => AddressingMode::Relative,
        // BCS
        0xB0 => AddressingMode::Relative,
        // BEQ
        0xF0 => AddressingMode::Relative,
        // BIT
        0x24 => AddressingMode::ZeroPage,
        0x2C => AddressingMode::Absolute,
        // BMI
        0x30 => AddressingMode::Relative,
        // BNE
        0xD0 => AddressingMode::Relative,
        // BPL
        0x10 => AddressingMode::Relative,
        // BRK
        0x00 => AddressingMode::Implied,
        // BVC
        0x50 => AddressingMode::Relative,
        // BVS
        0x70 => AddressingMode::Relative,
        // CLC
        0x18 => AddressingMode::Implied,
        // CLD
        0xD8 => AddressingMode::Implied,
        // CLI
        0x58 => AddressingMode::Implied,
        // CLV
        0xB8 => AddressingMode::Implied,
        // CMP
        0xC9 => AddressingMode::Immediate,
        0xC5 => AddressingMode::ZeroPage,
        0xD5 => AddressingMode::ZeroPage_X,
        0xCD => AddressingMode::Absolute,
        0xDD => AddressingMode::Absolute_X,
        0xD9 => AddressingMode::Absolute_Y,
        0xC1 => AddressingMode::Indirect_X,
        0xD1 => AddressingMode::Indirect_Y,
        // CPX
        0xE0 => AddressingMode::Immediate,
        0xE4 => AddressingMode::ZeroPage,
        0xEC => AddressingMode::Absolute,
        // CPY
        0xC0 => AddressingMode::Immediate,
        0xC4 => AddressingMode::ZeroPage,
        0xCC => AddressingMode::Absolute,
        // DEC
        0xC6 => AddressingMode::ZeroPage,
        0xD6 => AddressingMode::ZeroPage_X,
        0xCE => AddressingMode::Absolute,
        0xDE => AddressingMode::Absolute_X,
        // DEX
        0xCA => AddressingMode::Implied,
        // DEY
        0x88 => AddressingMode::Implied,
        // EOR
        0x49 => AddressingMode::Immediate,
        0x45 => AddressingMode::ZeroPage,
        0x55 => AddressingMode::ZeroPage_X,
        0x4D => AddressingMode::Absolute,
        0x5D => AddressingMode::Absolute_X,
        0x59 => AddressingMode::Absolute_Y,
        0x41 => AddressingMode::Indirect_X,
        0x51 => AddressingMode::Indirect_Y,
        // INC
        0xE6 => AddressingMode::ZeroPage,
        0xF6 => AddressingMode::ZeroPage_X,
        0xEE => AddressingMode::Absolute,
        0xFE => AddressingMode::Absolute_X,
        // INX
        0xE8 => AddressingMode::Implied,
        // INY
        0xC8 => AddressingMode::Implied,
        // JMP
        0x4C => AddressingMode::Absolute,
        0x6C => AddressingMode::Indirect,
        // JSR
        0x20 => AddressingMode::Absolute,
        // LDA
        0xA9 => AddressingMode::Immediate,
        0xA5 => AddressingMode::ZeroPage,
        0xB5 => AddressingMode::ZeroPage_X,
        0xAD => AddressingMode::Absolute,
        0xBD => AddressingMode::Absolute_X,
        0xB9 => AddressingMode::Absolute_Y,
        0xA1 => AddressingMode::Indirect_X,
        0xB1 => AddressingMode::Indirect_Y,
        // LDX
        0xA2 => AddressingMode::Immediate,
        0xA6 => AddressingMode::ZeroPage,
        0xB6 => AddressingMode::ZeroPage_Y,
        0xAE => AddressingMode::Absolute,
        0xBE => AddressingMode::Absolute_Y,
        // LDY
        0xA0 => AddressingMode::Immediate,
        0xA4 => AddressingMode::ZeroPage,
        0xB4 => AddressingMode::ZeroPage_X,
        0xAC => AddressingMode::Absolute,
        0xBC => AddressingMode::Absolute_X,
        // LSR
        0x4A => AddressingMode::Accumulator,
        0x46 => AddressingMode::ZeroPage,
        0x56 => AddressingMode::ZeroPage_X,
        0x4E => AddressingMode::Absolute,
        0x5E => AddressingMode::Absolute_X,
        // NOP
        0xEA => AddressingMode::Implied,
        // ORA
        0x09 => AddressingMode::Immediate,
        0x05 => AddressingMode::ZeroPage,
        0x15 => AddressingMode::ZeroPage_X,
        0x0D => AddressingMode::Absolute,
        0x1D => AddressingMode::Absolute_X,
        0x19 => AddressingMode::Absolute_Y,
        0x01 => AddressingMode::Indirect_X,
        0x11 => AddressingMode::Indirect_Y,
        // PHA
        0x48 => AddressingMode::Implied,
        // PHP
        0x08 => AddressingMode::Implied,
        // PLA
        0x68 => AddressingMode::Implied,
        // PLP
        0x28 => AddressingMode::Implied,
        // ROL
        0x2A => AddressingMode::Accumulator,
        0x26 => AddressingMode::ZeroPage,
        0x36 => AddressingMode::ZeroPage_X,
        0x2E => AddressingMode::Absolute,
        0x3E => AddressingMode::Absolute_X,
        // ROR
        0x6A => AddressingMode::Accumulator,
        0x66 => AddressingMode::ZeroPage,
        0x76 => AddressingMode::ZeroPage_X,
        0x6E => AddressingMode::Absolute,
        0x7E => AddressingMode::Absolute_X,
        // RTI
        0x40 => AddressingMode::Implied,
        // RTS
        0x60 => AddressingMode::Implied,
        // SBC
        0xE9 => AddressingMode::Immediate,
        0xE5 => AddressingMode::ZeroPage,
        0xF5 => AddressingMode::ZeroPage_X,
        0xED => AddressingMode::Absolute,
        0xFD => AddressingMode::Absolute_X,
        0xF9 => AddressingMode::Absolute_Y,
        0xE1 => AddressingMode::Indirect_X,
        0xF1 => AddressingMode::Indirect_Y,
        // SEC
        0x38 => AddressingMode::Implied,
        // SED
        0xF8 => AddressingMode::Implied,
        // SEI
        0x78 => AddressingMode::Implied,
        // STA
        0x85 => AddressingMode::ZeroPage,
        0x95 => AddressingMode::ZeroPage_X,
        0x8D => AddressingMode::Absolute,
        0x9D => AddressingMode::Absolute_X,
        0x99 => AddressingMode::Absolute_Y,
        0x81 => AddressingMode::Indirect_X,
        0x91 => AddressingMode::Indirect_Y,
        // STX
        0x86 => AddressingMode::ZeroPage,
        0x96 => AddressingMode::ZeroPage_Y,
        0x8E => AddressingMode::Absolute,
        // STY
        0x84 => AddressingMode::ZeroPage,
        0x94 => AddressingMode::ZeroPage_X,
        0x8C => AddressingMode::Absolute,
        // TAX
        0xAA => AddressingMode::Implied,
        // TAY
        0xA8 => AddressingMode::Implied,
        // TSX
        0xBA => AddressingMode::Implied,
        // TXA
        0x8A => AddressingMode::Implied,
        // TXS
        0x9A => AddressingMode::Implied,
        // TYA
        0x98 => AddressingMode::Implied,
        _ => panic!("Got unrecognized instruction while mapping addressing mode. ")
    }
}

// Used for debugging - Austin Haskell 
#[allow(dead_code)]
pub fn map_instruction_to_name(instruction: u8) -> &'static str {
    match instruction {
        // ADC
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => "ADC",
        // AND
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => "AND",
        // ASL
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E => "ASL",
        // BCC
        0x90 => "BCC",
        // BCS
        0xB0 => "BCS",
        // BEQ
        0xF0 => "BEQ",
        // BIT
        0x24 | 0x2C => "BIT",
        // BMI
        0x30 => "BMI",
        // BNE
        0xD0 => "BNE",
        // BPL
        0x10 => "BPL",
        // BRK
        0x00 => "BRK",
        // BVC
        0x50 => "BVC",
        // BVS
        0x70 => "BVS",
        // CLC
        0x18 => "CLC",
        // CLD
        0xD8 => "CLD",
        // CLI
        0x58 => "CLI",
        // CLV
        0xB8 => "CLV",
        // CMP
        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => "CMP",
        // CPX
        0xE0 | 0xE4 | 0xEC => "CPX",
        // CPY
        0xC0 | 0xC4 | 0xCC => "CPY",
        // DEC
        0xC6 | 0xD6 | 0xCE | 0xDE => "DEC",
        // DEX
        0xCA => "DEX",
        // DEY
        0x88 => "DEY",
        // EOR
        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => "EOR",
        // INC
        0xE6 | 0xF6 | 0xEE | 0xFE => "INC",
        // INX
        0xE8 => "INX",
        // INY
        0xC8 => "INY",
        // JMP
        0x4C | 0x6C => "JMP",
        // JSR
        0x20 => "JSR",
        // LDA
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => "LDA",
        // LDX
        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => "LDX",
        // LDY
        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => "LDY",
        // LSR
        0x4A | 0x46 | 0x56 | 0x4E | 0x5E => "LSR",
        // NOP
        0xEA => "NOP",
        // ORA
        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => "ORA",
        // PHA
        0x48 => "PHA",
        // PHP
        0x08 => "PHP",
        // PLA
        0x68 => "PLA",
        // PLP
        0x28 => "PLP",
        // ROL
        0x2A | 0x26 | 0x36 | 0x2E | 0x3E => "ROL",
        // ROR
        0x6A | 0x66 | 0x76 | 0x6E | 0x7E => "ROR",
        // RTI
        0x40 => "RTI",
        // RTS
        0x60 => "RTS",
        // SBC
        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => "SBC",
        // SEC
        0x38 => "SEC",
        // SED
        0xF8 => "SED",
        // SEI
        0x78 => "SEI",
        // STA
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => "STA",
        // STX
        0x86 | 0x96 | 0x8E => "STX",
        // STY
        0x84 | 0x94 | 0x8C => "STY",
        // TAX
        0xAA => "TAX",
        // TAY
        0xA8 => "TAY",
        // TSX
        0xBA => "TSX",
        // TXA
        0x8A => "TXA",
        // TXS
        0x9A => "TXS",
        // TYA
        0x98 => "TYA",
        _ => panic!("Got unrecognized instruction while mapping addressing mode. ")
    }
}

pub fn addressing_mode_to_program_counter_advancement_amount(mode: AddressingMode) -> u16 {
    match mode {
        AddressingMode::Implied => 1,
        AddressingMode::Accumulator => 1,
        AddressingMode::Immediate => 2,
        AddressingMode::ZeroPage => 2,
        AddressingMode::ZeroPage_X => 2,
        AddressingMode::ZeroPage_Y => 2,
        AddressingMode::Absolute => 3,
        AddressingMode::Absolute_X => 3,
        AddressingMode::Absolute_Y => 3,
        AddressingMode::Indirect_X => 2,
        AddressingMode::Indirect_Y => 2,
        AddressingMode::Indirect => 3,
        AddressingMode::Relative => 0,
        _ => 0
    }
}

// Used while debugging - Austin Haskell 8/21/2021
#[allow(dead_code)]
pub fn print_bytes_in_hex(bytes: &Vec<u8>, num_bytes: u32, grouping: u8) {
    let mut count: u32 = 0;
    for val in bytes {
        count += 1;

        print!("{:02X?} ", val);

        if count % grouping as u32 == 0 {
            println!();
        }

        if count >= num_bytes {
            break;
        }
    }
}