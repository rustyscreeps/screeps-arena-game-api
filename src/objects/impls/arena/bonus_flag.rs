use crate::{objects::GameObject, prelude::*, Part};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "arena/season_2/power_split/basic/prototypes")]
extern "C" {
    #[wasm_bindgen(js_name = BonusFlag)]
    pub static BONUS_FLAG_PROTOTYPE: Object;

    /// An object that applies an effect of the specified type to all creeps
    /// belonging to the player who captured it.
    #[wasm_bindgen(extends = GameObject)]
    #[derive(Clone)]
    pub type BonusFlag;

    /// Returns `Some(true)` for your flag, `Some(false)` for a hostile flag,
    /// `None` for a neutral flag.
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &BonusFlag) -> Option<bool>;

    /// Returns `Some(true)` for your flag, `Some(false)` for a hostile flag,
    /// `None` for a neutral flag.
    #[wasm_bindgen(method, getter, js_name = bonusType)]
    pub fn bonus_type(this: &BonusFlag) -> Part;
}

impl HasPosition for BonusFlag {
    fn pos(&self) -> Position {
        Position {
            x: self.x(),
            y: self.y(),
        }
    }
}
