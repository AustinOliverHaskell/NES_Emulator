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
fn asl() {
    assert!(false);
}

#[test]
fn bit() {
    assert!(false);
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
    assert!(false);
}

#[test]
fn inc_zero_page_x() {
    assert!(false);
}

#[test]
fn inc_absolute() {
    assert!(false);
}

#[test]
fn inc_absolute_x() {
    assert!(false);
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
fn rol() {
    assert!(false);
}

#[test]
fn ror() {
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