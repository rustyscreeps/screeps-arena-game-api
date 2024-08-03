use crate::{
    game::pathfinder::Position,
    objects::{GameObject, Structure},
    HasPosition,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = StructureWall)]
    pub static STRUCTURE_WALL_PROTOTYPE: Object;

    /// An object representing a [`StructureWall`], which blocks movement of all
    /// creeps.
    #[wasm_bindgen(extends = GameObject, extends = Structure)]
    #[derive(Clone)]
    pub type StructureWall;
}

impl HasPosition for StructureWall {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
