use crate::{
    objects::GameObject,
    prelude::*,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = Structure)]
    pub static STRUCTURE_PROTOTYPE: Object;

    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type Structure;

    /// The current amount of hit points of the structure.
    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &Structure) -> u32;

    /// The maximum amount of hit points of the structure.
    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hits_max(this: &Structure) -> u32;
}

impl<T> HasHits for T where T: AsRef<Structure> {
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}
