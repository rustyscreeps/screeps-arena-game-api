//! Typed bindings to the Screeps Arena API for WASM Rust AIs.
#![recursion_limit = "128"]
// to build locally with doc_cfg enabled, run:
// `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
#![cfg_attr(docsrs, feature(doc_cfg))]
// disable deprecation warnings - TODO need to figure out how to get wasm_bindgen's new thread_local
// attribute working
#![allow(deprecated)]

pub mod constants;
pub mod enums;
pub mod game;
pub mod objects;
pub mod traits;

pub use crate::{constants::*, enums::*, objects::*, traits::*};

pub mod prelude {
    pub use crate::{game::pathfinder::Position, traits::*};
}
