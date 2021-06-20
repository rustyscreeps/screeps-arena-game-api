mod arena;
mod construction_site;
mod creep;
mod owned_structure;
mod resource;
mod game_object;
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
    arena::*,
    construction_site::ConstructionSite,
    creep::{Creep, CREEP_PROTOTYPE},
    owned_structure::{OwnedStructure, OWNED_STRUCTURE_PROTOTYPE},
    resource::{Resource, RESOURCE_PROTOTYPE},
    game_object::{GameObject, GAME_OBJECT_PROTOTYPE},
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
