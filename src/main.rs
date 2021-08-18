mod file_format;
mod instructions;
mod cpu;
mod addressing_modes;

#[cfg(test)]
pub mod tests {
    pub mod cpu_test_branching;
    pub mod cpu_test_status_flags;
    pub mod cpu_test_stack;
    pub mod cpu_test_bit_shifting;
    pub mod cpu_test_misc;
}

use std::fs::File;
use std::io::Read;

use cpu::CPU;



fn main() {

    let mut file = File::open("C:\\Users\\austi\\Desktop\\Rust Projects\\NES_Emulator\\roms\\super_mario_bros.nes").expect("no file found");
    let mut file_contents: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut file_contents);

    print_bytes_in_hex(&file_contents, 100, 4);

    let mut cpu = CPU::new(file_contents);
    cpu.run_program();
}

fn print_bytes_in_hex(bytes: &Vec<u8>, num_bytes: u32, grouping: u8) {
    let mut count: u32 = 0;
    for val in bytes {
        count += 1;

        print!("{:02X?} ", val);

        if count % grouping as u32 == 0 {
            println!();
        }

        if count >= num_bytes {
            break;
        }
    }
}