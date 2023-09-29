
pub mod game {
    use anyhow::{Context,Result};
    use super::input;
    use serde;
    use chrono;

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Game {
        profile: Profile,
        character_names: Vec<String>,
        inputs: Vec<input::Input>
    }

    impl Game {
        pub fn new(profile: Profile, character_names: Vec<String>, inputs: Vec<input::Input>) -> Self {
            Self {
                profile,
                character_names,
                inputs
            }
        }

        pub fn profile(&self) -> &Profile {
            &self.profile
        }

        pub fn character_names(&self) -> &[String] {
            &self.character_names
        }

        pub fn inputs(&self) -> &[input::Input] {
            &self.inputs
        }

        pub fn parse_sequence(&self, symbols: &str) -> Result<input::Sequence> {
            let entries: Vec<input::Entry> = symbols.split(',')
                .map(|s| s.trim())
                .map(|s| {
                    input::Entry::Input(
                        self.find_input(s)
                            .context(format!("Unknown input: {s}"))
                            .unwrap()
                            .symbol
                            .clone())
                }).collect();

            Ok(input::Sequence::new(entries))

        }

        pub fn find_input(&self, symbol: &str) -> Option<&input::Input> {
            for input in self.inputs() {
                if symbol == input.symbol.symbol {
                    return Some(input);
                }
            }

            None
        }

    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Profile {
        name: String,
        developer: String,
        publisher: String,
        release_date: chrono::NaiveDate,
        website_url: String,
        wikipedia_page_url: String,
        platform_names: Vec<String>
    }

    impl Profile {
        pub fn new(
            name: String,
            developer: String,
            publisher: String,
            release_date: chrono::NaiveDate,
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

        pub fn name(&self) -> &str {
            &self.name
        }
        
        pub fn developer(&self) -> &str {
            &self.developer
        }
        
        pub fn publisher(&self) -> &str {
            &self.publisher
        }

        pub fn release_date(&self) -> &chrono::NaiveDate{
            &self.release_date
        }

        pub fn website_url(&self) -> &str {
            &self.website_url
        }

        pub fn wikipedia_page_url(&self) -> &str {
            &self.wikipedia_page_url
        }

        pub fn platform_names(&self) -> &Vec<String> {
            &self.platform_names
        }
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Character {
        name: String,
        moves: Vec<Move>
    }

    impl Character {
        pub fn new(name: String, moves: Vec<Move>) -> Self {
            Self { name, moves }
        }

        pub fn name(&self) -> &str {
            &self.name
        }
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct InputBinding {
        name: String,
        input: input::Input
    }

    impl InputBinding {
        pub fn new(name: String, input: input::Input) -> Self {
            Self { name, input }
        }
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Move {
        name: String,
        inputs: input::Sequence
    }

    impl Move {
        pub fn new(name: String, inputs: input::Sequence) -> Self {
            Self { name, inputs }
        }
    }
}

pub mod platform {
    use serde;
    use super::input::Input;

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Platform {
        input_devices: Vec<InputDevice>
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct InputDevice {
        inputs: Vec<Input>
    }
}

pub mod input {
    use serde;
    use super::game;

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Symbol {
        pub(crate) symbol: String
    }
    
    impl Symbol {
        pub fn new(symbol: String) -> Self {
            Self { symbol }
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Input {
        name: String,
        pub(crate) symbol: Symbol
    }

    impl Input {
        pub fn new(name: String, symbol: Symbol) -> Self {
            Self { name, symbol }
        }

    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Combination {
        inputs: Vec<Symbol>
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum Entry {
        Input(Symbol),
        Combination(Combination),
        Hold(u64),
        Wait(u64),
        Note(String),
        Neutral 
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct Sequence {
        entries: Vec<Entry>
    }

    impl Sequence {
        pub fn new(entries: Vec<Entry>) -> Self {
            Self { entries }
        }

    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
