//! `*Type` constants.

// use std::{borrow::Cow, str::FromStr};
use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// /// Translates `STRUCTURE_*` constants.
// #[wasm_bindgen]
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
// pub enum StructureType {
//     Spawn = "spawn",
//     Extension = "extension",
//     Road = "road",
//     Wall = "constructedWall",
//     Rampart = "rampart",
//     KeeperLair = "keeperLair",
//     Portal = "portal",
//     Controller = "controller",
//     Link = "link",
//     Storage = "storage",
//     Tower = "tower",
//     Observer = "observer",
//     PowerBank = "powerBank",
//     PowerSpawn = "powerSpawn",
//     Extractor = "extractor",
//     Lab = "lab",
//     Terminal = "terminal",
//     Container = "container",
//     Nuker = "nuker",
//     Factory = "factory",
//     InvaderCore = "invaderCore",
// }

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

//     /// Translates the `CONTROLLER_STRUCTURES` constant
//     #[inline]
//     pub fn controller_structures(self, current_rcl: u32) -> u32 {
//         use self::StructureType::*;

//         match self {
//             Spawn => match current_rcl {
//                 0 => 0,
//                 1..=6 => 1,
//                 7 => 2,
//                 _ => 3,
//             },
//             Extension => match current_rcl {
//                 0 | 1 => 0,
//                 2 => 5,
//                 3 => 10,
//                 4 => 20,
//                 5 => 30,
//                 6 => 40,
//                 7 => 50,
//                 _ => 60,
//             },
//             Road => 2500,
//             Wall => match current_rcl {
//                 0 | 1 => 0,
//                 _ => 2500,
//             },
//             Rampart => match current_rcl {
//                 0 | 1 => 0,
//                 _ => 2500,
//             },
//             Link => match current_rcl {
//                 0..=4 => 0,
//                 5 => 2,
//                 6 => 3,
//                 7 => 4,
//                 _ => 6,
//             },
//             Storage => match current_rcl {
//                 0..=3 => 0,
//                 _ => 1,
//             },
//             Tower => match current_rcl {
//                 0 | 1 | 2 => 0,
//                 3 | 4 => 1,
//                 5 | 6 => 2,
//                 7 => 3,
//                 _ => 6,
//             },
//             Observer => match current_rcl {
//                 0..=7 => 0,
//                 _ => 1,
//             },
//             PowerSpawn => match current_rcl {
//                 0..=7 => 0,
//                 _ => 1,
//             },
//             Extractor => match current_rcl {
//                 0..=5 => 0,
//                 _ => 1,
//             },
//             Lab => match current_rcl {
//                 0..=5 => 0,
//                 6 => 3,
//                 7 => 6,
//                 _ => 10,
//             },
//             Terminal => match current_rcl {
//                 0..=5 => 0,
//                 _ => 1,
//             },
//             Container => 5,
//             Nuker => match current_rcl {
//                 0..=7 => 0,
//                 _ => 1,
//             },
//             Factory => match current_rcl {
//                 0..=6 => 0,
//                 _ => 1,
//             },
//             _ => 0,
//         }
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

// /// Translates `SUBSCRIPTION_TOKEN` and `INTERSHARD_RESOURCES` constants.
// #[wasm_bindgen]
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
// pub enum IntershardResourceType {
//     // no longer used, not implemented
//     // SubscriptionToken = "token",
//     CpuUnlock = "cpuUnlock",
//     Pixel = "pixel",
//     AccessKey = "accessKey",
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
