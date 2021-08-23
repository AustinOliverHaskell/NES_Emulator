use crate::cpu::*;

#[test]
fn bcc_carry_set() {
    let program: Vec<u8> = vec![0x90, 0x0F];
    let mut cpu = CPU::new(program);
    cpu.status = CARRY;

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location);
}

#[test]
fn bcc_carry_not_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0x90, jump_offset];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bcs_carry_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0xB0, jump_offset];
    let mut cpu = CPU::new(program);
    cpu.status = CARRY;

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bcs_carry_not_set() {
    let program: Vec<u8> = vec![0xB0, 0x0F];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location);
}

#[test]
fn beq_zero_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0xF0, jump_offset];
    let mut cpu = CPU::new(program);
    cpu.status = ZERO;

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn beq_zero_not_set() {
    let program: Vec<u8> = vec![0xF0, 0x0F];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location);
}

#[test]
fn bmi_negative_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0x30, jump_offset];
    let mut cpu = CPU::new(program);
    cpu.status = NEGATIVE;

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bmi_negative_not_set() {
    let program: Vec<u8> = vec![0x30, 0x0F];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location);
}

#[test]
fn bne_zero_set() {
    let program: Vec<u8> = vec![0xD0, 0x0F];
    let mut cpu = CPU::new(program);
    cpu.status = ZERO;

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location);
}

#[test]
fn bne_zero_not_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0xD0, jump_offset];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bpl_negative_set() {
    let program: Vec<u8> = vec![0x10, 0x0F];
    let mut cpu = CPU::new(program);
    cpu.status = NEGATIVE;

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location);
}

#[test]
fn bpl_negative_not_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0x10, jump_offset];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bvc_overflow_not_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0x50, jump_offset];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bvc_overflow_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0x50, jump_offset];
    let mut cpu = CPU::new(program);
    cpu.status = OVERFLOW;

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bvs_overflow_not_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0x70, jump_offset];
    let mut cpu = CPU::new(program);

    let expected_branch_location = cpu.program_counter + 2;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn bvs_overflow_set() {
    let jump_offset = 0x0F;
    let program: Vec<u8> = vec![0x70, jump_offset];
    let mut cpu = CPU::new(program);
    cpu.status = OVERFLOW;

    let expected_branch_location = cpu.program_counter as i16 + jump_offset as i16;

    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, expected_branch_location as u16);
}

#[test]
fn jmp_absolute() {
    let jump_addr = 0x4000;
    let program: Vec<u8> = vec![0x4C, 0x00, 0x40];
    let mut cpu = CPU::new(program);

    cpu.run_next_instruction();
    cpu.dump_memory(0x8000, 0x8100);
    println!("0x{:04x} but is supposed to be 0x{:04x}", cpu.program_counter, jump_addr);

    assert_eq!(cpu.program_counter, jump_addr);
}

#[test]
fn jmp_indirect() {
    assert!(false);
}

#[test]
fn jsr() {
    assert!(false);
}

#[test]
fn rts() {
    assert!(false);
}