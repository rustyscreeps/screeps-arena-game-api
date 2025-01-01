//! Various constants translated as small enums.

use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{convert::Infallible, fmt, str::FromStr};
use wasm_bindgen::prelude::*;
//use crate::constants::find::Find;

// Bindgen does not correctly handle i8 negative return values. Use custom
// return values.
/// Translates return code constants.
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, FromPrimitive, Deserialize_repr, Serialize_repr,
)]
#[repr(i8)]
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
    NoPath = -2,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    InvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    /* RclNotEnough = -14,
     * GclNotEnough = -15, */
}

impl wasm_bindgen::convert::IntoWasmAbi for ReturnCode {
    type Abi = i32;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        (self as i32).into_abi()
    }
}

impl wasm_bindgen::convert::FromWasmAbi for ReturnCode {
    type Abi = i32;

    #[inline]
    unsafe fn from_abi(js: i32) -> Self {
        Self::from_i32(js).unwrap()
    }
}

impl wasm_bindgen::describe::WasmDescribe for ReturnCode {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::I32)
    }
}

impl TryFrom<JsValue> for ReturnCode {
    type Error = String;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        value
            .as_f64()
            .and_then(|f| Self::from_i32(f as i32))
            .ok_or_else(|| "expected number for return code".to_owned())
    }
}

/// Translates direction constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Direction {
    Top = 1,
    TopRight = 2,
    Right = 3,
    BottomRight = 4,
    Bottom = 5,
    BottomLeft = 6,
    Left = 7,
    TopLeft = 8,
}

impl Direction {
    /// Whether the direction is orthogonal.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps_arena::Direction::*;
    ///
    /// assert_eq!(Top.is_orthogonal(), true);
    /// assert_eq!(TopRight.is_orthogonal(), false);
    /// ```
    pub fn is_orthogonal(self) -> bool {
        use Direction::*;

        matches!(self, Top | Right | Bottom | Left)
    }

    /// Whether the direction is diagonal.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps_arena::Direction::*;
    ///
    /// assert_eq!(Top.is_diagonal(), false);
    /// assert_eq!(TopRight.is_diagonal(), true);
    /// ```
    pub fn is_diagonal(self) -> bool {
        !self.is_orthogonal()
    }

    /// Rotate the direction by a specified number of steps clockwise if
    /// positive or counter-clockwise if negative.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps_arena::Direction::*;
    ///
    /// assert_eq!(Top.multi_rot(1), TopRight);
    /// assert_eq!(Top.multi_rot(2), Right);
    /// assert_eq!(Top.multi_rot(-1), TopLeft);
    /// assert_eq!(Top.multi_rot(-2), Left);
    /// assert_eq!(Top.multi_rot(64), Top);
    /// ```
    pub fn multi_rot(self, times: i8) -> Self {
        let raw_dir = ((self as u8) - 1).wrapping_add_signed(times) % 8 + 1;
        // unwrap should be optimized away, as the integer we ended up with
        // is always a valid value
        Self::from_u8(raw_dir).unwrap()
    }

    /// Rotate the direction clockwise by one step.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps_arena::Direction::*;
    ///
    /// assert_eq!(Top.rot_cw(), TopRight);
    /// ```
    pub fn rot_cw(self) -> Self {
        self.multi_rot(1)
    }

    /// Rotate the direction counter-clockwise by one step.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps_arena::Direction::*;
    ///
    /// assert_eq!(Top.rot_ccw(), TopLeft);
    /// ```
    pub fn rot_ccw(self) -> Self {
        self.multi_rot(-1)
    }
}

impl From<Direction> for (i8, i8) {
    /// Returns the change in (x, y) when moving in each direction.
    #[inline]
    fn from(direction: Direction) -> (i8, i8) {
        match direction {
            Direction::Top => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::BottomRight => (1, 1),
            Direction::Bottom => (0, 1),
            Direction::BottomLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::TopLeft => (-1, -1),
        }
    }
}

impl ::std::ops::Neg for Direction {
    type Output = Direction;

    /// Negates this direction. Top goes to Bottom, TopRight goes to BottomLeft,
    /// etc.
    ///
    /// Example usage:
    ///
    /// ```
    /// use screeps_arena::Direction::*;
    ///
    /// assert_eq!(-Top, Bottom);
    /// assert_eq!(-BottomRight, TopLeft);
    /// assert_eq!(-Left, Right);
    /// ```
    #[inline]
    fn neg(self) -> Direction {
        use Direction::*;

        match self {
            Top => Bottom,
            TopRight => BottomLeft,
            Right => Left,
            BottomRight => TopLeft,
            Bottom => Top,
            BottomLeft => TopRight,
            Left => Right,
            TopLeft => BottomRight,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match *self {
            Direction::Top => "↑",
            Direction::TopRight => "↗",
            Direction::Right => "→",
            Direction::BottomRight => "↘",
            Direction::Bottom => "↓",
            Direction::BottomLeft => "↙",
            Direction::Left => "←",
            Direction::TopLeft => "↖",
        };
        f.write_str(ch)
    }
}

/// Translates `TERRAIN_*` constants.
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    IntoEnumIterator,
)]
#[repr(u8)]
pub enum Terrain {
    // There's no constant for plains, but the absense of a terrain value indicates a plain
    Plain = 0,
    // TERRAIN_MASK_WALL
    Wall = 1,
    // TERRAIN_MASK_SWAMP
    Swamp = 2,
    /* TERRAIN_MASK_LAVA, unimplemented in game
     * Lava = 4, */
}

/// Translates body part constants.
#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Serialize)]
pub enum Part {
    Move = "move",
    Work = "work",
    Carry = "carry",
    Attack = "attack",
    RangedAttack = "ranged_attack",
    Tough = "tough",
    Heal = "heal",
    //Claim = "claim",
}

impl Part {
    /// Translates the `BODYPART_COST` constant.
    #[inline]
    pub fn cost(self) -> u32 {
        match self {
            Part::Move => 50,
            Part::Work => 100,
            Part::Carry => 50,
            Part::Attack => 80,
            Part::RangedAttack => 150,
            Part::Tough => 10,
            Part::Heal => 250,
            //Part::Claim => 600,
            // I guess bindgen is adding a `#[non_exhaustive]` onto the enum and forcing us to do
            // this:
            _ => 0,
        }
    }

    // /// Helper function for deserializing from a string rather than a fake
    // /// integer value.
    // pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self,
    // D::Error> {     let s: Cow<'de, str> = Cow::deserialize(d)?;
    //     Self::from_str(&s).map_err(|_| {
    //         D::Error::invalid_value(
    //             Unexpected::Str(&s),
    //             &"a known constant string in BODYPARTS_ALL",
    //         )
    //     })
    // }
}

impl FromStr for Part {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "move" => Ok(Part::Move),
            "work" => Ok(Part::Work),
            "carry" => Ok(Part::Carry),
            "attack" => Ok(Part::Attack),
            "ranged_attack" => Ok(Part::RangedAttack),
            "tough" => Ok(Part::Tough),
            "heal" => Ok(Part::Heal),
            //"claim" => Ok(Part::Claim),
            _ => panic!("unknown part type"),
        }
    }
}
