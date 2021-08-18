use crate::cpu::*;

#[test]
fn clc() {
    let program: Vec<u8> = vec![0x18];
    let mut cpu = CPU::new(program);
    cpu.status = CARRY;

    cpu.run_next_instruction();
    assert_eq!(0, cpu.status);
}

#[test]
fn cld() {
    let program: Vec<u8> = vec![0xD8];
    let mut cpu = CPU::new(program);
    cpu.status = DECIMAL_MODE;

    cpu.run_next_instruction();
    assert_eq!(0, cpu.status);
}

#[test]
fn cli() {
    let program: Vec<u8> = vec![0x58];
    let mut cpu = CPU::new(program);
    cpu.status = INTERRUPT_DISABLE;

    cpu.run_next_instruction();
    assert_eq!(0, cpu.status);
}

#[test]
fn clv() {
    let program: Vec<u8> = vec![0xB8];
    let mut cpu = CPU::new(program);
    cpu.status = OVERFLOW;

    cpu.run_next_instruction();
    assert_eq!(0, cpu.status);
}

#[test]
fn sec() {
    let program: Vec<u8> = vec![0x38];
    let mut cpu = CPU::new(program);
    
    cpu.run_next_instruction();
    assert_eq!(cpu.status, CARRY);
}

#[test]
fn sed() {
    let program: Vec<u8> = vec![0xF8];
    let mut cpu = CPU::new(program);
    
    cpu.run_next_instruction();
    assert_eq!(cpu.status, DECIMAL_MODE);
}

#[test]
fn sei() {
    let program: Vec<u8> = vec![0x78];
    let mut cpu = CPU::new(program);
    
    cpu.run_next_instruction();
    assert_eq!(cpu.status, INTERRUPT_DISABLE);
}