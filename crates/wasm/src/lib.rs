use wasm_bindgen::prelude::*;

/// Encode plain English to Lesshand using default rules.
#[wasm_bindgen]
pub fn encode(input: &str) -> String {
    lesshand::enc::enc_with_default_rules(input)
}

/// Decode Lesshand to plain English using default rules.
#[wasm_bindgen]
pub fn decode(input: &str) -> String {
    lesshand::dec::dec_with_default_rules(input)
}
