extern crate console_error_panic_hook;

use wasm_bindgen::prelude::wasm_bindgen;
// use http::Response;
use worker::*;

#[wasm_bindgen]
pub async fn add(a: u32, b: u32) -> u32 {
    console_error_panic_hook::set_once();
    a + b
}

// #[event(fetch)]
// async fn fetch(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<http::Response<String>> {
//     Ok(Response::builder()
//         .body("test".to_string())?)
// }
