extern crate console_error_panic_hook;

use wasm_bindgen::prelude::wasm_bindgen;
use worker::*;

#[wasm_bindgen]
pub async fn add(a: u32, b: u32) -> u32 {
    console_error_panic_hook::set_once();
    a + b
}
