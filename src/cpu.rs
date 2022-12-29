pub const NEGATIVE: u8          = 0b1000_0000;
pub const OVERFLOW: u8          = 0b0100_0000;
pub const BREAK:    u8          = 0b0010_0000;

pub const DECIMAL_MODE: u8      = 0b0000_1000;
pub const INTERRUPT_DISABLE: u8 = 0b0000_0100;
pub const ZERO: u8              = 0b0000_0010;
pub const CARRY: u8             = 0b0000_0001;

pub const SIGN_BIT: u8          = 0b1000_0000;

pub const PROGRAM_START_ADDR:  u16 = 0x8000;
pub const PROGRAM_READ_START:  u16 = 0xFFFC;
pub const STACK_START_ADDR:    u16 = 0x0100;
pub const INITIAL_STACK_VALUE: u8  = 0xFF;

use crate::addressing_modes::*;
use crate::util::*;

pub struct CPU {
    pub program_counter: u16,
    pub registers: Registers,
    pub stack_pointer: u8,
    pub status: u8,
    pub memory: [u8; 0xFFFF]
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
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

    #[allow(dead_code)]
    pub fn dump_status(&self) {
        println!("Carry     : {:?}", self.status & CARRY             != 0);
        println!("Zero      : {:?}", self.status & ZERO              != 0);
        println!("Interrupt : {:?}", self.status & INTERRUPT_DISABLE != 0);
        println!("Decimal   : {:?}", self.status & DECIMAL_MODE      != 0);
        println!("Break     : {:?}", self.status & BREAK             != 0);
        println!("Negative  : {:?}", self.status & CARRY             != 0);
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
                print!(" {:02x} ", byte);
            }

            line += 1;
            if line % display_width == 0 {
                println!();
            }
        }
        println!();
    }

    // Used for debugging - Austin Haskell 8/17/2021
    #[allow(dead_code)]
    pub fn dump_memory_to_human_readable_file(&mut self, start: u16, end: u16, filepath: String) {
        let display_width: u32 = 16;
        let mut line = start as u32;

        use std::fs::File;
        use std::io::Write;
        use std::io::BufWriter;

        let file = File::create(filepath.clone());
        if file.is_err() {
            println!("Could not create log for [{:?}]", filepath);
        }
        let mut file_out = BufWriter::new(file.unwrap());   

        for byte in self.memory[start as usize..end as usize].iter() {
            if line % display_width == 0  || line == 0 {
                file_out.write(format!("0x{:04x} ", line).as_bytes()).expect("Failed to format memdump line number");
            }

            if *byte == 0 {
                file_out.write(format!(" .. ").as_bytes()).expect("Failed to format memdump '..'");
            }
            else {
                file_out.write(format!(" {:02x} ", byte).as_bytes()).expect("Failed to write byte value for memdump");
            }

            line += 1;
            if line % display_width == 0 {
                file_out.write(format!("\n").as_bytes()).expect("Failed to write newline to memdump");
            }
        }
        file_out.flush().expect("Failed to flush memdump");
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
            if self.memory[self.program_counter as usize] == 0x00 {
                break;
            }
            self.run_next_instruction();

            // I'm not sure if this will cause issues -Austin Haskell 8/20/2021
            if self.program_counter >= 0xFFFF {
                break;
            }
        }
    }

    pub fn run_next_instruction(&mut self) {        
        let instruction = self.memory[self.program_counter as usize];
        let mode: AddressingMode = map_instruction_to_addressing_mode(instruction);
        match instruction {
            /* ----- ADC ----- */
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(mode),
            /* ----- AND ----- */
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(mode),
            /* ----- ASL ----- */
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(mode),
            /* ----- BCC ----- */
            0x90 => self.branch(CARRY, false),
            /* ----- BCS ----- */
            0xB0 => self.branch(CARRY, true),
            /* ----- BEQ ----- */
            0xF0 => self.branch(ZERO, true),
            /* ----- BIT ----- */
            0x24 | 0x2C => self.bit(mode),
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
            0x18 => self.clear_carry_bit(),
            /* ----- CLD ----- */
            0xD8 => self.clear_decimal_bit(),
            /* ----- CLI ----- */
            0x58 => self.clear_interrupt_disable_bit(),
            /* ----- CLV ----- */
            0xB8 => self.clear_overflow_bit(),
            /* ----- CMP ----- */
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.cmp(mode),
            /* ----- CPX ----- */
            0xE0 | 0xE4 | 0xEC => self.cpx(mode),
            /* ----- CPY ----- */
            0xC0 | 0xC4 | 0xCC => self.cpy(mode),
            /* ----- DEC ----- */
            0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(mode),
            /* ----- DEX ----- */
            0xCA => self.dex(),
            /* ----- DEY ----- */
            0x88 => self.dey(),
            /* ----- EOR ----- */
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => println!("EOR is not implemented."),
            /* ----- INC ----- */
            0xE6 | 0xF6 | 0xEE | 0xFE => self.inc(mode),
            /* ----- INX ----- */
            0xE8 => self.inx(),
            /* ----- INY ----- */
            0xC8 => self.iny(),
            /* ----- JMP ----- */
            0x4C | 0x6C => self.jmp(mode),
            /* ----- JSR ----- */
            0x20 => self.jsr(mode),
            /* ----- LDA ----- */
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(mode),
            /* ----- LDX ----- */
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(mode),
            /* ----- LDY ----- */
            0xA0 | 0xB4 | 0xA4 | 0xAC | 0xBC => self.ldy(mode),
            /* ----- LSR ----- */
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.lsr(mode),
            /* ----- NOP ----- */
            0xEA => {},
            /* ----- ORA ----- */
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => println!("ORA is not implemented. "),
            /* ----- PHA ----- */
            0x48 => self.pha(),
            /* ----- PHP ----- */
            0x08 => self.php(),
            /* ----- PLA ----- */
            0x68 => self.pla(),
            /* ----- PLP ----- */
            0x28 => self.plp(),
            /* ----- ROL ----- */
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.rol(mode),
            /* ----- ROR ----- */
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E=> self.ror(mode),
            /* ----- RTI ----- */
            0x40 => println!("RTI is not implemented."),
            /* ----- RTS ----- */
            0x60 => self.rts(),
            /* ----- SBC ----- */
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1=> println!("SBC is not implemented."),
            /* ----- SEC ----- */
            0x38 => self.set_carry_bit(),
            /* ----- SED ----- */
            0xF8 => self.set_decimal_bit(),
            /* ----- SEI ----- */
            0x78 => self.set_interrupt_disable_bit(),
            /* ----- STA ----- */
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(mode),
            /* ----- STX ----- */
            0x86 | 0x96 | 0x8E => self.stx(mode),
            /* ----- STY ----- */
            0x84 | 0x94 | 0x8C => self.sty(mode),
            /* ----- TAX ----- */
            0xAA => self.tax(),
            /* ----- TAY ----- */
            0xA8 => self.tay(),
            /* ----- TSX ----- */
            0xBA => self.tsx(),
            /* ----- TXA ----- */
            0x8A => self.txa(),
            /* ----- TXS ----- */
            0x9A => self.txs(),
            /* ----- TYA ----- */
            0x98 => self.tya(),
            /* ----- Unsupported ----- */
            _ => println!("Got unrecognized instruction, skipping. {:?}", instruction)
        }
        
        // Dont advance the program counter if it was a jmp instruction - Austin Haskell 8/21/2021
        if instruction != 0x4C && instruction != 0x6C && instruction != 0x20 {
            self.program_counter += addressing_mode_to_program_counter_advancement_amount(mode);
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

        if byte & SIGN_BIT != 0 {
            self.set_negative_bit();
        } else {
            self.clear_negative_bit();
        }
    }

    pub fn load(&mut self, addr: u16) -> u8 {
        #[cfg(test)]
        println!("Reading memory address at 0x{:04x} and got value 0x{:02x}", addr, self.memory[addr as usize]);
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        #[cfg(test)]
        println!("Writing the value 0x{:02x} to address 0x{:04x}", data, addr);
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

    fn push_to_stack_16(&mut self, data: u16) {
        let low: u8  = (data & 0xFF) as u8;
        let high: u8 = ((data >> 8) & 0xFF) as u8;

        self.push_to_stack(high);
        self.push_to_stack(low);
    }

    fn pop_from_stack(&mut self) -> u8 {
        let pop_val = self.load(self.get_stack_memory_addr());
        self.stack_pointer += 1;

        pop_val
    }

    fn pop_from_stack_16(&mut self) -> u16 {
        let low: u8  = self.pop_from_stack();
        let high: u8 = self.pop_from_stack();

        ((high as u16) << 8) | (low as u16)
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
        let val = self.load(addr);

        self.update_negative_and_zero(val);
        self.registers.a = val;
    }

    fn ldx(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let val = self.load(addr);

        self.update_negative_and_zero(val);
        self.registers.x = val;
    }

    fn ldy(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let val = self.load(addr);

        self.update_negative_and_zero(val);
        self.registers.y = val;
    }

    fn lsr(&mut self, mode: AddressingMode) {
        if mode != AddressingMode::Accumulator {
            let addr = get_operator_from_addressing_mode(self, mode);
            let mut val = self.load(addr);

            self.set_status_bit_if_bit_set(0b0000_0001, CARRY, val);

            val = val >> 1;
            self.write(addr, val);
            self.update_negative_and_zero(val);
        } else {
            self.set_status_bit_if_bit_set(0b0000_0001, CARRY, self.registers.a);

            self.registers.a = self.registers.a >> 1;
            self.update_negative_and_zero(self.registers.a);
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

    fn and(&mut self, mode: AddressingMode) {
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
        self.set_status_bit_if_bit_set(SIGN_BIT, NEGATIVE, val);
        
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

        self.set_status_bit_if_bit_set(SIGN_BIT, CARRY, val);

        if val == 0 {
            self.set_zero_bit();
        } else {
            self.clear_zero_bit();
        }

        val = val << 1;

        self.set_status_bit_if_bit_set(SIGN_BIT, NEGATIVE, val);

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
        self.set_status_bit_if_bit_set(SIGN_BIT, CARRY, val);

        val = val << 1;
        if old_carry != 0 {
            val = val | 0b0000_0001;
        }

        self.set_status_bit_if_bit_set(SIGN_BIT, NEGATIVE, val);

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
            val = val | SIGN_BIT;
        }

        self.set_status_bit_if_bit_set(SIGN_BIT, NEGATIVE, val);

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

        self.set_status_bit_if_bit_set(SIGN_BIT, NEGATIVE, val);
    }

    fn cpy(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let val = self.load(addr);

        if self.registers.y > val {
            self.set_carry_bit();
        } else {
            self.clear_carry_bit();
        }

        if self.registers.y == val {
            self.set_zero_bit();
        } else {
            self.clear_zero_bit();
        }

        self.set_status_bit_if_bit_set(SIGN_BIT, NEGATIVE, val);
    }

    fn adc(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let val = self.load(addr);

        let mut scratch_value: u16 = val as u16 + self.registers.a as u16;
        if self.status & CARRY != 0 {
            scratch_value += 1;
        }

        // Overflow occoured. 
        if scratch_value > 0xFF {
            self.set_carry_bit();
        } else {
            self.clear_carry_bit();
        }   

        let sign_of_sum     = scratch_value as u8 & SIGN_BIT;
        let register_a_sign = self.registers.a    & SIGN_BIT;
        let operand_a_sign  = val                 & SIGN_BIT;

        if sign_of_sum != register_a_sign && 
           sign_of_sum != operand_a_sign {
            self.status = self.status | OVERFLOW;
        } else {
            self.clear_overflow_bit();
        }

        self.registers.a = (scratch_value & 0xFF) as u8;

        self.update_negative_and_zero(self.registers.a);
    }

    fn dec(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let mut val = self.load(addr);

        val = val.wrapping_sub(1);

        self.update_negative_and_zero(val);

        self.write(addr, val);
    }

    fn sta(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        
        self.write(addr, self.registers.a);
    }

    fn sty(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        
        self.write(addr, self.registers.y);
    }

    fn stx(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        
        self.write(addr, self.registers.x);
    }

    fn jsr(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);

        self.push_to_stack_16(self.program_counter - 1);
        self.program_counter = addr;
    }

    fn rts(&mut self) {
        self.program_counter = self.pop_from_stack_16();
    } 

    fn cmp(&mut self, mode: AddressingMode) {
        let addr = get_operator_from_addressing_mode(self, mode);
        let val = self.load(addr);

        let answer: i8 = (self.registers.a as i8).wrapping_sub(val as i8);

        self.update_negative_and_zero(answer as u8);

        if self.registers.a >= val {
            self.set_carry_bit();
        } else {
            self.clear_carry_bit();
        }
    }
}