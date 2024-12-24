//! Additional constants representing internal game mechanics that aren't
//! included in the game's constants

/// Percentage of energy spent on construction that is list if the construction
/// site is destroyed by being stepped on by a hostile creep.
pub const CONSTRUCTION_SITE_STOMP_RATIO: f32 = 0.5;

/// Hits per creep body part.
pub const CREEP_HITS_PER_PART: u32 = 100;

/// Fatigue points removed per effective move part per tick.
pub const MOVE_POWER: u32 = 2;

/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 1.
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_1: u32 = 10;
/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 2.
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_2: u32 = 4;
/// Hits of damage per effective ranged attack part per
/// [`Creep::ranged_mass_attack`] action at range 3.
///
/// [`Creep::ranged_mass_attack`]: crate::objects::Creep::ranged_mass_attack
pub const RANGED_MASS_ATTACK_POWER_RANGE_3: u32 = 1;

pub const ROOM_WIDTH: u8 = 100;

pub const ROOM_HEIGHT: u8 = 100;
