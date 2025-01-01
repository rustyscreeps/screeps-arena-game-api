use crate::{constants::ResourceType, objects::GameObject, prelude::*};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "arena/season_beta/collect_and_control/advanced/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = ScoreCollector)]
    pub static SCORE_COLLECTOR_PROTOTYPE: Object;

    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type ScoreCollector;

    /// The type of the resource this collector accepts.
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &ScoreCollector) -> ResourceType;

    /// Whether you have control over this collector.
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &ScoreCollector) -> bool;

    /// Current collected score number of the owner player.
    #[wasm_bindgen(method, getter)]
    pub fn score(this: &ScoreCollector) -> u32;

    /// Total number of score needed to win instantly.
    #[wasm_bindgen(method, getter = scoreTotal)]
    pub fn score_total(this: &ScoreCollector) -> u32;
}

impl HasPosition for ScoreCollector {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
// impl JsContainerFromValue for ScoreCollector {
//     fn from_value(val: JsValue) -> Self {
//         val.unchecked_into()
//     }
// }
