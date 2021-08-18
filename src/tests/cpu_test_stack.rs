use crate::cpu::*;

#[test]
fn pha() {
    let test_value = 0xF6; // Number is arbitrary

    let program: Vec<u8> = vec![0x48];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.peep_stack(), test_value);
}

#[test]
fn php() {
    let test_status: u8 = NEGATIVE | CARRY;

    let program: Vec<u8> = vec![0x08];
    let mut cpu = CPU::new(program);
    cpu.status = test_status;

    cpu.run_next_instruction();

    assert_eq!(cpu.peep_stack(), test_status);
}

#[test]
fn pla() {
    let test_value = 0xA7; // Number is arbitrary

    let program: Vec<u8> = vec![0x68];
    let mut cpu = CPU::new(program);
    cpu.stack_pointer -= 1;
    cpu.memory[cpu.get_stack_memory_addr() as usize] = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.registers.a, test_value);
}

#[test]
fn plp() {
    let test_status: u8 = NEGATIVE | CARRY;

    let program: Vec<u8> = vec![0x28];
    let mut cpu = CPU::new(program);
    cpu.stack_pointer -= 1;
    cpu.memory[cpu.get_stack_memory_addr() as usize] = test_status;

    cpu.run_next_instruction();

    assert_eq!(cpu.status, test_status);
}

#[test]
fn tsx() {
    let program: Vec<u8> = vec![0xBA];
    let mut cpu = CPU::new(program);

    cpu.run_next_instruction();
    
    assert_eq!(cpu.registers.x, INITIAL_STACK_VALUE);

    // @todo: Check for status
}

#[test]
fn txs() {
    let test_val = 0x88; // Number is arbitrary
    let program: Vec<u8> = vec![0x9A];
    let mut cpu = CPU::new(program);
    cpu.registers.x = test_val;

    cpu.run_next_instruction();

    assert_eq!(cpu.stack_pointer, test_val);
}

#[test]
fn brk() {
    assert!(false);
}