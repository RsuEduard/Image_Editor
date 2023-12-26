use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::{self, general_purpose}};
use image::load_from_memory;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale called".into());

    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());
}