use crate::{
    game::pathfinder::{FindPathOptions, Position},
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
    #[wasm_bindgen(method, getter, js_name = "id")]
    pub fn id_internal(this: &GameObject) -> JsValue;

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
    ) -> Array;

    #[wasm_bindgen(method, js_name = findInRange)]
    fn find_in_range_internal(this: &GameObject, positions: &Array, range: u8) -> Array;

    #[wasm_bindgen(method, js_name = findClosestByRange)]
    fn find_closest_by_range_internal(this: &GameObject, positions: &Array) -> Option<Object>;

    #[wasm_bindgen(method, js_name = findClosestByPath)]
    fn find_closest_by_path_internal(
        this: &GameObject,
        positions: &Array,
        options: Option<&FindPathOptions>,
    ) -> Option<Object>;

    #[wasm_bindgen(method, js_name = getRangeTo)]
    pub fn get_range_to(this: &GameObject, pos: &Object) -> u8;
}

impl GameObject {
    /* Although Creep.id is documented as a string, in practice it is sometimes
     * of type number.
     * This seems to happen in swamp & spawn but not in ctf.
     * This function returns a JsString always, converting if necessary.
     */
    pub fn id(&self) -> JsString {
        let i = self.id_internal();
        let as_str = i.dyn_into::<JsString>();
        match as_str {
            Ok(s) => s,
            Err(i) => {
                let as_f64 = i.as_f64();
                match as_f64 {
                    Some(f) => JsString::from(format!("{f}")),
                    None => i.unchecked_into::<JsString>(), // this will probably crash
                }
            }
        }
    }

    /// Find all objects in the specified linear range.
    pub fn find_in_range<T>(&self, targets: &[T], range: u8) -> Vec<T>
    where
        T: HasPosition + JsCast,
    {
        let array = targets.iter().map(|t| t.as_ref()).collect();
        let result = self.find_in_range_internal(&array, range);
        result
            .into_iter()
            .filter_map(|js_val| js_val.dyn_into().ok())
            .collect()
    }

    /// Find a position with the shortest linear distance from the given position.
    pub fn find_closest_by_range<T>(&self, targets: &[T]) -> Option<T>
    where
        T: HasPosition + JsCast,
    {
        let array = targets.iter().map(|t| t.as_ref()).collect();
        let result = self.find_closest_by_range_internal(&array)?;
        result.dyn_into().ok()
    }

    /// Find a position with the shortest path from the given position.
    pub fn find_closest_by_path<T>(
        &self,
        targets: &[T],
        options: Option<&FindPathOptions>,
    ) -> Option<T>
    where
        T: HasPosition + JsCast,
    {
        let array = targets.iter().map(|t| t.as_ref()).collect();
        let result = self.find_closest_by_path_internal(&array, options)?;
        result.dyn_into().ok()
    }
}

impl HasPosition for GameObject {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
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

    fn find_path_to(&self, pos: &Object, options: Option<&FindPathOptions>) -> Array {
        GameObject::find_path_to(self.as_ref(), pos, options)
    }

    fn find_in_range<U>(&self, positions: &[U], range: u8) -> Vec<U>
    where
        U: HasPosition + JsCast,
    {
        GameObject::find_in_range(self.as_ref(), positions, range)
    }

    fn find_closest_by_range<U>(&self, positions: &[U]) -> Option<U>
    where
        U: HasPosition + JsCast,
    {
        GameObject::find_closest_by_range(self.as_ref(), positions)
    }

    fn find_closest_by_path<U>(
        &self,
        positions: &[U],
        options: Option<&FindPathOptions>,
    ) -> Option<U>
    where
        U: HasPosition + JsCast,
    {
        GameObject::find_closest_by_path(self.as_ref(), positions, options)
    }

    fn get_range_to(&self, pos: &Object) -> u8 {
        GameObject::get_range_to(self.as_ref(), pos)
    }
}
