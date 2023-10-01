use std::{path::PathBuf, path::Path, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use spreadsheet_ods::{self, RowRange, ColRange, CellRange};
use fgcd_model as model;

pub mod game;

const EXT_FODS: &str = ".fods";

struct Spreadsheet {
    name: &'static str,
    sheets: &'static [Sheet]
}

struct Sheet {
    name: &'static str,
    orientation: SheetOrientation,
    headings: &'static [SheetHeading]
}

enum SheetOrientation {
    Horizontal,
    Vertical
}

struct SheetHeading {
    name: &'static str,
    rowcol: RowCol
}

pub struct RowCol(u32, u32);

impl RowCol {
    pub const fn row(&self) -> u32 {
        self.0
    }

    pub const fn column(&self) -> u32 {
        self.1
    }
}

