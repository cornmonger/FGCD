use anyhow::{Context, Result};
use model::{game::{Move, Game}, input::{Sequence, Input, Entry}};
use crate::CHARACTERS;

use super::super::*;
use fgcd_model::game::Character;


enum Sheets {
    //Profile, 
    Moves,
}

impl Sheets {
    const fn title(&self) -> &'static str {
        match *self {
            //Sheets::Profile => "Profile",
            Sheets::Moves => "Moves",
        }
    }
}

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

pub fn read_character<P>(character_name: &str, game: &Game, path: &P) -> Result<Character>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(CHARACTERS).join(character_name.to_string() + EXT_ODS) };

    let workbook = spreadsheet_ods::read_ods(path)?;
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

