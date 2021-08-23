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

    assert_eq!(cpu.load(0xFF), test_value << 1);
}

#[test]
fn rol_zero_page_x() {
    let test_value = 0xFF;
    let expected_write_addr: u16 = (0xFF as u8).wrapping_add(0x11) as u16;

    let program: Vec<u8> = vec![0x36, 0xFF];
    let mut cpu = CPU::new(program);

    cpu.registers.x = 0x11;
    cpu.write(expected_write_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(expected_write_addr), test_value << 1);
}

#[test]
fn rol_absolute() {
    let test_value = 0xFF;
    let test_addr = 0x3030;
    let program: Vec<u8> = vec![0x2E, 0x30, 0x30];
    let mut cpu = CPU::new(program);

    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value << 1);
}

#[test]
fn rol_absolute_x() {
    let test_value = 0xFF;
    let test_addr = 0x3030;
    let program: Vec<u8> = vec![0x3E, 0x00, 0x30];
    let mut cpu = CPU::new(program);
    cpu.registers.x = 0x30;

    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value << 1);
}

#[test]
fn ror_accumulator() {
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0x6A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.registers.a, test_value >> 1);
}


#[test]
fn ror_carry_set_after_rotation() {
    let test_value = 0x01;
    let program: Vec<u8> = vec![0x6A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;
    
    cpu.run_next_instruction();

    assert_eq!(cpu.status, CARRY);
}

#[test]
fn ror_carry_set_before_rotation() {
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0x6A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;
    cpu.status = cpu.status | CARRY;

    cpu.run_next_instruction();

    assert_eq!(cpu.registers.a, test_value);
}

#[test]
fn ror_zero_value() {
    let test_value = 0x00;
    let program: Vec<u8> = vec![0x6A];
    let mut cpu = CPU::new(program);
    cpu.registers.a = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.status, ZERO);
}

#[test]
fn ror_zero_page() {
    let test_value = 0xFF;
    let test_addr: u16 = 0x0030;
    let program: Vec<u8> = vec![0x66, test_addr as u8];
    let mut cpu = CPU::new(program);
    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value >> 1);
}

#[test]
fn ror_zero_page_x() {
    let test_value = 0xFF;
    let test_addr: u16 = 0x0030;
    let program: Vec<u8> = vec![0x76, (test_addr as u8).wrapping_sub(0x11)];
    let mut cpu = CPU::new(program);
    cpu.write(test_addr, test_value);
    cpu.registers.x = 0x11;

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value >> 1);
}

#[test]
fn ror_absolute() {
    let test_value = 0xFF;
    let test_addr: u16 = 0x0030;
    let program: Vec<u8> = vec![0x6E, (test_addr & 0xFF) as u8, ((test_addr >> 8) & 0xFF) as u8];
    let mut cpu = CPU::new(program);
    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value >> 1);
}

#[test]
fn ror_absolute_x() {
    let test_value = 0xFF;
    let mut test_addr: u16 = 0x0030;
    let program: Vec<u8> = vec![0x7E, (test_addr & 0xFF) as u8, ((test_addr >> 8) & 0xFF) as u8];
    let mut cpu = CPU::new(program);
    test_addr += 0x11;
    cpu.registers.x = 0x11;
    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value >> 1);
}

#[test]
fn lsr_accumulator() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn lsr_zero_page() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn lsr_zero_page_x() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn lsr_absolute() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn lsr_absolute_x() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}