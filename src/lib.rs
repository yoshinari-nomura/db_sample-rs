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
//     fn from_str<'b>(line: &'b str) -> Action<'a> {}
// }

//////////////////
// ProfOrCommand
//////////////////
#[derive(Debug)]
pub enum ProfOrCommand {
    Profile(Profile),
    Command(Command),
}

//////////////////
// Command
//////////////////
#[derive(Debug)]
pub struct Command {}

impl Command {
    pub fn new() -> Command {
        Command {}
    }
}

//////////////////
// Profile
//////////////////
#[derive(Debug)]
pub struct Profile {
    id: String,
    name: String,
    birthday: String,
    address: String,
    description: String,
}

impl Profile {
    pub fn new(columns: Vec<&str>) -> Profile {
        Profile {
            id: columns[0].to_string(),
            name: columns[1].to_string(),
            birthday: columns[2].to_string(),
            address: columns[3].to_string(),
            description: columns[4].to_string(),
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
        println!("exec command {:?}", command);
    }

    pub fn push(&mut self, profile: Profile) -> () {
        println!("pushed: {:?}", profile);
        self.profiles.push(profile);
    }
}
