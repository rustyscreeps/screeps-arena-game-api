use enum_dispatch::enum_dispatch;

use crate::{objects::*, prelude::*};

#[enum_dispatch(Attackable)]
pub enum AttackableObject {
    Creep,
    StructureContainer,
    StructureExtension,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureTower,
    StructureWall,
}

impl From<AttackableObject> for GameObject {
    fn from(attackable: AttackableObject) -> Self {
        use AttackableObject::*;

        match attackable {
            Creep(o) => GameObject::from(o),
            StructureContainer(o) => GameObject::from(o),
            StructureExtension(o) => GameObject::from(o),
            StructureRampart(o) => GameObject::from(o),
            StructureRoad(o) => GameObject::from(o),
            StructureSpawn(o) => GameObject::from(o),
            StructureTower(o) => GameObject::from(o),
            StructureWall(o) => GameObject::from(o),
        }
    }
}

impl AsRef<GameObject> for AttackableObject {
    fn as_ref(&self) -> &GameObject {
        use AttackableObject::*;

        match self {
            Creep(o) => o.as_ref(),
            StructureContainer(o) => o.as_ref(),
            StructureExtension(o) => o.as_ref(),
            StructureRampart(o) => o.as_ref(),
            StructureRoad(o) => o.as_ref(),
            StructureSpawn(o) => o.as_ref(),
            StructureTower(o) => o.as_ref(),
            StructureWall(o) => o.as_ref(),
        }
    }
}

// #[enum_dispatch(HasCooldown)]
// pub enum CooldownObject {
//     Deposit,
//     StructureExtractor,
//     StructureFactory,
//     StructureLab,
//     StructureLink,
//     StructureNuker,
//     StructureTerminal,
// }

// #[enum_dispatch(HasId)]
// pub enum ObjectWithId {
//     Deposit,
//     Mineral,
//     Nuke,
//     Resource,
//     Ruin,
//     #[cfg(feature = "enable-score")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
//     ScoreCollector,
//     #[cfg(feature = "enable-score")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
//     ScoreContainer,
//     Source,
//     StructureContainer,
//     StructureController,
//     StructureExtension,
//     StructureExtractor,
//     StructureFactory,
//     StructureInvaderCore,
//     StructureKeeperLair,
//     StructureLab,
//     StructureLink,
//     StructureNuker,
//     StructureObserver,
//     StructurePortal,
//     StructurePowerBank,
//     StructurePowerSpawn,
//     StructureRampart,
//     StructureRoad,
//     StructureSpawn,
//     StructureStorage,
//     StructureTerminal,
//     StructureTower,
//     StructureWall,
//     #[cfg(feature = "enable-symbols")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
//     SymbolContainer,
//     #[cfg(feature = "enable-symbols")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
//     SymbolDecoder,
//     Tombstone,
// }

// #[enum_dispatch(MaybeHasId)]
// pub enum ObjectWithMaybeId {
//     ConstructionSite,
//     Creep,
//     Deposit,
//     Mineral,
//     Nuke,
//     PowerCreep,
//     Resource,
//     Ruin,
//     #[cfg(feature = "enable-score")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
//     ScoreCollector,
//     #[cfg(feature = "enable-score")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
//     ScoreContainer,
//     Source,
//     StructureContainer,
//     StructureController,
//     StructureExtension,
//     StructureExtractor,
//     StructureFactory,
//     StructureInvaderCore,
//     StructureKeeperLair,
//     StructureLab,
//     StructureLink,
//     StructureNuker,
//     StructureObserver,
//     StructurePortal,
//     StructurePowerBank,
//     StructurePowerSpawn,
//     StructureRampart,
//     StructureRoad,
//     StructureSpawn,
//     StructureStorage,
//     StructureTerminal,
//     StructureTower,
//     StructureWall,
//     #[cfg(feature = "enable-symbols")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
//     SymbolContainer,
//     #[cfg(feature = "enable-symbols")]
//     #[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
//     SymbolDecoder,
//     Tombstone,
// }

#[enum_dispatch(HasStore)]
pub enum StoreObject {
    Creep,
    StructureContainer,
    StructureExtension,
    StructureSpawn,
    StructureTower,
}

/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(OwnedStructureProperties)]
pub enum OwnedStructureObject {
    StructureExtension,
    StructureRampart,
    StructureSpawn,
    StructureTower,
}

// todo TryFrom<Structure> for OwnedStructureObject

/// Any enum representing any game object that inherits the [`GameObject`] type.
#[enum_dispatch(GameObjectProperties)]
pub enum TypedGameObject {
    #[cfg(feature = "enable-body-part")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-body-part")))]
    BodyPart,
    #[cfg(feature = "enable-bonus-flag")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-bonus-flag")))]
    BonusFlag,
    ConstructionSite,
    Creep,
    #[cfg(feature = "enable-flag")]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-flag")))]
    Flag,
    Resource,
    Source,
    StructureContainer,
    StructureExtension,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureTower,
    StructureWall,
}

// todo there's no StructureType, hrmmm
/// Enum used for converting a [`Structure`] into a typed object of its specific
/// structure type.
#[enum_dispatch(StructureProperties)]
#[derive(Clone)]
pub enum StructureObject {
    StructureContainer,
    StructureExtension,
    StructureRampart,
    StructureRoad,
    StructureSpawn,
    StructureTower,
    StructureWall,
}

// impl From<JsValue> for StructureObject {
//     fn from(reference: JsValue) -> Self {
//         let structure: Structure = reference.unchecked_into();

//         structure.into()
//     }
// }

// todo should implement using checking of the object's prototype, need to work
// out how to do that efficiently impl From<Structure> for StructureObject {
//     fn from(structure: Structure) -> Self {
//         use crate::constants::StructureType::*;

//         match structure.structure_type() {
//             Container =>
// Self::StructureContainer(structure.unchecked_into()),             Extension
// => Self::StructureExtension(structure.unchecked_into()),             Rampart
// => Self::StructureRampart(structure.unchecked_into()),             Road =>
// Self::StructureRoad(structure.unchecked_into()),             Spawn =>
// Self::StructureSpawn(structure.unchecked_into()),             Tower =>
// Self::StructureTower(structure.unchecked_into()),             Wall =>
// Self::StructureWall(structure.unchecked_into()),             _ =>
// panic!("unknown structure type for conversion into enum"),         }
//     }
// }

impl StructureObject {
    pub fn as_structure(&self) -> &Structure {
        match self {
            Self::StructureContainer(s) => s.as_ref(),
            Self::StructureExtension(s) => s.as_ref(),
            Self::StructureRampart(s) => s.as_ref(),
            Self::StructureRoad(s) => s.as_ref(),
            Self::StructureSpawn(s) => s.as_ref(),
            Self::StructureTower(s) => s.as_ref(),
            Self::StructureWall(s) => s.as_ref(),
        }
    }

    pub fn as_owned(&self) -> Option<&dyn OwnedStructureProperties> {
        match self {
            Self::StructureContainer(_) => None,
            Self::StructureExtension(s) => Some(s),
            Self::StructureRampart(s) => Some(s),
            Self::StructureRoad(_) => None,
            Self::StructureSpawn(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureWall(_) => None,
        }
    }

    pub fn as_has_store(&self) -> Option<&dyn HasStore> {
        match self {
            Self::StructureContainer(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRampart(_) => None,
            Self::StructureRoad(_) => None,
            Self::StructureSpawn(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureWall(_) => None,
        }
    }

    pub fn as_transferable(&self) -> Option<&dyn Transferable> {
        match self {
            Self::StructureContainer(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRampart(_) => None,
            Self::StructureRoad(_) => None,
            Self::StructureSpawn(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureWall(_) => None,
        }
    }

    pub fn as_withdrawable(&self) -> Option<&dyn Withdrawable> {
        match self {
            Self::StructureContainer(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRampart(_) => None,
            Self::StructureRoad(_) => None,
            Self::StructureSpawn(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureWall(_) => None,
        }
    }

    pub fn as_attackable(&self) -> Option<&dyn Attackable> {
        match self {
            Self::StructureContainer(s) => Some(s),
            Self::StructureExtension(s) => Some(s),
            Self::StructureRampart(s) => Some(s),
            Self::StructureRoad(s) => Some(s),
            Self::StructureSpawn(s) => Some(s),
            Self::StructureTower(s) => Some(s),
            Self::StructureWall(s) => Some(s),
        }
    }
}
