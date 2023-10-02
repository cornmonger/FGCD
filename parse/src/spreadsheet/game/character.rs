use std::fs;
use anyhow::{Context, Result};
use icu_locid::locale;
use strum::{IntoEnumIterator};
use model::{game::{Move, Game}, input::{Sequence, Input, Entry}};
use crate::{CHARACTERS, character_filepath};
use super::super::*;
use fgcd_model::game::Character;


enum Sheets {
    //Profile, 
    Moves,
}

impl Sheets {
    const fn orientation(&self) -> SheetOrientation {
        match *self {
            //Sheets::Profile =>  SheetOrientation::Horizontal,
            _ => SheetOrientation::Vertical
        }
    }

    const fn title(&self) -> &'static str {
        match *self {
            //Sheets::Profile => "Profile",
            Sheets::Moves => "Moves",
        }
    }
}


#[derive(Debug, PartialEq, strum::EnumIter)]
enum MoveHeadings {
    Name,
    Type,
    Context,
    Input 
}

impl MoveHeadings {
    pub const fn title(&self) -> &'static str {
        match *self {
            MoveHeadings::Name => "Name",
            MoveHeadings::Type => "Type",
            MoveHeadings::Context => "Context",
            MoveHeadings::Input => "Input",
        }
    }

    pub const fn rowcol(&self) -> RowCol {
        match *self {
            MoveHeadings::Name          => RowCol(0,0),
            MoveHeadings::Type          => RowCol(0,1),
            MoveHeadings::Context       => RowCol(0,2),
            MoveHeadings::Input         => RowCol(0,3),
        }
    }

    pub const fn row(&self) -> u32 {
        self.rowcol().row()
    }

    pub const fn column(&self) -> u32 {
        self.rowcol().column()
    }
}

/**
 * @param path Either the FGCD data directory or the file path for the character data file
 */
pub fn read_character<P>(character_name: &str, game: &Game, path: &P) -> Result<Character>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { character_filepath(character_name, game.name(), &path, EXT_FODS) };

    let workbook = spreadsheet_ods::read_fods(path)?;
    let moves_sheet = workbook.iter_sheets().find(|s| s.name() == Sheets::Moves.title() )
        .context("We ain't found Sheet")?;

    let mut moves: Vec<Move> = Vec::new();

    for row in MoveHeadings::Name.row()+1 .. moves_sheet.used_rows() {
        let value = moves_sheet.value(row, MoveHeadings::Name.column()).as_str_opt();
        if value.is_none() {
            break;
        }

        let name = value.unwrap().to_string();
        let symbols = moves_sheet.value(row, MoveHeadings::Input.column())
                .as_str_opt().context("Missing symbol column")?
                .to_string();

        let sequence = Sequence::from_game(&game, &symbols)?; //TODO
        moves.push(Move::new(name, sequence));
    }


    let character = Character::new(character_name.to_string(), moves);
    Ok(character)
}

pub fn new_character<P>(game: &Game, character_name: &str, data_dir: &P) -> Result<PathBuf>
where
    P: ?Sized + AsRef<OsStr>
{
    let mut workbook = spreadsheet_ods::WorkBook::new(locale!("en_US"));

    let (header_vertical_style, header_horizontal_style) = create_styles(&mut workbook);

    let style_for = |orientation: SheetOrientation| -> &spreadsheet_ods::CellStyleRef {
        match orientation {
            SheetOrientation::Vertical => &header_vertical_style,
            SheetOrientation::Horizontal => &header_horizontal_style
        }
    };

    // MOVES 
    let mut moves_sheet = spreadsheet_ods::Sheet::new(Sheets::Moves.title());
    for heading in MoveHeadings::iter() {
        moves_sheet.set_styled_value(heading.row(), heading.column(), heading.title(), style_for(Sheets::Moves.orientation()));
    }

    workbook.push_sheet(moves_sheet);

    let character_filepath = character_filepath(character_name, game.name(), data_dir, EXT_FODS);
    spreadsheet_ods::write_fods(&mut workbook, &character_filepath)?;

    Ok(character_filepath)
}

