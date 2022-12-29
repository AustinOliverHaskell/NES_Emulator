use std::env;
use std::fs::*;
use clap::*;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProgramArguments {
    pub run_integration_tests: bool
}

impl ProgramArguments {
    pub fn new() -> Option<Self> {

        /*if env::args().len() == 1 {
            let raw_config = read_to_string("config");
            let config: Self; 
            match raw_config {
                Ok(val) => config = serde_json::from_str(&val).unwrap(),
                _ => return None
            }

            return Some(config);

        } 
        else */{
            let arguments = App::new("NES Emulator")
                    .version(env!("CARGO_PKG_VERSION"))
                    .author("Austin Haskell")
                    .about("NES (Nintendo Entertainment System) Emulator")
                    .arg(Arg::with_name("integration_tests")
                        .short("i")
                        .long("integration")
                        .takes_value(false)
                        .help("If present, will skip running the emulator and will instead run it's integration tests. "))
                    .get_matches();    
    
            return Some(ProgramArguments {
                run_integration_tests: arguments.is_present("integration_tests")
            })
        }
    }

    #[allow(unused)]
    pub fn dump(&self, path: &str) {
        let arg_dump = serde_json::to_string(&self);
        match arg_dump {
            Err(_) => {
                println!("Error: Failed to dump arguments to file. ");
                return;
            },
            _ => { }
        }

        let raw_arg_file = File::create(path);
        let mut arg_file;
        match raw_arg_file {
            Err(_) => {
                println!("Failed to create/open argument file: {:?}", path);
                return;
            },
            Ok(val) => arg_file = val
        }
        arg_file.write_all(arg_dump.unwrap().as_bytes()).unwrap();
    }
}