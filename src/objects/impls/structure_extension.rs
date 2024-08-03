use crate::{
    game::pathfinder::Position,
    objects::{GameObject, OwnedStructure, Store, Structure},
    prelude::*,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = StructureExtension)]
    pub static STRUCTURE_EXTENSION_PROTOTYPE: Object;

    /// An object representing a [`StructureExtension`], which can store energy
    /// to be used by spawns in the room to spawn creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtension)
    #[wasm_bindgen(extends = GameObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone)]
    pub type StructureExtension;

    /// The [`Store`] of the extension, which contains information about the
    /// amount of energy in it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtension.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureExtension) -> Store;
}

impl HasStore for StructureExtension {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl HasPosition for StructureExtension {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
