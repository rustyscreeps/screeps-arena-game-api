use crate::constants::ResourceType;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object that represents the cargo within an entity in the game world.
    #[wasm_bindgen]
    pub type Store;

    /// You can get specific resources from the store by addressing them as
    /// object properties
    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn get(this: &Store, ty: ResourceType) -> Option<u32>;

    /// Returns capacity of this store for the specified resource. For a general
    /// purpose store, it returns total capacity if resource is undefined.
    #[wasm_bindgen(method, js_name = getCapacity)]
    pub fn get_capacity(this: &Store, ty: Option<ResourceType>) -> u32;

    /// Returns free capacity for the store. For a limited store, it returns the
    /// capacity available for the specified resource if resource is defined and
    /// valid for this store.
    #[wasm_bindgen(method, js_name = getFreeCapacity)]
    pub fn get_free_capacity(this: &Store, ty: Option<ResourceType>) -> i32;

    /// Returns the capacity used by the specified resource. For a general
    /// purpose store, it returns total used capacity if resource is undefined.
    #[wasm_bindgen(method, js_name = getUsedCapacity)]
    pub fn get_used_capacity(this: &Store, ty: Option<ResourceType>) -> u32;
}

impl Store {
    pub fn store_types(&self) -> Vec<ResourceType> {
        Object::keys(self.unchecked_ref())
            .iter()
            .filter_map(|v| ResourceType::from_js_value(&v))
            .collect()
    }
}
