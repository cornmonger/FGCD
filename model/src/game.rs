use anyhow::{Context,Result};
use crate::input::Entry;

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

    pub fn find_input(&self, symbol: &str) -> Option<&input::Input> {
        for input in self.inputs() {
            if symbol == input.symbol {
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

