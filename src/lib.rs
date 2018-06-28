use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

//////////////////
// Action
//////////////////
#[derive(Debug)]
pub enum Action<'a> {
    Append(Profile),
    Quit,
    Count,
    Print(i32),
    Read(&'a str),
    Write(&'a str),
    Find(&'a str),
    Sort(u8),
    Unknown(&'a str),
}

//////////////////
// Date
//////////////////

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Date {
    y: u32,
    m: u8,
    d: u8,
}

impl Date {
    pub fn new(y: u32, m: u8, d: u8) -> Date {
        Date { y, m, d }
    }
}

// Display trait implements to_string
impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.y, self.m, self.d)
    }
}

// implement "2018-06-30".parse::<Date>
impl FromStr for Date {
    type Err = ParseIntError;

    fn from_str(date_str: &str) -> Result<Self, Self::Err> {
        let ymd: Vec<&str> = date_str.split("-").collect();
        let y = ymd[0].parse::<u32>()?;
        let m = ymd[1].parse::<u8>()?;
        let d = ymd[2].parse::<u8>()?;
        Ok(Date { y, m, d })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_DATE: Date = Date {
        y: 2018,
        m: 8,
        d: 31,
    };

    #[test]
    fn create_date_from_string() {
        assert_eq!(Date::from_str("2018-8-31").unwrap(), TEST_DATE);
    }

    #[test]
    fn format_date_string() {
        assert_eq!("2018-8-31", format!("{}", TEST_DATE));
    }

    #[test]
    fn date_is_string() {
        assert_eq!("2018-8-31", format!("{}", TEST_DATE));
    }
}

//////////////////
// Command
//////////////////
#[derive(Debug)]
pub struct Command {
    name: String,
}

impl Command {
    pub fn new(name: &str) -> Command {
        Command {
            name: name.to_string(),
        }
    }
}

//////////////////
// Profile
//////////////////
#[derive(Debug)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub birthday: Date,
    pub home: String,
    pub comment: String,
}

impl Profile {
    pub fn new(columns: Vec<&str>) -> Profile {
        Profile {
            id: columns[0].parse::<u32>().unwrap(),
            name: columns[1].to_string(),
            birthday: columns[2].parse::<Date>().unwrap(),
            home: columns[3].to_string(),
            comment: columns[4].to_string(),
        }
    }
}

//////////////////
// ProfileDB
//////////////////
#[derive(Debug)]
pub struct ProfileDB {
    profiles: Vec<Profile>,
}

impl ProfileDB {
    pub fn new() -> ProfileDB {
        ProfileDB {
            profiles: Vec::new(),
        }
    }

    pub fn exec(&self, command: Command) -> () {
        println!("exec command {}", command.name);
    }

    pub fn push(&mut self, profile: Profile) -> () {
        println!("pushed: {:?}", profile);
        self.profiles.push(profile);
    }
}
