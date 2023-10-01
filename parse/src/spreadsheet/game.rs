use std::fs;
use super::*;
use super::super::*;
use anyhow::{Context, Result};
use fgcd_model::game::Game;
use model::input::{Input, Symbol};
use icu_locid::locale;
use strum::{IntoEnumIterator};

pub mod character;

enum Sheets {
    Profile, 
    Characters,
    Inputs,
    //MoveContext,
    //MoveTypes
}

impl Sheets {
    pub const fn orientation(&self) -> SheetOrientation {
        match *self {
            Sheets::Profile =>  SheetOrientation::Horizontal,
            _ => SheetOrientation::Vertical
        }
    }

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

#[derive(Debug, PartialEq, strum::EnumIter)]
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

#[derive(Debug, PartialEq, strum::EnumIter)]
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

#[derive(Debug, PartialEq, strum::EnumIter)]
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

pub fn read_game<P>(path: &P) -> Result<Game>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let filepath = if path.is_file() { path } else { PathBuf::from(path).join(Models::Game.name().to_string() + EXT_FODS) };
    print!("{}", filepath.to_str().unwrap());
    let workbook = spreadsheet_ods::read_fods(filepath)?;

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
            inputs_sheet.value(row, InputsHeadings::Symbol.column())
                .as_str_opt().context("Missing symbol column")?
                .to_string()
        );
        
        inputs.push(input);
    }

    Ok(Game::new(profile, character_names, inputs))
}


pub fn new_game<P>(game_name: &str, data_dir: &P) -> Result<PathBuf>
where
    P: ?Sized + AsRef<OsStr>
{
    let game_data_dir = PathBuf::from(data_dir).join(game_name);
    if !game_data_dir.exists() {
        fs::create_dir(&game_data_dir)?;
    }

    let mut workbook = spreadsheet_ods::WorkBook::new(locale!("en_US"));

    let mut header_vertical_style = spreadsheet_ods::CellStyle::new_empty();
    header_vertical_style.set_font_bold();
    header_vertical_style.set_text_align(spreadsheet_ods::style::units::TextAlign::Center);
    let header_vertical_style = workbook.add_cellstyle(header_vertical_style);

    let mut header_horizontal_style = spreadsheet_ods::CellStyle::new_empty();
    header_horizontal_style.set_font_bold();
    header_horizontal_style.set_text_align(spreadsheet_ods::style::units::TextAlign::Left);
    let header_horizontal_style = workbook.add_cellstyle(header_horizontal_style);

    let style_for = |orientation: SheetOrientation| -> &spreadsheet_ods::CellStyleRef {
        match orientation {
            SheetOrientation::Vertical => &header_vertical_style,
            SheetOrientation::Horizontal => &header_horizontal_style
        }
    };

    // PROFILE
    let mut profile_sheet = spreadsheet_ods::Sheet::new(Sheets::Profile.title());
    for heading in ProfileHeadings::iter() {
        profile_sheet.set_styled_value(heading.row(), heading.column(), heading.title(), style_for(Sheets::Profile.orientation()));
    }

    // CHARACTERS
    let mut characters_sheet = spreadsheet_ods::Sheet::new(Sheets::Characters.title());
    for heading in CharactersHeadings::iter() {
        characters_sheet.set_styled_value(heading.row(), heading.column(), heading.title(), style_for(Sheets::Characters.orientation()));
    }

    // INPUTS
    let mut inputs_sheet = spreadsheet_ods::Sheet::new(Sheets::Inputs.title());
    for heading in InputsHeadings::iter() {
        inputs_sheet.set_styled_value(heading.row(), heading.column(), heading.title(), style_for(Sheets::Inputs.orientation()));
    }

    workbook.push_sheet(profile_sheet);
    workbook.push_sheet(characters_sheet);
    workbook.push_sheet(inputs_sheet);

    let game_filepath = game_data_dir.join(Models::Game.name().to_string() + EXT_FODS);
    spreadsheet_ods::write_fods(&mut workbook, &game_filepath)?;
    Ok(game_filepath)
}


