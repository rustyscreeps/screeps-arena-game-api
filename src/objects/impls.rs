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

#[cfg(all(feature = "enable-body-part", feature = "arena-basic"))]
pub use self::arena::basic::{BodyPart, BODY_PART_PROTOTYPE};

#[cfg(all(feature = "enable-flag", feature = "arena-basic"))]
pub use self::arena::basic::{Flag, FLAG_PROTOTYPE};

#[cfg(all(feature = "enable-area-effect", feature = "arena-basic"))]
pub use self::arena::basic::{AreaEffect, AREA_EFFECT_PROTOTYPE};

#[cfg(all(feature = "enable-score", feature = "arena-basic"))]
pub use self::arena::basic::{ScoreCollector, SCORE_COLLECTOR_PROTOTYPE};

#[cfg(all(feature = "enable-body-part", feature = "arena-advanced"))]
pub use self::arena::advanced::{BodyPart, BODY_PART_PROTOTYPE};

#[cfg(all(feature = "enable-flag", feature = "arena-advanced"))]
pub use self::arena::advanced::{Flag, FLAG_PROTOTYPE};

#[cfg(all(feature = "enable-area-effect", feature = "arena-advanced"))]
pub use self::arena::advanced::{AreaEffect, AREA_EFFECT_PROTOTYPE};

#[cfg(all(feature = "enable-score", feature = "arena-advanced"))]
pub use self::arena::advanced::{ScoreCollector, SCORE_COLLECTOR_PROTOTYPE};
