use serde;
use anyhow::{Context, Result};
use super::game::Game;

pub type Symbol = String;

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
pub enum Entry {
    Input(Symbol),
    Combination(Vec<Symbol>),
    Hold(u64),
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

    pub fn parse_sequence(game: &Game, symbols: &str) -> Result<Sequence> {
        let mut entries: Vec<Entry> = Vec::new();

        for token in symbols.split(',').map(|s| s.trim()) {
            if (token.contains('+')) {
                let combo: Vec<Entry> = token.split('+')
                    .map(|s| s.trim())
                    .map(|s| Self::parse_sequence_token(game, s).unwrap())
                    .collect();
                entries.push(Entry::Combination(combo));
            } else {
                entries.push(Self::parse_sequence_token(game, token));
            }
        }

        Ok(Sequence::new(entries))
    }

    fn parse_sequence_token(game: &Game, token: &str) -> Result<Entry> {
        let entry = Entry::Input(
            game.find_input(token)
            .context(format!("Unknown input: {token}"))
            .unwrap()
            .symbol
            .clone()
        );

        Ok(entry)
    }
}