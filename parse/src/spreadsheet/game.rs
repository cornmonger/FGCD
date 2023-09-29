use super::*;
use super::super::*;
use anyhow::{Context, Result};
use fgcd_model::game::Game;
use model::input::{Input, Symbol};

pub mod character;

enum Sheets {
    Profile, 
    Characters,
    Inputs,
    //MoveContext,
    //MoveTypes
}

impl Sheets {
    const fn title(&self) -> &'static str {
        match *self {
            Sheets::Profile => "Profile",
            Sheets::Characters => "Characters",
            Sheets::Inputs => "Inputs",
            //Sheets::MoveContext => "Move Context",
            //Sheets::MoveTypes => "Move Types",
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
    pub const fn title(&self) -> &'static str {
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

enum CharactersHeadings {
    Name
}

impl CharactersHeadings {
    pub const fn title(&self) -> &'static str {
        match *self {
            CharactersHeadings::Name => "Name",
        }
    }

    pub const fn rowcol(&self) -> RowCol {
        match *self {
            CharactersHeadings::Name => RowCol(0,0),
        }
    }

    pub const fn row(&self) -> u32 {
        self.rowcol().row()
    }

    pub const fn column(&self) -> u32 {
        self.rowcol().column()
    }
}
enum InputsHeadings {
    Name,
    Symbol
}

impl InputsHeadings {
    pub const fn title(&self) -> &'static str {
        match *self {
            InputsHeadings::Name => "Name",
            InputsHeadings::Symbol => "Symbol",
        }
    }

    pub const fn rowcol(&self) -> RowCol {
        match *self {
            InputsHeadings::Name =>   RowCol(0,0),
            InputsHeadings::Symbol => RowCol(0,1),
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
        Sheet { name: Sheets::Profile.title(), orientation: SheetOrientation::Horizontal, headings: &[
            SheetHeading { name: ProfileHeadings::Name.title(), rowcol: ProfileHeadings::Name.rowcol() },
            SheetHeading { name: ProfileHeadings::Developer.title(), rowcol: ProfileHeadings::Developer.rowcol() },
            SheetHeading { name: ProfileHeadings::Publisher.title(), rowcol: ProfileHeadings::Publisher.rowcol() },
            SheetHeading { name: ProfileHeadings::ReleaseDate.title(), rowcol: ProfileHeadings::ReleaseDate.rowcol()},
            SheetHeading { name: ProfileHeadings::Website.title(), rowcol: ProfileHeadings::Website.rowcol() },
            SheetHeading { name: ProfileHeadings::Wikipedia.title(), rowcol: ProfileHeadings::Wikipedia.rowcol() },
            SheetHeading { name: ProfileHeadings::Platforms.title(), rowcol: ProfileHeadings::Platforms.rowcol() },
        ] }
    ]
};



pub fn read_game<P>(path: &P) -> Result<Game>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { PathBuf::from(path).join(String::from(Models::Game.name()) + EXT_ODS) };
    let workbook = spreadsheet_ods::read_ods(path)?;

    // PROFILE
    let profile_sheet = workbook.iter_sheets().find(|s| s.name() == Sheets::Profile.title() )
        .context("We ain't found Sheet")?;

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

    // CHARACTERS
    let characters_sheet = workbook.iter_sheets().find(|s| s.name() == Sheets::Characters.title() )
        .context("We ain't found Sheet")?;

    let mut character_names: Vec<String> = Vec::new();

    for row in CharactersHeadings::Name.row()+1 .. characters_sheet.used_rows() {
        let value = characters_sheet.value(row, CharactersHeadings::Name.column()).as_str_opt();
        if let Some(name) = value {
            character_names.push(name.to_string());
        } else {
            break;
        }
    }

    // INPUTS
    let inputs_sheet = workbook.iter_sheets().find(|s| s.name() == Sheets::Inputs.title() )
        .context("We ain't found Sheet")?;

    let mut inputs: Vec<Input> = Vec::new();

    for row in InputsHeadings::Name.row()+1 .. inputs_sheet.used_rows() {
        let value = inputs_sheet.value(row, InputsHeadings::Name.column()).as_str_opt();
        if value.is_none() {
            break;
        }

        let name = value.unwrap().to_string();
        let input = Input::new(
            name,
            Symbol::new(inputs_sheet.value(row, InputsHeadings::Symbol.column())
                .as_str_opt().context("Missing symbol column")?
                .to_string())
        );
        
        inputs.push(input);
    }

    Ok(Game::new(profile, character_names, inputs))
}
