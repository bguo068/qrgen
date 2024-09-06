use qrcode_generator::QrCodeEcc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_image_from_url(url: &str) -> Result<Vec<u8>, JsValue> {
    qrcode_generator::to_png_to_vec(url, QrCodeEcc::Medium, 1000)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
