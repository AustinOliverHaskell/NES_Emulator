use crate::cpu::*;

#[test]
fn asl_accumulator() {
    let test_value: u8 = 0xE0;
    let program: Vec<u8> = vec![0x0A];  
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.registers.a, test_value << 1);
    assert_eq!(cpu.status, CARRY | NEGATIVE);
}

#[test]
fn asl_accumulator_w_zero_sets_zero_flag() {
    let test_value: u8 = 0x00;
    let program: Vec<u8> = vec![0x0A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.status, ZERO);
}

#[test]
fn asl_zero_page() {
    let test_value: u8 = 0xE0;
    let program: Vec<u8> = vec![0x06, 0xFF];
    let mut cpu = CPU::new(program);
    cpu.write(0x00FF, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0xFF], test_value << 1);
    assert_eq!(cpu.status, CARRY | NEGATIVE);
}

#[test]
fn asl_zero_page_x() {
    let test_value: u8 = 0xE0;
    let program: Vec<u8> = vec![0x16, 0x0F];
    let mut cpu = CPU::new(program);
    cpu.write(0x00FF, test_value);
    cpu.registers.x = 0xF0;

    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0xFF], test_value << 1);
    assert_eq!(cpu.status, CARRY | NEGATIVE);
}

#[test]
fn asl_absolute() {
    let test_value: u8 = 0xE0;
    let program: Vec<u8> = vec![0x0E, 0x00, 0x30];
    let mut cpu = CPU::new(program);
    cpu.write(0x3000, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0x3000], test_value << 1);
    assert_eq!(cpu.status, CARRY | NEGATIVE);
}

#[test]
fn asl_absolute_x() {
    let test_value: u8 = 0xE0;
    let program: Vec<u8> = vec![0x1E, 0x00, 0x30];
    let mut cpu = CPU::new(program);
    cpu.write(0x3030, test_value);
    cpu.registers.x = 0x30;

    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0x3030], test_value << 1);
    assert_eq!(cpu.status, CARRY | NEGATIVE);
}

#[test]
fn rol_accumulator() {
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0x2A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.registers.a, test_value << 1);
}

#[test]
fn rol_carry_set_after_rotation() {
    let test_value = 0x80;
    let program: Vec<u8> = vec![0x2A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;
    
    cpu.run_next_instruction();

    assert_eq!(cpu.status, CARRY);
}

#[test]
fn rol_carry_set_before_rotation() {
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0x2A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;
    cpu.status = cpu.status | CARRY;

    cpu.run_next_instruction();

    assert_eq!(cpu.registers.a, test_value);
}

#[test]
fn rol_zero_value() {
    let test_value = 0x00;
    let program: Vec<u8> = vec![0x2A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.status, ZERO);
}

#[test]
fn rol_zero_page() {
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0x26, 0xFF];
    let mut cpu = CPU::new(program);
    cpu.write(0xFF, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0xFF], test_value << 1);
}

#[test]
fn rol_zero_page_x() {
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0x36, 0xFF];
    let mut cpu = CPU::new(program);
    cpu.registers.x = 0x11;
    cpu.write(0x11, test_value);

    cpu.run_next_instruction();

    cpu.dump_memory(0x00, 0xFF);

    assert_eq!(cpu.memory[0x11], test_value << 1);
}

#[test]
fn rol_absolute() {
    assert!(false);

}

#[test]
fn rol_absolute_x() {
    assert!(false);
}

#[test]
fn ror_accumulator() {
    assert!(false);
}

#[test]
fn ror_zero_page() {
    assert!(false);
}

#[test]
fn ror_zero_page_x() {
    assert!(false);

}

#[test]
fn ror_absolute() {
    assert!(false);

}

#[test]
fn ror_absolute_x() {
    assert!(false);
}