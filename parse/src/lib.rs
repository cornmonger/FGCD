use std::{path::PathBuf, path::Path, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use spreadsheet_ods::{self, RowRange, ColRange, CellRange};
use fgcd_model as model;

pub mod spreadsheet;
pub mod binary;

const GAMES: &'static str = "games";
const CHARACTERS: &'static str = "characters";
const PLATFORMS: &'static str = "platforms";
const DEVICES: &'static str = "devices";

pub enum Models {
    Game,
    Character
}

impl Models {
    const fn name(&self) -> &'static str {
        match *self {
            Models::Game => "Game",
            Models::Character => "Character"
        }
    }
}

pub fn game_data_dir<P>(game_name: &str, data_dir: &P) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    PathBuf::from(data_dir)
        .join(GAMES)
        .join(game_name)
}

pub fn game_filepath<P>(game_name: &str, data_dir: &P, extension: &str) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    game_data_dir(game_name, data_dir)
        .join(Models::Game.name().to_string() + extension)
}

pub fn character_filepath<P>(character_name: &str, game_name: &str, data_dir: &P, extension: &str) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    game_data_dir(game_name, data_dir)
        .join(CHARACTERS)
        .join(character_name.to_string() + extension)
}