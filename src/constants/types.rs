//! `*Type` constants.

// use std::{borrow::Cow, str::FromStr};
use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// impl StructureType {
//     /// Translates the `CONSTRUCTION_COST` constant.
//     #[inline]
//     pub fn construction_cost(self) -> Option<u32> {
//         use self::StructureType::*;

//         let cost = match self {
//             Spawn => 15_000,
//             Extension => 3_000,
//             Road => 300,
//             Wall => 1,
//             Rampart => 1,
//             Link => 5_000,
//             Storage => 30_000,
//             Tower => 5_000,
//             Observer => 8_000,
//             PowerSpawn => 100_000,
//             Extractor => 5_000,
//             Lab => 50_000,
//             Terminal => 100_000,
//             Container => 5_000,
//             Nuker => 100_000,
//             Factory => 100_000,
//             _ => return None,
//         };
//         Some(cost)
//     }

//     /// Translates the `*_HITS` constants, initial hits for structures
//     #[inline]
//     pub fn initial_hits(self) -> Option<u32> {
//         use self::StructureType::*;
//         use super::numbers::*;

//         let hits = match self {
//             Spawn => SPAWN_HITS,
//             Extension => EXTENSION_HITS,
//             Road => ROAD_HITS,
//             Wall => WALL_HITS,
//             Rampart => RAMPART_HITS,
//             Link => LINK_HITS,
//             Storage => STORAGE_HITS,
//             Tower => TOWER_HITS,
//             Observer => OBSERVER_HITS,
//             PowerBank => POWER_BANK_HITS,
//             PowerSpawn => POWER_SPAWN_HITS,
//             Extractor => EXTENSION_HITS,
//             Lab => LAB_HITS,
//             Terminal => TERMINAL_HITS,
//             Container => CONTAINER_HITS,
//             Nuker => NUKER_HITS,
//             Factory => FACTORY_HITS,
//             InvaderCore => INVADER_CORE_HITS,
//             _ => return None,
//         };
//         Some(hits)
//     }
// }

/// Resource type constant for all possible types of resources.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum ResourceType {
    Energy = "energy",
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    Score = "score",
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreX = "score_x",
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreY = "score_y",
    #[cfg(feature = "enable-score")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreZ = "score_z",
}

#[cfg(feature = "enable-area-effect")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-area-effect")))]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
pub enum EffectType {
    Freeze = "freeze",
    Damage = "damage",
}
