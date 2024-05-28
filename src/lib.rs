//! Typed bindings to the Screeps Arena API for WASM Rust AIs.
#![recursion_limit = "128"]
// to build locally with doc_cfg enabled, run:
// `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
#![cfg_attr(docsrs, feature(doc_cfg))]
// temporary workaround for https://github.com/rust-lang/rust-clippy/issues/12377
// fix not being in current stable rust 1.78; should be fixed in 1.79
#![allow(clippy::empty_docs)]

pub mod constants;
pub mod enums;
pub mod game;
pub mod objects;
pub mod traits;
// pub mod containers;

pub use crate::{constants::*, enums::*, objects::*, traits::*};

// /// Traits which implement base functionalities for Screeps types.
// ///
// /// # Example
// ///
// /// ```no_run
// /// use js_sys::{JsString, Reflect};
// /// use screeps::{prelude::*, Creep, game};
// ///
// /// let c = game::creeps().get(String::from("Bob")).unwrap();
// ///
// /// // `HasId` trait brought in from prelude
// /// let id = c.try_id().unwrap();
// /// ```
// ///
// /// This module contains all base functionality traits, and no structures.
pub mod prelude {
    pub use crate::traits::*;
}
