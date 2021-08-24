use crate::{
    constants::Part,
    objects::GameObject,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "arena")]
extern "C" {
    #[wasm_bindgen(js_name = BodyPart)]
    pub static BODY_PART_PROTOTYPE: Object;

    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type BodyPart;

    /// Returns the type of the body part.
    #[wasm_bindgen(method, getter = type)]
    pub fn part_type(this: &BodyPart) -> Part;
}


// impl JsContainerFromValue for BodyPart {
//     fn from_value(val: JsValue) -> Self {
//         val.unchecked_into()
//     }
// }
