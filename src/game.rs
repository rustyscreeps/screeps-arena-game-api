//! The main interface to objects in the Screeps game world.
//!
//! This contains all functionality from the [`Game`] object in Screeps. That
//! generally means all state which is true this tick throughout the world.
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game)
use serde::Deserialize;
use wasm_bindgen::prelude::*;

pub mod utils;
pub mod pathfinder;

#[wasm_bindgen(module = "game")]
extern "C" {
    #[wasm_bindgen(js_name = arenaInfo)]
    static ARENA_INFO: JsValue;
}

pub fn arena_info() -> ArenaInfo {
    serde_wasm_bindgen::from_value(ARENA_INFO.clone()).expect("expected valid arenaInfo object")
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArenaInfo {
    /// The name of the arena.
    pub name: String,
    /// Currently equals to 1 for basic arena and 2 for advanced.
    pub level: u8,
    /// Currently equals to "alpha".
    pub season: String,
    /// Game ticks limit.
    pub ticks_limit: u32,
    /// CPU wall time execution limit per one tick (except the first tick)
    pub cpu_time_limit: u32,
    /// CPU wall time limit on the first tick.
    pub cpu_time_limit_first_tick: u32,
}
