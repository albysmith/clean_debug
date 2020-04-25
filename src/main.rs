use std::fs;
use std::fs::File;
use std::io::{Read, Write};

mod debug_parser;
use debug_parser::*;

fn main() {
    let debug_file =
        &fs::read_to_string("C:/Users/bad wife/Documents/Egosoft/X4/77065308/x4debug.log");
    if let Ok(log) = debug_file {
        let logdata = parse_debug(log);

        println!("General: {}", logdata.general.len());
        println!("ERROR: {}", logdata.error.len());
        println!("Scripts: {}", logdata.scripts.len());
        println!("ScriptsVerbose: {}", logdata.scripts_verbose.len());
        println!("EconomyVerbose: {}", logdata.economy_verbose.len());
        println!("Combat: {}", logdata.combat.len());
        println!("Savegame: {}", logdata.savegame.len());
        println!("None: {}", logdata.none.len());

        let mut print_string = String::new();
        for entry in logdata.general {
            print_string.push_str(&entry.string)
        }
        for entry in logdata.error {
            print_string.push_str(&entry.string)
        }
        for entry in logdata.scripts {
            print_string.push_str(&entry.string)
        }
        for entry in logdata.scripts_verbose {
            print_string.push_str(&entry.string)
        }
        for entry in logdata.economy_verbose {
            print_string.push_str(&entry.string)
        }
        for entry in logdata.combat {
            print_string.push_str(&entry.string)
        }
        for entry in logdata.savegame {
            print_string.push_str(&entry.string)
        }
        for entry in logdata.none {
            print_string.push_str(&entry.string)
        }
        let mut outputfile = File::create("x_output/penis.log").expect("something");
        outputfile
            .write_all(&print_string.as_bytes())
            .expect("else");
    }
    // else {
    //     // error path
    // }
}
