use std::{path::PathBuf, path::Path, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use anyhow::{Context, Result};
use bincode;
use model::game::Character;
use model::game::Game;
use crate::CHARACTERS;
use super::super::*;
use super::*;

pub fn read_character<P>(character_name: &str, _game: &Game, path: &P) -> Result<Character>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(CHARACTERS).join(character_name.to_string() + EXT_BIN) };
 
    let bufreader = BufReader::new(File::open(path)?);
    let character = bincode::deserialize_from(bufreader)?;
    Ok(character)
}

pub fn write_character<P>(character: &Character, _game: &Game, path: &P) -> Result<()>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(CHARACTERS).join(character.name().to_string() + EXT_BIN) };
 
    let mut bufwriter = BufWriter::new(File::create(path)?);
    bincode::serialize_into(&mut bufwriter, &character)?;
    Ok(())
}

pub fn read_character_bytes(_game: &Game, bytes: &[u8]) -> Result<Character> {
    Ok(bincode::deserialize(bytes)?)
}
