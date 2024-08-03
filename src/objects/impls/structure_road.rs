use crate::{
    game::pathfinder::Position,
    objects::{GameObject, Structure},
    HasPosition,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = StructureRoad)]
    pub static STRUCTURE_ROAD_PROTOTYPE: Object;

    /// An object representing a [`StructureRoad`], which allows creeps to move
    /// onto this position for half of the fatigue of moving onto a plains tile,
    /// as well as through terrain walls.
    #[wasm_bindgen(extends = GameObject, extends = Structure)]
    #[derive(Clone)]
    pub type StructureRoad;

}

impl HasPosition for StructureRoad {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
