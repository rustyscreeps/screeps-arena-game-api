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

#[wasm_bindgen(raw_module = "/game")]
extern "C" {
    #[wasm_bindgen(js_name = arenaInfo)]
    static ARENA_INFO: JsValue;
}

pub fn arena_info() -> ArenaInfo {
    serde_wasm_bindgen::from_value(ARENA_INFO.clone()).expect("expected valid arenaInfo object")
}

#[derive(Deserialize, Debug)]
pub struct ArenaInfo {
    pub name: String,
    pub level: u8,
    pub season: String,
}
