use crate::objects::*;
use js_sys::Object;
use wasm_bindgen::prelude::*;

pub trait PrototypeConstant {
    type Item: From<JsValue>;

    fn prototype(&self) -> &Object;
}

macro_rules! typesafe_prototype_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path);
        )*
    ) => (
        $(
            #[allow(bad_style)]
            $vis struct $constant_name;
            impl PrototypeConstant for $constant_name {
                type Item = $result;

                #[inline]
                fn prototype(&self) -> &Object {
                    &$value
                }
            }
        )*
    );
}

typesafe_prototype_constants! {
    pub struct CONSTRUCTION_SITE = (CONSTRUCTION_SITE_PROTOTYPE, ConstructionSite);
    pub struct CREEP = (CREEP_PROTOTYPE, Creep);
    pub struct RESOURCE = (RESOURCE_PROTOTYPE, Resource);
    pub struct SOURCE = (SOURCE_PROTOTYPE, Source);
    pub struct STRUCTURE = (STRUCTURE_PROTOTYPE, Structure);
    pub struct OWNED_STRUCTURE = (OWNED_STRUCTURE_PROTOTYPE, OwnedStructure);
    pub struct STRUCTURE_CONTAINER = (STRUCTURE_CONTAINER_PROTOTYPE, StructureContainer);
    pub struct STRUCTURE_EXTENSION = (STRUCTURE_EXTENSION_PROTOTYPE, StructureExtension);
    pub struct STRUCTURE_RAMPART = (STRUCTURE_RAMPART_PROTOTYPE, StructureRampart);
    pub struct STRUCTURE_ROAD = (STRUCTURE_ROAD_PROTOTYPE, StructureRoad);
    pub struct STRUCTURE_SPAWN = (STRUCTURE_SPAWN_PROTOTYPE, StructureSpawn);
    pub struct STRUCTURE_TOWER = (STRUCTURE_TOWER_PROTOTYPE, StructureTower);
    pub struct STRUCTURE_WALL = (STRUCTURE_WALL_PROTOTYPE, StructureWall);
}

#[cfg(feature = "enable-body-part")]
typesafe_prototype_constants! {
    pub struct BODY_PART = (BODY_PART_PROTOTYPE, BodyPart);
}

#[cfg(feature = "enable-bonus-flag")]
typesafe_prototype_constants! {
    pub struct BONUS_FLAG = (BONUS_FLAG_PROTOTYPE, BonusFlag);
}

#[cfg(feature = "enable-flag")]
typesafe_prototype_constants! {
    pub struct FLAG = (FLAG_PROTOTYPE, Flag);
}

#[cfg(feature = "enable-area-effect")]
typesafe_prototype_constants! {
    pub struct AREA_EFFECT = (AREA_EFFECT_PROTOTYPE, AreaEffect);
}

#[cfg(feature = "enable-score")]
typesafe_prototype_constants! {
    pub struct SCORE_COLLECTOR = (SCORE_COLLECTOR_PROTOTYPE, ScoreCollector);
}
