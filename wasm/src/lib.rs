mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use chrono;
use serde_wasm_bindgen;
use fgcd_model as model;
use fgcd_parse;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> Result<JsValue, JsValue> {
    set_panic_hook();

    /*let game_profile = model::game::Profile::new(
        String::from("Game Name"),
        String::from("Publishing House"),
        String::from("Big Corp"),
        chrono::Utc::now().date_naive(),
        String::from("http://foo.com"),
        String::from("http://wikipedia.org"),
        vec![String::from("Xbox"), String::from("Gameboy")]
    );*/

    let game = fgcd_parse::read_game_bin("Game.bin");
    //let game = model::game::Game::new(game_profile);
    Ok(serde_wasm_bindgen::to_value(&game)?)
}
