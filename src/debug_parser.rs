use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(PartialEq)]
enum ErrTag {
    General,
    Error,
    Scripts,
    ScriptsVerbose,
    EconomyVerbose,
    Combat,
    Savegame,
    None,
}

#[derive(Default, Debug, Clone)]
pub struct LogData {
    pub general: Vec<General>,
    pub error: Vec<Error>,
    pub scripts: Vec<Scripts>,
    pub scripts_verbose: Vec<ScriptsVerbose>,
    pub economy_verbose: Vec<EconomyVerbose>,
    pub combat: Vec<Combat>,
    pub savegame: Vec<Savegame>,
    pub none: Vec<None>,
}

#[derive(Default, Debug, Clone)]
pub struct General {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}
#[derive(Default, Debug, Clone)]
pub struct Error {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}
#[derive(Default, Debug, Clone)]
pub struct Scripts {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}
#[derive(Default, Debug, Clone)]
pub struct ScriptsVerbose {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}
#[derive(Default, Debug, Clone)]
pub struct EconomyVerbose {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}
#[derive(Default, Debug, Clone)]
pub struct Combat {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}
#[derive(Default, Debug, Clone)]
pub struct Savegame {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}
#[derive(Default, Debug, Clone)]
pub struct None {
    pub string: String,
    pub time: f64,
    pub tag: String,
    pub message: String,
}

pub fn parse_debug(debug: &String) -> LogData {
    // let mut parsed_log_entries = LogEntries::default();
    let mut logdata: LogData = LogData::default();
    let mut print_string = String::new();

    let log = debug.replace("\r\n", " NEWLINE ");
    let mut timeflag = false;
    let mut string = String::new();
    let mut message = String::new();
    let mut time = 0.0;
    let mut enum_flag = ErrTag::General;
    for word in log.split_whitespace() {
        if timeflag {
            // println!("{}", word);
            string.push_str(&format!(" {}", word));
            time = word.parse().expect("parse word as num");
            // println!("{}", time);
            timeflag = false;
        } else if let Some(tag) = sort_log(word) {
            // do something with the old string and get it out of here
            if (enum_flag == ErrTag::General
                && (string.contains("======================================")
                    || string.contains("(error: 14)")
                    || string.contains(".sig")))
                || (enum_flag == ErrTag::Error && string.contains(".sig"))
            {
            } else {
                string = string.replace("NEWLINE", "\r\n");
                match enum_flag {
                    ErrTag::General => set_general_struct(&mut logdata, string, time, message),
                    ErrTag::Error => set_error_struct(&mut logdata, string, time, message),
                    ErrTag::Scripts => set_scripts_struct(&mut logdata, string, time, message),
                    ErrTag::ScriptsVerbose => {
                        set_scripts_verbose_struct(&mut logdata, string, time, message)
                    }
                    ErrTag::EconomyVerbose => {
                        set_economy_verbose_struct(&mut logdata, string, time, message)
                    }
                    ErrTag::Combat => set_combat_struct(&mut logdata, string, time, message),
                    ErrTag::Savegame => set_savegame_struct(&mut logdata, string, time, message),
                    ErrTag::None => set_none_struct(&mut logdata, string, time, message),
                }
            };

            // new string and new enumflag
            enum_flag = tag;
            string = word.to_string();
            message = String::new();
            timeflag = true;
        } else if timeflag == false {
            //  oldstring
            string.push_str(&format!(" {}", word));
            message.push_str(&format!("{} ", word))
        }

        // some handling for adding the time to the error as some TIME type
    }

    // for entry in logdata.general {
    //     // list.push(entry.string);
    //     print_string.push_str(&entry.string)
    // }
    // for entry in logdata.error {
    //     print_string.push_str(&entry.string)
    // }
    // for entry in logdata.scripts {
    //     print_string.push_str(&entry.string)
    // }
    // for entry in logdata.scripts_verbose {
    //     print_string.push_str(&entry.string)
    // }
    // for entry in logdata.economy_verbose {
    //     print_string.push_str(&entry.string)
    // }
    // for entry in logdata.combat {
    //     print_string.push_str(&entry.string)
    // }
    // for entry in logdata.savegame {
    //     print_string.push_str(&entry.string)
    // }
    // for entry in logdata.none {
    //     print_string.push_str(&entry.string)
    // }
    // logdata.general.into_iter().map(|x| print_string.push_str(&x.string));
    // logdata.error.into_iter().map(|x| print_string.push_str(&x.string));
    // logdata.scripts.into_iter().map(|x| print_string.push_str(&x.string));
    // logdata.scripts_verbose.into_iter().map(|x| print_string.push_str(&x.string));
    // logdata.economy_verbose.into_iter().map(|x| print_string.push_str(&x.string));
    // logdata.combat.into_iter().map(|x| print_string.push_str(&x.string));
    // logdata.savegame.into_iter().map(|x| print_string.push_str(&x.string));
    // logdata.none.into_iter().map(|x| print_string.push_str(&x.string));

    // let mut outputfile = File::create("x_output/penis.log").expect("something");
    // outputfile.write_all(&print_string.as_bytes()).expect("else");
    // orbtk::initialize();
    // println!("{:#?}", logdata.general.len())
    logdata
}

fn sort_log(entry: &str) -> Option<ErrTag> {
    match entry {
        "[General]" => Some(ErrTag::General),
        "[=ERROR=]" => Some(ErrTag::Error),
        "[Scripts]" => Some(ErrTag::Scripts),
        "[Scripts_Verbose]" => Some(ErrTag::ScriptsVerbose),
        "[Economy_Verbose]" => Some(ErrTag::EconomyVerbose),
        "[Combat]" => Some(ErrTag::Combat),
        "[Savegame]" => Some(ErrTag::Savegame),
        "[None]" => Some(ErrTag::None),
        _ => None,
    }
}

fn set_general_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    logdata.general.push(General {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "General".to_string(),
    })
}
fn set_error_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    logdata.error.push(Error {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "Error".to_string(),
    })
}
fn set_scripts_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    
    logdata.scripts.push(Scripts {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "Scripts".to_string(),
    })
}
fn set_scripts_verbose_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    logdata.scripts_verbose.push(ScriptsVerbose {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "Scripts_Verbose".to_string(),
    })
}
fn set_economy_verbose_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    logdata.economy_verbose.push(EconomyVerbose {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "Economy_Verbose".to_string(),

    })
}
fn set_combat_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    logdata.combat.push(Combat {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "Combat".to_string(),
    })
}
fn set_savegame_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    logdata.savegame.push(Savegame {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "Savegame".to_string(),
    })
}
fn set_none_struct(logdata: &mut LogData, word: String, time: f64, message: String) {
    logdata.none.push(None {
        string: word.to_string(),
        time: time,
        message: message,
        tag: "None".to_string(),
    })
}

// fn main() {
//     let debug_file =
//         &fs::read_to_string("C:/Users/bad wife/Documents/Egosoft/X4/77065308/x4debug.log");
//     if let Ok(debug) = debug_file {
//         let mut clean_debug = "".to_string();
//         for line in debug.lines() {
//             if !line.contains(".sig")
//                 && !line.contains("(error: 14)")
//                 && !line.contains("======================================")
//             {
//                 clean_debug.push_str(line);
//                 clean_debug.push_str("\n");
//             }
//         }
//         let mut outputfile = File::create("E:/Rust/Projects/x4_debug_parser/x_output/clean_debug.log").expect("something");
//         outputfile.write_all(&clean_debug.as_bytes()).expect("else");
//     }
// }
