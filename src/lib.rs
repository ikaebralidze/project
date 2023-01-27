use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Greayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();

    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loadded".into());

    img = img.grayscale();
    log(&"Grayscale applied".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new image".into());

    let encode_img = encode(&buffer);
    let data_url = format!("data:img/png;base64,{}", encode_img);

    data_url
}
