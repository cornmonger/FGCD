use fgcd_parse;

fn main() {
    let game = fgcd_parse::read_game_ods("/home/roylaurie/sync/github/elseclub/FGCD/data/target/data/games/Mortal Kombat 1/Game.ods");    
    println!("From ODS: {:#?}", game);

    fgcd_parse::write_game_bin(&game, "/home/roylaurie/sync/github/elseclub/FGCD/data/target/data/games/Mortal Kombat 1/Game.bin");

    let game_read = fgcd_parse::read_game_bin("/home/roylaurie/sync/github/elseclub/FGCD/data/target/data/games/Mortal Kombat 1/Game.bin");
    println!("From binary: {:#?}", game_read);
}
