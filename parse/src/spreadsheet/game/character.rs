use anyhow::{Context, Result};
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
    Command
}

impl MoveHeadings {
    pub const fn title(&self) -> &'static str {
        match *self {
            MoveHeadings::Name => "Name",
            MoveHeadings::Type => "Type",
            MoveHeadings::Context => "Context",
            MoveHeadings::Command => "Command",
        }
    }

    pub const fn rowcol(&self) -> RowCol {
        match *self {
            MoveHeadings::Name          => RowCol(0,0),
            MoveHeadings::Type          => RowCol(1,0),
            MoveHeadings::Context       => RowCol(2,0),
            MoveHeadings::Command       => RowCol(3,0),
        }
    }

    pub const fn row(&self) -> u32 {
        self.rowcol().row()
    }

    pub const fn column(&self) -> u32 {
        self.rowcol().column()
    }
}


const CHARACTERS: &str = "characters";

pub fn read_character<P>(path: &P, character_name: &str) -> Result<Character>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(CHARACTERS).join(character_name.to_string() + EXT_ODS) };

    let workbook = spreadsheet_ods::read_ods(path)?;
    let moves_sheet = workbook.iter_sheets().find(|s| s.name() == Sheets::Moves.title() )
        .context("We ain't found Sheet")?;

    let character = Character::new(character_name.to_string());
    Ok(character)
}

