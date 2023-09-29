use std::{path::PathBuf, path::Path, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use anyhow::{Context, Result};
use bincode;
use fgcd_model::game::Game as Game;
use super::super::*;
use super::*;

pub mod character;


pub fn read_game<P>(path: &P) -> Result<Game>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(String::from(Models::Game.name()) + EXT_BIN) };
 
    let bufreader = BufReader::new(File::open(path)?);
    let game = bincode::deserialize_from(bufreader)?;
    Ok(game)
}

pub fn write_game<P>(game: &model::game::Game, path: &P) -> Result<()>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(String::from(Models::Game.name()) + EXT_BIN) };
 
    let mut bufwriter = BufWriter::new(File::create(path)?);
    bincode::serialize_into(&mut bufwriter, &game)?;
    Ok(())
}

pub fn read_game_bytes(bytes: &[u8]) -> Result<Game> {
    Ok(bincode::deserialize(bytes)?)
}