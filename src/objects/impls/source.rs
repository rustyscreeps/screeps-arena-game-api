use crate::{game::pathfinder::Position, objects::GameObject, HasPosition};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = Source)]
    pub static SOURCE_PROTOTYPE: Object;

    /// A [`Source`], which can be harvested for energy.
    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type Source;

    /// Current amount of energy in the source.
    #[wasm_bindgen(method, getter)]
    pub fn energy(this: &Source) -> u32;

    /// Maximum amount of energy in the source.
    #[wasm_bindgen(method, getter = energyCapacity)]
    pub fn energy_capacity(this: &Source) -> u32;

}

impl HasPosition for Source {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
