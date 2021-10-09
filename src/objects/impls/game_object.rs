use crate::{
    game::pathfinder::{FindPathOptions, SearchResults},
    prelude::*,
};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = GameObject)]
    pub static GAME_OBJECT_PROTOTYPE: Object;

    #[derive(Clone)]
    pub type GameObject;

    /// Returns true if this object is live in the game at the moment. Check
    /// this property to verify cached or newly created object instances.
    #[wasm_bindgen(method, getter)]
    pub fn exists(this: &GameObject) -> bool;

    /// The unique ID of this object that you can use in
    /// /game/utils::getObjectById
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &GameObject) -> JsString;

    /// The X coordinate in the room.
    #[wasm_bindgen(method, getter)]
    pub fn x(this: &GameObject) -> u8;

    /// The Y coordinate in the room.
    #[wasm_bindgen(method, getter)]
    pub fn y(this: &GameObject) -> u8;

    /// If defined, then this object will disappear after this number of ticks.
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &GameObject) -> Option<u32>;

    #[wasm_bindgen(method, js_name = findPathTo)]
    pub fn find_path_to(
        this: &GameObject,
        pos: &Object,
        options: Option<&FindPathOptions>,
    ) -> SearchResults;

    #[wasm_bindgen(method, js_name = findInRange)]
    pub fn find_in_range(this: &GameObject, positions: &Array, range: u8) -> Array;

    #[wasm_bindgen(method, js_name = findClosestByRange)]
    pub fn find_closest_by_range(this: &GameObject, positions: &Array) -> Option<Object>;

    #[wasm_bindgen(method, js_name = findClosestByPath)]
    pub fn find_closest_by_path(
        this: &GameObject,
        positions: &Array,
        options: Option<&FindPathOptions>,
    ) -> Option<Object>;

    #[wasm_bindgen(method, js_name = getRangeTo)]
    pub fn get_range_to(this: &GameObject, pos: &Object) -> u8;
}

impl<T> GameObjectProperties for T
where
    T: AsRef<GameObject>,
{
    fn exists(&self) -> bool {
        GameObject::exists(self.as_ref())
    }

    fn id(&self) -> JsString {
        GameObject::id(self.as_ref())
    }

    fn x(&self) -> u8 {
        GameObject::x(self.as_ref())
    }

    fn y(&self) -> u8 {
        GameObject::y(self.as_ref())
    }

    fn ticks_to_decay(&self) -> Option<u32> {
        GameObject::ticks_to_decay(self.as_ref())
    }

    fn find_path_to(&self, pos: &Object, options: Option<&FindPathOptions>) -> SearchResults {
        GameObject::find_path_to(self.as_ref(), pos, options)
    }

    fn find_in_range(&self, positions: &Array, range: u8) -> Array {
        GameObject::find_in_range(self.as_ref(), positions, range)
    }

    fn find_closest_by_range(&self, positions: &Array) -> Option<Object> {
        GameObject::find_closest_by_range(self.as_ref(), positions)
    }

    fn find_closest_by_path(
        &self,
        positions: &Array,
        options: Option<&FindPathOptions>,
    ) -> Option<Object> {
        GameObject::find_closest_by_path(self.as_ref(), positions, options)
    }

    fn get_range_to(&self, pos: &Object) -> u8 {
        GameObject::get_range_to(self.as_ref(), pos)
    }
}
