use enum_dispatch::enum_dispatch;
use js_sys::{Array, JsString, Object};

use crate::{
    enums::*,
    game::pathfinder::{FindPathOptions, Position},
    objects::*,
};

#[enum_dispatch]
pub trait HasHits {
    /// Retrieve the current hits of this object.
    fn hits(&self) -> u32;

    /// Retrieve the maximum hits of this object.
    fn hits_max(&self) -> u32;
}

#[enum_dispatch]
pub trait HasCooldown {
    /// The number of ticks until the object can be used again.
    fn cooldown(&self) -> u32;
}

pub trait HasPosition {
    /// The position of the object
    fn pos(&self) -> Position;
    fn range_to(&self, has_pos: &impl HasPosition) -> u8 {
        let other = has_pos.pos();
        std::cmp::max(
            self.pos().x.abs_diff(other.x),
            self.pos().y.abs_diff(other.y),
        )
    }
}

// pub trait HasNativeId {
//     fn native_id(&self) -> JsString;
// }

// pub trait MaybeHasNativeId {
//     fn native_id(&self) -> Option<JsString>;
// }

// impl<T> MaybeHasNativeId for T where T: HasNativeId {
//     fn native_id(&self) -> Option<JsString> {
//         Some(<Self as HasNativeId>::native_id(&self))
//     }
// }

// pub trait Resolvable: From<JsValue> {}

// impl<T> Resolvable for T where T: MaybeHasTypedId<T> + From<JsValue> {}

// #[enum_dispatch]
// pub trait HasId {
//     /// Object ID of the object, which can be used to efficiently fetch a
//     /// fresh reference to the object on subsequent ticks.
//     fn raw_id(&self) -> RawObjectId;
// }

// impl<T> HasId for T where T: HasNativeId {
//     fn raw_id(&self) -> RawObjectId {
//         let id: String = self.native_id().into();

//         RawObjectId::from_str(&id).expect("expected object ID to be
// parseable")     }
// }

// #[enum_dispatch]
// pub trait HasTypedId<T> {
//     /// Object ID of the object, which can be used to efficiently fetch a
//     /// fresh reference to the object on subsequent ticks.
//     fn id(&self) -> ObjectId<T>;
// }

// impl<T> HasTypedId<T> for T where T: HasId {
//     fn id(&self) -> ObjectId<T> {
//         self.raw_id().into()
//     }
// }

// impl<T> HasTypedId<T> for &T where T: HasId {
//     fn id(&self) -> ObjectId<T> {
//         self.raw_id().into()
//     }
// }

// #[enum_dispatch]
// pub trait MaybeHasId {
//     /// Object ID of the object, which can be used to efficiently fetch a
//     /// fresh reference to the object on subsequent ticks, or `None` if the
//     /// object doesn't currently have an id.
//     fn try_raw_id(&self) -> Option<RawObjectId>;
// }

// impl<T> MaybeHasId for T where T: MaybeHasNativeId {
//     fn try_raw_id(&self) -> Option<RawObjectId> {
//         self
//             .native_id()
//             .map(String::from)
//             .map(|id| RawObjectId::from_str(&id).expect("expected object ID
// to be parseable"))     }
// }

// #[enum_dispatch]
// pub trait MaybeHasTypedId<T> {
//     /// Object ID of the object, which can be used to efficiently fetch a
//     /// fresh reference to the object on subsequent ticks, or `None` if the
//     /// object doesn't currently have an id.
//     fn try_id(&self) -> Option<ObjectId<T>>;
// }

// impl<T> MaybeHasTypedId<T> for T where T: MaybeHasId {
//     fn try_id(&self) -> Option<ObjectId<T>> {
//         self
//             .try_raw_id()
//             .map(Into::into)
//     }
// }

// impl<T> MaybeHasTypedId<T> for &T where T: MaybeHasId {
//     fn try_id(&self) -> Option<ObjectId<T>> {
//         self
//             .try_raw_id()
//             .map(Into::into)
//     }
// }

// #[enum_dispatch]
// pub trait MaybeHasPosition {
//     /// Position of the object, or `None` if the object is a power creep not
//     /// spawned on the current shard.
//     fn pos(&self) -> Option<Position>;
// }

#[enum_dispatch]
pub trait HasStore {
    /// The store of the object, containing information about the resources it
    /// is holding.
    fn store(&self) -> Store;
}

#[enum_dispatch]
pub trait OwnedStructureProperties {
    /// Returns true for your structure, false for a hostile structure,
    /// undefined for a neutral structure.
    fn my(&self) -> Option<bool>;
}

#[enum_dispatch]
pub trait GameObjectProperties {
    /// Returns true if this object is live in the game at the moment. Check
    /// this property to verify cached or newly created object instances.
    fn exists(&self) -> bool;

    /// The unique ID of this object that you can use in
    /// /game/utils::getObjectById
    fn id(&self) -> JsString;

    /// The X coordinate in the room.
    fn x(&self) -> u8;

    /// The Y coordinate in the room.
    fn y(&self) -> u8;

    /// If defined, then this object will disappear after this number of ticks.
    fn ticks_to_decay(&self) -> Option<u32>;

    fn find_path_to(&self, pos: &Object, options: Option<&FindPathOptions>) -> Array;

    fn find_in_range(&self, positions: &Array, range: u8) -> Array;

    fn find_closest_by_range(&self, positions: &Array) -> Option<Object>;

    fn find_closest_by_path(
        &self,
        positions: &Array,
        options: Option<&FindPathOptions>,
    ) -> Option<Object>;

    fn get_range_to(&self, pos: &Object) -> u8;
}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.transfer`.
///
/// # Contracts
///
/// The reference returned from `AsRef<GameObject>::as_ref` must be a valid
/// target for `Creep.transfer`.
pub trait Transferable: AsRef<GameObject> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.withdraw`.
///
/// # Contracts
///
/// The reference returned from `AsRef<GameObject>::as_ref` must be a valid
/// target for `Creep.withdraw`.
pub trait Withdrawable: AsRef<GameObject> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.attack`.
///
/// # Contracts
///
/// The reference returned from `AsRef<GameObject>::as_ref` must be a valid
/// target for `Creep.attack`.
pub trait Attackable: HasHits + AsRef<GameObject> {}

// NOTE: keep impls for Structure* in sync with accessor methods in
// src/objects/structure.rs

impl Transferable for Creep {}
impl Transferable for StructureContainer {}
impl Transferable for StructureExtension {}
impl Transferable for StructureSpawn {}
impl Transferable for StructureTower {}
#[cfg(feature = "enable-score")]
impl Transferable for ScoreCollector {}

// NOTE: keep impls for Structure* in sync with accessor methods in
// src/objects/structure.rs

impl Withdrawable for StructureExtension {}
impl Withdrawable for StructureContainer {}
impl Withdrawable for StructureSpawn {}
impl Withdrawable for StructureTower {}

// NOTE: keep impls for Structure* in sync with accessor methods in
// src/objects/structure.rs

impl Attackable for Creep {}
impl Attackable for OwnedStructure {}
impl Attackable for StructureContainer {}
impl Attackable for StructureExtension {}
impl Attackable for StructureRampart {}
impl Attackable for StructureRoad {}
impl Attackable for StructureSpawn {}
impl Attackable for StructureTower {}
impl Attackable for StructureWall {}
