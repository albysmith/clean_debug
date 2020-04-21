use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

fn main() {
    let debug_file =
        &fs::read_to_string("C:/Users/bad wife/Documents/Egosoft/X4/77065308/x4debug.log");
    if let Ok(debug) = debug_file {
        let mut clean_debug = "".to_string();
        for line in debug.lines() {
            if !line.contains(".sig")
                && !line.contains("(error: 14)")
                && !line.contains("======================================")
            {
                clean_debug.push_str(line);
                clean_debug.push_str("\n");
            }
        }
        let mut outputfile = File::create("E:/Rust/Projects/x4_debug_parser/x_output/clean_debug.log").expect("something");
        outputfile.write_all(&clean_debug.as_bytes()).expect("else");
    }
}
