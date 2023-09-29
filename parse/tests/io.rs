
#[cfg(test)]
mod tests {
    use std::path::{Path,PathBuf};
    use fgcd_parse;

    const MORTAL_KOMBAT_1: &str = "Mortal Kombat 1";
    const JOHNNY_CAGE: &str = "Johnny Cage";

    pub fn game_data_target_dir(game_name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("data")
            .join("target")
            .join("data")
            .join("games")
            .join(game_name)
    }

    #[test]
    pub fn game_spreadsheets_to_binary() {
        let mkone_dir = game_data_target_dir(MORTAL_KOMBAT_1);

        let game = fgcd_parse::spreadsheet::game::read_game(&mkone_dir).unwrap();    
        println!("Game from ODS: {:#?}", game);

        // Write Game from ODS to binary
        fgcd_parse::binary::game::write_game(&game, &mkone_dir).unwrap();

        let game_read = fgcd_parse::binary::game::read_game(&mkone_dir);
        println!("Game from binary: {:#?}", game_read);

        let character = fgcd_parse::spreadsheet::game::character::read_character(&game, JOHNNY_CAGE, &mkone_dir).unwrap();
        println!("Character from ODS: {:#?}", character);
    }
}