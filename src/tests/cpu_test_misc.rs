use crate::cpu::*;

#[test]
fn adc_immediate() {
    let test_val_a = 0x80;
    let test_val_b = 0x30;

    let program: Vec<u8> = vec![0x69, test_val_a];

    let mut cpu = CPU::new(program);
    cpu.registers.a = test_val_b;
    cpu.run_next_instruction();

    assert_eq!(cpu.registers.a, test_val_a.wrapping_add(test_val_b));
}

#[test]
fn adc_zero_page() {
    let program: Vec<u8> = vec![0x65, 0xFF];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn adc_zero_page_x() {
    let program: Vec<u8> = vec![0x75, 0xFF];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn adc_absolute() {
    let program: Vec<u8> = vec![0x6D, 0xFF];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn adc_absolute_x() {
    let program: Vec<u8> = vec![0x7D, 0xFF];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn adc_absolute_y() {
    let program: Vec<u8> = vec![0x79, 0xFF];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn adc_indirect_x() {
    let program: Vec<u8> = vec![0x61, 0xFF];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn adc_indirect_y() {
    let program: Vec<u8> = vec![0x71, 0xFF];
    let mut cpu = CPU::new(program);

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
fn cpx_immediate_equal() {
    let test_value = 0xFF;

    let program: Vec<u8> = vec![0xE0, test_value];
    let mut cpu = CPU::new(program);
    cpu.registers.x = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.status, ZERO | NEGATIVE);
}

#[test]
fn cpx_immediate_x_greater() {
    let test_value = 0xFF;

    let program: Vec<u8> = vec![0xE0, test_value - 1];
    let mut cpu = CPU::new(program);
    cpu.registers.x = test_value;

    cpu.run_next_instruction();

    assert_eq!(cpu.status, CARRY | NEGATIVE);
}

#[test]
fn cpx_zero_page() {
    assert!(false);
}

#[test]
fn cpx_absolute() {
    assert!(false);
}

#[test]
fn cpy_immediate_equal() {
    assert!(false);
}

#[test]
fn cpy_immediate_y_greater() {
    assert!(false);
}

#[test]
fn cpy_zero_page() {
    assert!(false);
}

#[test]
fn cpy_absolute() {
    assert!(false);
}

#[test]
fn dec_zero_page() {
    let test_addr: u16 = 0x30;
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0xC6, test_addr as u8];
    let mut cpu = CPU::new(program);
    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value - 1);
}

#[test]
fn dec_zero_page_x() {
    let mut test_addr: u16 = 0x30;
    let test_value = 0xFF;

    let program: Vec<u8> = vec![0xD6, test_addr as u8];

    let mut cpu = CPU::new(program);
    cpu.registers.x = 0x11;
    test_addr += 0x11;
    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value - 1);
}

#[test]
fn dec_absolute() {
    let test_addr: u16 = 0x3030;
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0xCE, (test_addr & 0xFF) as u8, ((test_addr >> 8) & 0xFF) as u8];
    let mut cpu = CPU::new(program);
    
    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value - 1);
}

#[test]
fn dec_absolute_x() {
    let mut test_addr: u16 = 0x3030;
    let test_value = 0xFF;
    let program: Vec<u8> = vec![0xDE, (test_addr & 0xFF) as u8, ((test_addr >> 8) & 0xFF) as u8];

    let mut cpu = CPU::new(program);
    cpu.registers.x += 0x11;
    test_addr += 0x11;
    cpu.write(test_addr, test_value);

    cpu.run_next_instruction();

    assert_eq!(cpu.load(test_addr), test_value - 1);
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
fn lda_immediate() {
    assert!(false);
}

#[test]
fn lda_zero_page() {
    assert!(false);
}

#[test]
fn lda_zero_page_x() {
    assert!(false);
}

#[test]
fn lda_absolute() {
    assert!(false);
}

#[test]
fn lda_absolute_x() {
    assert!(false);
}

#[test]
fn lda_absolute_y() {
    assert!(false);
}

#[test]
fn lda_indirect_x() {
    assert!(false);
}

#[test]
fn lda_indirect_y() {
    assert!(false);
}

#[test]
fn ldx_immediate() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);


    assert!(false);
}

#[test]
fn ldx_zero_page() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);


    assert!(false);
}

#[test]
fn ldx_zero_page_y() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);


    assert!(false);
}

#[test]
fn ldx_absolute() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);


    assert!(false);
}

#[test]
fn ldx_absolute_y() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);


    assert!(false);
}

#[test]
fn ldy_immediate() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn ldy_zero_page() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn ldy_zero_page_x() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn ldy_absolute() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

    assert!(false);
}

#[test]
fn ldy_absolute_x() {
    let program: Vec<u8> = vec![0xA2];
    let mut cpu = CPU::new(program);

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
fn sbc() {
    assert!(false);
}

#[test]
fn sta_zero_page() {
    assert!(false);
}

#[test]
fn sta_zero_page_x() {
    assert!(false);
}

#[test]
fn sta_absolute() {
    assert!(false);
}

#[test]
fn sta_absolute_x() {
    assert!(false);
}

#[test]
fn sta_absolute_y() {
    assert!(false);
}

#[test]
fn sta_indirect_x() {
    assert!(false);
}

#[test]
fn sta_indirect_y() {
    assert!(false);
}

#[test]
fn stx_zero_page() {
    assert!(false);
}

#[test]
fn stx_zero_page_y() {
    assert!(false);
}

#[test]
fn stx_absolute() {
    assert!(false);
}

#[test]
fn sty_zero_page() {
    assert!(false);
}

#[test]
fn sty_zero_page_x() {
    assert!(false);
}

#[test]
fn sty_absolute() {
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