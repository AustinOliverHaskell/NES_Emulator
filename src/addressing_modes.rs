use crate::cpu::CPU;

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   Accumulator,
   Relative,
   Indirect
}

pub fn get_operator_from_addressing_mode(cpu: &mut CPU, addressing_mode: AddressingMode) -> u16 {
    
    let counter = cpu.program_counter + 1;
    match addressing_mode {
        AddressingMode::Immediate => counter,
        AddressingMode::ZeroPage => cpu.load(counter) as u16,
        AddressingMode::Absolute => cpu.load16(counter),
        AddressingMode::ZeroPage_X => {
            let first_byte = cpu.load(counter);
            let address: u16 = first_byte.wrapping_add(cpu.registers.x) as u16;
            address
        },
        AddressingMode::ZeroPage_Y => { 
            let first_byte = cpu.load(counter);
            let address: u16 = first_byte.wrapping_add(cpu.registers.y) as u16;
            address
        },
        AddressingMode::Absolute_X => {
            let mem = cpu.load16(counter);
            let address = mem.wrapping_add(cpu.registers.x as u16);
            address
        },
        AddressingMode::Absolute_Y => {
            let mem = cpu.load16(counter);
            let address = mem.wrapping_add(cpu.registers.y as u16);
            address
        }
        AddressingMode::Indirect_X => {
            let base_ptr = cpu.load(counter);
            
            let ptr = (base_ptr as u8).wrapping_add(cpu.registers.x);
            let low = cpu.load(ptr as u16);
            let high = cpu.load(ptr.wrapping_add(1) as u16);
            (high as u16) << 8 | (low as u16) 
        },
        AddressingMode::Indirect_Y => {
            let base_ptr = cpu.load(counter);
            
            let ptr = (base_ptr as u8).wrapping_add(cpu.registers.y);
            let low = cpu.load(ptr as u16);
            let high = cpu.load(ptr.wrapping_add(1) as u16);
            (high as u16) << 8 | (low as u16) 
        }
        _ => panic!()
    }

}