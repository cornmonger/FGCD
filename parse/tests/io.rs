
#[cfg(test)]
mod tests {
    use std::path::{Path,PathBuf};
    use fgcd_parse;

    const MORTAL_KOMBAT_1: &str = "Mortal Kombat 1";
    const JOHNNY_CAGE: &str = "Johnny Cage";

    fn game_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("data")
            .join("target")
            .join("data")
            .join("games")
    }

    fn game_data_target_dir(game_name: &str) -> PathBuf {
        game_data_dir().join(game_name)
    }

    #[test]
    fn game_spreadsheets_to_binary() {
        let mkone_dir = game_data_target_dir(MORTAL_KOMBAT_1);
        print!("{}", &mkone_dir.to_str().unwrap());

        let game = fgcd_parse::spreadsheet::game::read_game(&mkone_dir).unwrap();    
        println!("Game from ODF: {:#?}", game);

        // Write Game from ODS to binary
        fgcd_parse::binary::game::write_game(&game, &mkone_dir).unwrap();

        let game_read = fgcd_parse::binary::game::read_game(&mkone_dir);
        println!("Game from binary: {:#?}", game_read);

        let character = fgcd_parse::spreadsheet::game::character::read_character(JOHNNY_CAGE, &game, &mkone_dir).unwrap();
        println!("Character from ODF: {:#?}", character);

        fgcd_parse::binary::game::character::write_character(&character, &game, &mkone_dir).unwrap();

        let character_binary = fgcd_parse::binary::game::character::read_character(JOHNNY_CAGE, &game, &mkone_dir);
        println!("Character from binary: {:#?}", character_binary);
    }

    #[test]
    fn new_game_spreadsheet() {
        const CLAYFIGHTER_128: &str = "Clayfighter 128";
        let clayfighter_game_filepath = fgcd_parse::spreadsheet::game::new_game(CLAYFIGHTER_128, &game_data_dir()).unwrap();
    }
}