use std::{path::PathBuf, path::Path, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use spreadsheet_ods::{self, RowRange, ColRange, CellRange};
use fgcd_model as model;

pub mod spreadsheet;
pub mod binary;

const CHARACTERS: &'static str = "characters";

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

