use std::cmp::{max, min, Ordering};
use std::fmt::{self, Display};
use std::fs;
use std::io::Write;
use std::num::ParseIntError;
use std::str::FromStr;

////////////////////////////////////////////////////////////////
// Action
///
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
    Error(&'a str),
}

impl<'a> Action<'a> {
    pub fn append(profile_db: &mut ProfileDB, profile: Profile) {
        eprintln!("Command Append: {:?}", profile);
        profile_db.push(profile)
    }

    pub fn quit() {
        eprintln!("Command: Quit");
        std::process::exit(0)
    }

    pub fn count(profile_db: &ProfileDB) {
        eprintln!("Command: Count");
        println!("{} profile(s)", profile_db.len());
    }

    pub fn print(profile_db: &ProfileDB, nitems: i32) {
        let mut s = 0;
        let mut e = profile_db.len();

        // head |nitems| if nitems > 0
        if nitems > 0 {
            e = min(nitems as usize, e);
        };

        // tail |nitems| if nitems < 0
        if nitems < 0 {
            let n = -nitems as usize;
            s = max(e, n) - n
        };
        eprintln!("Command: Print {} ({}..{})", nitems, s, e);
        profile_db.print(s..e);
    }

    pub fn write(profile_db: &mut ProfileDB, filename: &str) -> Result<(), String> {
        eprintln!("Command: Write {}", filename);
        profile_db.save(filename)
    }

    pub fn find(profile_db: &ProfileDB, word: &str) {
        eprintln!("Command: Find {}", word);
        profile_db.find(word)
    }

    pub fn sort(profile_db: &mut ProfileDB, key: u8) {
        eprintln!("Command: Sort key:{}", key);
        profile_db.sort(key)
    }
}

////////////////////////////////////////////////////////////////
/// Date - my date
///
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

/// Display trait implements to_string
impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.y, self.m, self.d)
    }
}

/// implement "2018-06-30".parse::<Date>
impl FromStr for Date {
    type Err = ParseIntError;

    fn from_str(date_str: &str) -> Result<Self, Self::Err> {
        let ymd: Vec<&str> = date_str.split("-").collect();
        let y = ymd[0].parse::<u32>()?;
        let m = ymd[1].parse::<u8>()?;
        let d = ymd[2].parse::<u8>()?;
        Ok(Date { y, m, d })
    }

    // XXX: should be called from `from_str`: fn is_valid_date(y: u32, m: u8, d: u8) {}
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

////////////////////////////////////////////////////////////////
/// Profile
///
#[derive(Debug)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub birthday: Date,
    pub home: String,
    pub comment: String,
}

/// Display trait implements to_string
impl Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\nbirthday: {}\nhome: {}\ncomment: {}",
            self.id, self.name, self.birthday, self.home, self.comment
        )
    }
}

impl Profile {
    pub fn new(id: u32, name: String, birthday: Date, home: String, comment: String) -> Self {
        Profile {
            id,
            name,
            birthday,
            home,
            comment,
        }
    }

    pub fn from_vector(columns: Vec<&str>) -> Profile {
        Profile {
            id: columns[0].parse::<u32>().unwrap(),
            name: columns[1].to_string(),
            birthday: columns[2].parse::<Date>().unwrap(),
            home: columns[3].to_string(),
            comment: columns[4].to_string(),
        }
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{}\n",
            self.id, self.name, self.birthday, self.home, self.comment
        )
    }

    pub fn find(&self, word: &str) -> bool {
        if self.id.to_string() == word
            || self.name == word
            || self.birthday.to_string() == word
            || self.home == word
            || self.comment == word
        {
            true
        } else {
            false
        }
    }

    pub fn compare_by(&self, other: &Profile, key: u8) -> Ordering {
        match key {
            1 => self.id.cmp(&other.id),
            2 => self.name.cmp(&other.name),
            3 => self.birthday.cmp(&other.birthday),
            4 => self.home.cmp(&other.home),
            5 => self.comment.cmp(&other.comment),
            _ => panic!("Unknown sort key {}", key),
        }
    }
}

////////////////////////////////////////////////////////////////
/// Collection of Profile
///
#[derive(Debug)]
pub struct ProfileDB {
    profiles: Vec<Profile>,
}

impl ProfileDB {
    pub fn new() -> Self {
        ProfileDB {
            profiles: Vec::new(),
        }
    }

    pub fn find(&self, word: &str) {
        for p in &self.profiles {
            if p.find(word) {
                println!("{}", p)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    pub fn print(&self, range: std::ops::Range<usize>) {
        for p in &self.profiles[range] {
            println!("{}", p)
        }
    }

    pub fn push(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    pub fn save(&self, filename: &str) -> Result<(), String> {
        let mut f = fs::File::create(filename).map_err(|e| e.to_string())?;
        for p in &self.profiles {
            f.write(&*p.to_csv().as_bytes()).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn sort(&mut self, key: u8) {
        self.profiles.sort_by(|a, b| a.compare_by(b, key));
    }
}
