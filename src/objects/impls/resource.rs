use crate::{
    constants::ResourceType, game::pathfinder::Position, objects::GameObject, HasPosition,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = Resource)]
    pub static RESOURCE_PROTOTYPE: Object;

    /// A [`Resource`] is an object representing resources that have been
    /// dropped and can be picked up.
    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type Resource;

    /// The amount of dropped resource.
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Resource) -> u32;

    /// One of the RESOURCE_* constants.
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &Resource) -> ResourceType;
}

impl HasPosition for Resource {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
