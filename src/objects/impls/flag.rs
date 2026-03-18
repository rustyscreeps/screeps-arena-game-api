use crate::{objects::GameObject, prelude::*};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes/flag")]
extern "C" {
    #[wasm_bindgen(js_name = Flag)]
    pub static FLAG_PROTOTYPE: Object;

    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type Flag;

    /// Returns `Some(true)` for your flag, `Some(false)` for a hostile flag,
    /// `None` for a neutral flag.
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &Flag) -> Option<bool>;
}

impl HasPosition for Flag {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
// impl JsContainerFromValue for Flag {
//     fn from_value(val: JsValue) -> Self {
//         val.unchecked_into()
//     }
// }
