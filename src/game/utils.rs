use wasm_bindgen::prelude::*;

use js_sys::{Array, JsString, Object};

use crate::{
    objects::{GameObject, ConstructionSite},
    constants::{Direction, Terrain, ReturnCode},
    game::pathfinder::{FindPathOptions, SearchResults},
};

#[wasm_bindgen(raw_module = "/game/utils")]
extern "C" {
    /// Get count of game ticks passed since the start of the game
    #[wasm_bindgen(js_name = getTicks)]
    pub fn get_ticks() -> u32;

    /// Get CPU wall time elapsed in the current tick in nanoseconds.
    #[wasm_bindgen(js_name = getCpuTime)]
    pub fn get_cpu_time() -> u32;

    /// Get an object with the specified unique ID.
    #[wasm_bindgen(js_name = getObjectById)]
    pub fn get_object_by_id(id: &JsString) -> Option<GameObject>;

    /// Get all objects in the game.
    #[wasm_bindgen(js_name = getObjects)]
    pub fn get_objects() -> Array;

    /// Get all objects in the game with the specified prototype, for example, all creeps
    #[wasm_bindgen(js_name = getObjectsByPrototype)]
    pub fn get_objects_by_prototype(prototype: &Object) -> Array;

    /// Use this method to get heap statistics for your virtual machine
    #[wasm_bindgen(js_name = getHeapStatistics)]
    pub fn get_heap_statistics() -> HeapStatistics;

    /// Get linear direction by differences of x and y
    #[wasm_bindgen(js_name = getDirection)]
    pub fn get_direction(dx: u8, dy: u8) -> Direction;

    /// Find an optimal path between fromPos and toPos. Unlike searchPath,
    /// findPath avoid all obstacles by default (unless costMatrix is specified).
    #[wasm_bindgen(js_name = findPath)]
    pub fn find_path(from_pos: &Object, to_pos: &Object, options: Option<&FindPathOptions>) -> SearchResults;

    /// Get linear range between two objects. a and b may be any object containing x and y properties.
    #[wasm_bindgen(js_name = getRange)]
    pub fn get_range(a: &Object, b: &Object) -> u8;

    /// Get an integer representation of the terrain at the given position. pos should be an object containing x and y properties. Returns TERRAIN_WALL, TERRAIN_SWAMP, or 0.
    #[wasm_bindgen(js_name = getTerrainAt)]
    pub fn get_terrain_at(pos: &Object) -> Terrain;

    /// Find all positions from the given positions array within the specified linear range.
    #[wasm_bindgen(js_name = findInRange)]
    pub fn find_in_range(from_pos: &Object, positions: &Array, range: u8) -> Array;

    /// Find a position with the shortest linear distance from the given position, or null otherwise.
    #[wasm_bindgen(js_name = findClosestByRange)]
    pub fn find_closest_by_range(from_pos: &Object, positions: &Array) -> Option<Object>;

    /// Find a position with the shortest path from the given position, or null otherwise.
    #[wasm_bindgen(js_name = findClosestByPath)]
    pub fn find_closest_by_path(from_pos: &Object, positions: &Array, options: Option<&FindPathOptions>) -> Option<Object>;

    /// Create a new construction site at the specified location. Returns the ConstructionSite object instance. You can create maximum 10 active construction sites.
    #[wasm_bindgen(js_name = createConstructionSite)]
    fn create_construction_site_internal(x: u8, y: u8, structure_type: &Object) -> CreateConstructionSiteResult;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    type CreateConstructionSiteResult;

    #[wasm_bindgen(method, getter)]
    fn object(this: &CreateConstructionSiteResult) -> Option<ConstructionSite>;


    #[wasm_bindgen(method, getter)]
    fn error(this: &CreateConstructionSiteResult) -> ReturnCode;
}

pub fn create_construction_site(x: u8, y: u8, structure_type: &Object) -> Result<ConstructionSite, ReturnCode> {
    let r = create_construction_site_internal(x, y, structure_type);

    match r.object() {
        Some(cs) => Ok(cs),
        None => Err(r.error()),
    }
}

#[wasm_bindgen]
extern "C" {
    /// Object with info about the memory heap of your virtual machine.
    #[wasm_bindgen]
    pub type HeapStatistics;

    /// The total heap consumed.
    #[wasm_bindgen(method, getter)]
    pub fn total_heap_size(this: &HeapStatistics) -> u32;

    /// The total heap consumed by executable code.
    #[wasm_bindgen(method, getter)]
    pub fn total_heap_size_executable(this: &HeapStatistics) -> u32;

    /// The total amount of heap committed to memory.
    #[wasm_bindgen(method, getter)]
    pub fn total_physical_size(this: &HeapStatistics) -> u32;

    /// Amount of heap available for allocation.
    #[wasm_bindgen(method, getter)]
    pub fn total_available_size(this: &HeapStatistics) -> u32;

    /// Total heap consumed by application data.
    #[wasm_bindgen(method, getter)]
    pub fn used_heap_size(this: &HeapStatistics) -> u32;

    /// The allowed limit for total heap memory.
    #[wasm_bindgen(method, getter)]
    pub fn heap_size_limit(this: &HeapStatistics) -> u32;

    /// Total amount of memory obtained by malloc.
    #[wasm_bindgen(method, getter)]
    pub fn malloced_memory(this: &HeapStatistics) -> u32;

    /// Maximum amount of memory obtained by malloc.
    #[wasm_bindgen(method, getter)]
    pub fn peak_malloced_memory(this: &HeapStatistics) -> u32;

    /// Whether the virtual machine overwrites memory as it deallocates -
    /// usually 0.
    #[wasm_bindgen(method, getter)]
    pub fn does_zap_garbage(this: &HeapStatistics) -> u32;

    /// External allocations that are outside of the v8 heap but still count
    /// against the memory limit.
    #[wasm_bindgen(method, getter)]
    pub fn externally_allocated_size(this: &HeapStatistics) -> u32;
}