mod file_format;
mod cpu;
mod addressing_modes;
mod util;
mod integration_tests;
mod graphics {
    mod windows_display;
}

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

fn main() {
    let integration_bin_path: &Path     = Path::new("./integration_tests/");
    let output_dump_path: &Path = Path::new("./integration_tests_results/");

    let err = run_integration_tests(integration_bin_path, true, output_dump_path);
    if err.is_err() {
        println!("{:?}", err.unwrap_err());
        println!("Failed to run integration tests. ");
    }
}