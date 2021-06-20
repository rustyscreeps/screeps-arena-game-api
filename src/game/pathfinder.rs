use js_sys::{Array, Object};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[wasm_bindgen(raw_module = "/game/path-finder")]
extern "C" {
    /// Find an optimal path between origin and goal. Note that searchPath without costMatrix specified (see below) use terrain data only.
    #[wasm_bindgen(js_name = searchPath)]
    pub fn search_path(
        origin: &Object,
        goal: &Object,
        options: Option<&SearchPathOptions>,
    ) -> SearchResults;
}

#[wasm_bindgen(raw_module = "/game/path-finder")]
extern "C" {
    #[wasm_bindgen]
    pub type CostMatrix;

    /// Creates a new CostMatrix containing 0's for all positions. searchPath use terrain cost for positions with 0 cost
    #[wasm_bindgen(constructor)]
    pub fn new() -> CostMatrix;

    /// Copy this CostMatrix into a new CostMatrix with the same data.
    #[wasm_bindgen(method)]
    pub fn clone(this: &CostMatrix) -> CostMatrix;

    /// Set the cost of a position in this CostMatrix.
    #[wasm_bindgen(method)]
    pub fn set(this: &CostMatrix, x: u8, y: u8, cost: u8); 

    /// Get the cost of a position in this CostMatrix.
    #[wasm_bindgen(method)]
    pub fn get(this: &CostMatrix, x: u8, y: u8) -> u8;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type SearchPathOptions;

    /// CostMatrix (Container for custom navigation cost data)
    #[wasm_bindgen(method, setter = costMatrix)]
    pub fn cost_matrix(this: &SearchPathOptions, cost_matrix: &CostMatrix);

    /// Cost for walking on plain positions. The default is 1
    #[wasm_bindgen(method, setter = plainCost)]
    pub fn plain_cost(this: &SearchPathOptions, cost: u8);

    /// Cost for walking on swamp positions. The default is 5
    #[wasm_bindgen(method, setter = swampCost)]
    pub fn swamp_cost(this: &SearchPathOptions, cost: u8);

    /// Instead of searching for a path to the goals this will search for a path away from the goals. The cheapest path that is out of range of every goal will be returned. The default is false
    #[wasm_bindgen(method, setter = flee)]
    pub fn flee(this: &SearchPathOptions, val: bool);

    /// number (The maximum allowed pathfinding operations. The default value is 50000)
    #[wasm_bindgen(method, setter = maxOps)]
    pub fn max_ops(this: &SearchPathOptions, ops: u32);

    /// The maximum allowed cost of the path returned. The default is Infinity.
    #[wasm_bindgen(method, setter = maxCost)]
    pub fn max_cost(this: &SearchPathOptions, cost: f64);

    /// Weight from 1 to 9 to apply to the heuristic in the A* formula F = G + weight * H. The default value is 1.2
    #[wasm_bindgen(method, setter = heuristicWeight)]
    pub fn heuristic_weight(this: &SearchPathOptions, weight: f64);
}

impl SearchPathOptions {
    pub fn new() -> SearchPathOptions {
        Object::new().unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type FindPathOptions;

    /// CostMatrix (Container for custom navigation cost data)
    #[wasm_bindgen(method, setter = costMatrix)]
    pub fn cost_matrix(this: &FindPathOptions, cost_matrix: &CostMatrix);

    /// Cost for walking on plain positions. The default is 1
    #[wasm_bindgen(method, setter = plainCost)]
    pub fn plain_cost(this: &FindPathOptions, cost: u8);

    /// Cost for walking on swamp positions. The default is 5
    #[wasm_bindgen(method, setter = swampCost)]
    pub fn swamp_cost(this: &FindPathOptions, cost: u8);

    /// Instead of searching for a path to the goals this will search for a path away from the goals. The cheapest path that is out of range of every goal will be returned. The default is false
    #[wasm_bindgen(method, setter = flee)]
    pub fn flee(this: &FindPathOptions, val: bool);

    /// number (The maximum allowed pathfinding operations. The default value is 50000)
    #[wasm_bindgen(method, setter = maxOps)]
    pub fn max_ops(this: &FindPathOptions, ops: u32);

    /// The maximum allowed cost of the path returned. The default is Infinity.
    #[wasm_bindgen(method, setter = maxCost)]
    pub fn max_cost(this: &FindPathOptions, cost: f64);

    /// Weight from 1 to 9 to apply to the heuristic in the A* formula F = G + weight * H. The default value is 1.2
    #[wasm_bindgen(method, setter = heuristicWeight)]
    pub fn heuristic_weight(this: &FindPathOptions, weight: f64);

    /// objects which should not be treated as obstacles during the search
    #[wasm_bindgen(method, setter = ignore)]
    pub fn ignore(this: &FindPathOptions, ignore: &Array);
}

impl FindPathOptions {
    pub fn new() -> FindPathOptions {
        Object::new().unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    /// An object representing the results of a [`PathFinder::search`].
    #[wasm_bindgen]
    pub type SearchResults;

    /// Get the path that was found, an [`Array`] of [`RoomPosition`]. May be
    /// incomplete.
    #[wasm_bindgen(method, getter, js_name = path)]
    fn path_internal(this: &SearchResults) -> Array;

    /// The number of operations the pathfinding operation performed.
    #[wasm_bindgen(method, getter)]
    pub fn ops(this: &SearchResults) -> u32;

    /// Total cost of all tiles used in the path
    #[wasm_bindgen(method, getter)]
    pub fn cost(this: &SearchResults) -> u32;

    /// Whether this search successfully found a complete path.
    #[wasm_bindgen(method, getter)]
    pub fn incomplete(this: &SearchResults) -> bool;
}

impl SearchResults {
    pub fn path (&self) -> Vec<Position> {
        self
            .path_internal()
            .iter()
            .map(|p| serde_wasm_bindgen::from_value(p).expect("expected valid Position in path results"))
            .collect()
    }
}
