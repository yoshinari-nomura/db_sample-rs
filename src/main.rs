extern crate db_sample;
use db_sample::*;

use std::io::{self, BufRead};

// | コマンド | 意味                            | 備考                       |
// |----------+---------------------------------+----------------------------|
// | %Q       | 終了(Quit)                      |                            |
// | %C       | 登録件数などの表示(Check)       |                            |
// | %P n     | 先頭からn件表示 (Print)         | n=0: 全件, n<0: 後ろ -n 件 |
// | %R file  | fileから読み込み(Read)          |                            |
// | %W file  | fileへ書き出し(Write)           |                            |
// | %F word  | wordを検索(Find)                | 結果を%Pと同じ形式で表示   |
// | %S n     | データをn番目の項目で整列(Sort) | 表示はしない               |

fn main() {
    let file = io::stdin();
    let mut profile_data_store = ProfileDB::new();

    for line in file.lock().lines() {
        match parse_line(&line.unwrap()) {
            Action::Append(profile) => profile_data_store.push(profile),
            Action::Quit => std::process::exit(0),
            Action::Count => println!("Command: Count"),
            Action::Print(start) => println!("Command: Print {}", start),
            Action::Read(file) => println!("Command: Read {}", file),
            Action::Write(file) => println!("Command: Write {}", file),
            Action::Find(word) => println!("Command: Find {}", word),
            Action::Sort(key) => println!("Command: Sort {}", key),
            Action::Unknown(error) => println!("Error: {}", error),
            // profile_data_store.exec(command),
        }
    }
}

fn parse_line(line: &str) -> Action {
    if line.starts_with("%") {
        parse_cmd(&line[1..])
    } else {
        if let Ok(profile) = parse_csv(line) {
            Action::Append(profile)
        } else {
            Action::Unknown("CSV parse error")
        }
    }
}

fn parse_cmd(line: &str) -> Action {
    let error = Action::Unknown("Unknown command");

    if line.len() < 1 {
        return error;
    }

    let command = &line[0..1];
    let param = &line[1..].trim();

    match command {
        "Q" => Action::Quit,
        "C" => Action::Count,
        "P" => Action::Print(param.parse::<i32>().unwrap()),
        "R" => Action::Read(param),
        "W" => Action::Write(param),
        "F" => Action::Find(param),
        "S" => Action::Sort(param.parse::<u8>().unwrap()),
        _ => error,
    }
}

fn parse_csv(line: &str) -> Result<Profile, &'static str> {
    let columns: Vec<&str> = line.split(',').collect();

    // id, name, birthday, address, description
    if columns.len() == 5 {
        Ok(Profile::new(columns))
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
