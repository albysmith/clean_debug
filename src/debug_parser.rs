use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::str::FromStr;

// #[derive(PartialEq)]
// pub enum ErrTag {
//     General,
//     Error,
//     Scripts,
//     ScriptsVerbose,
//     EconomyVerbose,
//     Combat,
//     Savegame,
//     None,
//     Custom,
// }

#[derive(Default, Debug, Clone)]
pub struct Entry {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}

pub fn parse_debug(debug: &String) -> (Vec<Entry>, Vec<String>) {
    let mut logdata = vec![];
    let mut tag_list = vec![];
    let log = debug.replace("\r\n", " NEWLINE ");
    let mut timeflag = false;
    let mut string = String::new();
    let mut message = String::new();
    let mut time = 0.0;
    let mut enum_flag = String::new();
    for word in log.split_whitespace() {
        if timeflag {
            // println!("{}", word);
            string.push_str(&format!(" {}", word));
            time = word.parse().expect("parse word as num");
            // println!("{}", time);
            timeflag = false;
        } else if let Some(tag) = sort_log(word) {
            if !tag_list.contains(&tag) {
                tag_list.push(tag.clone());
            }
            // do something with the old string and get it out of here
            if string.contains("======================================")
                || string.contains("(error: 14)")
                || string.contains(".sig")
                || (string.contains("*** Context:md") && !string.contains("NEWLINE"))
            {
                string = word.to_string();
                message = String::new();
                timeflag = true;
            // } else if string.contains("*** Context:md") && !string.contains("NEWLINE") {
            //     string = word.to_string();
            //     message = String::new();
            //     timeflag = true;
            } else {
                string = string.replace("NEWLINE", "\r\n");
                message = message.replace("NEWLINE", "\r\n");
                set_entry_struct(&mut logdata, string, time, message, enum_flag);
                string = word.to_string();
                message = String::new();
                timeflag = true;
            };
            enum_flag = tag;
        } else if timeflag == false {
            string.push_str(&format!(" {}", word));
            message.push_str(&format!("{} ", word))
        }

        // some handling for adding the time to the error as some TIME type
    }
    (logdata, tag_list)
}

fn sort_log(entry: &str) -> Option<String> {
    if entry.contains("[") && entry.to_string().pop() == Some(']') {
        let mut tag = entry.replace("[", "").replace("]", "").to_lowercase();
        if tag == "=error=".to_string() {
            tag = "error".to_string()
        }
        return Some(tag);
    } else {
        return None;
    }
}

fn set_entry_struct(
    logdata: &mut Vec<Entry>,
    word: String,
    time: f64,
    message: String,
    tag: String,
) {
    logdata.push(Entry {
        string: word,
        time: time,
        message: message,
        tag: tag,
    })
}

pub fn print_clean_log(logdata: &Vec<&Entry>, out_folder: &String) {
    let mut print_string = String::new();
    let mut old_tag = "".to_string();
    for entry in logdata {
        if old_tag == entry.tag {
            if entry.string.contains("\r\n ") {
                let new = entry.string.replace("\r\n ", "\r\n   ");
                print_string.push_str(&format!("  {}", &new))
            } else{
            print_string.push_str(&format!("  {}", &entry.string))}
        } else {
            old_tag = entry.tag.clone();
            print_string.push_str(&entry.string);
        }
    }
    let mut outputfile =
        File::create(format!("{}/filtered_debug.log", out_folder)).expect("something");
    outputfile
        .write_all(&print_string.as_bytes())
        .expect("else");
}
