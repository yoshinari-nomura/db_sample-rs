extern crate db_sample;
use db_sample::*;
use std::io::{self, BufRead, BufReader};
use std::fs;

fn main() {
    let file = io::stdin();
    let mut profile_data_store = ProfileDB::new();

    parse_lines(Box::new(file), &mut profile_data_store);
}

/// Parse lines from `file` and act on `profile_db`.
///
/// FILE would be Stdin or File.
///
/// # Exaample
/// ```
/// file = Box::new(fs::File::open("filename.csv").unwrap());
/// file = Box::new(io::stdin());
/// ```
fn parse_lines(file: Box<io::Read>, profile_db: &mut ProfileDB) {
    let buf = BufReader::new(file);

    for line in buf.lines() {
        do_action(parse_line(&line.unwrap()), profile_db);
    }
}

fn do_action(action: Action, profile_db: &mut ProfileDB) {
    match action {
        Action::Append(profile) => Action::append(profile_db, profile),
        Action::Quit => Action::quit(),
        Action::Count => Action::count(profile_db),
        Action::Print(start) => Action::print(profile_db, start),
        Action::Read(file) => parse_lines(Box::new(fs::File::open(file).unwrap()), profile_db),
        Action::Write(file) => Action::write(profile_db, file).unwrap(),
        Action::Find(word) => Action::find(profile_db, word),
        Action::Sort(key) => Action::sort(profile_db, key),
        Action::Error(message) => println!("Error: {}", message),
    }
}

fn parse_line(line: &str) -> Action {
    if line.starts_with("%") {
        if let Ok(cmd) = parse_cmd(&line[1..]) {
            cmd
        } else {
            Action::Error("illegal command format")
        }

    } else {
        if let Ok(profile) = parse_csv(line) {
            Action::Append(profile)
        } else {
            Action::Error("illegal CSV format")
        }
    }
}

fn parse_cmd(line: &str) -> Result<Action, &'static str> {
    if line.len() < 1 {
        return Ok(Action::Error("unknown command"))
    }

    let command = &line[0..1];
    let param = &line[1..].trim();

    let action = match command {
        "Q" => Action::Quit,
        "C" => Action::Count,
        "P" => Action::Print(param.parse::<i32>().map_err(|_e| "illegal command format")?),
        "R" => Action::Read(param),
        "W" => Action::Write(param),
        "F" => Action::Find(param),
        "S" => Action::Sort(param.parse::<u8>().map_err(|_e| "illegal command format")?),
        _ => Action::Error(command),
    };
    Ok(action)
}

fn parse_csv(line: &str) -> Result<Profile, &'static str> {
    let columns: Vec<&str> = line.split(',').collect();

    // id, name, birthday, address, description
    if columns.len() == 5 {
        Ok(Profile::from_vector(columns))
    } else {
        Err("CSV format error: length != 5")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_CSV_STRING: &str = "1,nom,2018-6-28,okayama,hello!";

    #[test]
    fn parse_valid_csv() {
        let p = parse_csv(TEST_CSV_STRING).unwrap();
        assert_eq!(p.id, 1);
        assert_eq!(p.name, "nom");
        assert_eq!(p.birthday, Date::new(2018, 6, 28));
        assert_eq!(p.home, "okayama");
        assert_eq!(p.comment, "hello!");
    }
}
