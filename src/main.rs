use std::io::Read;
use std::fs::File;

mod file_format;
mod cpu;
mod addressing_modes;
mod util;
mod integration_tests;
mod graphics {
    mod windows_display;
}
mod ppu;
mod arguments;

use arguments::ProgramArguments;
use file_format::*;

#[cfg(test)]
pub mod tests {
    pub mod test_util;
    pub mod cpu_test_branching;
    pub mod cpu_test_status_flags;
    pub mod cpu_test_stack;
    pub mod cpu_test_bit_bashing;
    pub mod cpu_test_misc;
}

use std::path::Path;
use integration_tests::*;
use ppu::PPU;

fn main() {

    let args = ProgramArguments::new();
    if args.is_none() {
        println!("Failed to get program arguments. See help screen for more info. ");
        return;
    }

    let mut f = File::open("/Users/austinhaskell/Documents/roms/cpu_dummy_reads.nes").unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    INES::from_bytes(buffer);

    let mut ppu = PPU::new();

    let args = args.unwrap();
    if args.run_integration_tests {
        let integration_bin_path: &Path     = Path::new("./integration_tests/");
        let output_dump_path: &Path = Path::new("./integration_tests_results/");

        let err = run_integration_tests(integration_bin_path, true, output_dump_path);
        if err.is_err() {
            println!("{:?}", err.unwrap_err());
            println!("Failed to run integration tests. ");
        }
    }

    loop {
        ppu.update();
    }
}