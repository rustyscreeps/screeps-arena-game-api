use crate::{constants::EffectType, objects::GameObject, prelude::*};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "arena/season_beta/collect_and_control/advanced/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = AreaEffect)]
    pub static AREA_EFFECT_PROTOTYPE: Object;

    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type AreaEffect;

    /// Returns the type of the body part.
    #[wasm_bindgen(method, getter)]
    pub fn effect(this: &AreaEffect) -> EffectType;
}

impl HasPosition for AreaEffect {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}

// impl JsContainerFromValue for AreaEffect {
//     fn from_value(val: JsValue) -> Self {
//         val.unchecked_into()
//     }
// }
