use clap::{App, Arg};
use std::env;
use std::fs;
use std::io::{self, stdin, stdout, Read, Write};
// use std::fs::File;
// use std::io::{Read, Write};

mod debug_parser;
use debug_parser::*;

// TODO
// Create parsing for custom tags in debug text
// Make everything generic to support new tags
// (no special structs or functions, etc. try to remove named match statements)
//

fn main() {
    let matches = App::new("Log Horizon")
        .arg(
            Arg::with_name("log_file")
                .help("the log file to parse")
                .short("l")
                .long("log")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tags")
                .help("the tags you want to view")
                .short("t")
                .long("tags")
                .default_value("error"),
        )
        .get_matches();

    let log = check_log_file(matches.value_of("log_file"));
    let logdata = parse_debug(&log);
    print_clean_log(logdata.clone());

    if let Some(value) = matches.value_of("tags") {
        match_data(value.to_string(), &logdata)
    }
}

fn more_input() -> String {
    let mut stdout = stdout();
    stdout.write(b"Set New Filter: choose from list below. Type end to close program.\n  general | error | scripts | scripts_verbose | economy_verbose | combat | savegame | none\n").unwrap();
    stdout.flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap_or_default();
    buffer
}

fn match_data(values: String, logdata: &LogData) {
    // if values == "end".to_string() {
    //     close_program();
    // } else {
    for tag in values.split_whitespace() {
        match tag {
            "general" => println!("General: {}", logdata.general.len()),
            "error" => println!("ERROR: {}", logdata.error.len()),
            "scripts" => println!("Scripts: {}", logdata.scripts.len()),
            "scripts_verbose" => println!("ScriptsVerbose: {}", logdata.scripts_verbose.len()),
            "economy_verbose" => println!("EconomyVerbose: {}", logdata.economy_verbose.len()),
            "combat" => println!("Combat: {}", logdata.combat.len()),
            "savegame" => println!("Savegame: {}", logdata.savegame.len()),
            "none" => println!("None: {}", logdata.none.len()),
            "all" => println!("And you thought you would get them all muahahaha"),
            "end" => close_program(),
            _ => println!("Found a Qux"),
        };
    }

    let result = more_input();
    match_data(result, logdata)
    // }
}

fn close_program() {
    std::process::exit(0x0100);
}

fn error_then_close(message: &str) {
    let mut stdout = stdout();
    stdout.write(message.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    close_program();
}

fn check_log_file(debug_path: Option<&str>) -> String {
    if let Some(path) = debug_path {
        let result = &fs::read_to_string(path);
        if let Ok(contents) = result {
            return contents.to_owned();
        } else {
            error_then_close(
                "Debug log not found at path supplied. Press any key to close program.",
            );
            return String::new();
        }
    } else {
        let env_path = &env::current_dir().expect("current dir").join("x4debug.log");
        let result = &fs::read_to_string(env_path);
        if let Ok(contents) = result {
            return contents.to_owned();
        } else {
            error_then_close("Debug file not contained in current directory, and path not supplied. Press any key to close program.");
            return String::new();
        }
    }
}
