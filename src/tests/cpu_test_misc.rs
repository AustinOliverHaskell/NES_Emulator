use crate::cpu::*;

#[test]
fn adc() {
    assert!(false);
}

#[test]
fn and() {
    assert!(false);
}

#[test]
fn bit_zero_page() {
    let program: Vec<u8> = vec![0x24, 0xFF];
    let mut cpu = CPU::new(program);
    cpu.registers.a = 0x00;
    cpu.write(0x00FF, 0xFF);

    cpu.run_next_instruction();

    assert_eq!(cpu.status, OVERFLOW | NEGATIVE | ZERO);
}

#[test]
fn bit_absolute() {
    let program: Vec<u8> = vec![0x2C, 0xFF, 0x00];
    let mut cpu = CPU::new(program);
    cpu.registers.a = 0x00;
    cpu.write(0x00FF, 0xFF);

    cpu.run_next_instruction();

    assert_eq!(cpu.status, OVERFLOW | NEGATIVE | ZERO);
}

#[test]
fn cmp() {
    assert!(false);
}

#[test]
fn cpx() {
    assert!(false);
}

#[test]
fn cpy() {
    assert!(false);
}

#[test]
fn dec() {
    assert!(false);
}

#[test]
fn dex() {
    let program: Vec<u8> = vec![0xCA];
    let mut cpu = CPU::new(program);
    cpu.registers.x = 10;
    
    cpu.run_next_instruction();
    assert_eq!(cpu.registers.x, 9);
}

#[test]
fn dey() {
    let program: Vec<u8> = vec![0x88];
    let mut cpu = CPU::new(program);
    cpu.registers.y = 10;
    
    cpu.run_next_instruction();
    assert_eq!(cpu.registers.y, 9);
}

#[test]
fn eor() {
    assert!(false);
}

#[test]
fn inc_zero_page() {
    let program: Vec<u8> = vec![0xE6, 0xDD, 0xE6, 0xDD];
    let mut cpu = CPU::new(program);

    cpu.run_next_instruction();
    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0xDD], 2);
}


#[test]
fn inc_value_is_zero() {
    let program: Vec<u8> = vec![0xE6, 0xDD];
    let mut cpu = CPU::new(program);
    cpu.write(0xDD, 0xFF);

    cpu.run_next_instruction();

    assert_eq!(cpu.status, ZERO);
}

#[test]
fn inc_negative_set() {
    let program: Vec<u8> = vec![0xE6, 0xDD];
    let mut cpu = CPU::new(program);
    cpu.write(0xDD, 0xFE);

    cpu.run_next_instruction();

    assert_eq!(cpu.status, NEGATIVE);
}

#[test]
fn inc_zero_page_x() {
    let program: Vec<u8> = vec![0xF6, 0xDD, 0xF6, 0xDD];
    let mut cpu = CPU::new(program);
    cpu.registers.x = 0x11;

    cpu.run_next_instruction();
    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0xEE], 2);
}

#[test]
fn inc_absolute() {
    let program: Vec<u8> = vec![0xEE, 0x00, 0xDD, 0xEE, 0x00, 0xDD];
    let mut cpu = CPU::new(program);

    cpu.run_next_instruction();
    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0xDD00], 2);
}

#[test]
fn inc_absolute_x() {
    let program: Vec<u8> = vec![0xFE, 0x00, 0xDD, 0xFE, 0x00, 0xDD];
    let mut cpu = CPU::new(program);
    cpu.registers.x = 0x11;

    cpu.run_next_instruction();
    cpu.run_next_instruction();

    assert_eq!(cpu.memory[0xDD11], 2);
}

#[test]
fn inx() {
    let program: Vec<u8> = vec![0xE8];
    let mut cpu = CPU::new(program);
    
    cpu.run_next_instruction();
    assert_eq!(cpu.registers.x, 1);
}

#[test]
fn iny() {
    let program: Vec<u8> = vec![0xC8];
    let mut cpu = CPU::new(program);
    
    cpu.run_next_instruction();
    assert_eq!(cpu.registers.y, 1);
}

#[test]
fn jsr() {
    assert!(false);
}

#[test]
fn lda() {
    assert!(false);
}

#[test]
fn ldx() {
    assert!(false);
}

#[test]
fn ldy() {
    assert!(false);
}

#[test]
fn lsr() {
    assert!(false);
}

#[test]
fn nop() {
    let program: Vec<u8> = vec![0xEA];
    let mut cpu = CPU::new(program);
    let counter = cpu.program_counter;
    cpu.run_next_instruction();
    assert_eq!(cpu.program_counter, counter + 1);
}

#[test]
fn ora() {
    assert!(false);
}

#[test]
fn rti() {
    assert!(false);
}

#[test]
fn rts() {
    assert!(false);
}

#[test]
fn sbc() {
    assert!(false);
}


#[test]
fn sta() {
    assert!(false);
}

#[test]
fn stx() {
    assert!(false);
}

#[test]
fn sty() {
    assert!(false);
}

#[test]
fn tax() {
    let program: Vec<u8> = vec![0xAA];
    let mut cpu = CPU::new(program);
    cpu.registers.a = 0x10;

    cpu.run_next_instruction();
    assert_eq!(0x10, cpu.registers.x);
}

#[test]
fn tay() {
    let program: Vec<u8> = vec![0xA8];
    let mut cpu = CPU::new(program);
    cpu.registers.a = 0x10;

    cpu.run_next_instruction();
    assert_eq!(0x10, cpu.registers.y);
}

#[test]
fn txa() {
    let program: Vec<u8> = vec![0x8A];
    let mut cpu = CPU::new(program);
    cpu.registers.x = 0x10;

    cpu.run_next_instruction();
    assert_eq!(0x10, cpu.registers.a);

    // @todo: Check for status
}

#[test]
fn tya() {
    let program: Vec<u8> = vec![0x98];
    let mut cpu = CPU::new(program);
    cpu.registers.y = 0x10;

    cpu.run_next_instruction();
    assert_eq!(0x10, cpu.registers.a);
}