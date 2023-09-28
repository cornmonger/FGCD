use std::{path::PathBuf, path::Path, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use bincode;
use fgcd_model as model;
use super::*;
use super::super::*;

const EXT_BIN: &str = ".bin";

pub fn read_game<P>(path: &P) -> model::game::Game
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(String::from(Models::Game.name()) + EXT_BIN) };
 
    let bufreader = BufReader::new(File::open(path).unwrap());
    let game = bincode::deserialize_from(bufreader).unwrap();
    game
}

pub fn write_game<P>(game: &model::game::Game, path: &P)
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(String::from(Models::Game.name()) + EXT_BIN) };
 
    let mut bufwriter = BufWriter::new(File::create(path).unwrap());
    bincode::serialize_into(&mut bufwriter, &game).unwrap();
}

pub fn read_game_bytes(bytes: &[u8]) -> model::game::Game {
    bincode::deserialize(bytes).unwrap()
}