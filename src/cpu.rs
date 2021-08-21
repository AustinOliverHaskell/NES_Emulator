pub const ZERO: u8              = 0b1000_0000;
pub const OVERFLOW: u8          = 0b0100_0000;

pub const DECIMAL_MODE: u8      = 0b0000_1000;
pub const INTERRUPT_DISABLE: u8 = 0b0000_0100;
pub const NEGATIVE: u8          = 0b0000_0010;
pub const CARRY: u8             = 0b0000_0001;

pub const PROGRAM_START_ADDR:  u16 = 0x8000;
pub const PROGRAM_READ_START:  u16 = 0xFFFC;
pub const STACK_START_ADDR:    u16 = 0x0100;
pub const INITIAL_STACK_VALUE: u8  = 0xFF;

use crate::addressing_modes::*;

pub struct CPU {
    pub program_counter: u16,
    pub registers: Registers,
    pub stack_pointer: u8,
    pub status: u8,
    pub memory: [u8; 0xFFFF]
}

impl CPU 
{
    pub fn new(program: Vec<u8>) -> Self {

        let mut memory: [u8; 0xFFFF] = [0; 0xFFFF];
        memory[PROGRAM_START_ADDR as usize .. (PROGRAM_START_ADDR as usize + program.len())].copy_from_slice(&program[..]);

        let mut cpu = CPU {
            program_counter: PROGRAM_START_ADDR,
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
            },
            stack_pointer: INITIAL_STACK_VALUE,
            status: 0,
            memory: memory
        };

        cpu.write16(PROGRAM_READ_START, PROGRAM_START_ADDR);
        cpu.reset();
        cpu
    }

    // Used for debugging - Austin Haskell 8/17/2021
    #[allow(dead_code)]
    pub fn dump_cpu(&mut self) {
        println!("----- CPU STATE -----");
        println!(" A: {:?}", self.registers.a);
        println!(" X: {:?}", self.registers.x);
        println!(" Y: {:?}", self.registers.y);
        println!(" Status: 0x{:08b}", self.status);
        println!(" Program Counter: {:?} aka 0x{:04x}", self.program_counter, self.program_counter);
        println!(" Stack is at location {:?} w/value on top of {:?}", 
                self.stack_pointer, self.memory[(STACK_START_ADDR | self.stack_pointer as u16) as usize]);
        println!(" Next Instruction: 0x{:02x}", self.memory[self.program_counter as usize]);
    }

    // Used for debugging - Austin Haskell 8/17/2021
    #[allow(dead_code)]
    pub fn dump_memory(&mut self, start: u16, end: u16) {
        let display_width: u32 = 16;
        let mut line = start as u32;
        for byte in self.memory[start as usize..end as usize].iter() {
            if line % display_width == 0  || line == 0 {
                print!("0x{:04x} ", line);
            }

            if *byte == 0 {
                print!(" .. ");
            }
            else {
                print!("0x{:02x}", byte);
            }

            line += 1;
            if line % display_width == 0 {
                println!();
            }
        }
        println!();
    }

    pub fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.x = 0;
        self.registers.y = 0;
        self.status = 0;

        self.program_counter = self.load16(PROGRAM_READ_START);
    }

    pub fn run_program(&mut self) {
        loop {
            self.run_next_instruction();
        }
    }

    pub fn run_next_instruction(&mut self) {
        let instruction = self.memory[self.program_counter as usize];
        match instruction {
            /* ----- ADC ----- */
            0x69 => println!("ADC (Immediate Mode) is not implemented. "),
            0x65 => println!("ADC (Zero Page) is not implemented. "),
            0x75 => println!("ADC (Zero Page X) is not implemented. "),
            0x6D => println!("ADC (Absolute) is not implemented. "),
            0x7D => println!("ADC (Absolute X) is not implemented. "),
            0x79 => println!("ADC (Absolute Y) is not implemented. "),
            0x61 => println!("ADC (Indirect X) is not implemented. "),
            0x71 => println!("ADC (Indirect Y) is not implemented. "),
            /* ----- AND ----- */
            0x29 => {
                self.and(); 
                self.program_counter += 1;
            },
            0x25 => println!("AND (Zero Page) is not implemented. "),
            0x35 => println!("AND (Zero Page X) is not implemented. "),
            0x2D => println!("AND (Absolute) is not implemented. "),
            0x3D => println!("AND (Absolute X) is not implemented. "),
            0x39 => println!("AND (Absolute Y) is not implemented. "),
            0x21 => println!("AND (Indirect X) is not implemented. "),
            0x31 => println!("AND (Indirect Y) is not implemented. "),
            /* ----- ASL ----- */
            0x0A => {
                self.asl(AddressingMode::Accumulator);
                self.program_counter += 1;
            },
            0x06 => {
                self.asl(AddressingMode::ZeroPage);
                self.program_counter += 2;
            },
            0x16 => {
                self.asl(AddressingMode::ZeroPage_X);
                self.program_counter += 2;
            },
            0x0E => {
                self.asl(AddressingMode::Absolute);
                self.program_counter += 3;
            },
            0x1E => {
                self.asl(AddressingMode::Absolute_X);
                self.program_counter += 3;
            },
            /* ----- BCC ----- */
            0x90 => self.branch(CARRY, false),
            /* ----- BCS ----- */
            0xB0 => self.branch(CARRY, true),
            /* ----- BEQ ----- */
            0xF0 => self.branch(ZERO, true),
            /* ----- BIT ----- */
            0x24 => {
                self.bit(AddressingMode::ZeroPage);
                self.program_counter += 2;
            },
            0x2C => {
                self.bit(AddressingMode::Absolute);
                self.program_counter += 3;
            },
            /* ----- BMI ----- */
            0x30 => self.branch(NEGATIVE, true),
            /* ----- BNE ----- */
            0xD0 => self.branch(ZERO, false),
            /* ----- BPL ----- */
            0x10 => self.branch(NEGATIVE, false),
            /* ----- BRK ----- */
            0x00 => println!("BRK is not implemented."),
            /* ----- BVC ----- */
            0x50 => self.branch(OVERFLOW, false),
            /* ----- BVS ----- */
            0x70 => self.branch(OVERFLOW, true),
            /* ----- CLC ----- */
            0x18 => {
                self.clear_carry_bit();
                self.program_counter += 1;
            },
            /* ----- CLD ----- */
            0xD8 => {
                self.clear_decimal_bit();
                self.program_counter += 1;
            },
            /* ----- CLI ----- */
            0x58 => {
                self.clear_interrupt_disable_bit();
                self.program_counter += 1;
            },
            /* ----- CLV ----- */
            0xB8 => {
                self.clear_overflow_bit();
                self.program_counter += 1;
            },
            /* ----- CMP ----- */
            0xC9 => println!("CMP (Immidiate) is not implemented."),
            0xC5 => println!("CMP (Zero Page) is not implemented."),
            0xD5 => println!("CMP (Zero Page X) is not implemented."),
            0xCD => println!("CMP (Absolute) is not implemented."),
            0xDD => println!("CMP (Absolute X) is not implemented."),
            0xD9 => println!("CMP (Absolute Y) is not implemented."),
            0xC1 => println!("CMP (Indirext X) is not implemented."),
            0xD1 => println!("CMP (Indirect Y) is not implemented."),
            /* ----- CPX ----- */
            0xE0 => {
                self.cpx(AddressingMode::Immediate);
                self.program_counter += 2;
            },
            0xE4 => {
                self.cpx(AddressingMode::ZeroPage);
                self.program_counter += 2;
            },
            0xEC => {
                self.cpx(AddressingMode::Absolute);
                self.program_counter += 3;
            },
            /* ----- CPY ----- */
            0xC0 => println!("CPY (Immediate) is not implemented."),
            0xC4 => println!("CPY (Zero Page) is not implemented."),
            0xCC => println!("CPY (Absolute) is not implemented."),
            /* ----- DEC ----- */
            0xC6 => println!("DEC (Zero Page) is not implemented."),
            0xD6 => println!("DEC (Zero Page X) is not implemented."),
            0xCE => println!("DEC (Absolute) is not implemented."),
            0xDE => println!("DEC (Absolute X) is not implemented."),  
            /* ----- DEX ----- */
            0xCA => {
                self.dex();
                self.program_counter += 1;
            },
            /* ----- DEY ----- */
            0x88 => {
                self.dey();
                self.program_counter += 1;
            },
            /* ----- EOR ----- */
            0x49 => println!("EOR (Immidiate) is not implemented."),
            0x45 => println!("EOR (Zero Page) is not implemented."),
            0x55 => println!("EOR (Zero Page X) is not implemented."),
            0x4D => println!("EOR (Absolute) is not implemented."),
            0x5D => println!("EOR (Absolute X) is not implemented."),
            0x59 => println!("EOR (Absolute Y) is not implemented."),
            0x41 => println!("EOR (Indirext X) is not implemented."),
            0x51 => println!("EOR (Indirect Y) is not implemented."),
            /* ----- INC ----- */
            0xE6 => {
                self.inc(AddressingMode::ZeroPage);
                self.program_counter += 2;
            },
            0xF6 => {
                self.inc(AddressingMode::ZeroPage_X);
                self.program_counter += 2;
            },
            0xEE => {
                self.inc(AddressingMode::Absolute);
                self.program_counter += 3;
            },
            0xFE => {
                self.inc(AddressingMode::Absolute_X);
                self.program_counter += 3;
            },
            /* ----- INX ----- */
            0xE8 => {
                self.inx();
                self.program_counter += 1;
            },
            /* ----- INY ----- */
            0xC8 => {
                self.iny();
                self.program_counter += 1;
            },
            /* ----- JMP ----- */
            0x4C => self.jmp(AddressingMode::Absolute),
            0x6C => println!("JMP (Indirect) is not implemented."),
            /* ----- JSR ----- */
            0x20 => println!("JSR (Absolute) is not implemented."),
            /* ----- LDA ----- */
            0xA9 => self.lda(AddressingMode::Immediate),
            0xA5 => self.lda(AddressingMode::ZeroPage),
            0xB5 => println!("LDA (Zero Page X) is not implemented."),
            0xAD => {
                self.lda(AddressingMode::Absolute);
                self.program_counter += 1;
            },
            0xBD => println!("LDA (Absolute X) is not implemented."),
            0xB9 => println!("LDA (Absolute Y) is not implemented."),
            0xA1 => println!("LDA (Indirect X) is not implemented."),
            0xB1 => println!("LDA (Indirect Y) is not implemented."),
            /* ----- LDX ----- */
            0xA2 => self.ldx(AddressingMode::Immediate),
            0xA6 => println!("LDX (Zero Page) is not implemented."),
            0xB6 => println!("LDX (Zero Page Y) is not implemented."),
            0xAE => println!("LDX (Absolute) is not implemented."),
            0xBE => println!("LDX (Absolute Y) is not implemented."),
            /* ----- LDY ----- */
            0xA0 => self.ldy(AddressingMode::Immediate),
            0xA4 => println!("LDY (Zero Page) is not implemented."),
            0xB4 => println!("LDY (Zero Page X) is not implemented."),
            0xAC => println!("LDY (Absolute) is not implemented."),
            0xBC => println!("LDY (Absolute X) is not implemented."),
            /* ----- LSR ----- */
            0x4A => {
                self.lsr(AddressingMode::Immediate);
                self.program_counter += 1;
            },
            0x46 => {
                self.lsr(AddressingMode::ZeroPage);
                self.program_counter += 2;
            },
            0x56 => {
                self.lsr(AddressingMode::ZeroPage_X);
                self.program_counter += 2;
            },
            0x4E => {
                self.lsr(AddressingMode::Absolute);
                self.program_counter += 3;
            },
            0x5E => {
                self.lsr(AddressingMode::Absolute_X);
                self.program_counter += 3;
            },
            /* ----- NOP ----- */
            0xEA => self.program_counter += 1,
            /* ----- ORA ----- */
            0x09 => println!("ORA (Immediate) is not implemented."),
            0x05 => println!("ORA (Zero Page) is not implemented."),
            0x15 => println!("ORA (Zero Page X) is not implemented."),
            0x0D => println!("ORA (Absolute) is not implemented."),
            0x1D => println!("ORA (Absolute X) is not implemented."),
            0x19 => println!("ORA (Absolute Y) is not implemented."),
            0x01 => println!("ORA (Indirect X) is not implemented."),
            0x11 => println!("ORA (Indirect Y) is not implemented."),
            /* ----- PHA ----- */
            0x48 => self.pha(),
            /* ----- PHP ----- */
            0x08 => self.php(),
            /* ----- PLA ----- */
            0x68 => self.pla(),
            /* ----- PLP ----- */
            0x28 => self.plp(),
            /* ----- ROL ----- */
            0x2A => {
                self.rol(AddressingMode::Accumulator);
                self.program_counter += 1;
            },
            0x26 => {
                self.rol(AddressingMode::ZeroPage);
                self.program_counter += 2;
            },
            0x36 => {
                self.rol(AddressingMode::ZeroPage_X);
                self.program_counter += 2;
            },
            0x2E => {
                self.rol(AddressingMode::Absolute);
                self.program_counter += 3;
            },
            0x3E => {
                self.rol(AddressingMode::Absolute_X);
                self.program_counter += 3;
            },  
            /* ----- ROR ----- */
            0x6A => {
                self.ror(AddressingMode::Accumulator);
                self.program_counter += 1;
            },
            0x66 => {
                self.ror(AddressingMode::ZeroPage);
                self.program_counter += 2;
            },
            0x76 => {
                self.ror(AddressingMode::ZeroPage_X);
                self.program_counter += 2;
            },
            0x6E => {
                self.ror(AddressingMode::Absolute);
                self.program_counter += 3;
            },
            0x7E => {
                self.ror(AddressingMode::Absolute_X);
                self.program_counter += 3;
            },
            /* ----- RTI ----- */
            0x40 => println!("RTI is not implemented."),
            /* ----- RTS ----- */
            0x60 => println!("RTS is not implemented."),
            /* ----- SBC ----- */
            0xE9 => println!("SBC (Immediate) is not implemented."),
            0xE5 => println!("SBC (Zero Page) is not implemented."),
            0xF5 => println!("SBC (Zero Page X) is not implemented."),
            0xED => println!("SBC (Absolute) is not implemented."),
            0xFD => println!("SBC (Absolute X) is not implemented."),
            0xF9 => println!("SBC (Absolute Y) is not implemented."),
            0xE1 => println!("SBC (Indirect X) is not implemented."),
            0xF1 => println!("SBC (Indirect Y) is not implemented."),
            /* ----- SEC ----- */
            0x38 => {
                self.set_carry_bit();
                self.program_counter += 1;
            },
            /* ----- SED ----- */
            0xF8 => {
                self.set_decimal_bit();        
                self.program_counter += 1;
            },
            /* ----- SEI ----- */
            0x78 => {
                self.set_interrupt_disable_bit();
                self.program_counter += 1;
            },
            /* ----- STA ----- */
            0x85 => println!("STA (Zero Page) is not implemented."),
            0x95 => println!("STA (Zero Page X) is not implemented."),
            0x8D => println!("STA (Absolute) is not implemented."),
            0x9D => println!("STA (Absolute X) is not implemented."),
            0x99 => println!("STA (Absolute Y) is not implemented."),
            0x81 => println!("STA (Indirect X) is not implemented."),
            0x91 => println!("STA (Indirect Y) is not implemented."),
            /* ----- STX ----- */
            0x86 => println!("STX (Zero Page) is not implemented."),
            0x96 => println!("STX (Zero Page X) is not implemented."),
            0x8E => println!("STX (Absolute) is not implemented."),
            /* ----- STY ----- */
            0x84 => println!("STY (Zero Page) is not implemented."),
            0x94 => println!("STY (Zero Page X) is not implemented."),
            0x8C => println!("STY (Absolute) is not implemented."),
            /* ----- TAX ----- */
            0xAA => {
                self.tax();
                self.program_counter += 1;
            },
            /* ----- TAY ----- */
            0xA8 => {
                self.tay();
                self.program_counter += 1;
            },
            /* ----- TSX ----- */
            0xBA => {
                self.tsx();
                self.program_counter += 1;
            },
            /* ----- TXA ----- */
            0x8A => {
                self.txa();
                self.program_counter += 1;
            },
            /* ----- TXS ----- */
            0x9A => {
                self.txs();
                self.program_counter += 1;
            },
            /* ----- TYA ----- */
            0x98 => {
                self.tya();
                self.program_counter += 1;
            },
            /* ----- Unsupported ----- */
            _ => println!("Got unrecognized instruction, quitting. {:?}", instruction)
        }
    }

    fn set_zero_bit(&mut self) {
        self.status = self.status | ZERO;
    }

    fn clear_zero_bit(&mut self) {
        self.status = self.status & !ZERO;
    }

    fn set_negative_bit(&mut self) {
        self.status = self.status | NEGATIVE;
    }

    fn clear_negative_bit(&mut self) {
        self.status = self.status & !NEGATIVE;
    }

    fn set_carry_bit(&mut self) {
        self.status = self.status | CARRY;
    }

    fn clear_carry_bit(&mut self) {
        self.status = self.status & !CARRY;
    }

    fn set_decimal_bit(&mut self) {
        self.status = self.status | DECIMAL_MODE;
    }

    fn clear_decimal_bit(&mut self) {
        self.status = self.status & !DECIMAL_MODE;
    }

    fn set_interrupt_disable_bit(&mut self) {
        self.status = self.status | INTERRUPT_DISABLE;
    }

    fn clear_interrupt_disable_bit(&mut self) {
        self.status = self.status & !INTERRUPT_DISABLE;
    }

    fn clear_overflow_bit(&mut self) {
        self.status = self.status & !OVERFLOW;
    }

    fn set_status_bit_if_bit_set(&mut self, bit_to_check: u8, bit_to_set: u8, val: u8) {
        if val & bit_to_check != 0 {
            self.status = self.status | bit_to_set;
        } else {
            self.status = self.status & !bit_to_set;
        }
    }

    fn grab_next_byte_and_advance_counter(&mut self) -> u8 {
        let val = self.memory[self.program_counter as usize];
        self.program_counter += 1;
        val
    }

    fn update_negative_and_zero(&mut self, byte: u8) {
        if byte == 0 {
            self.set_zero_bit();
        } else {
            self.clear_zero_bit();
        }

        if byte & 0b1000_0000 != 0 {
            self.set_negative_bit();
        } else {
            self.clear_negative_bit();
        }
    }

    pub fn load(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        println!("Writing to address 0x{:04x}", addr);
        self.memory[addr as usize] = data;
    }

    pub fn load16(&mut self, addr: u16) -> u16 {
        (self.load(addr+1) as u16) << 8 | self.load(addr) as u16
    }

    pub fn write16(&mut self, addr: u16, data: u16) {
        self.write(addr+1, (data >> 8) as u8);
        self.write(addr,    data       as u8);
    }

    pub fn get_stack_memory_addr(&self) -> u16 {
        STACK_START_ADDR | self.stack_pointer as u16
    }

    fn push_to_stack(&mut self, data: u8) {
        self.stack_pointer -= 1;
        self.write(self.get_stack_memory_addr(), data);
    }

    fn pop_from_stack(&mut self) -> u8 {
        let pop_val = self.load(self.get_stack_memory_addr());
        self.stack_pointer += 1;

        pop_val
    }

    // Used for debugging - Austin Haskell 8/17/2021
    #[allow(dead_code)]
    pub fn peep_stack(&mut self) -> u8 {
        self.load(self.get_stack_memory_addr())
    }

    fn branch(&mut self, flag: u8, checking_if_set: bool) {
        let displacement = self.load(self.program_counter + 1) as i8;

        if checking_if_set && self.status & flag != 0 ||
          !checking_if_set && self.status & flag == 0  {

            println!("Displacement {:?}", displacement);
            let branch_addr = self.program_counter as i16 + displacement as i16;
            println!("branching {:?} -> {:?} (displacement of {:?})", self.program_counter , branch_addr as u16, displacement as i8);
            self.program_counter = branch_addr as u16;

        } else {
            self.program_counter += 2;
        }
    }

    fn pha(&mut self) {
        self.push_to_stack(self.registers.a);
    }

    fn php(&mut self) {
        self.push_to_stack(self.status);
    }

    fn pla(&mut self) {
        self.registers.a = self.pop_from_stack();
    }

    fn plp(&mut self) {
        self.status = self.pop_from_stack();
    }

    fn txa(&mut self) {
        self.registers.a = self.registers.x;
        self.update_negative_and_zero(self.registers.a);
    }

    fn tya(&mut self) {
        self.registers.a = self.registers.y;
        self.update_negative_and_zero(self.registers.a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.registers.x;
    }

    fn tay(&mut self) {
        self.registers.y = self.registers.a;
        self.update_negative_and_zero(self.registers.y);
    }

    fn tax(&mut self) {
        self.registers.x = self.registers.a;
        self.update_negative_and_zero(self.registers.x);
    }

    fn lda(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let value = self.load(addr);

        self.update_negative_and_zero(value);
        self.registers.a = value;
    }

    fn ldx(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let value = self.load(addr);

        self.update_negative_and_zero(value);
        self.registers.x = value;
    }

    fn ldy(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let value = self.load(addr);

        self.update_negative_and_zero(value);
        self.registers.y = value;
    }

    fn lsr(&mut self, mode: AddressingMode) {
        if mode != AddressingMode::Accumulator {
            let addr = get_operator_from_addressing_mode(self, mode);
            let mut value = self.load(addr);
            if value & 0b0000_0001 != 0 {
                self.set_carry_bit();
            } else {
                self.clear_carry_bit();
            }
            value = value >> 1;
            self.write(addr, value);
        } else {
            if self.registers.a & 0b0000_0001 != 0 {
                self.set_carry_bit();
            } else {
                self.clear_carry_bit();
            }
            self.registers.a = self.registers.a >> 1;
        }
    }

    fn inc(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let mut val = self.load(addr);

        val = val.wrapping_add(1);

        self.update_negative_and_zero(val);

        self.write(addr, val);
    }

    fn inx(&mut self) {
        self.registers.x += 1;
        self.update_negative_and_zero(self.registers.x);
    }

    fn iny(&mut self) {
        self.registers.y += 1;
        self.update_negative_and_zero(self.registers.y);
    }

    fn dex(&mut self) {
        self.registers.x -= 1;
        self.update_negative_and_zero(self.registers.x);
    }

    fn dey(&mut self) {
        self.registers.y -= 1;
        self.update_negative_and_zero(self.registers.y);
    }

    fn and(&mut self) {
        let param = self.grab_next_byte_and_advance_counter();
        let mem_at_location = self.load(param as u16);

        self.registers.a = self.registers.a & mem_at_location;

        self.update_negative_and_zero(self.registers.a);
    }

    fn jmp(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        
        self.program_counter = addr;
    }

    fn tsx(&mut self) {
        self.registers.x = self.stack_pointer;
        self.update_negative_and_zero(self.registers.x);
    }

    fn bit(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let val = self.load(addr);

        self.set_status_bit_if_bit_set(0b0100_0000, OVERFLOW, val);
        self.set_status_bit_if_bit_set(0b1000_0000, NEGATIVE, val);
        
        if val & self.registers.a == 0 {
            self.set_zero_bit();
        } else {
            self.clear_zero_bit();
        }
    }

    fn asl(&mut self, mode: AddressingMode) {

        let mut addr: u16 = 0;
        let mut val: u8;

        if mode == AddressingMode::Accumulator {
            val = self.registers.a; 
        } else {
            addr = get_operator_from_addressing_mode(self, mode);
            val = self.load(addr); 
        }

        self.set_status_bit_if_bit_set(0b1000_0000, CARRY, val);

        if val == 0 {
            self.set_zero_bit();
        } else {
            self.clear_zero_bit();
        }

        val = val << 1;

        self.set_status_bit_if_bit_set(0b1000_0000, NEGATIVE, val);

        if mode == AddressingMode::Accumulator {
            self.registers.a = val;
        } else {
            self.write(addr, val);
        }
    }

    fn rol(&mut self, mode: AddressingMode) {
        let mut addr: u16 = 0;
        let mut val: u8;

        if mode == AddressingMode::Accumulator {
            val = self.registers.a;

            if val == 0 {
                self.set_zero_bit();
                return;
            } else {
                self.clear_zero_bit();
            }

        } else {
            addr = get_operator_from_addressing_mode(self, mode);
            println!("Addr from get_operator 0x{:04x}", addr);
            val = self.load(addr);
        }

        let old_carry = self.status & CARRY;
        self.set_status_bit_if_bit_set(0b1000_0000, CARRY, val);

        val = val << 1;
        if old_carry != 0 {
            val = val | 0b0000_0001;
        }

        self.set_status_bit_if_bit_set(0b1000_0000, NEGATIVE, val);

        if mode == AddressingMode::Accumulator {
            self.registers.a = val;
        } else {
            self.write(addr, val);
        }
    }

    fn ror(&mut self, mode: AddressingMode) {
        let mut addr: u16 = 0;
        let mut val: u8;

        if mode == AddressingMode::Accumulator {
            val = self.registers.a;

            if val == 0 {
                self.set_zero_bit();
                return;
            } else {
                self.clear_zero_bit();
            }

        } else {
            addr = get_operator_from_addressing_mode(self, mode);
            println!("Addr from get_operator 0x{:04x}", addr);
            val = self.load(addr);
        }

        let old_carry = self.status & CARRY;
        self.set_status_bit_if_bit_set(0b0000_0001, CARRY, val);

        val = val >> 1;
        if old_carry != 0 {
            val = val | 0b1000_0000;
        }

        self.set_status_bit_if_bit_set(0b1000_0000, NEGATIVE, val);

        if mode == AddressingMode::Accumulator {
            self.registers.a = val;
        } else {
            self.write(addr, val);
        }
    }

    fn cpx(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let val = self.load(addr);

        if self.registers.x > val {
            self.set_carry_bit();
        } else {
            self.clear_carry_bit();
        }

        if self.registers.x == val {
            self.set_zero_bit();
        } else {
            self.clear_zero_bit();
        }

        self.set_status_bit_if_bit_set(0b1000_0000, NEGATIVE, val);
    }
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
}