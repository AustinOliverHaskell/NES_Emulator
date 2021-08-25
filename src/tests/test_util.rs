use crate::cpu::CPU;
use crate::addressing_modes::AddressingMode;
use crate::util::map_instruction_to_addressing_mode;

pub fn create_program_from_command_and_address(instruction: u8, addr: u16) -> Vec<u8>{
    let mut program: Vec<u8> = Vec::new();

    program.push(instruction);
    program.push((addr & 0xFF) as u8);

    if addr > 0xFF {
        program.push((addr >> 8) as u8);
    }

    program
}

pub fn create_test_cpu_from_addr_and_instruction(instruction: u8, addr: u16) -> CPU {

    let addressing_mode = map_instruction_to_addressing_mode(instruction);
    let mut program = create_program_from_command_and_address(instruction, addr);
    let addressing_bump = 0x33;

    match addressing_mode {
        AddressingMode::Absolute_X |
        AddressingMode::Absolute_Y |
        AddressingMode::Indirect_X |
        AddressingMode::Indirect_Y |
        AddressingMode::ZeroPage_X |
        AddressingMode::ZeroPage_Y 
            => {
                program[1] = program[1].wrapping_sub(addressing_bump);
            },
        _ => {}
    }

    let mut cpu = CPU::new(program);

    match addressing_mode {
        AddressingMode::Absolute_X |
        AddressingMode::ZeroPage_X |
        AddressingMode::Indirect_X 
            => cpu.registers.y = addressing_bump,

        AddressingMode::Absolute_Y |
        AddressingMode::ZeroPage_Y |
        AddressingMode::Indirect_Y 
            => cpu.registers.x = addressing_bump,

        _ => {}
    }

    cpu
}

pub fn setup_cpu_for_addressing_mode(cpu: &mut CPU, test_addr: u16, mode: AddressingMode) {
    let high: u8 = (test_addr >> 8) as u8;
    let low : u8 = test_addr as u8;


}