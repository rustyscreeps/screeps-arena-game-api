mod arena;
mod construction_site;
mod creep;
mod game_object;
mod owned_structure;
mod resource;
mod source;
mod store;
mod structure;
mod structure_container;
mod structure_extension;
mod structure_rampart;
mod structure_road;
mod structure_spawn;
mod structure_tower;
mod structure_wall;

pub use self::{
    construction_site::{ConstructionSite, CONSTRUCTION_SITE_PROTOTYPE},
    creep::{Creep, CREEP_PROTOTYPE},
    game_object::{GameObject, GAME_OBJECT_PROTOTYPE},
    owned_structure::{OwnedStructure, OWNED_STRUCTURE_PROTOTYPE},
    resource::{Resource, RESOURCE_PROTOTYPE},
    source::{Source, SOURCE_PROTOTYPE},
    store::Store,
    structure::{Structure, STRUCTURE_PROTOTYPE},
    structure_container::{StructureContainer, STRUCTURE_CONTAINER_PROTOTYPE},
    structure_extension::{StructureExtension, STRUCTURE_EXTENSION_PROTOTYPE},
    structure_rampart::{StructureRampart, STRUCTURE_RAMPART_PROTOTYPE},
    structure_road::{StructureRoad, STRUCTURE_ROAD_PROTOTYPE},
    structure_spawn::{StructureSpawn, STRUCTURE_SPAWN_PROTOTYPE},
    structure_tower::{StructureTower, STRUCTURE_TOWER_PROTOTYPE},
    structure_wall::{StructureWall, STRUCTURE_WALL_PROTOTYPE},
};

#[cfg(feature = "enable-area-effect")]
pub use self::arena::{AreaEffect, AREA_EFFECT_PROTOTYPE};

#[cfg(feature = "enable-body-part")]
pub use self::arena::{BodyPart, BODY_PART_PROTOTYPE};

#[cfg(feature = "enable-bonus-flag")]
pub use self::arena::{BonusFlag, BONUS_FLAG_PROTOTYPE};

#[cfg(feature = "enable-flag")]
pub use self::arena::{Flag, FLAG_PROTOTYPE};

#[cfg(feature = "enable-score")]
pub use self::arena::{ScoreCollector, SCORE_COLLECTOR_PROTOTYPE};
