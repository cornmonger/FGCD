use std::{path::Path, io::{BufWriter, BufReader}, fs::File};
use spreadsheet_ods;
use bincode;
use fgcd_model as model;

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

enum GameSheets {
    Profile, 
    Inputs,
    MoveContext,
    MoveTypes
}

impl GameSheets {
    const fn name(&self) -> &'static str {
        match *self {
            GameSheets::Profile => "Profile",
            GameSheets::Inputs => "Inputs",
            GameSheets::MoveContext => "Move Context",
            GameSheets::MoveTypes => "Move Types",
        }
    }
}

enum Spreadsheets {
    Game,
}

impl Spreadsheets {
    const fn name(&self) -> &'static str {
        match *self {
            Spreadsheets::Game => "Game"
        }
    }
}

enum GameProfileHeadings {
    Name,
    Developer,
    Publisher,
    ReleaseDate,
    Website,
    Wikipedia,
    Platforms
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

impl GameProfileHeadings {
    pub const fn name(&self) -> &'static str {
        match *self {
            GameProfileHeadings::Name => "Name",
            GameProfileHeadings::Developer => "Developer",
            GameProfileHeadings::Publisher => "Publisher",
            GameProfileHeadings::ReleaseDate => "Release Date",
            GameProfileHeadings::Website => "Website",
            GameProfileHeadings::Wikipedia => "Wikipedia",
            GameProfileHeadings::Platforms => "Platforms"
        }
    }

    pub const fn rowcol(&self) -> RowCol {
        match *self {
            GameProfileHeadings::Name           => RowCol(0,0),
            GameProfileHeadings::Developer      => RowCol(1,0),
            GameProfileHeadings::Publisher      => RowCol(2,0),
            GameProfileHeadings::ReleaseDate    => RowCol(3,0),
            GameProfileHeadings::Website        => RowCol(4,0),
            GameProfileHeadings::Wikipedia      => RowCol(5,0),
            GameProfileHeadings::Platforms      => RowCol(6,0) 
        }
    }

    pub const fn row(&self) -> u32 {
        self.rowcol().row()
    }

    pub const fn column(&self) -> u32 {
        self.rowcol().column()
    }
}


const GAME_SPREADSHEET: Spreadsheet = Spreadsheet {
    name: Spreadsheets::Game.name(),
    sheets: &[
        Sheet { name: GameSheets::Profile.name(), orientation: SheetOrientation::Horizontal, headings: &[
            SheetHeading { name: GameProfileHeadings::Name.name(), rowcol: GameProfileHeadings::Name.rowcol() },
            SheetHeading { name: GameProfileHeadings::Developer.name(), rowcol: GameProfileHeadings::Developer.rowcol() },
            SheetHeading { name: GameProfileHeadings::Publisher.name(), rowcol: GameProfileHeadings::Publisher.rowcol() },
            SheetHeading { name: GameProfileHeadings::ReleaseDate.name(), rowcol: GameProfileHeadings::ReleaseDate.rowcol()},
            SheetHeading { name: GameProfileHeadings::Website.name(), rowcol: GameProfileHeadings::Website.rowcol() },
            SheetHeading { name: GameProfileHeadings::Wikipedia.name(), rowcol: GameProfileHeadings::Wikipedia.rowcol() },
            SheetHeading { name: GameProfileHeadings::Platforms.name(), rowcol: GameProfileHeadings::Platforms.rowcol() },
        ] }
    ]
};



pub fn read_game_ods<P>(path: &P) -> model::game::Game
where
    P: ?Sized + AsRef<Path>
{
    let workbook = spreadsheet_ods::read_ods(path).unwrap();
    let profile_sheet = workbook.iter_sheets().find(|s| s.name() == GameSheets::Profile.name() ).unwrap();

    let game_profile = model::game::Profile::new(
        profile_sheet.value(GameProfileHeadings::Name.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(GameProfileHeadings::Developer.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(GameProfileHeadings::Publisher.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(GameProfileHeadings::ReleaseDate.row(), 1)
            .as_date_opt().unwrap(),
        profile_sheet.value(GameProfileHeadings::Website.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(GameProfileHeadings::Wikipedia.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(GameProfileHeadings::Platforms.row(), 1)
            .as_str_opt().unwrap().to_string()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    );

    let game = model::game::Game::new(game_profile);
    game
}

pub fn write_game_bin<P>(game: &model::game::Game, path: &P)
where
    P: ?Sized + AsRef<Path>
{
    let mut bufwriter = BufWriter::new(File::create(path).unwrap());
    bincode::serialize_into(&mut bufwriter, &game).unwrap();
}

pub fn read_game_bin<P>(path: &P) -> model::game::Game
where
    P: ?Sized + AsRef<Path>
{
    let bufreader = BufReader::new(File::open(path).unwrap());
    let game = bincode::deserialize_from(bufreader).unwrap();
    game
}

pub fn read_game_bytes(bytes: &[u8]) -> model::game::Game {
    bincode::deserialize(bytes).unwrap()
}