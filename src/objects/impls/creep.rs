use crate::{
    constants::{Direction, Part, ResourceType, ReturnCode},
    game::pathfinder::{FindPathOptions, Position},
    objects::{ConstructionSite, GameObject, Resource, Source, Store},
    prelude::*,
};
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "game/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = Creep)]
    pub static CREEP_PROTOTYPE: Object;

    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type Creep;

    /// An array describing the creep’s body.
    #[wasm_bindgen(method, getter = body)]
    fn body_internal(this: &Creep) -> Array;

    /// Fatigue indicator of the creep. It can move only when fatigue equals to
    /// 0.
    #[wasm_bindgen(method, getter)]
    pub fn fatigue(this: &Creep) -> u32;

    /// The current amount of hit points of the creep.
    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &Creep) -> u32;

    /// The maximum amount of hit points of the creep.
    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hits_max(this: &Creep) -> u32;

    /// Whether it is your creep.
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &Creep) -> bool;

    /// An object that contains store contents of this creep.
    #[wasm_bindgen(final, method, getter)]
    pub fn store(this: &Creep) -> Store;

    /// Attack another creep or structure in a short-ranged attack. Requires the
    /// ATTACK body part. The target has to be at an adjacent square to the
    /// creep.
    #[wasm_bindgen(final, method, js_name = attack)]
    fn attack_internal(this: &Creep, target: &GameObject) -> ReturnCode;

    /// Build a structure at the target construction site using carried energy.
    #[wasm_bindgen(final, method)]
    pub fn build(this: &Creep, target: &ConstructionSite) -> ReturnCode;

    /// Drop a resource onto the ground.
    #[wasm_bindgen(final, method)]
    pub fn drop(this: &Creep, ty: ResourceType, amount: Option<u32>) -> ReturnCode;

    /// Collect energy from a source.
    #[wasm_bindgen(final, method, js_name = harvest)]
    pub fn harvest(this: &Creep, target: &Source) -> ReturnCode;

    /// Heal self or another creep. It will restore the target creep’s damaged
    /// body parts function and increase the hits counter. Requires the HEAL
    /// body part. The target has to be at an adjacent square to the creep.
    #[wasm_bindgen(final, method)]
    pub fn heal(this: &Creep, target: &Creep) -> ReturnCode;

    /// Move the creep one square in the specified direction.
    #[wasm_bindgen(final, method, js_name = move)]
    pub fn move_direction(this: &Creep, direction: Direction) -> ReturnCode;

    /// Accept an attempt by another creep to pull this one.
    #[wasm_bindgen(final, method, js_name = move)]
    pub fn move_pulled_by(this: &Creep, target: &Creep) -> ReturnCode;

    /// Find the optimal path to the target within the same room and move to it.
    /// A shorthand to consequent calls of findPathTo() and move() methods.
    /// target can be any object containing x and y properties. opts is an
    /// optional object containing additional options. See /game/utils::findPath
    /// for details.
    #[wasm_bindgen(final, method, js_name = moveTo)]
    pub fn move_to(this: &Creep, target: &JsValue, options: Option<&FindPathOptions>)
        -> ReturnCode;

    /// Take a resource from the ground.
    #[wasm_bindgen(final, method)]
    pub fn pickup(this: &Creep, target: &Resource) -> ReturnCode;

    /// Help another creep to follow this creep. The fatigue generated for the
    /// target's move will be added to the creep instead of the target.
    #[wasm_bindgen(final, method)]
    pub fn pull(this: &Creep, target: &Creep) -> ReturnCode;

    /// A ranged attack against another creep or structure.
    #[wasm_bindgen(final, method, js_name = rangedAttack)]
    fn ranged_attack_internal(this: &Creep, target: &GameObject) -> ReturnCode;

    /// Heal another creep at a distance.
    #[wasm_bindgen(final, method, js_name = rangedHeal)]
    pub fn ranged_heal(this: &Creep, target: &Creep) -> ReturnCode;

    /// A ranged attack against all hostile creeps or structures within 3
    /// squares range.
    #[wasm_bindgen(final, method, js_name = rangedMassAttack)]
    pub fn ranged_mass_attack(this: &Creep) -> ReturnCode;

    /// This Creep attribute is only documented in the typescript typings.
    #[wasm_bindgen(method, getter)]
    pub fn spawning(this: &Creep) -> bool;

    // todo not yet in game but should be like this
    // #[wasm_bindgen(final, method)]
    // pub fn repair(this: &Creep, target: &GameObject) -> ReturnCode;

    /// Transfer resources to a structure or a creep.
    #[wasm_bindgen(final, method, js_name = transfer)]
    fn transfer_internal(
        this: &Creep,
        target: &GameObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> ReturnCode;

    /// Withdraw resources from a structure.
    #[wasm_bindgen(final, method, js_name = withdraw)]
    pub fn withdraw(
        this: &Creep,
        target: &GameObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> ReturnCode;
}

#[wasm_bindgen]
extern "C" {
    /// A [`BodyPart`] of a creep.
    ///
    /// [Screeps documentation](https://docs-ptr.screeps.com/api/#Creep.body)
    #[wasm_bindgen]
    pub type BodyPart;

    #[wasm_bindgen(method, getter = type)]
    pub fn part(this: &BodyPart) -> Part;

    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &BodyPart) -> u32;
}

impl Creep {
    pub fn body(&self) -> Vec<BodyPart> {
        self.body_internal().iter().map(BodyPart::from).collect()
    }

    pub fn attack<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Attackable,
    {
        Self::attack_internal(self, target.as_ref())
    }

    pub fn ranged_attack<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Attackable,
    {
        Self::ranged_attack_internal(self, target.as_ref())
    }

    pub fn transfer<T>(&self, target: &T, ty: ResourceType, amount: Option<u32>) -> ReturnCode
    where
        T: Sized + Transferable,
    {
        Self::transfer_internal(self, target.as_ref(), ty, amount)
    }
}

// impl JsContainerFromValue for Creep {
//     fn from_value(val: JsValue) -> Self {
//         val.unchecked_into()
//     }
// }

impl HasHits for Creep {
    fn hits(&self) -> u32 {
        Self::hits(self)
    }

    fn hits_max(&self) -> u32 {
        Self::hits_max(self)
    }
}

impl HasStore for Creep {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl HasPosition for Creep {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
