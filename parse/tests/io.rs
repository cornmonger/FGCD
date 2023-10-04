
#[cfg(test)]
mod tests {
    use std::path::{Path,PathBuf};
    use fgcd_parse;

    const MORTAL_KOMBAT_1: &str = "Mortal Kombat 1";
    const JOHNNY_CAGE: &str = "Johnny Cage";

    fn fgcd_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("data")
            .join("target")
            .join("data")
    }

    #[test]
    fn game_spreadsheets_to_binary() {
        let data_dir = fgcd_data_dir();

        let mkone_dir = fgcd_parse::game_data_dir(MORTAL_KOMBAT_1, &data_dir);
        assert!(mkone_dir.exists(),
            "Game dir should exist");
        let mkone_game_filepath = fgcd_parse::game_filepath(MORTAL_KOMBAT_1, &data_dir, fgcd_parse::spreadsheet::EXT_FODS);
        assert!(mkone_game_filepath.exists(),
            "Game FODS file should exist: {}", mkone_game_filepath.to_str().unwrap());

        let game = fgcd_parse::spreadsheet::game::read_game(MORTAL_KOMBAT_1, &data_dir).unwrap();    
        println!("Game from ODF: {:#?}", game);

        // Write Game from ODS to binary
        fgcd_parse::binary::game::write_game(&game, &data_dir).unwrap();

        let game_read = fgcd_parse::binary::game::read_game(MORTAL_KOMBAT_1, &data_dir);
        println!("Game from binary: {:#?}", game_read);

        let character = fgcd_parse::spreadsheet::game::character::read_character(JOHNNY_CAGE, &game, &data_dir).unwrap();
        println!("Character from ODF: {:#?}", character);

        fgcd_parse::binary::game::character::write_character(&character, &game, &data_dir).unwrap();

        let character_binary = fgcd_parse::binary::game::character::read_character(JOHNNY_CAGE, &game, &data_dir);
        println!("Character from binary: {:#?}", character_binary);
    }

    #[test]
    fn new_game_spreadsheet() {
        let data_dir = fgcd_data_dir();
        const CLAYFIGHTER_128: &str = "Clayfighter 128";
        const CREAMSICKLE: &str = "Creamsickle";
        let clayfighter_game_filepath = fgcd_parse::spreadsheet::game::new_game(CLAYFIGHTER_128, &data_dir).unwrap();
        let clayfighter_game = fgcd_parse::spreadsheet::game::read_game(CLAYFIGHTER_128, &clayfighter_game_filepath).unwrap();
        let creamsickle_character_filepath = fgcd_parse::spreadsheet::game::character::new_character(&clayfighter_game, CREAMSICKLE, &data_dir).unwrap();
        let creamsickle_character = fgcd_parse::spreadsheet::game::character::read_character(CREAMSICKLE, &clayfighter_game, &data_dir).unwrap();
    }
}