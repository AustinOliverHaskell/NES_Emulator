use regex::Regex;
use std::fs::File;
use std::fs::read_dir;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::io::Read;

use crate::cpu::CPU;

pub fn run_integration_tests(asm6502_bin_dir: &Path, dump_logs: bool, log_path: &Path) -> Result<(), String> {

    let possible_file_list = read_dir(asm6502_bin_dir);
    if possible_file_list.is_err() {
        return Err(String::from("Err: Bin directory does not exist. "));
    }
    for file_data in possible_file_list.unwrap() {
        let file_data = file_data.unwrap();
        let file_name = file_data.file_name().into_string().unwrap();
        
        let extention_pattern = ".*\\.bin";
        let extention_regex = Regex::new(extention_pattern).unwrap();
        if extention_regex.is_match(&file_name) {
            println!("Running File {:?}", file_data.path());
            let mut program: Vec<u8> = Vec::new();
            let mut file = File::open(file_data.path());
            if file.is_err() {
                continue;
            }
            file.unwrap().read_to_end(&mut program).unwrap();
            let mut cpu = CPU::new(program);

            cpu.run_program();

            if dump_logs {
                let name_without_extention = &file_name[0..file_name.len()-4];
                cpu.dump_memory_to_human_readable_file(0x0000, 0xFFFF, format!("{0}{1}.memdump", log_path.to_str().unwrap(), name_without_extention));
            }
        }
    }

    /*let mut file = File::open("integration_tests/write_first_0xff.bin").expect("no file found");
    let mut file_contents: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut file_contents);

    let mut cpu = CPU::new(file_contents);
    cpu.run_program();

    cpu.dump_memory(0x0000, 0x1000);
    cpu.dump_cpu();
*/
    Ok(())
}