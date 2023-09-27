extern crate derive_more;
use derive_more::Display;
use spreadsheet_ods;
use std::{path::Path};
use fgcd_model;

const GAME_SHEET_NAMES: [&str;4] = [ "Profile", "Inputs", "Move Context", "Move Types" ];

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
    position: (u32, u32)
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

impl GameProfileHeadings {
    const fn name(&self) -> &'static str {
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

    const fn position(&self) -> (u32,u32) {
        match *self {
            GameProfileHeadings::Name           => (0,0),
            GameProfileHeadings::Developer      => (1,0),
            GameProfileHeadings::Publisher      => (2,0),
            GameProfileHeadings::ReleaseDate    => (3,0),
            GameProfileHeadings::Website        => (4,0),
            GameProfileHeadings::Wikipedia      => (5,0),
            GameProfileHeadings::Platforms      => (6,0) 
        }
    }


}


const GAME_SPREADSHEET: Spreadsheet = Spreadsheet {
    name: Spreadsheets::Game.name(),
    sheets: &[
        Sheet { name: GameSheets::Profile.name(), orientation: SheetOrientation::Horizontal, headings: &[
            SheetHeading { name: GameProfileHeadings::Name.name(), position: GameProfileHeadings::Name.position() },
            SheetHeading { name: GameProfileHeadings::Developer.name(), position: GameProfileHeadings::Developer.position() },
            SheetHeading { name: GameProfileHeadings::Publisher.name(), position: GameProfileHeadings::Publisher.position() },
            SheetHeading { name: GameProfileHeadings::ReleaseDate.name(), position: GameProfileHeadings::ReleaseDate.position()},
            SheetHeading { name: GameProfileHeadings::Website.name(), position: GameProfileHeadings::Website.position() },
            SheetHeading { name: GameProfileHeadings::Wikipedia.name(), position: GameProfileHeadings::Wikipedia.position() },
            SheetHeading { name: GameProfileHeadings::Platforms.name(), position: GameProfileHeadings::Platforms.position() },
        ] }
    ]
};



pub fn read_game_ods<P>(path: &P)
where
    P: ?Sized + AsRef<Path>
{
    let workbook = spreadsheet_ods::read_ods(path).unwrap();
    let profile_sheet = workbook.iter_sheets().find(|s| s.name() == GameSheets::Profile.name() ).unwrap();

    let game_profile = fgcd_model::GameProfile {
        name: profile_sheet.value(GameProfileHeadings::Name.position().0, 1).as_str_opt().unwrap().to_string(),
        developer: profile_sheet.value(GameProfileHeadings::Developer.position().0, 1).as_str_opt().unwrap().to_string(),
        publisher: profile_sheet.value(GameProfileHeadings::Publisher.position().0, 1).as_str_opt().unwrap().to_string(),
        release_date: profile_sheet.value(GameProfileHeadings::ReleaseDate.position().0, 1).as_date_opt().unwrap().to_string(),
        website_url: profile_sheet.value(GameProfileHeadings::Website.position().0, 1).as_str_opt().unwrap().to_string(),
        wikipedia_page_url: profile_sheet.value(GameProfileHeadings::Wikipedia.position().0, 1).as_str_opt().unwrap().to_string(),
        platform_names: profile_sheet.value(GameProfileHeadings::Platforms.position().0, 1).as_str_opt().unwrap().to_string(),
    };

    println!("{:#?}", game_profile)

}