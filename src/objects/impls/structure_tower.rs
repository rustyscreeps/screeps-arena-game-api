use crate::{
    constants::ReturnCode,
    game::pathfinder::Position,
    objects::{Creep, GameObject, OwnedStructure, Store, Structure},
    prelude::*,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = StructureTower)]
    pub static STRUCTURE_TOWER_PROTOTYPE: Object;

    #[wasm_bindgen(extends = GameObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone)]
    pub type StructureTower;

    /// A Store object that contains cargo of this structure.
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureTower) -> Store;

    /// The remaining amount of ticks while this tower cannot be used.
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureTower) -> u32;

    /// Remotely attack any creep or structure in range.
    #[wasm_bindgen(method, js_name = attack)]
    fn attack_internal(this: &StructureTower, target: &GameObject) -> ReturnCode;

    /// Remotely heal any creep in range.
    #[wasm_bindgen(method, js_name = heal)]
    pub fn heal(this: &StructureTower, target: &Creep) -> ReturnCode;

    // todo unsure if this ends up existing?
    // #[wasm_bindgen(method, js_name = repair)]
    // pub fn repair(this: &StructureTower, target: &Structure) -> ReturnCode;
}

impl HasCooldown for StructureTower {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl StructureTower {
    pub fn attack<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Attackable,
    {
        Self::attack_internal(self, target.as_ref())
    }
}

impl HasStore for StructureTower {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl HasPosition for StructureTower {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
