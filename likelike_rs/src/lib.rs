use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<string>")]
    pub type StringArray;
}

#[wasm_bindgen]
pub fn my_bad_words() -> StringArray {
    vec![
        "chink", "cunt", "cunts", "fag", "fagging", "faggitt", "faggot", "faggs", "fagot",
        "fagots", "fags", "jap", "homo", "nigger", "niggers", "n1gger", "nigg3r",
    ]
    .into_iter()
    .map(JsValue::from)
    .collect::<Array>()
    .unchecked_into::<StringArray>()
}
