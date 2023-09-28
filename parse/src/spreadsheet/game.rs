use super::*;
use super::super::*;

enum Sheets {
    Profile, 
    Characters,
    Inputs,
    MoveContext,
    MoveTypes
}

impl Sheets {
    const fn name(&self) -> &'static str {
        match *self {
            Sheets::Profile => "Profile",
            Sheets::Characters => "Characters",
            Sheets::Inputs => "Inputs",
            Sheets::MoveContext => "Move Context",
            Sheets::MoveTypes => "Move Types",
        }
    }
}

enum ProfileHeadings {
    Name,
    Developer,
    Publisher,
    ReleaseDate,
    Website,
    Wikipedia,
    Platforms
}

impl ProfileHeadings {
    pub const fn name(&self) -> &'static str {
        match *self {
            ProfileHeadings::Name => "Name",
            ProfileHeadings::Developer => "Developer",
            ProfileHeadings::Publisher => "Publisher",
            ProfileHeadings::ReleaseDate => "Release Date",
            ProfileHeadings::Website => "Website",
            ProfileHeadings::Wikipedia => "Wikipedia",
            ProfileHeadings::Platforms => "Platforms"
        }
    }

    pub const fn rowcol(&self) -> RowCol {
        match *self {
            ProfileHeadings::Name           => RowCol(0,0),
            ProfileHeadings::Developer      => RowCol(1,0),
            ProfileHeadings::Publisher      => RowCol(2,0),
            ProfileHeadings::ReleaseDate    => RowCol(3,0),
            ProfileHeadings::Website        => RowCol(4,0),
            ProfileHeadings::Wikipedia      => RowCol(5,0),
            ProfileHeadings::Platforms      => RowCol(6,0) 
        }
    }

    pub const fn row(&self) -> u32 {
        self.rowcol().row()
    }

    pub const fn column(&self) -> u32 {
        self.rowcol().column()
    }
}

enum CharacterHeadings {
    Name
}

impl CharacterHeadings {
    pub const fn title(&self) -> &'static str {
        match *self {
            CharacterHeadings::Name => "Name",
        }
    }

    pub const fn rowcol(&self) -> RowCol {
        match *self {
            CharacterHeadings::Name => RowCol(0,0),
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
    name: Models::Game.name(),
    sheets: &[
        Sheet { name: Sheets::Profile.name(), orientation: SheetOrientation::Horizontal, headings: &[
            SheetHeading { name: ProfileHeadings::Name.name(), rowcol: ProfileHeadings::Name.rowcol() },
            SheetHeading { name: ProfileHeadings::Developer.name(), rowcol: ProfileHeadings::Developer.rowcol() },
            SheetHeading { name: ProfileHeadings::Publisher.name(), rowcol: ProfileHeadings::Publisher.rowcol() },
            SheetHeading { name: ProfileHeadings::ReleaseDate.name(), rowcol: ProfileHeadings::ReleaseDate.rowcol()},
            SheetHeading { name: ProfileHeadings::Website.name(), rowcol: ProfileHeadings::Website.rowcol() },
            SheetHeading { name: ProfileHeadings::Wikipedia.name(), rowcol: ProfileHeadings::Wikipedia.rowcol() },
            SheetHeading { name: ProfileHeadings::Platforms.name(), rowcol: ProfileHeadings::Platforms.rowcol() },
        ] }
    ]
};


const EXT_ODS: &str = ".ods";

pub fn read_game<P>(path: &P) -> model::game::Game
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(String::from(Models::Game.name()) + EXT_ODS) };
    let workbook = spreadsheet_ods::read_ods(path).unwrap();

    let profile_sheet = workbook.iter_sheets().find(|s| s.name() == Sheets::Profile.name() ).unwrap();

    let profile = model::game::Profile::new(
        profile_sheet.value(ProfileHeadings::Name.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(ProfileHeadings::Developer.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(ProfileHeadings::Publisher.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(ProfileHeadings::ReleaseDate.row(), 1)
            .as_date_opt().unwrap(),
        profile_sheet.value(ProfileHeadings::Website.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(ProfileHeadings::Wikipedia.row(), 1)
            .as_str_opt().unwrap().to_string(),
        profile_sheet.value(ProfileHeadings::Platforms.row(), 1)
            .as_str_opt().unwrap().to_string()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    );

    let characters_sheet = workbook.iter_sheets().find(|s| s.name() == Sheets::Characters.name() ).unwrap();
    let mut character_names: Vec<String> = Vec::new();

    for row in CharacterHeadings::Name.row()+1 .. characters_sheet.used_rows() {
        let value = characters_sheet.value(row, CharacterHeadings::Name.column()).as_str_opt();
        if let Some(name) = value {
            character_names.push(name.to_string());
        } else {
            break;
        }
    }

    model::game::Game::new(profile, character_names)
}

const CHARACTERS: &str = "characters";

/*pub fn read_character_spreadsheet<P>(path: &P, character_name: &str) -> model::game::Character
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if filepath.is_file() { path } else { PathBuf::from(path).join(GAME_ODS) };

    let workbook = spreadsheet_ods::read_ods(path).unwrap();
    let profile_sheet = workbook.iter_sheets().find(|s| s.name() == GameSheets::Profile.name() ).unwrap();
}*/

