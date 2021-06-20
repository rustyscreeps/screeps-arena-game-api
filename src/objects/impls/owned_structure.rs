use crate::{
    objects::{GameObject, Structure},
    prelude::*
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "/game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = OwnedStructure)]
    pub static OWNED_STRUCTURE_PROTOTYPE: Object;

    /// Parent class for all [`Structure`] objects types which are (or can be)
    /// owned by a specific player.
    #[wasm_bindgen(extends = GameObject, extends = Structure)]
    #[derive(Clone)]
    pub type OwnedStructure;

    /// Returns `Some(true)` for your structure, `Some(false)` for a hostile structure, `None` for a neutral structure.
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &OwnedStructure) -> Option<bool>;
}

impl<T> OwnedStructureProperties for T where T: AsRef<OwnedStructure> {
    fn my(&self) -> Option<bool> {
        OwnedStructure::my(self.as_ref())
    }
}
