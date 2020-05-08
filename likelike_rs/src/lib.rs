use js_sys::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn my_bad_words() -> Array {
    vec![
        "chink", "cunt", "cunts", "fag", "fagging", "faggitt", "faggot", "faggs", "fagot",
        "fagots", "fags", "jap", "homo", "nigger", "niggers", "n1gger", "nigg3r",
    ]
    .into_iter()
    .map(JsValue::from)
    .collect()
}
