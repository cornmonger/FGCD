
#[cfg(test)]
mod tests {
    use std::path::{Path,PathBuf};
    use fgcd_parse;

    const MORTAL_KOMBAT_1: &str = "Mortal Kombat 1";

    #[test]
    pub fn game_spreadsheet_to_binary() {
        let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("data").join("target").join("data");
        let game_data_target_dir = PathBuf::from(data_dir).join("games");
        let mkone_game_data_target_dir = PathBuf::from(game_data_target_dir).join(MORTAL_KOMBAT_1);

        let game = fgcd_parse::spreadsheet::game::read_game(&mkone_game_data_target_dir);    
        println!("From ODS: {:#?}", game);

        fgcd_parse::binary::game::write_game(&game, &mkone_game_data_target_dir);

        let game_read = fgcd_parse::binary::game::read_game(&mkone_game_data_target_dir);
        println!("From binary: {:#?}", game_read);
    }
}