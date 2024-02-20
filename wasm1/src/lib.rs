extern crate console_error_panic_hook;
extern crate wasm_bindgen;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterAction {
    Append,
    Remove,
    Keep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    Standard,
    Boundingbox,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterInput {
    #[serde(alias = "filtertype")]
    pub filter_action: FilterAction,
    #[serde(alias = "type")]
    pub filter_type: FilterType,
}

#[wasm_bindgen]
pub fn json_data(filter_input: JsValue) -> u8 {
    console_error_panic_hook::set_once();

    let _filter_input: Option<Vec<Vec<FilterInput>>> =
        serde_wasm_bindgen::from_value(filter_input).unwrap();

    1
}
