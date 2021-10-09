use crate::{
    constants::{Part, ReturnCode},
    objects::{Creep, GameObject, OwnedStructure, Store, Structure},
    prelude::*,
};
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = StructureSpawn)]
    pub static STRUCTURE_SPAWN_PROTOTYPE: Object;

    /// An object representing a [`StructureSpawn`], which creates your creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn)
    #[wasm_bindgen(extends = GameObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone)]
    pub type StructureSpawn;

    /// The [`Store`] of the spawn, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureSpawn) -> Store;

    /// Create a new creep with the specified body part [`Array`], name
    /// [`JsString`], and optional spawning options. Note that successfully
    /// spawning will store data in `Memory.creeps[creep_name]` _regardless
    /// of whether any memory data was passed in the options object_ and enable
    /// the default serialization behavior of the `Memory` object, which may
    /// hamper attempts to directly use `RawMemory`. todo, add note+docs
    /// about how to replace Memory and/or delete RawMemory._parsed
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.spawnCreep)
    #[wasm_bindgen(method, js_name = spawnCreep)]
    fn spawn_creep_internal(this: &StructureSpawn, body: &Array) -> SpawnCreepResult;
}

impl StructureSpawn {
    pub fn spawn_creep(&self, body: &[Part]) -> Result<Creep, ReturnCode> {
        let body = body.iter().cloned().map(JsValue::from).collect();
        let r = Self::spawn_creep_internal(self, &body);

        match r.object() {
            Some(c) => Ok(c),
            None => Err(r.error()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    type SpawnCreepResult;

    #[wasm_bindgen(method, getter)]
    fn object(this: &SpawnCreepResult) -> Option<Creep>;

    #[wasm_bindgen(method, getter)]
    fn error(this: &SpawnCreepResult) -> ReturnCode;
}

impl HasStore for StructureSpawn {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
