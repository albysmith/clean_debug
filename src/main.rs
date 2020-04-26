use clap::{App, Arg};
use std::env;
use std::fs;
use std::io::{self, stdin, stdout, Read, Write};
// use std::fs::File;
// use std::io::{Read, Write};

mod debug_parser;
use debug_parser::*;

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
            Arg::with_name("out_file")
                .help("the output file folder (folder MUST exist)")
                .short("o")
                .long("output")
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
    let output = check_out_file(matches.value_of("out_file"));
    let parsed_log = parse_debug(&log);
    let logdata = parsed_log.0;
    let tag_list = parsed_log.1;
    // print_clean_log(logdata.clone());

    if let Some(value) = matches.value_of("tags") {
        match_data(value.to_string(), &logdata, &tag_list, &output)
    }
}

fn more_input(tag_list: &Vec<String>) -> String {
    let mut stdout = stdout();
    let mut text =
        "Set New Filter: choose from list below. Type end to close program.\n ".to_string();
    for tag in tag_list {
        text.push_str(&format!(" | {}", tag));
    }
    text.push_str("\n");
    // let text: String = tag_list.iter().map(|tag| tag.push_str(" |")).collect();
    stdout.write(text.as_bytes()).unwrap();
    stdout.flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap_or_default();
    buffer
}

fn match_data(values: String, logdata: &Vec<Entry>, tag_list: &Vec<String>, output: &String) {
    if values == "end\r\n".to_string() {
        close_program();
    } else {
        let tags: Vec<&str> = values.split_whitespace().collect();
        let filter: Vec<&Entry> = logdata
            .iter()
            .filter(|log| tags.contains(&log.tag.as_str()))
            .collect();
        print_clean_log(&filter, &output);
        let result = more_input(tag_list);
        match_data(result, logdata, tag_list, output)
    }
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

fn check_out_file(debug_path: Option<&str>) -> String {
    if let Some(path) = debug_path {
        return path.to_string()
    } else {
        let env_path = env::current_dir().expect("current dir");
        return env_path.to_str().expect("path").to_string();
    }
}
