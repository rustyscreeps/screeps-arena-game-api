use crate::{constants::ReturnCode, objects::GameObject};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "/game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = ConstructionSite)]
    pub static CONSTRUCTION_SITE_PROTOTYPE: Object;

    /// A [`ConstructionSite`] which is an object representing a structure under
    /// construction.
    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type ConstructionSite;

    /// Whether it is your [`ConstructionSite`].
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &ConstructionSite) -> bool;

    /// The current construction progress.
    #[wasm_bindgen(method, getter)]
    pub fn progress(this: &ConstructionSite) -> u32;

    /// The total construction progress needed for the structure to be built.
    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(this: &ConstructionSite) -> u32;

    /// Remove this [`ConstructionSite`].
    #[wasm_bindgen(method)]
    pub fn remove(this: &ConstructionSite) -> ReturnCode;
}
