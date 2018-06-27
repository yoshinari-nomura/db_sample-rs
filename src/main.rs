extern crate db_sample;
use db_sample::*;

use std::io::{self, BufRead};

// #[derive(Debug)]
// enum Action<'a> {
//     Append(Profile),
//     Quit,
//     Check,
//     Print(i32),
//     Read(&'a str),
//     Write(&'a str),
//     Find(&'a str),
//     Sort(u8),
//     Error(&'a str),
// }
//
// impl<'a> Action<'a> {
//     fn from_str<'b>(line: &'b str) -> Action<'a> {
//     }
// }

fn main() {
    let file = io::stdin();
    let mut profile_data_store = ProfileDB::new();

    for line in file.lock().lines() {
        match parse_line(&line.unwrap()) {
            Ok(ProfOrCommand::Profile(profile)) => profile_data_store.push(profile),
            Ok(ProfOrCommand::Command(command)) => profile_data_store.exec(command),
            Err(e) => println!("{}", e),
        }
    }
}

fn parse_line(line: &str) -> Result<ProfOrCommand, &'static str> {
    if line.starts_with("%") {
        Ok(ProfOrCommand::Command(Command::new()))
    } else {
        if let Ok(profile) = parse_csv(line) {
            Ok(ProfOrCommand::Profile(profile))
        } else {
            Err("CSV parse error")
        }
    }
}

fn parse_csv(line: &str) -> Result<Profile, &'static str> {
    let columns: Vec<&str> = line.split(',').collect();

    // id, name, birthday, address, description
    if columns.len() != 5 {
        return Err("CSV format error: length != 5");
    } else {
        Ok(Profile::new(columns))
    }
}
