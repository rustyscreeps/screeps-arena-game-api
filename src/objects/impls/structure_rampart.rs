use crate::{
    objects::{OwnedStructure, GameObject, Structure},
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "/game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = StructureRampart)]
    pub static STRUCTURE_RAMPART_PROTOTYPE: Object;

    /// An object representing a [`StructureRampart`], which is selectively
    /// walkable and protects creeps and structures at the same position.
    #[wasm_bindgen(extends = GameObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone)]
    pub type StructureRampart;
}
