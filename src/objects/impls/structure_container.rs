use crate::{
    objects::{GameObject, Store, Structure},
    prelude::*,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "/game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = StructureContainer)]
    pub static STRUCTURE_CONTAINER_PROTOTYPE: Object;

    /// An object representing a [`StructureContainer`], which can store
    /// resources and does not block creep movement.
    #[wasm_bindgen(extends = GameObject, extends = Structure)]
    #[derive(Clone)]
    pub type StructureContainer;

    /// The [`Store`] of the container, which contains information about what
    /// resources it is it holding.
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureContainer) -> Store;

}

impl HasStore for StructureContainer {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
