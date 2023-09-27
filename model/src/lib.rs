
pub mod game {
    use super::input;
    use serde;

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Game {
        profile: Profile,
        //characters: Vec<Character>
    }

    impl Game {
        pub fn new(profile: Profile) -> Self {
            Self {
                profile
            }
        }
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Profile {
        name: String,
        developer: String,
        publisher: String,
        release_date: String,
        website_url: String,
        wikipedia_page_url: String,
        platform_names: Vec<String>
    }

    impl Profile {
        pub fn new(
            name: String,
            developer: String,
            publisher: String,
            release_date: String,
            website_url: String,
            wikipedia_page_url: String,
            platform_names: Vec<String>) -> Self
        {
            Self {
                name,
                developer,
                publisher,
                release_date,
                website_url,
                wikipedia_page_url,
                platform_names
            }
        }
    }

    #[derive(Debug)]
    pub struct Character {
        name: String
    }

    impl Character {
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    #[derive(Debug)]
    pub struct InputBinding {
        name: String,
        input: input::Input
    }

    impl InputBinding {
        pub fn new(name: String, input: input::Input) -> Self {
            Self { name, input }
        }
    }

    #[derive(Debug)]
    pub struct Move {
        name: String,
        command: input::Command
    }

    impl Move {
        pub fn new(name: String, command: input::Command) -> Self {
            Self { name, command }
        }
    }
}

pub mod platform {
    use super::input::Input;

    #[derive(Debug)]
    pub struct Platform {
        input_devices: Vec<InputDevice>
    }

    #[derive(Debug)]
    pub struct InputDevice {
        inputs: Vec<Input>
    }
}

pub mod input {
    #[derive(Debug)]
    pub struct Input {
        name: String,
        symbol: String 
    }

    #[derive(Debug)]
    pub struct Combination {
        inputs: Vec<Input>
    }

    #[derive(Debug)]
    pub enum Entry {
        Input(Input),
        Combination(Combination),
        Hold(u64),
        Wait(u64),
        Note(String),
        Neutral 
    }

    #[derive(Debug)]
    pub struct Command {
        name: String,
        entries: Vec<Entry>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
