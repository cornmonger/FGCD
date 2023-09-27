mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use chrono;
use serde_wasm_bindgen;
use fgcd_model as model;
use fgcd_parse;
use web_sys;
use wasm_bindgen_futures;
use js_sys;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn greet() -> Result<JsValue, JsValue> {
    set_panic_hook();

    let window = web_sys::window().unwrap();
    let request = web_sys::Request::new_with_str("Game.bin").unwrap();
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(response.is_instance_of::<web_sys::Response>());
    let response: web_sys::Response = response.dyn_into().unwrap();

    let buf_promise = response.array_buffer().unwrap();
    let buf = wasm_bindgen_futures::JsFuture::from(buf_promise)
        .await
        .unwrap();
    assert!(buf.is_instance_of::<js_sys::ArrayBuffer>());

    let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&buf);
    let mut bytes = vec![0; typebuf.length() as usize];
    typebuf.copy_to(&mut bytes[..]);

    let game = fgcd_parse::read_game_bytes(&bytes);
    Ok(serde_wasm_bindgen::to_value(&game)?)
}
