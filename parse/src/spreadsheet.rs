use std::{path::PathBuf, path::Path, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use spreadsheet_ods::{self, RowRange, ColRange, CellRange};
use fgcd_model as model;

pub mod game;

pub const EXT_FODS: &str = ".fods";

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

/* Returns (vertical, horizontal) spreadsheet style references after adding them to a workbook */
fn create_styles(workbook: &mut spreadsheet_ods::WorkBook) -> (spreadsheet_ods::CellStyleRef, spreadsheet_ods::CellStyleRef) {
    let mut header_vertical_style = spreadsheet_ods::CellStyle::new_empty();
    header_vertical_style.set_font_bold();
    header_vertical_style.set_text_align(spreadsheet_ods::style::units::TextAlign::Center);
    let header_vertical_style = workbook.add_cellstyle(header_vertical_style);

    let mut header_horizontal_style = spreadsheet_ods::CellStyle::new_empty();
    header_horizontal_style.set_font_bold();
    header_horizontal_style.set_text_align(spreadsheet_ods::style::units::TextAlign::Left);
    let header_horizontal_style = workbook.add_cellstyle(header_horizontal_style);

    (header_vertical_style, header_horizontal_style)
}